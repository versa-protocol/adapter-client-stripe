use stripe::Invoice;

use crate::model::SenderReceiptHeader;

pub fn transform_stripe_invoice(invoice: Invoice) -> SenderReceiptHeader {
    let sender_client_id = std::env::var("CLIENT_ID").unwrap_or_default();

    SenderReceiptHeader {
        id: invoice.id.to_string(),
        currency: invoice
            .currency
            .and_then(|currency| Some(currency.to_string()))
            .unwrap_or_default(),
        amount: invoice.amount_due.unwrap_or_default(),
        subtotal: invoice.subtotal.unwrap_or_default(),
        date_time: invoice.created.unwrap_or_default(),
        sender_client_id,
        third_party: None,
    }
}
