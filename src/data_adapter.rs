use std::str::FromStr;

use stripe_shared::{Invoice, RecurringInterval};

use versa::schema::receipt::{
    Action, Adjustment, AdjustmentType, Currency, Customer, Footer, Header, Interval, Itemization,
    Receipt, SchemaVersion, Subscription, SubscriptionItem, SubscriptionType,
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
                    metadata: Vec::new(),
                })
            } else {
                None
            }
        }
        None => None,
    };

    let mut actions = vec![];
    if let Some(invoice_hosted_url) = invoice.hosted_invoice_url {
        actions.push(Action {
            name: "View in Stripe".into(),
            url: invoice_hosted_url,
        });
    }

    Receipt {
        schema_version: SchemaVersion::from_str("1.11.0").unwrap(),
        footer: Footer {
            actions: actions,
            supplemental_text: Some("*Sent via Versa*".into()),
        },
        header: Header {
            total: invoice.total,
            currency: Currency::Usd, // invoice.currency.expect("Invoices must have an associated currency"),
            customer,
            location: None,
            mcc: None,
            invoice_number: invoice.id.and_then(|id| Some(id.to_string())),
            subtotal: invoice.subtotal,
            third_party: None,
            invoiced_at: invoice.created,
            paid: invoice.amount_paid,
            invoice_asset_id: None,
            receipt_asset_id: None,
        },
        itemization: Itemization {
            general: Default::default(),
            lodging: Default::default(),
            ecommerce: Default::default(),
            car_rental: Default::default(),
            transit_route: Default::default(),
            subscription: Some(Subscription {
                subscription_items: invoice
                    .lines
                    .data
                    .into_iter()
                    .filter_map(|i| invoice_item_to_subscription(i))
                    .collect(),
                invoice_level_adjustments: Vec::new(),
            }),
            flight: Default::default(),
        },
        payments: Vec::new(),
    }
}

fn invoice_item_to_subscription(i: stripe_shared::InvoiceLineItem) -> Option<SubscriptionItem> {
    let period = i.period;
    let Some(price) = i.price else { return None };
    Some(SubscriptionItem {
        current_period_end: Some(period.end),
        current_period_start: Some(period.start),
        description: i.description.unwrap_or("Missing Description".into()),
        adjustments: i
            .discounts
            .into_iter()
            .filter_map(|d| {
                if let Some(d) = d.into_object() {
                    Some(Adjustment {
                        amount: d.coupon.amount_off.unwrap_or_default(),
                        name: d.coupon.name,
                        adjustment_type: AdjustmentType::Discount,
                        // discount_type: match d.coupon.percent_off {
                        //     Some(_) => DiscountType::Percentage,
                        //     None => DiscountType::Fixed,
                        // },
                        rate: None,
                    })
                } else {
                    None
                }
            })
            .collect(),
        interval: price
            .recurring
            .as_ref()
            .and_then(|r| Some(transform_interval(r.interval))),
        interval_count: price
            .recurring
            .as_ref()
            .and_then(|r| Some(r.interval_count as i64)), // should be u64 ?
        metadata: Vec::new(),
        quantity: i.quantity.and_then(|q| Some(q as f64)),
        taxes: Vec::new(),
        subscription_type: match price.type_ {
            stripe_shared::PriceType::OneTime => SubscriptionType::OneTime,
            stripe_shared::PriceType::Recurring => SubscriptionType::Recurring,
        },
        unit_cost: price.unit_amount.and_then(|c| Some(c as f64)),
        amount: i.amount,
    })
}

pub fn transform_interval(interval: RecurringInterval) -> Interval {
    match interval {
        RecurringInterval::Day => Interval::Day,
        RecurringInterval::Week => Interval::Week,
        RecurringInterval::Month => Interval::Month,
        RecurringInterval::Year => Interval::Year,
    }
}
