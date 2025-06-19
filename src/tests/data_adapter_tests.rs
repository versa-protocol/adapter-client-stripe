use stripe_shared::RecurringInterval;
use versa::schema::receipt::Interval;

use adapter_client_stripe::data_adapter::transform_interval;

#[test]
fn test_transform_interval_day() {
    assert_eq!(transform_interval(RecurringInterval::Day), Interval::Day);
}

#[test]
fn test_transform_interval_week() {
    assert_eq!(transform_interval(RecurringInterval::Week), Interval::Week);
}

#[test]
fn test_transform_interval_month() {
    assert_eq!(
        transform_interval(RecurringInterval::Month),
        Interval::Month
    );
}

#[test]
fn test_transform_interval_year() {
    assert_eq!(transform_interval(RecurringInterval::Year), Interval::Year);
}

// Test the transformation function behavior with minimal setup
// Since we can't easily construct stripe_shared types, we focus on testing
// the logic that doesn't require complex type construction
