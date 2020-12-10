use fastly::http::header::HeaderName;
use fastly::http::header::{CONTENT_LENGTH, CONTENT_TYPE, DATE};

/// This should be changed to match the name of your storage backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
pub(crate) const BACKEND_NAME: &str = "google_storage";

/// Allowlist of headers for responses to the client.
pub(crate) static ALLOWED_HEADERS: [HeaderName; 3] = [CONTENT_LENGTH, CONTENT_TYPE, DATE];

/// The name of the bucket to serve content from. By default, this is an example bucket on a mock endpoint.
///
/// This example bucket is accessible to anyone who is authenticated with a service account,
/// so you can try this demo with your own credentials.
pub(crate) const BUCKET_NAME: &str = "fastly-demo-auth-reqd";

/// The host that the bucket is served on. This is used to make requests to the backend.
pub(crate) const BUCKET_HOST: &str = "storage.googleapis.com";

/// The storage service to use. `s3` for S3 or `storage` for GCS.
pub(crate) const BUCKET_SERVICE: &str = "storage";

/// The storage service region to use.
pub(crate) const BUCKET_REGION: &str = "auto";

/// Access key ID for the storage service.
/// Generate this with `$ gsutil hmac create <service account email>`
pub(crate) const BUCKET_ACCESS_KEY_ID: &str = "GOOG1E...<access key>";

/// Secret access key for the storage service.
/// Generated alongside the access key ID.
pub(crate) const BUCKET_SECRET_ACCESS_KEY: &str = "<secret key>";
