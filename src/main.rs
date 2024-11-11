mod matching;
use matching::engine::Engine;
use matching::orderbook::{Order, OrderBook, OrderType};

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
}
