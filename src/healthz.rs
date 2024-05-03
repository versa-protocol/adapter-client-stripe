use serde::Serialize;
use std::{env, time::SystemTime};

#[derive(Serialize)]
pub struct ServiceInfo {
    pub service_name: String,
    pub service_profile: String,
    pub service_version: String,
    pub system_time: SystemTime,
}

pub async fn service_info() -> axum::Json<ServiceInfo> {
    let service_name = "adapter-client-stripe".to_string();
    let service_profile = std::env::var("PROFILE").unwrap().to_string();
    let service_version = env!("CARGO_PKG_VERSION").to_string();
    let system_time = SystemTime::now();
    let service_info = ServiceInfo {
        service_name,
        service_profile,
        service_version,
        system_time,
    };
    axum::Json(service_info)
}
