# Compute@Edge static content starter kit for Rust

Speed up your websites with a Compute@Edge environment that demonstrates serving content from a static bucket, redirects, security and performance headers, and a 404 page.

**For more details about this and other starter kits for Compute@Edge, see the [Fastly developer hub](https://developer.fastly.com/solutions/starters)**

## Features

 * Prefix the backend bucket hostname with your bucket prefix
 * Serve a 404 page if the requested page is not found
 * Remove extra headers sent by your storage provider, such as `x-goog-*`
 * Add Content Security Policy and other security-related headers
 * Respond to CORS preflight requests
 * Redirect requests for directories to index.html
 * Serve robots.txt
 * Authenticate requests to the origin with AWS Signature Version 4
 * Add caching policy to content
 * Strip query strings
 * Add `Link: rel=preload` header to pre-fetch critical assets like fonts

## Usage

When deploying your project, the Fastly CLI (`v1.1.0`) will prompt you to enter a `Backend`. You can enter your bucket host here, or just enter `mock-s3.edgecompute.app` on port `443` if you want to experiment with our mock backend.

```
Backend (hostname or IP address, or leave blank to stop adding backends): mock-s3.edgecompute.app
Backend port number: [80] 443
Backend name: [backend_1] bucket_origin
```

If your content is already in a bucket which is public to the internet, or in a private bucket which supports AWSv4-compatible authentication, you can get started right away by modifying `src/config.rs`. The values you will need to set are:

 * `BACKEND_NAME` - This should match the name of your storage backend in the Fastly UI.
 * `BUCKET_NAME` - The name of the bucket you want to access.
 * `BUCKET_HOST` - The hostname of the storage service, e.g. `storage.googleapis.com`, excluding your bucket prefix.

For private buckets, set these values also:

 * `BUCKET_SERVICE` - The service, as defined in your provider's AWSv4 docs, that you are using. `s3` for S3 or `storage` for GCS.
 * `BUCKET_REGION` - The region, as defined in your provider's AWSv4 docs, that you are using. `auto` is fine for GCS.

Optionally, you can update these values to configure the default functionality of the starter kit:

 * `ALLOWED_HEADERS` - The headers that you want to allow from the origin to be passed to the user. This means headers such as `x-goog-metadata` will be removed by default.
 * `ASSET_REGEX` - The regex used to determine assets to be preloaded for a given response body. Defaults to files in `/assets/`.
 * `CONTENT_SECURITY_POLICY` - The value of the `Content-Security-Policy` header used to determine origins that resources can be loaded from.

If your bucket requires authentication, you will need to create an [edge dictionary](https://docs.fastly.com/en/guides/about-edge-dictionaries) named `bucket_auth` with the following values:

 * `access_key_id` - The HMAC access key ID for your service account with read access.
 * `secret_access_key` - The HMAC secret key for your service account with read access.

 In addition to this, you will need to update the `Cargo.toml` to replace `default = []` with `default = ["auth"]`. This will include the dependencies required to generate signed requests in your package.

## Understanding the code

This starter is feature-packed, and requires some extra dependencies on top of the [`fastly`](https://docs.rs/fastly) crate to handle signing requests for S3/GCS. If you are using a public bucket for your origin, these dependencies will not be included.

This starter includes implementations of common patterns explained in our [using Compute@Edge](/learning/compute/rust/) and [VCL migration](/learning/compute/migrate/) guides. Any of the code you see here can be modified or built upon to suit your project's needs.

To learn more about how Fastly communicates with your bucket host, read the [Integrating third party services as backends](https://developer.fastly.com/learning/integrations/backends/) guide. This is based on a VCL service, but you can utilise many of the same patterns with the help of the [VCL migration](/learning/compute/migrate/) guide.

## Security issues

Please see our [SECURITY.md](SECURITY.md) for guidance on reporting security-related issues.
