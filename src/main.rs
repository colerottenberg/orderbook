#[derive(Debug)]
enum OrderType {
    Bid,
    Ask,
}

#[derive(Debug)]
struct Price {
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

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.integral < other.integral {
            return Some(std::cmp::Ordering::Less);
        } else if self.integral > other.integral {
            return Some(std::cmp::Ordering::Greater);
        } else {
            if self.fractional < other.fractional {
                return Some(std::cmp::Ordering::Less);
            } else if self.fractional > other.fractional {
                return Some(std::cmp::Ordering::Greater);
            } else {
                return Some(std::cmp::Ordering::Equal);
            }
        }
    }
}

impl Price {
    fn new(price: f64) -> Price {
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

#[derive(Debug)]
struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    fn new(price: f64) -> Limit {
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
struct Order {
    size: f64,
    order_type: OrderType,
}

impl Order {
    fn new(order_type: OrderType, size: f64) -> Order {
        Order { order_type, size }
    }
}

fn main() {
    let price = Price::new(123.456);
    let mut limit = Limit::new(123.456);
    let buy = Order::new(OrderType::Bid, 1.0);

    limit.add(buy);
    println!("{:?}", limit);
}
