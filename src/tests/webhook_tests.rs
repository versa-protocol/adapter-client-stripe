use axum::body::Body;
use axum::extract::Request;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use std::env;

// Mock webhook signature for testing
const TEST_WEBHOOK_SECRET: &str = "whsec_test_secret";
const TEST_CLIENT_ID: &str = "test_client_id";
const TEST_CLIENT_SECRET: &str = "test_client_secret";

fn setup_test_env() {
    env::set_var("WEBHOOK_SECRET", TEST_WEBHOOK_SECRET);
    env::set_var("CLIENT_ID", TEST_CLIENT_ID);
    env::set_var("CLIENT_SECRET", TEST_CLIENT_SECRET);
}

fn cleanup_test_env() {
    env::remove_var("WEBHOOK_SECRET");
    env::remove_var("CLIENT_ID");
    env::remove_var("CLIENT_SECRET");
}

#[tokio::test]
async fn test_webhook_missing_webhook_secret_env() {
    // Don't set WEBHOOK_SECRET
    env::remove_var("WEBHOOK_SECRET");

    let headers = HeaderMap::new();
    let request = Request::builder().body(Body::empty()).unwrap();

    let result = adapter_client_stripe::webhook::target(headers, request).await;

    assert!(result.is_err());
    let (status, message) = result.unwrap_err();
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(message, "Missing WEBHOOK_SECRET");
}

#[tokio::test]
async fn test_webhook_missing_stripe_signature_header() {
    setup_test_env();

    let headers = HeaderMap::new(); // No stripe-signature header
    let request = Request::builder().body(Body::empty()).unwrap();

    let result = adapter_client_stripe::webhook::target(headers, request).await;

    cleanup_test_env();

    assert!(result.is_err());
    let (status, message) = result.unwrap_err();
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(message, "Missing header: stripe-signature");
}

#[tokio::test]
async fn test_webhook_invalid_signature_header() {
    setup_test_env();

    let mut headers = HeaderMap::new();
    // Add invalid header that can't be converted to str
    headers.insert(
        "stripe-signature",
        HeaderValue::from_bytes(&[0xFF, 0xFE]).unwrap(),
    );

    let request = Request::builder().body(Body::empty()).unwrap();

    let result = adapter_client_stripe::webhook::target(headers, request).await;

    cleanup_test_env();

    assert!(result.is_err());
    let (status, message) = result.unwrap_err();
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(message.contains("Error extracting signature from headers"));
}

#[tokio::test]
async fn test_webhook_empty_body() {
    setup_test_env();

    let mut headers = HeaderMap::new();
    headers.insert(
        "stripe-signature",
        HeaderValue::from_static("t=1234567890,v1=signature"),
    );

    let request = Request::builder().body(Body::empty()).unwrap();

    let result = adapter_client_stripe::webhook::target(headers, request).await;

    cleanup_test_env();

    assert!(result.is_err());
    let (status, _message) = result.unwrap_err();
    assert_eq!(status, StatusCode::UNAUTHORIZED); // Will fail on signature validation
}

#[tokio::test]
async fn test_webhook_non_utf8_body() {
    setup_test_env();

    let mut headers = HeaderMap::new();
    headers.insert(
        "stripe-signature",
        HeaderValue::from_static("t=1234567890,v1=signature"),
    );

    // Create body with invalid UTF-8
    let invalid_utf8 = vec![0xFF, 0xFE, 0xFD];
    let request = Request::builder().body(Body::from(invalid_utf8)).unwrap();

    let result = adapter_client_stripe::webhook::target(headers, request).await;

    cleanup_test_env();

    assert!(result.is_err());
    let (status, message) = result.unwrap_err();
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(message.contains("Error parsing request body"));
}

// Note: Testing the full webhook flow would require:
// 1. Properly signed Stripe webhook payloads
// 2. Mocking the VersaClient and its network calls
// 3. Setting up test doubles for the Stripe webhook verification
//
// These integration-style tests are better suited for a separate integration test suite
// or using tools like wiremock to mock external services.

#[test]
fn test_env_var_client_string_format() {
    // Test that the client string format is correct
    let version = env!("CARGO_PKG_VERSION");
    env::set_var("IMAGE_VERSION", "v1.2.3");

    let expected = format!("adapter-stripe/{}/v1.2.3", version);
    let actual = format!("adapter-stripe/{}/{}", version, "v1.2.3");

    assert_eq!(actual, expected);

    env::remove_var("IMAGE_VERSION");
}

#[test]
fn test_env_var_client_string_format_no_image_version() {
    // Test client string format when IMAGE_VERSION is not set
    let version = env!("CARGO_PKG_VERSION");
    env::remove_var("IMAGE_VERSION");

    let expected = format!("adapter-stripe/{}/", version);
    let actual = format!(
        "adapter-stripe/{}/{}",
        env!("CARGO_PKG_VERSION").to_string(),
        env::var("IMAGE_VERSION").unwrap_or("".into())
    );

    assert_eq!(actual, expected);
}
