use serde::{Deserialize, Serialize};

use crate::account::*;
use crate::model::*;

pub struct OrderRequest {
    pub symbol: String,
    pub qty: f64,
    pub price: f64,
    pub stop_price: Option<f64>,
    pub order_side: OrderSide,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub new_client_order_id: Option<String>,
    pub is_isolated: bool,
    pub side_effect_type: SideEffectType,
}

pub struct OrderQuoteQuantityRequest {
    pub symbol: String,
    pub quote_order_qty: f64,
    pub price: f64,
    pub order_side: OrderSide,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub new_client_order_id: Option<String>,
    pub is_isolated: bool,
    pub side_effect_type: SideEffectType,
}

/// NO_SIDE_EFFECT, MARGIN_BUY, AUTO_REPAY; default NO_SIDE_EFFECT.
pub enum SideEffectType {
    NoSideEffect,
    MarginBuy,
    AutoRepay,
}

impl From<SideEffectType> for String {
    fn from(item: SideEffectType) -> Self {
        match item {
            SideEffectType::NoSideEffect => String::from("NO_SIDE_EFFECT"),
            SideEffectType::MarginBuy => String::from("MARGIN_BUY"),
            SideEffectType::AutoRepay => String::from("AUTO_REPAY"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub symbol: String,
    pub order_id: u64,
    pub client_order_id: String,
    pub transact_time: u64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(with = "string_or_float")]
    pub cummulative_quote_qty: f64,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub stop_price: f64,
    pub status: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub side: String,
    pub fills: Option<Vec<FillInfo>>,
    pub is_isolated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_buy_borrow_amount: Option<f64>,
    pub margin_borrow_asset: Option<String>,
}

fn default_stop_price() -> f64 {
    0.0
}
