//! Compute@Edge static content starter kit program.

use fastly::{Error, Request, Response, Body};
use fastly::http::header::{
  ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
  ACCESS_CONTROL_MAX_AGE, ACCESS_CONTROL_REQUEST_HEADERS, ACCESS_CONTROL_REQUEST_METHOD,
  CACHE_CONTROL, ORIGIN, CONTENT_SECURITY_POLICY, X_FRAME_OPTIONS, CONTENT_LENGTH,
  CONTENT_TYPE, DATE, STRICT_TRANSPORT_SECURITY, REFERRER_POLICY
};
use fastly::http::{StatusCode, HeaderValue, header::HeaderName, Method};

/// The name of a backend server associated with this service.
///
/// This should be changed to match the name of your own backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
const BACKEND_NAME: &str = "bucket_host";

/// The name of the bucket to serve content from. By default, this is an example bucket on a mock endpoint.
const BUCKET_NAME: &str = "example-bucket";

/// The host that the bucket is served on. This is used to make requests to the backend.
const BUCKET_HOST: &str = "mock-s3.edgecompute.app";

/// Allowlist of headers for responses to the client.
const ALLOWED_HEADERS: [HeaderName; 3] = [CONTENT_LENGTH, CONTENT_TYPE, DATE];

/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.
#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
  // Used later to generate CORS headers.
  // Usually you would want an allowlist of domains here, but this example allows any origin to make requests.
  let allowed_origins = match req.get_header("origin") {
    Some(val) => val.clone(),
    _ => HeaderValue::from_str("*").unwrap(),
  };

  // Respond to CORS preflight requests
  if req.get_method() == Method::OPTIONS && req.get_header(ORIGIN).is_some()
    && (req.get_header(ACCESS_CONTROL_REQUEST_HEADERS).is_some() || req.get_header(ACCESS_CONTROL_REQUEST_METHOD).is_some()) {
    return Ok(Response::from_body(Body::new())
        .with_status(StatusCode::NO_CONTENT)
        .with_header(ACCESS_CONTROL_ALLOW_ORIGIN, allowed_origins)
        .with_header(ACCESS_CONTROL_ALLOW_METHODS, "GET,HEAD,POST,OPTIONS")
        .with_header(ACCESS_CONTROL_MAX_AGE, "86400")
        .with_header(CACHE_CONTROL, "public, max-age=86400")
    );
  }

  // Store a reference to the original request path
  let original_path = req.get_path();

  let path = if original_path.ends_with('/') {
    // If the path ends with a separator, prepend bucket name and append index.html.
    format!("/{}{}index.html", BUCKET_NAME, original_path)
  } else {
    // Otherwise just prepend the bucket name.
    format!("/{}{}", BUCKET_NAME, original_path)
  };
  req.set_path(&path);

  // Set the `Host` header to the bucket host rather than our C@E endpoint.
  req.set_header("Host", BUCKET_HOST);

  // Authenticate the request to the origin. TODO: AwsV4
  req.set_header("Authorization", "Bearer letmein");

  // Copy the modified client request to create a backend request.
  let mut bereq = copy_request(&req);

  // Send the request to the backend and assign its response to `beresp`.
  let mut beresp = bereq.send(BACKEND_NAME)?;

  // If backend response is 404, try for index.html
  if (beresp.get_status() == StatusCode::NOT_FOUND || beresp.get_status() == StatusCode::FORBIDDEN) && !path.ends_with("index.html") {
    // Copy the original request and append index.html.
    bereq = copy_request(&req);
    bereq.set_path(&format!("{}/index.html", bereq.get_path()));

    // Send the request to the backend.
    beresp = bereq.send(BACKEND_NAME)?;
  }

  // If backend response is still 404, serve the 404.html file from the bucket.
  if beresp.get_status() == StatusCode::NOT_FOUND || beresp.get_status() == StatusCode::FORBIDDEN {
    // Copy the original request and replace the path with /index.html.
    bereq = copy_request(&req);
    bereq.set_path(format!("/{}/404.html", BUCKET_NAME).as_str());

    // Send the request to the backend.
    beresp = bereq.send(BACKEND_NAME)?;
  }

  filter_headers(&mut beresp);

  // Apply referrer-policy and HSTS to HTML pages
  if let Some(header) = beresp.get_header("content-type") {
    if header.to_str().unwrap().starts_with("text/html") {
      beresp.set_header(
        REFERRER_POLICY,
        "origin-when-cross-origin",
      );
      beresp.set_header(
          STRICT_TRANSPORT_SECURITY,
          "max-age=2592000",
      );
    }
  }

  // Apply Access-Control-Allow-Origin to allow cross-origin resource sharing
  beresp.set_header(ACCESS_CONTROL_ALLOW_ORIGIN, allowed_origins);

  // Set Content-Security-Policy header to prevent loading content from other origins
  beresp.set_header(CONTENT_SECURITY_POLICY, "default-src 'self';");

  // Set X-Frame-Options header to prevent other origins embedding the site
  beresp.set_header(X_FRAME_OPTIONS, "SAMEORIGIN");

  // Return the backend response to the client.
  return Ok(beresp);
}

fn filter_headers(resp: &mut Response) {
  let mut to_remove: Vec<HeaderName> = Vec::new();
  for header in resp.get_header_names() {
    if !ALLOWED_HEADERS.contains(header) {
      to_remove.push(header.clone());
    }
  }
  for header in to_remove {
    resp.remove_header(header);
  }
}

fn copy_request(req: &Request) -> Request {
  let mut new = Request::new(req.get_method(), req.get_url());
  req.get_header_names().for_each(|h| new.set_header(h, req.get_header(h).unwrap()));
  return new;
}