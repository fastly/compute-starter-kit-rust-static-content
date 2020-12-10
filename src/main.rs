//! Compute@Edge static content starter kit program.

mod config;
mod awsv4;

use chrono::Utc;
use fastly::{Body, Error, Request, Response, http::header::AUTHORIZATION};
use fastly::http::header::{
  ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
  ACCESS_CONTROL_MAX_AGE, ACCESS_CONTROL_REQUEST_HEADERS, ACCESS_CONTROL_REQUEST_METHOD,
  CACHE_CONTROL, ORIGIN, CONTENT_SECURITY_POLICY, X_FRAME_OPTIONS,
  CONTENT_TYPE, STRICT_TRANSPORT_SECURITY, REFERRER_POLICY, LOCATION
};
use fastly::http::{StatusCode, HeaderValue, header::HeaderName, Method};
use crate::awsv4::hash;

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

  // Respond to CORS preflight requests.
  if req.get_method() == Method::OPTIONS && req.get_header(ORIGIN).is_some()
    && (req.get_header(ACCESS_CONTROL_REQUEST_HEADERS).is_some() || req.get_header(ACCESS_CONTROL_REQUEST_METHOD).is_some()) {
    return Ok(create_cors_response(allowed_origins));
  }

  // Respond to requests for robots.txt.
  if req.get_path() == "/robots.txt" {
    return Ok(Response::from_body("User-agent: *\nAllow: /").with_header(CONTENT_TYPE, "text/plain"));
  }

  // Append index.html if path is a directory.
  if req.get_path().ends_with('/') {
    req.set_path(&format!("{}index.html", req.get_path()));
  }

  // Assign the path to a variable to be used later.
  let original_path = req.get_path().to_owned();

  // Set the `Host` header to the bucket host rather than our C@E endpoint.
  req.set_header("Host", format!("{}.{}", config::BUCKET_NAME, config::BUCKET_HOST));

  // Copy the modified client request to create a backend request.
  let mut bereq = copy_request(&req);

  // Authenticate the initial request to the origin.
  set_authentication_headers(&mut bereq);

  // Send the request to the backend and assign its response to `beresp`.
  let mut beresp = bereq.send(config::BACKEND_NAME)?;

  // If backend response is 404, try for index.html
  if is_not_found(&beresp) && !original_path.ends_with("index.html") {
    // Copy the original request and append index.html to the path.
    bereq = copy_request(&req);
    bereq.set_path(&format!("{}/index.html", original_path));

    // Send the request to the backend.
    set_authentication_headers(&mut bereq);
    beresp = bereq.send(config::BACKEND_NAME)?;

    // If file exists, trigger redirect with `/` appended to path.
    // This means the canonical URL for index pages will always end with a trailing slash.
    if !is_not_found(&beresp) {
      beresp = Response::new().with_status(StatusCode::MOVED_PERMANENTLY).with_header(LOCATION, &format!("{}/", original_path));
      return Ok(beresp);
    }
  }

  // If backend response is still 404, serve the 404.html file from the bucket.
  if is_not_found(&beresp) {
    // Copy the original request and replace the path with /404.html.
    bereq = copy_request(&req);
    bereq.set_path("/404.html");

    // Send the request to the backend.
    set_authentication_headers(&mut bereq);
    beresp = bereq.send(config::BACKEND_NAME)?;
  }

  filter_headers(&mut beresp);

  // Apply referrer-policy and HSTS to HTML pages.
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

  // Apply Access-Control-Allow-Origin to allow cross-origin resource sharing.
  beresp.set_header(ACCESS_CONTROL_ALLOW_ORIGIN, allowed_origins);

  // Set Content-Security-Policy header to prevent loading content from other origins.
  beresp.set_header(CONTENT_SECURITY_POLICY, "default-src 'self'; style-src 'self' fonts.googleapis.com; font-src fonts.gstatic.com");

  // Set X-Frame-Options header to prevent other origins embedding the site.
  beresp.set_header(X_FRAME_OPTIONS, "SAMEORIGIN");

  // Return the backend response to the client.
  Ok(beresp)
}

/// Determines if a backend response indicates the requested file not existing.
fn is_not_found(resp: &Response) -> bool {
  resp.get_status() == StatusCode::NOT_FOUND || resp.get_status() == StatusCode::FORBIDDEN
}

/// Sets authentication headers for a given request.
fn set_authentication_headers(req: &mut Request) {
  let now = Utc::now();
  let sig = awsv4::aws_v4_auth("",  req.get_method().as_str(), req.get_path(), now);
  req.set_header(AUTHORIZATION, sig);
  req.set_header("x-amz-content-sha256", hash("".to_string()));
  req.set_header("x-amz-date", now.format("%Y%m%dT%H%M%SZ").to_string());
}

/// Removes all headers but those defined in `ALLOWED_HEADERS` from a response.
fn filter_headers(resp: &mut Response) {
  let mut to_remove: Vec<HeaderName> = Vec::new();
  for header in resp.get_header_names() {
    if !config::ALLOWED_HEADERS.contains(header) {
      to_remove.push(header.clone());
    }
  }
  for header in to_remove {
    resp.remove_header(header);
  }
}

/// Create a response to a CORS preflight request.
fn create_cors_response(allowed_origins: HeaderValue) -> Response {
  Response::from_body(Body::new())
    .with_status(StatusCode::NO_CONTENT)
    .with_header(ACCESS_CONTROL_ALLOW_ORIGIN, allowed_origins)
    .with_header(ACCESS_CONTROL_ALLOW_METHODS, "GET,HEAD,POST,OPTIONS")
    .with_header(ACCESS_CONTROL_MAX_AGE, "86400")
    .with_header(CACHE_CONTROL, "public, max-age=86400")
}

/// Create a copy of a request with the same method, URL, and headers.
fn copy_request(req: &Request) -> Request {
  let mut new = Request::new(req.get_method(), req.get_url());
  req.get_header_names().for_each(|h| new.set_header(h, req.get_header(h).unwrap()));
  new
}