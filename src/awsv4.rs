use chrono::DateTime;
use chrono::Utc;
use hmac_sha256::{Hash, HMAC};

use crate::config::{
    BUCKET_ACCESS_KEY_ID, BUCKET_HOST, BUCKET_NAME, BUCKET_REGION, BUCKET_SECRET_ACCESS_KEY,
    BUCKET_SERVICE,
};

/// SHA256 HMAC
fn sign(key: Vec<u8>, input: String) -> [u8; 32] {
    HMAC::mac(input.as_bytes(), &key)
}

/// Create a hex output of the hash
pub fn hash(input: String) -> String {
    hex::encode(Hash::hash(input.as_bytes()))
}

/// Generate an AWSv4 signature for a given request.
pub fn aws_v4_auth(payload: &str, method: &str, path: &str, now: DateTime<Utc>) -> String {
    let amz_content_256 = hash(payload.to_string());
    let x_amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();
    let x_amz_today = now.format("%Y%m%d").to_string();

    // The spec says we should urlencode everything but the `/`
    let encoded_path: String = urlencoding::encode(path);
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

    let scope = format!(
        "{}/{}/{}/aws4_request",
        x_amz_today, BUCKET_REGION, BUCKET_SERVICE
    );

    let signed_canonical_request = hash(canonical_request);

    let string_to_sign = format!(
        "AWS4-HMAC-SHA256\n{}\n{}\n{}",
        x_amz_date, scope, signed_canonical_request
    );

    // Generate the signature through the multi-step signing process
    let k_secret = format!("AWS4{}", &BUCKET_SECRET_ACCESS_KEY);
    let k_date = sign(k_secret.as_bytes().to_vec(), x_amz_today);
    let k_region = sign(k_date.to_vec(), BUCKET_REGION.to_string());
    let k_service = sign(k_region.to_vec(), BUCKET_SERVICE.to_string());
    let k_signing = sign(k_service.to_vec(), "aws4_request".to_string());

    // Final signature
    let signature = hex::encode(sign(k_signing.to_vec(), string_to_sign));

    // Generate the Authorization header value
    format!(
        "AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
        BUCKET_ACCESS_KEY_ID, scope, signed_headers, signature
    )
}
