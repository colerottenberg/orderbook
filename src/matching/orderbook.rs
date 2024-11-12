use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
};

#[derive(Debug)]
pub enum OrderType {
    Bid,
    Ask,
}

#[derive(Debug, Clone, Copy)]
pub struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl PartialEq for Price {
    fn eq(&self, other: &Self) -> bool {
        self.integral == other.integral && self.fractional == other.fractional
    }
}

impl Default for Price {
    fn default() -> Self {
        Price {
            integral: 0,
            fractional: 0,
            scalar: 100000,
        }
    }
}

impl Eq for Price {}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Price {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.integral < other.integral {
            Ordering::Less
        } else if self.integral > other.integral {
            Ordering::Greater
        } else {
            if self.fractional < other.fractional {
                Ordering::Less
            } else if self.fractional > other.fractional {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
    }
}

impl Price {
    pub fn new(price: f64) -> Price {
        let scalar = 100000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;

        Price {
            integral,
            fractional,
            scalar,
        }
    }
}

impl From<f64> for Price {
    fn from(price: f64) -> Self {
        Price::new(price)
    }
}

impl Into<f64> for Price {
    fn into(self) -> f64 {
        self.integral as f64 + (self.fractional as f64 / self.scalar as f64)
    }
}

#[derive(Debug)]
pub struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    pub fn new(price: f64) -> Limit {
        Limit {
            price: Price::new(price),
            orders: Vec::new(),
        }
    }

    fn add(&mut self, order: Order) {
        self.orders.push(order)
    }

    fn volume(&self) -> f64 {
        self.orders
            .iter()
            .map(|order| order.size)
            .reduce(|a, b| a + b)
            .unwrap_or(0.0)
    }

    /// Used for filling orders at a certain limit
    fn fill(&mut self, market_order: &mut Order) {
        for limit_order in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0;
                }
                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0;
                }
            }

            if market_order.is_filled() {
                break;
            }
        }
    }
}

#[derive(Debug)]
pub struct Order {
    size: f64,
    order_type: OrderType,
}

impl Order {
    pub fn new(order_type: OrderType, size: f64) -> Order {
        Order { order_type, size }
    }

    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }
}

#[derive(Debug)]
pub struct OrderBook {
    asks: BTreeMap<Price, Limit>,
    bids: BTreeMap<Price, Limit>,
}

impl OrderBook {
    /// Create a new order book
    ///
    /// # Example
    /// ```
    /// use matching::orderbook::OrderBook;
    /// let order_book = OrderBook::new();
    /// ```
    pub fn new() -> OrderBook {
        OrderBook {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
        }
    }

    pub fn place_market_order(&mut self, order: &mut Order) {
        let limits = match order.order_type {
            OrderType::Ask => self.bid_limits(), // If we are selling, we need the buyers
            OrderType::Bid => self.ask_limits(), // Vice Versa
        };
        for limit_order in limits {
            limit_order.fill(order);
        }
    }

    /// Returns the ask limits sorted by price of each limit
    pub fn ask_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.asks.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a, b| a.price.cmp(&b.price));
        limits
    }

    /// Collects the BTree of the Bids and collects it into a Vec and sorts by highest price
    pub fn bid_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.bids.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a, b| b.price.cmp(&a.price));
        limits
    }

    /// Add an order to the order book
    ///
    /// # Arguments
    /// * `order` - The order to add to the order book
    /// * `price` - The price of the order
    ///
    /// # Example
    /// ```
    /// use matching::orderbook::{OrderBook, Order, OrderType};
    /// let mut order_book = OrderBook::new();
    /// let order = Order::new(OrderType::Bid, 100.0);
    /// order_book.add(order, 1000.00);
    /// ```
    pub fn add(&mut self, order: Order, price: f64) {
        match order.order_type {
            OrderType::Ask => {
                let limit = self
                    .asks
                    .entry(Price::new(price))
                    .or_insert(Limit::new(price));
                limit.add(order);
            }
            OrderType::Bid => {
                let limit = self
                    .bids
                    .entry(Price::new(price))
                    .or_insert(Limit::new(price));
                limit.add(order);
            }
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> Self {
        TradingPair { base, quote }
    }
}

impl From<(String, String)> for TradingPair {
    fn from(pair: (String, String)) -> Self {
        TradingPair {
            base: pair.0,
            quote: pair.1,
        }
    }
}

impl Into<String> for TradingPair {
    fn into(self) -> String {
        format!("{}/{}", self.base, self.quote)
    }
}

#[cfg(test)]
pub mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn limit_order_single_fill() {
        let mut limit = Limit::new(1000.00);
        let buy_limit_order = Order::new(OrderType::Bid, 100.0);

        limit.add(buy_limit_order);

        let mut market_sell_order = Order::new(OrderType::Ask, 99.0);
        limit.fill(&mut market_sell_order);
        println!("{:?}", limit);
        assert!(market_sell_order.is_filled());
        assert_eq!(limit.orders.get(0).unwrap().size, 1.0);
    }

    #[test]
    fn limit_order_multi_fill() {
        let price = Price::new(1000.00);
        let mut limit = Limit::new(1000.00);
        let buy_limit_order_a = Order::new(OrderType::Bid, 50.0);
        let buy_limit_order_b = Order::new(OrderType::Bid, 50.0);

        limit.add(buy_limit_order_a);
        limit.add(buy_limit_order_b);

        let mut market_sell_order = Order::new(OrderType::Ask, 99.0);
        limit.fill(&mut market_sell_order);
        println!("{:?}", limit);
        assert!(market_sell_order.is_filled());
        assert!(limit.orders.get(0).unwrap().is_filled());
        assert!(!limit.orders.get(1).unwrap().is_filled())
    }

    #[test]
    fn limit_total_volume() {
        let mut limit = Limit::new(1000.00);

        let buy_limit_order_a = Order::new(OrderType::Bid, 50.0);
        let buy_limit_order_b = Order::new(OrderType::Bid, 50.0);

        limit.add(buy_limit_order_a);
        limit.add(buy_limit_order_b);

        assert_eq!(limit.volume(), 100.0);

        let mut market_sell_order = Order::new(OrderType::Ask, 99.0);

        limit.fill(&mut market_sell_order);

        assert_eq!(limit.volume(), 1.0);
    }

    #[test]
    fn orderbook_fill_market_order() {
        let mut orderbook = OrderBook::new();
        orderbook.add(Order::new(OrderType::Ask, 10.0), 100.0);
        orderbook.add(Order::new(OrderType::Ask, 5.0), 200.0);
        orderbook.add(Order::new(OrderType::Ask, 15.0), 500.0);
        orderbook.add(Order::new(OrderType::Ask, 10.0), 100.0);

        let mut market = Order::new(OrderType::Bid, 10.0);
        orderbook.place_market_order(&mut market);

        let ask_limits = orderbook.ask_limits();
        let matched_limits = ask_limits.get(0).unwrap();
        assert_eq!(matched_limits.price, Price::from(100.0));
        assert!(market.is_filled());

        let matched_order = matched_limits.orders.get(0);
        match matched_order {
            Some(mo) => {
                assert!(mo.is_filled())
            }
            None => eprintln!("Order No Longer Exists"),
        }
    }
}
