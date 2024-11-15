use super::orderbook::{Order, OrderBook, Price, TradingPair};
use std::collections::HashMap;

#[derive(Debug)]
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

    /// Place a limit order
    ///
    /// This function will place a limit order on the orderbook
    ///
    /// # Arguments
    /// * `trading_pair` - The trading pair to place the order on
    /// * `price` - The price of the order
    /// * `order` - The order to place
    ///
    /// # Returns
    /// * `Result<(), String>` - Ok(()) if the order was placed successfully, Err(String) if the orderbook does not exist
    ///
    /// # Example
    ///
    /// ```
    /// use matching::engine::Engine;
    /// use matching::orderbook::{Order, OrderType};
    /// let mut engine = Engine::new();
    /// let order = Order::new(OrderType::Bid, 100.0);
    /// engine.place_limit_order(TradingPair::new("BTC".to_string(), "USD".to_string()), 100.0, order);
    ///
    /// ```
    pub fn place_limit_order(
        &mut self,
        trading_pair: TradingPair,
        price: f64,
        order: Order,
    ) -> Result<(), String> {
        match self.orderbooks.get_mut(&trading_pair) {
            Some(orderbook) => {
                orderbook.add(order, price);
                Ok(())
            }
            None => Err("Orderbook does not exist".to_string()),
        }
    }
}
