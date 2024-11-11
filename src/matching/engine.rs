use super::orderbook::OrderBook;
use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> Self {
        TradingPair { base, quote }
    }
}

pub struct Engine {
    orderbooks: HashMap<TradingPair, OrderBook>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            orderbooks: HashMap::new(),
        }
    }

    /// Add an orderbook to the engine
    ///
    /// This function will add an orderbook to the engine but only if it does not already exist
    ///
    /// # Arguments
    /// * `trading_pair` - The trading pair to add
    /// * `orderbook` - The orderbook to add
    ///
    /// # Example
    /// ```
    /// use matching::engine::Engine;
    /// use matching::orderbook::{Order, OrderBook, OrderType};
    /// let mut engine = Engine::new();
    /// let orderbook = OrderBook::new();
    ///
    /// engine.add_orderbook(TradingPair::new("BTC".to_string(), "USD".to_string()), orderbook);
    /// ```
    pub fn add_orderbook(&mut self, trading_pair: TradingPair, orderbook: OrderBook) {
        self.orderbooks.entry(trading_pair).or_insert(orderbook);
    }
}
