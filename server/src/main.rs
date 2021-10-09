mod calculator;

use tonic::{transport::Server, Request, Response, Status};
use calculator::{calculator_server, Input, Output};

#[derive(Default)]
pub struct MyCalculator {}

#[tonic::async_trait]
impl calculator_server::Calculator for MyCalculator {
  async fn sum(&self, request: Request<Input>) -> Result<Response<Output>, Status> {
    Ok(Response::new(Output{
      value: request.get_ref().first_value + request.get_ref().second_value
    }))
  }

  async fn minus(&self, request: Request<Input>) -> Result<Response<Output>, Status> {
    Ok(Response::new(Output{
      value: request.get_ref().first_value - request.get_ref().second_value
    }))
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let addr = "0.0.0.0:8080".parse().unwrap();

  let calculator = MyCalculator::default();

  println!("Server listening on {}", addr);

  Server::builder()
    .add_service(calculator_server::CalculatorServer::new(calculator))
    .serve(addr)
    .await?;

  Ok(())
}
