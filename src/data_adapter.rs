use std::str::FromStr;

use stripe::{Invoice, RecurringInterval};

use versa::receipt::{
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

    let mut actions = Vec::<Action>::new();
    if let Some(invoice_hosted_url) = invoice.hosted_invoice_url {
        actions.push(Action {
            name: "View in Stripe".into(),
            url: invoice_hosted_url,
        });
    }

    Receipt {
        schema_version: SchemaVersion::from_str("1.5.1").unwrap(),
        footer: Footer {
            actions,
            supplemental_text: None,
        },
        header: Header {
            total: invoice.total.expect("Invoices must have a total"),
            currency: Currency::Usd, // invoice.currency.expect("Invoices must have an associated currency"),
            customer,
            location: None,
            mcc: None,
            invoice_number: Some(invoice.id.to_string()),
            subtotal: invoice.subtotal.unwrap_or(
                invoice
                    .amount_due
                    .expect("Invoices must have a subtotal or amount due"),
            ),
            third_party: None,
            invoiced_at: invoice.created.expect("Invoices must have a creation date"),
            paid: invoice.amount_paid.expect("Invoices must have been paid"),
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
                subscription_items: invoice_items_to_subscriptions(invoice.lines),
                invoice_level_adjustments: Vec::new(),
            }),
            flight: Default::default(),
        },
        payments: Vec::new(),
    }
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
                adjustments: i
                    .discounts
                    .and_then(|ds| {
                        ds.into_iter()
                            .map(|d| {
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
                            .collect()
                    })
                    .unwrap_or_default(),
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
                subscription_type: match price.type_.and_then(|t| {
                    Some(match t {
                        stripe::PriceType::OneTime => SubscriptionType::OneTime,
                        stripe::PriceType::Recurring => SubscriptionType::Recurring,
                    })
                }) {
                    Some(val) => val,
                    None => SubscriptionType::OneTime,
                },
                unit_cost: price.unit_amount.and_then(|c| Some(c as f64)),
                amount: i.amount,
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
