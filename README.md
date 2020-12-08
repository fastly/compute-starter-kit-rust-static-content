# Compute@Edge static content starter kit for Rust

Speed up your websites with a Compute@Edge environment that demonstrates serving content from a static bucket, redirects, security and performance headers, and a 404 page.

**For more details about this and other starter kits for Compute@Edge, see the [Fastly developer hub](https://developer.fastly.com/solutions/starters)**

## Features

- [x] Prefix the backend bucket hostname with your bucket prefix
- [x] Serve a 404 page if the requested page is not found
- [x] Remove extra headers sent by your storage provider, such as `x-goog-*`
- [x] Add Content Security Policy and other security-related headers
- [x] Respond to CORS preflight requests
- [x] Redirect requests for directories to index.html
- [ ] Authenticate requests to the origin with AWS Signature Version 4
- [ ] Add caching policy to content
- [ ] Add `Link: rel=preload` header to pre-fetch JavaScript and CSS
- [ ] Normalize query strings

## Understanding the code

This starter is intentionally lightweight, and requires no dependencies aside from the [`fastly`](https://docs.rs/fastly) crate. It will help you understand the basics of processing requests at the edge using Fastly. This starter includes implementations of common patterns explained in our [using Compute@Edge](/learning/compute/using/) and [VCL migration](/learning/compute/migrate) guides.

## Security issues

Please see our [SECURITY.md](SECURITY.md) for guidance on reporting security-related issues.