use stripe::Invoice;

use crate::receipt::{Header, Itemization, Receipt, Subscription, SubscriptionItem};

pub fn transform_stripe_invoice(invoice: Invoice) -> Receipt {
    let sender_client_id = std::env::var("CLIENT_ID").unwrap_or_default();

    Receipt {
        actions: Some(vec![]),
        header: Header {
            amount: todo!(),
            currency: todo!(),
            customer: todo!(),
            datetime: todo!(),
            location: todo!(),
            mcc: todo!(),
            receipt_id: todo!(),
            subtotal: todo!(),
            third_party: todo!(),
            date_time: todo!(),
        },
        itemization: Itemization {
            general: Default::default(),
            lodging: Default::default(),
            ecommerce: Default::default(),
            car_rental: Default::default(),
            transit_route: Default::default(),
            subscription: Subscription {
                subscription_items: vec![SubscriptionItem {
                    current_period_end: todo!(),
                    current_period_start: todo!(),
                    description: todo!(),
                    discounts: todo!(),
                    interval: todo!(),
                    interval_count: todo!(),
                    metadata: todo!(),
                    quantity: todo!(),
                    taxes: todo!(),
                    subscription_item_type: todo!(),
                    unit_cost: todo!(),
                }],
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
