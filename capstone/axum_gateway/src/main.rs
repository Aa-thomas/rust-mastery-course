// use std::{collections::HashMap, net::SocketAddr, sync::{Arc, atomic::{AtomicU64, Ordering}}};
// use axum::{routing::{get, post}, Router, response::IntoResponse, extract::{State, ws::{WebSocketUpgrade, Message, WebSocket}}, Json};
// use tower_http::cors::{Any, CorsLayer};
// use serde::{Deserialize, Serialize};
// use tracing::info;
// use tracing_subscriber::EnvFilter;
// use tokio::{sync::RwLock, time::{interval, Duration}};
//
// #[derive(Clone, Default)]
// struct AppState {
//     idempotency: Arc<RwLock<HashMap<String, String>>>,
//     order_seq: Arc<AtomicU64>,
// }
//
// type Arc<T> = std::sync::Arc<T>;
//
// #[derive(Debug, Deserialize)]
// struct OrderReq { symbol: String, side: String, qty: u64, #[serde(default)] r#type: String, #[serde(default)] price: Option<f64>, #[serde(default)] client_id: Option<String> }
//
// #[derive(Debug, Serialize)]
// struct OrderResp { status: String, order_id: String }
//
// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
//     tracing_subscriber::fmt().with_env_filter(filter).with_target(false).init();
//
//     let state = AppState { idempotency: Arc::new(RwLock::new(HashMap::new())), order_seq: Arc::new(AtomicU64::new(0)) };
//
//     let app = Router::new()
//         .route("/health", get(|| async { Json(serde_json::json!({ "status":"ok" })) }))
//         .route("/metrics", get(metrics))
//         .route("/orders", post(orders))
//         .route("/cancel", post(cancel))
//         .route("/ws/feed", get(ws_feed))
//         .with_state(state)
//         .layer(CorsLayer::new().allow_methods(Any).allow_headers(Any).allow_origin(Any));
//
//     let addr: SocketAddr = "0.0.0.0:8080".parse()?;
//     info!("Gateway on http://{addr}  |  WS: ws://{addr}/ws/feed  |  POST /orders  |  GET /metrics  |  GET /health");
//     axum::Server::bind(&addr).serve(app.into_make_service()).await?;
//     Ok(())
// }
//
// async fn metrics() -> impl IntoResponse {
//     ([(axum::http::header::CONTENT_TYPE, "text/plain; version=0.0.4")], "# HELP demo 1\n# TYPE demo counter\ndemo 1\n")
// }
//
// async fn orders(
//     State(state): State<AppState>,
//     headers: axum::http::HeaderMap,
//     Json(_req): Json<OrderReq>
// ) -> impl IntoResponse {
//     let key = headers.get("x-idempotency-key").and_then(|v| v.to_str().ok()).map(|s| s.to_string());
//     let mut idemp = state.idempotency.write().await;
//     if let Some(k) = key.clone() {
//         if let Some(existing) = idemp.get(&k) {
//             return Json(serde_json::json!({ "status":"duplicate", "order_id": existing }));
//         }
//     }
//     let next = state.order_seq.fetch_add(1, Ordering::Relaxed) + 1;
//     let oid = format!("ord_{:08}", next);
//     if let Some(k) = key { idemp.insert(k, oid.clone()); }
//     Json(serde_json::json!(OrderResp{ status:"accepted".into(), order_id: oid }))
// }
//
// #[derive(Debug, Deserialize)]
// struct CancelReq { order_id: String }
// async fn cancel(Json(req): Json<CancelReq>) -> impl IntoResponse {
//     Json(serde_json::json!({ "status":"cancelled", "order_id": req.order_id }))
// }
//
// async fn ws_feed(ws: WebSocketUpgrade) -> impl IntoResponse { ws.on_upgrade(handle_socket) }
// async fn handle_socket(mut socket: WebSocket) {
//     use axum::extract::ws::Message::*;
//     if let Some(Ok(Text(txt))) = socket.recv().await { tracing::info!("Client said: {txt}"); }
//     let snapshot = serde_json::json!({
//         "type":"snapshot","v":"1.0","symbol":"DEMO",
//         "bids":[[100.0,500],[99.9,800]],"asks":[[100.1,450],[100.2,900]],
//         "ts": now_ms()
//     }).to_string();
//     if socket.send(Message::Text(snapshot)).await.is_err() { return; }
//     let mut price = 100.0; let mut qty: i64 = 500; let mut iv = interval(Duration::from_millis(200));
//     loop {
//         iv.tick().await;
//         price += if now_ms() % 2 == 0 { 0.02 } else { -0.02 };
//         qty += if now_ms() % 2 == 0 { 20 } else { -20 };
//         if qty < 0 { qty = 100; }
//         let update = serde_json::json!({
//             "type":"l2_update","v":"1.0","symbol":"DEMO",
//             "side": if now_ms()%2==0 {"bid"} else {"ask"},
//             "price": (price*100.0).round()/100.0,"delta": qty,"ts": now_ms()
//         }).to_string();
//         if socket.send(Message::Text(update)).await.is_err() { break; }
//     }
// }
// fn now_ms() -> u128 { use std::time::{SystemTime, UNIX_EPOCH}; SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() }
