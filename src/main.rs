use axum::{
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct ExchangeRates {
    usd: f64,
    eur: f64,
}

#[derive(Serialize)]
struct FinancialSummary {
    income: u64,
    expenses: u64,
    balance: u64,
}

async fn get_exchange_rates() -> Json<ExchangeRates> {
    Json(ExchangeRates {
        usd: 0.00025,
        eur: 0.00023,
    })
}

async fn get_summary() -> Json<FinancialSummary> {
    Json(FinancialSummary {
        income: 2_000_000,
        expenses: 1_200_000,
        balance: 800_000,
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/exchange-rates", get(get_exchange_rates))
        .route("/summary", get(get_summary));

    //let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Servidor corriendo en http://{}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
