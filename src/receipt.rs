// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Receipt;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Receipt = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A Versa itemized receipt
#[derive(Debug, Serialize, Deserialize)]
pub struct Receipt {
    pub actions: Option<Vec<Action>>,
    pub header: Header,
    pub itemization: Itemization,
    pub payment: Option<Payment>,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub amount: i64,
    /// ISO 4217 currency code
    pub currency: Currency,
    pub customer: Option<Customer>,
    pub datetime: Option<i64>,
    pub location: Option<LocationClass>,
    pub mcc: Option<String>,
    pub receipt_id: String,
    pub subtotal: Option<i64>,
    pub third_party: Option<ThirdParty>,
    pub date_time: Option<serde_json::Value>,
}

/// ISO 4217 currency code
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Currency {
    Aud,
    Cad,
    Chf,
    Cnh,
    Eur,
    Gbp,
    Jpy,
    Usd,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    pub address: Option<AddressClass>,
    pub email: Option<String>,
    pub name: String,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressClass {
    pub city: Option<String>,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    pub postal_code: Option<String>,
    pub region: Option<String>,
    pub street_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationClass {
    pub address: Option<AddressClass>,
    pub google_place_id: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThirdParty {
    pub first_party_relation: FirstPartyRelation,
    /// Determines whether the merchant or third party gets top billing on the receipt
    pub make_primary: bool,
    pub merchant: Merchant,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FirstPartyRelation {
    Bnpl,
    #[serde(rename = "delivery_service")]
    DeliveryService,
    Marketplace,
    #[serde(rename = "payment_processor")]
    PaymentProcessor,
    Platform,
    #[serde(rename = "point_of_sale")]
    PointOfSale,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Merchant {
    /// Hex color
    pub brand_color: String,
    pub logo: Option<String>,
    pub name: String,
    pub website: Option<String>,
    pub id: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Itemization {
    pub general: HashMap<String, Option<serde_json::Value>>,
    pub lodging: HashMap<String, Option<serde_json::Value>>,
    pub ecommerce: HashMap<String, Option<serde_json::Value>>,
    pub car_rental: HashMap<String, Option<serde_json::Value>>,
    pub transit_route: HashMap<String, Option<serde_json::Value>>,
    pub subscription: Subscription,
    pub flight: HashMap<String, Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub subscription_items: Vec<SubscriptionItem>,
    pub invoice_level_discounts: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionItem {
    pub current_period_end: Option<i64>,
    pub current_period_start: Option<i64>,
    pub description: String,
    pub discounts: Option<Vec<DiscountElement>>,
    pub interval: Option<Interval>,
    pub interval_count: Option<i64>,
    pub metadata: Option<Vec<MetadatumElement>>,
    pub quantity: Option<f64>,
    pub taxes: Option<Vec<TaxElement>>,
    #[serde(rename = "type")]
    pub subscription_item_type: SubscriptionItemType,
    pub unit_cost: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountElement {
    pub amount: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub receip_type: DiscountType,
    pub rate: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscountType {
    Fixed,
    Percentage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Interval {
    Day,
    Month,
    Week,
    Year,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadatumElement {
    pub name: String,
    #[serde(rename = "type")]
    pub receip_type: MetadatumType,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetadatumType {
    Asin,
    Other,
    Sku,
    Unspsc,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionItemType {
    #[serde(rename = "one_time")]
    OneTime,
    Recurring,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxElement {
    pub amount: i64,
    pub name: String,
    pub rate: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    pub paid_at: i64,
    #[serde(rename = "type")]
    pub payment_type: PaymentType,
    pub card_payment: Option<serde_json::Value>,
    pub ach_payment: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentType {
    Ach,
    Card,
}
