use crate::account::*;
use crate::util::*;
use crate::model::*;
use crate::errors::*;
use std::collections::BTreeMap;
use crate::api::API;
use crate::api::Margin;

pub struct MarginOrderRequest {
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

impl Account {
    /// Place a custom margin order
    #[allow(clippy::too_many_arguments)]
    pub fn margin_custom_order<S, F>(
        &self, symbol: S, qty: F, price: f64, stop_price: Option<f64>, order_side: OrderSide,
        order_type: OrderType, time_in_force: TimeInForce, new_client_order_id: Option<String>,
        is_isolated: bool, side_effect_type: SideEffectType,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell: MarginOrderRequest = MarginOrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price,
            order_side,
            order_type,
            time_in_force,
            new_client_order_id,
            is_isolated,
            side_effect_type,
        };
        let order = self.build_margin_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Margin(Margin::Order), request)
    }

    fn build_margin_order(&self, margin_order: MarginOrderRequest) -> BTreeMap<String, String> {
        let mut margin_order_parameters: BTreeMap<String, String> = BTreeMap::new();

        margin_order_parameters.insert("symbol".into(), margin_order.symbol);
        margin_order_parameters.insert("side".into(), margin_order.order_side.into());
        margin_order_parameters.insert("type".into(), margin_order.order_type.into());
        margin_order_parameters.insert("quantity".into(), margin_order.qty.to_string());

        // keys 'isIsolated' and 'sideEffectType' are required by margin orders
        match margin_order.is_isolated {
            true => {
                margin_order_parameters.insert("isIsolated".into(), "TRUE".into());
            }
            false => {
                margin_order_parameters.insert("isIsolated".into(), "FALSE".into());
            }
        }
        margin_order_parameters.insert(
            "sideEffectType".into(),
            margin_order.side_effect_type.into(),
        );

        if let Some(stop_price) = margin_order.stop_price {
            margin_order_parameters.insert("stopPrice".into(), stop_price.to_string());
        }

        if margin_order.price != 0.0 {
            margin_order_parameters.insert("price".into(), margin_order.price.to_string());
            margin_order_parameters.insert("timeInForce".into(), margin_order.time_in_force.into());
        }

        if let Some(client_order_id) = margin_order.new_client_order_id {
            margin_order_parameters.insert("newClientOrderId".into(), client_order_id);
        }

        margin_order_parameters
    }
}
