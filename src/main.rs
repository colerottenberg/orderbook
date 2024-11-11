mod matching;
use matching::engine::Engine;
use matching::orderbook::{Order, OrderBook, OrderType, TradingPair};

fn main() {
    let buy_from_cole = Order::new(OrderType::Bid, 100.0);
    let buy_from_john = Order::new(OrderType::Bid, 200.0);

    let mut order_book = OrderBook::new();

    order_book.add(buy_from_cole, 100.0);
    order_book.add(buy_from_john, 100.0);

    let sell_to_jane = Order::new(OrderType::Ask, 100.0);
    let sell_to_jack = Order::new(OrderType::Ask, 200.0);

    order_book.add(sell_to_jane, 100.0);
    order_book.add(sell_to_jack, 100.0);
    println!("{:?}", order_book);

    let mut engine = Engine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());

    engine.add_orderbook(pair.clone(), order_book);
    println!("{:?}", engine);

    let pair_string: String = pair.clone().into();
    println!("Opening new market for {} trading pair", pair_string);

    println!("Placing limit order for 100.0 BTC at $100.0");

    let order = Order::new(OrderType::Bid, 100.0);

    match engine.place_limit_order(pair.clone(), 100.0, order) {
        Ok(_) => println!("Order placed successfully"),
        Err(e) => println!("Error placing order: {}", e),
    }
}
