use tonic::{transport::Server, Request, Response, Status};
use orderbook::order_book_service_server::{OrderBookService, OrderBookServiceServer};
use orderbook::{AddOrderRequest, AddOrderResponse, GetSpreadRequest, GetSpreadResponse};
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;

pub mod orderbook {
    tonic::include_proto!("orderbook");
}

#[derive(Debug, Default)]
pub struct MyOrderBookService {
    engine: Arc<Mutex<Engine>>,
}

#[tonic::async_trait]
impl OrderBookService for MyOrderBookService {
    async fn add_order(
        &self,
        request: Request<AddOrderRequest>,
    ) -> Result<Response<AddOrderResponse>, Status> {
        let req = request.into_inner();
        let trading_pair = TradingPair::new(req.trading_pair, "USD".to_string());
        let order_type = match req.order_type.as_str() {
            "Bid" => OrderType::Bid,
            "Ask" => OrderType::Ask,
            _ => return Err(Status::invalid_argument("Invalid order type")),
        };
        let order = Order::new(order_type, req.size);

        let mut engine = self.engine.lock().await;
        match engine.place_limit_order(trading_pair, req.price, order) {
            Ok(_) => Ok(Response::new(AddOrderResponse {
                status: "Order placed successfully".to_string(),
            })),
            Err(e) => Err(Status::internal(e)),
        }
    }

    async fn get_spread(
        &self,
        request: Request<GetSpreadRequest>,
    ) -> Result<Response<GetSpreadResponse>, Status> {
        let req = request.into_inner();
        let trading_pair = TradingPair::new(req.trading_pair, "USD".to_string());

        let engine = self.engine.lock().await;
        match engine.orderbooks.get(&trading_pair) {
            Some(orderbook) => match orderbook.spread() {
                Some(spread) => Ok(Response::new(GetSpreadResponse { spread })),
                None => Err(Status::not_found("No spread available")),
            },
            None => Err(Status::not_found("Orderbook not found")),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let order_book_service = MyOrderBookService::default();

    println!("OrderBookService listening on {}", addr);

    Server::builder()
        .add_service(OrderBookServiceServer::new(order_book_service))
        .serve(addr)
        .await?;

    Ok(())
}