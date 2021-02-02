use chrono::{Utc, DateTime};
use hmac_sha256::{Hash, HMAC};

use crate::config::{BUCKET_HOST, BUCKET_NAME, BUCKET_REGION, BUCKET_SERVICE};

/// SHA256 HMAC
fn sign<K: AsRef<[u8]>, I: AsRef<[u8]>>(key: K, input: I) -> [u8; 32] {
    HMAC::mac(input.as_ref(), key.as_ref())
}

/// Create a hex output of the hash
pub fn hash(input: String) -> String {
    hex::encode(Hash::hash(input.as_bytes()))
}

pub struct SignatureClient {
    pub access_key_id: String,
    pub secret_access_token: String,
}

impl SignatureClient {
    /// Generate an AWSv4 signature for a given request.
    /// Requests with payloads are not supported.
    pub fn aws_v4_auth(&self, method: &str, path: &str, now: DateTime<Utc>) -> String {
        let amz_content_256 = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"; // empty hash
        let x_amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();
        let x_amz_today = now.format("%Y%m%d").to_string();

        // The spec says we should urlencode everything but the `/`
        // The path is already urlencoded but potentially not in the
        // canonical format, so we normalize it.
        let raw_path = urlencoding::decode(path).unwrap();
        let encoded_path = urlencoding::encode(&raw_path);
        let final_encoded_path = encoded_path.replace("%2F", "/");

        // These must be sorted alphabetically
        let canonical_headers = format!(
            "host:{}\nx-amz-content-sha256:{}\nx-amz-date:{}\n",
            format!("{}.{}", BUCKET_NAME, BUCKET_HOST),
            amz_content_256,
            x_amz_date
        );

        let canonical_query = "";

        // These must be alphabetic
        let signed_headers = "host;x-amz-content-sha256;x-amz-date";

        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            method,
            final_encoded_path,
            canonical_query,
            canonical_headers,
            signed_headers,
            amz_content_256
        );

        let credential_scope = format!(
            "{}/{}/{}/aws4_request",
            x_amz_today, BUCKET_REGION, BUCKET_SERVICE
        );

        let signed_canonical_request = hash(canonical_request);

        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            x_amz_date, credential_scope, signed_canonical_request
        );

        // Generate the signature through the multi-step signing process
        let signature = [
            BUCKET_REGION,
            BUCKET_SERVICE,
            "aws4_request",
            &string_to_sign,
        ]
        .iter()
        .fold(
            sign(&format!("AWS4{}", self.secret_access_token), &x_amz_today),
            |acc, x| sign(&acc, x),
        );

        // Compose authorization header value
        format!(
            "AWS4-HMAC-SHA256 Credential={}/{},SignedHeaders={},Signature={}",
            self.access_key_id,
            credential_scope,
            signed_headers,
            hex::encode(signature)
        )
    }
}
