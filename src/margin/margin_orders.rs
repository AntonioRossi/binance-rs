use crate::account::*;
use crate::util::*;
use crate::errors::*;
use std::collections::BTreeMap;
use crate::api::API;
use crate::api::Margin;

use super::margin_model::*;

impl Account {
    // Place a MARGIN MARKET buy order with quote quantity - BUY
    pub fn margin_market_buy_using_quote_quantity<S, F>(
        &self, symbol: S, quote_order_qty: F, is_isolated: bool, side_effect_type: SideEffectType,
    ) -> Result<MarginTransaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = MarginOrderQuoteQuantityRequest {
            symbol: symbol.into(),
            quote_order_qty: quote_order_qty.into(),
            price: 0.0,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
            is_isolated,
            side_effect_type,
        };
        let order = self.build_margin_quote_quantity_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Margin(Margin::Order), request)
    }

    // Place a MARKET sell order with quote quantity - SELL
    pub fn margin_market_sell_using_quote_quantity<S, F>(
        &self, symbol: S, quote_order_qty: F, is_isolated: bool, side_effect_type: SideEffectType,
    ) -> Result<MarginTransaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = MarginOrderQuoteQuantityRequest {
            symbol: symbol.into(),
            quote_order_qty: quote_order_qty.into(),
            price: 0.0,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
            is_isolated,
            side_effect_type,
        };
        let order = self.build_margin_quote_quantity_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Margin(Margin::Order), request)
    }
    /// Place a custom margin order
    #[allow(clippy::too_many_arguments)]
    pub fn margin_custom_order<S, F>(
        &self, symbol: S, qty: F, price: f64, stop_price: Option<f64>, order_side: OrderSide,
        order_type: OrderType, time_in_force: TimeInForce, new_client_order_id: Option<String>,
        is_isolated: bool, side_effect_type: SideEffectType,
    ) -> Result<MarginTransaction>
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

    fn build_margin_quote_quantity_order(
        &self, order: MarginOrderQuoteQuantityRequest,
    ) -> BTreeMap<String, String> {
        let mut order_parameters: BTreeMap<String, String> = BTreeMap::new();

        order_parameters.insert("symbol".into(), order.symbol);
        order_parameters.insert("side".into(), order.order_side.into());
        order_parameters.insert("type".into(), order.order_type.into());
        order_parameters.insert("quoteOrderQty".into(), order.quote_order_qty.to_string());

        if order.price != 0.0 {
            order_parameters.insert("price".into(), order.price.to_string());
            order_parameters.insert("timeInForce".into(), order.time_in_force.into());
        }

        // keys 'isIsolated' and 'sideEffectType' are required by margin orders
        match order.is_isolated {
            true => {
                order_parameters.insert("isIsolated".into(), "TRUE".into());
            }
            false => {
                order_parameters.insert("isIsolated".into(), "FALSE".into());
            }
        }
        order_parameters.insert("sideEffectType".into(), order.side_effect_type.into());

        if let Some(client_order_id) = order.new_client_order_id {
            order_parameters.insert("newClientOrderId".into(), client_order_id);
        }

        order_parameters
    }
}
