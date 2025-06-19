use adapter_client_stripe::healthz::{service_info, ServiceInfo};
use std::time::{Duration, SystemTime};

#[tokio::test]
async fn test_service_info_returns_correct_data() {
    let before = SystemTime::now();
    let response = service_info().await;
    let after = SystemTime::now();

    // Extract the JSON data
    let info = response.0;

    // Check service name and version from Cargo.toml
    assert_eq!(info.service_name, "adapter-client-stripe");
    assert_eq!(info.service_version, env!("CARGO_PKG_VERSION"));

    // Check that system_time is between before and after
    assert!(info.system_time >= before);
    assert!(info.system_time <= after);
}

#[tokio::test]
async fn test_service_info_time_is_recent() {
    let response = service_info().await;
    let info = response.0;

    let now = SystemTime::now();
    let time_diff = now
        .duration_since(info.system_time)
        .unwrap_or(Duration::from_secs(0));

    // Should be less than 1 second difference
    assert!(time_diff < Duration::from_secs(1));
}

#[test]
fn test_service_info_struct_serialization() {
    let info = ServiceInfo {
        service_name: "test-service".to_string(),
        service_version: "1.0.0".to_string(),
        system_time: SystemTime::UNIX_EPOCH,
    };

    // Should serialize without panic
    let json = serde_json::to_string(&info).unwrap();

    // Check that all fields are present
    assert!(json.contains("\"service_name\":\"test-service\""));
    assert!(json.contains("\"service_version\":\"1.0.0\""));
    assert!(json.contains("\"system_time\""));
}

#[test]
fn test_service_info_struct_fields() {
    let now = SystemTime::now();
    let info = ServiceInfo {
        service_name: "my-service".to_string(),
        service_version: "2.1.0".to_string(),
        system_time: now,
    };

    assert_eq!(info.service_name, "my-service");
    assert_eq!(info.service_version, "2.1.0");
    assert_eq!(info.system_time, now);
}
