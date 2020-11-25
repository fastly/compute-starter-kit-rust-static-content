//! Compute@Edge static content starter kit program.

use fastly::{Error, http::HeaderValue, Request, Response};
use fastly::http::StatusCode;

/// The name of a backend server associated with this service.
///
/// This should be changed to match the name of your own backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
const BACKEND_NAME: &str = "bucket_host";

/// The name of the bucket to serve content from. By default, this is an example bucket hosted on GCS.
const BUCKET_NAME: &str = "betts-gcp-gcs-fastly-tutorial";

/// The host that the bucket is served on. This is used to make requests to the backend.
const BUCKET_HOST: &str = "storage.googleapis.com";

const ALLOWED_HEADERS: [&str; 3] = ["content-length", "content-type", "date"];

/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.
#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
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

  // Used later to generate CORS headers.
  let origin = req.get_header("origin");

  // Copy the modified client request to create a backend request.
  let mut bereq = copy_request(&req);

  // Send the request to the backend and assign its response to `beresp`.
  let mut beresp = bereq.send(BACKEND_NAME)?;

  // If backend response is 404, try for index.html
  if beresp.get_status() == StatusCode::NOT_FOUND && !path.ends_with("index.html") {
    // Copy the original request and append index.html.
    bereq = copy_request(&req);
    bereq.set_path(&format!("{}/index.html", bereq.get_path()));

    // Send the request to the backend.
    beresp = bereq.send(BACKEND_NAME)?;
  }

  // If backend response is still 404, serve the 404.html file from the bucket.
  if beresp.get_status() == StatusCode::NOT_FOUND {
    // Copy the original request and replace the path with /index.html.
    bereq = copy_request(&req);
    bereq.set_path(format!("/{}/404.html", BUCKET_NAME).as_str());

    // Send the request to the backend.
    beresp = bereq.send(BACKEND_NAME)?;
  }

  // Remove extraneous headers from the backend response.
  let mut to_remove: Vec<String> = Vec::new();
  beresp.get_header_names().map(|n| n.to_string()).for_each(|name| {
    if !ALLOWED_HEADERS.contains(&name.to_lowercase().as_str()) {
      to_remove.push(name);
    }
  });
  to_remove.iter().for_each(|h| {
    beresp.remove_header(h);
  });

  // Apply referrer-policy and HSTS to HTML pages
  if let Some(header) = beresp.get_header("content-type") {
    if header.to_str().unwrap().starts_with("text/html") {
      beresp.set_header(
        "referrer-policy",
        "origin-when-cross-origin",
      );
      beresp.set_header(
          "strict-transport-security",
          "dmax-age=3600",
      );
    }
  }

  // Apply Access-Control-Allow-Origin to allow cross-origin resource sharing
  // Usually you would want a whitelist of domains here, but this example allows any origin to make requests.
  let allowed_origins = match origin {
    Some(val) => val.clone(),
    _ => HeaderValue::from_str("*").unwrap(),
  };
  beresp.set_header("access-control-allow-origin", allowed_origins);

  // Return the backend response to the client.
  return Ok(beresp);
}

fn copy_request(req: &Request) -> Request {
  let mut new = Request::new(req.get_method(), req.get_url());
  req.get_header_names().for_each(|h| new.set_header(h, req.get_header(h).unwrap()));
  return new;
}