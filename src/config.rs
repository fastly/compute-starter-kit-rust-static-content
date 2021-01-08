use fastly::http::header::HeaderName;
use fastly::http::header::{CONTENT_LENGTH, CONTENT_TYPE, DATE};

/// This should be changed to match the name of your storage backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
pub(crate) const BACKEND_NAME: &str = "mock-s3.edgecompute.app";

/// Allowlist of headers for responses to the client.
pub(crate) static ALLOWED_HEADERS: [HeaderName; 3] = [CONTENT_LENGTH, CONTENT_TYPE, DATE];

/// The name of the bucket to serve content from. By default, this is an example bucket on a mock endpoint.
pub(crate) const BUCKET_NAME: &str = "mock-s3";

/// The host that the bucket is served on. This is used to make requests to the backend.
pub(crate) const BUCKET_HOST: &str = "edgecompute.app";

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

/// The regular expression to use when looking for assets to preload on a page.
pub(crate) const ASSET_REGEX: &str = r#"\s(?:src|href)="(/assets/.+?)""#;

/// Define a Content Security Policy for content that can load on your site.
pub(crate) const CONTENT_SECURITY_POLICY: &str = "default-src 'self'; style-src 'self' fonts.googleapis.com; font-src fonts.gstatic.com";
