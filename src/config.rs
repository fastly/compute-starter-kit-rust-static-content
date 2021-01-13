use fastly::http::header::HeaderName;
use fastly::http::header::{CONTENT_LENGTH, CONTENT_TYPE};

/// This should be changed to match the name of your storage backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
pub(crate) const BACKEND_NAME: &str = "mock-s3.edgecompute.app";

/// Allowlist of headers for responses to the client.
pub(crate) static ALLOWED_HEADERS: [HeaderName; 2] = [CONTENT_LENGTH, CONTENT_TYPE];

/// The name of the bucket to serve content from. By default, this is an example bucket on a mock endpoint.
pub(crate) const BUCKET_NAME: &str = "mock-s3";

/// The host that the bucket is served on. This is used to make requests to the backend.
pub(crate) const BUCKET_HOST: &str = "edgecompute.app";

/// The storage service to use. `s3` for S3 or `storage` for GCS.
pub(crate) const BUCKET_SERVICE: &str = "storage";

/// The storage service region to use.
pub(crate) const BUCKET_REGION: &str = "auto";

// If auth is required, configure your access keys in an edge dictionary named "bucket_auth":
/// access_key_id
/// secret_access_key

/// Define a Content Security Policy for content that can load on your site.
pub(crate) const CONTENT_SECURITY_POLICY: &str =
    "default-src 'self'; style-src 'self' fonts.googleapis.com; font-src fonts.gstatic.com";
