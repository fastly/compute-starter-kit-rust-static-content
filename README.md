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
- [x] Serve robots.txt
- [x] Authenticate requests to the origin with AWS Signature Version 4
- [x] Add caching policy to content
- [x] Strip query strings
- [x] Add `Link: rel=preload` header to pre-fetch JavaScript and CSS

## Usage

If your content is already in a bucket which is public to the internet, or in a private bucket which supports AWSv4-compatible authetication, you can get started right away by modifying `src/config.rs`. The values you will need to set are:

 * `BACKEND_NAME` - This should match the name of your storage backend in the Fastly UI
 * `BUCKET_NAME` - The name of the bucket you want to access
 * `BUCKET_HOST` - The hostname of the storage service, e.g. `storage.googleapis.com`

For authenticated buckets, set these values too:

 * `BUCKET_SERVICE` - The service, as defined in your provider's AWSv4 docs, that you are using. `s3` for S3 or `storage` for GCS.
 * `BUCKET_REGION` - The region, as defined in your provider's AWSv4 docs, that you are using. `auto` is fine for GCS.
 * `BUCKET_ACCESS_KEY_ID` - The HMAC access key ID for your service account with read access
 * `BUCKET_SECRET_ACCESS_KEY` - The HMAC secret key for your service account with read access

Optionally, you can update these values to configure the default functionality of the starter kit:

 * `ALLOWED_HEADERS` - The headers that you want to allow from the origin to be passed to the user
 * `ASSET_REGEX` - The regex used to determine assets to be preloaded for a given response body. Defaults to files in `/assets/`.
 * `CONTENT_SECURITY_POLICY` - The value of the `Content-Security-Policy` header used to determine origins that resources can be loaded from.

If your bucket doesn't require authentication, make sure to modify the `set_authentication_headers` function in `src/main.rs` to skip the generation of the AWSv4 signature.

## Understanding the code

This starter is feature-packed, and requires some extra dependencies dependencies on top of the [`fastly`](https://docs.rs/fastly) crate to handle signing requests for S3/GCS. If you are using a public bucket for your origin, you can remove these dependencies, the `awsv4.rs` file, and modify the `set_authentication_headers` method to reduce the size of your binary.

This starter includes implementations of common patterns explained in our [using Compute@Edge](/learning/compute/using/) and [VCL migration](/learning/compute/migrate) guides. Any of the code you see here can be modified or built upon to suit your project's needs.

## Security issues

Please see our [SECURITY.md](SECURITY.md) for guidance on reporting security-related issues.
