//! Compute@Edge static content starter kit program.

use fastly::http::StatusCode;
use fastly::{Error, Request, Response};

/// The name of a backend server associated with this service.
///
/// This should be changed to match the name of your own backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
const BACKEND_NAME: &str = "bucket_host";

const BUCKET_HOST: &str = "storage.googleapis.com";
const BUCKET_NAME: &str = "betts-gcp-gcs-fastly-tutorial";

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
    format!("/{}{}index.html", BUCKET_NAME, original_path)
  } else {
    format!("/{}{}", BUCKET_NAME, original_path)
  };

  req.set_path(&path);
  req.set_header("Host", BUCKET_HOST);

  let mut bereq = copy_request(&req);
  let mut beresp = bereq.send(BACKEND_NAME)?;

  if beresp.get_status() == StatusCode::NOT_FOUND {
    bereq = copy_request(&req);
    bereq.set_path(&format!("{}/index.html", bereq.get_path()));
    beresp = bereq.send(BACKEND_NAME)?;
  }

  if beresp.get_status() == StatusCode::NOT_FOUND {
    bereq = copy_request(&req);
    bereq.set_path(format!("/{}/404.html", BUCKET_NAME).as_str());
    beresp = bereq.send(BACKEND_NAME)?;
  }

  return Ok(beresp);
}

fn copy_request(req: &Request) -> Request {
  let mut new = Request::new(req.get_method(), req.get_url());
  req.get_header_names().for_each(|h| new.set_header(h, req.get_header(h).unwrap()));
  return new;
}