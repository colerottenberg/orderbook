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
}

#[derive(Debug)]
pub struct OrderBook {
    asks: BTreeMap<Price, Limit>,
    bids: BTreeMap<Price, Limit>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
        }
    }

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
