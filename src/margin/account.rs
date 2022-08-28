use crate::account::*;
use crate::util::*;
use crate::errors::*;
use std::collections::BTreeMap;
use crate::api::API;
use crate::api::Margin;
use crate::client::Client;

use super::model::{ OrderRequest, OrderQuoteQuantityRequest, SideEffectType, Transaction};

#[derive(Clone)]
pub struct MarginAccount {
    pub client: Client,
    pub recv_window: u64,
}

impl MarginAccount {
    // Place a MARGIN MARKET buy order with quote quantity - BUY
    pub fn market_buy_using_quote_quantity<S, F>(
        &self, symbol: S, quote_order_qty: F, is_isolated: bool, side_effect_type: SideEffectType,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderQuoteQuantityRequest {
            symbol: symbol.into(),
            quote_order_qty: quote_order_qty.into(),
            price: 0.0,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            new_client_order_id: None,
            is_isolated,
            side_effect_type,
        };
        let order = self.build_quote_quantity_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Margin(Margin::Order), request)
    }

    // Place a MARKET sell order with quote quantity - SELL
    pub fn market_sell_using_quote_quantity<S, F>(
        &self, symbol: S, quote_order_qty: F, is_isolated: bool, side_effect_type: SideEffectType,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderQuoteQuantityRequest {
            symbol: symbol.into(),
            quote_order_qty: quote_order_qty.into(),
            price: 0.0,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            new_client_order_id: None,
            is_isolated,
            side_effect_type,
        };
        let order = self.build_quote_quantity_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Margin(Margin::Order), request)
    }
    /// Place a custom margin order
    #[allow(clippy::too_many_arguments)]
    pub fn margin_custom_order<S, F>(
        &self, symbol: S, qty: F, price: f64, stop_price: Option<f64>, order_side: OrderSide,
        order_type: OrderType, new_client_order_id: Option<String>,
        is_isolated: bool, side_effect_type: SideEffectType,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price,
            order_side,
            order_type,
            new_client_order_id,
            is_isolated,
            side_effect_type,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Margin(Margin::Order), request)
    }

    fn build_order(&self, margin_order: OrderRequest) -> BTreeMap<String, String> {
        let mut order_parameters: BTreeMap<String, String> = BTreeMap::new();

        order_parameters.insert("symbol".into(), margin_order.symbol);
        order_parameters.insert("side".into(), margin_order.order_side.into());
        order_parameters.insert("type".into(), margin_order.order_type.into());
        order_parameters.insert("quantity".into(), margin_order.qty.to_string());

        // keys 'isIsolated' and 'sideEffectType' are required by margin orders
        match margin_order.is_isolated {
            true => {
                order_parameters.insert("isIsolated".into(), "TRUE".into());
            }
            false => {
                order_parameters.insert("isIsolated".into(), "FALSE".into());
            }
        }
        order_parameters.insert(
            "sideEffectType".into(),
            margin_order.side_effect_type.into(),
        );

        if let Some(stop_price) = margin_order.stop_price {
            order_parameters.insert("stopPrice".into(), stop_price.to_string());
        }

        if margin_order.price != 0.0 {
            order_parameters.insert("price".into(), margin_order.price.to_string());
        }

        if let Some(client_order_id) = margin_order.new_client_order_id {
            order_parameters.insert("newClientOrderId".into(), client_order_id);
        }

        order_parameters
    }

    fn build_quote_quantity_order(
        &self, order: OrderQuoteQuantityRequest,
    ) -> BTreeMap<String, String> {
        let mut order_parameters: BTreeMap<String, String> = BTreeMap::new();

        order_parameters.insert("symbol".into(), order.symbol);
        order_parameters.insert("side".into(), order.order_side.into());
        order_parameters.insert("type".into(), order.order_type.into());
        order_parameters.insert("quoteOrderQty".into(), order.quote_order_qty.to_string());

        if order.price != 0.0 {
            order_parameters.insert("price".into(), order.price.to_string());
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
