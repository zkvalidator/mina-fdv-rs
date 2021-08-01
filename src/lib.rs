use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize)]
pub struct PaymentDiff {
    address: String,
    expected: String,
    performed: String,
    diff: String,
}

#[wasm_bindgen]
pub async fn verify_enough_payments(
    max_epoch: u32,
    public_key: String,
    addresses: Vec<JsValue>,
    senders: Vec<JsValue>,
) -> Result<JsValue, JsValue> {
    let addresses = addresses
        .into_iter()
        .map(|x| x.as_string().unwrap())
        .collect::<Vec<_>>();
    let senders = senders
        .into_iter()
        .map(|x| x.as_string().unwrap())
        .collect::<Vec<_>>();
    let payment_diffs = mina_graphql_rs::verify_enough_payments(
        max_epoch as i64,
        &public_key,
        &addresses,
        &senders,
    )
    .await
    .map_err(|_| JsValue::from_str("could not verify"))?;
    let mut diffs = vec![];
    for (address, diff) in payment_diffs {
        diffs.push(PaymentDiff {
            address: address.clone(),
            expected: diff.expected.to_string(),
            performed: diff.performed.to_string(),
            diff: diff.diff.to_string(),
        })
    }

    JsValue::from_serde(&diffs).map_err(|_| JsValue::from_str("could not serialize diffs"))
    //diffs.into_iter().map(|d| JsValue::from_serde(&d)).collect::<Result<Vec<_>, _>>().map_err(JsValue::from_str("could not serialize diffs"))
}
