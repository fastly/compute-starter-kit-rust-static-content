# Compute@Edge static content starter kit for Rust

Speed up your websites with a Compute@Edge environment that demonstrates serving content from a static bucket, redirects, security and performance headers, and a 404 page.

**For more details about this and other starter kits for Compute@Edge, see the [Fastly developer hub](https://developer.fastly.com/solutions/starters)**

## Features

* Authenticate requests to the origin
* Prefix the backend request path with your bucket prefix
* Redirect directory requests to index.html
* Serve a 404 page if the requested page is not found
* Remove extra headers sent by your storage provider, such as `x-goog-*`
* Add caching policy to content
* Normalize query strings
* Add Content Security Policy and other security-related headers
* Respond to CORS preflight requests
* Add `Link: rel=preload` header to pre-fetch JavaScript and CSS

## Understanding the code

This starter is intentionally lightweight, and requires no dependencies aside from the [`fastly`](https://docs.rs/fastly) crate. It will help you understand the basics of processing requests at the edge using Fastly. This starter includes implementations of common patterns explained in our [using Compute@Edge](/learning/compute/using/) and [VCL migration](/learning/compute/migrate) guides.

## Security issues

Please see our [SECURITY.md](SECURITY.md) for guidance on reporting security-related issues.