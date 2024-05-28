use stripe::{Invoice, RecurringInterval};

use crate::receipt::{
    Currency, Customer, DiscountElement, DiscountType, Header, Interval, Itemization, Receipt,
    Subscription, SubscriptionItem, SubscriptionItemType,
};

pub fn transform_stripe_invoice(invoice: Invoice) -> Receipt {
    let customer = match invoice.customer {
        Some(c) => {
            if let Some(obj) = c.into_object() {
                Some(Customer {
                    address: None, // obj.address,
                    email: obj.email,
                    name: obj.name.unwrap_or("".into()),
                    phone: obj.phone,
                })
            } else {
                None
            }
        }
        None => None,
    };
    Receipt {
        actions: Some(vec![]),
        header: Header {
            amount: invoice.amount_paid.expect("Invoices must have been paid"),
            currency: Currency::Usd, // invoice.currency.expect("Invoices must have an associated currency"),
            customer,
            datetime: invoice.created, // datetime of payment?
            location: None,
            mcc: None,
            receipt_id: invoice.id.to_string(),
            subtotal: invoice.subtotal,
            third_party: None,
            date_time: None,
        },
        itemization: Itemization {
            general: Default::default(),
            lodging: Default::default(),
            ecommerce: Default::default(),
            car_rental: Default::default(),
            transit_route: Default::default(),
            subscription: Subscription {
                subscription_items: invoice_items_to_subscriptions(invoice.lines),
                invoice_level_discounts: None, // invoice.discount
            },
            flight: Default::default(),
        },
        payment: None,
        version: "0.2.0".into(),
    }

    // SenderReceiptHeader {
    //     id: invoice.id.to_string(),
    //     schema_version: "1".into(),
    //     currency: invoice
    //         .currency
    //         .and_then(|currency| Some(currency.to_string()))
    //         .unwrap_or_default(),
    //     amount: invoice.amount_due.unwrap_or_default(),
    //     subtotal: invoice.subtotal.unwrap_or_default(),
    //     date_time: invoice.created.unwrap_or_default(),
    //     sender_client_id,
    //     mcc: None,
    //     third_party: None,
    // }
}

fn invoice_items_to_subscriptions(
    lines: Option<stripe::List<stripe::InvoiceLineItem>>,
) -> Vec<SubscriptionItem> {
    let Some(lines) = lines else { return vec![] };

    lines
        .data
        .into_iter()
        .filter_map(|i| {
            let Some(period) = i.period else { return None };
            let Some(price) = i.price else { return None };
            Some(SubscriptionItem {
                current_period_end: period.end,
                current_period_start: period.start,
                description: i.description.unwrap_or("Missing Description".into()),
                discounts: i.discounts.and_then(|ds| {
                    ds.into_iter()
                        .map(|d| {
                            if let Some(d) = d.into_object() {
                                Some(DiscountElement {
                                    amount: d.coupon.amount_off.unwrap_or_default(),
                                    name: d.coupon.name.unwrap_or_default(), // should be optional?
                                    // typo ? // how to know?
                                    discount_type: match d.coupon.percent_off {
                                        Some(_) => DiscountType::Percentage,
                                        None => DiscountType::Fixed,
                                    },
                                    rate: None,
                                })
                            } else {
                                None
                            }
                        })
                        .collect()
                }),
                interval: price
                    .recurring
                    .as_ref()
                    .and_then(|r| Some(transform_interval(r.interval))),
                interval_count: price
                    .recurring
                    .as_ref()
                    .and_then(|r| Some(r.interval_count as i64)), // should be u64 ?
                metadata: None,
                quantity: i.quantity.and_then(|q| Some(q as f64)),
                taxes: None,
                subscription_item_type: match price.type_.and_then(|t| {
                    Some(match t {
                        stripe::PriceType::OneTime => SubscriptionItemType::OneTime,
                        stripe::PriceType::Recurring => SubscriptionItemType::Recurring,
                    })
                }) {
                    Some(val) => val,
                    None => SubscriptionItemType::OneTime,
                },
                unit_cost: price.unit_amount.and_then(|c| Some(c as f64)),
            })
        })
        .collect()
}

pub fn transform_interval(interval: RecurringInterval) -> Interval {
    match interval {
        RecurringInterval::Day => Interval::Day,
        RecurringInterval::Week => Interval::Week,
        RecurringInterval::Month => Interval::Month,
        RecurringInterval::Year => Interval::Year,
    }
}
