use std::sync::Arc;
use futures::SinkExt;
use redis::{Commands, RedisError};
use axum::extract::ws::WebSocket;
use futures_util::stream::SplitSink;
use serde_json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct MsgChannel {
  sockID: String,
  ip: String,
  sParam: String,
  sCode: String,
  flag: String,
  from: String,
  to: String,
  data: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct DataExchange {
    sParam: String,
    sCode: String,
    B0: String,
    B1: String,
    B2: String,
    B3: String,
    B4: String,
    B5: String,
    B6: String,
    B7: String,
    B8: String,
    B9: String,
  }

#[allow(unused)]
pub async fn subscribe(_sock_id: String, channel: &str, socket: Arc<tokio::sync::Mutex<SplitSink<WebSocket, axum::extract::ws::Message>>>) -> Result<(), RedisError> {
  let redis_client = redis::Client::open("redis://127.0.0.1/").unwrap();

  let mut sub_con = redis_client.get_connection().unwrap();

  let mut pub_sub = sub_con.as_pubsub();

  pub_sub.subscribe(&channel)?;

  loop {
      let msg = pub_sub.get_message().unwrap();
      let payload: String = msg.get_payload().unwrap();
      let _channel = msg.get_channel_name();
      let _msg_channel = crate::redis_manager::utils::get_redis_value::function("test").unwrap();
      
      let data: MsgChannel = serde_json::from_str(payload.as_str()).unwrap();

      if data.sParam == "A999" {
        tracing::info!("LOGIN - SECNAV => Commute vers BackOffice (A999)");
        let data_commut: DataExchange = serde_json::from_str(data.data.clone().as_str()).unwrap();
        let send_data = serde_json::to_string(&data_commut).unwrap();
        let _ = socket.lock().await.send(axum::extract::ws::Message::Text(send_data)).await;
      }
  }
}

#[allow(unused)]
pub async fn publish(channel: &str, message: &str) -> Result<(), RedisError> {
  let redis_client = redis::Client::open("redis://127.0.0.1/").unwrap();

  let mut pub_con = redis_client.get_connection().unwrap();
  let mut sub_con = redis_client.get_connection().unwrap();

  let mut pub_sub = sub_con.as_pubsub();

  pub_sub.subscribe(&channel)?;
  #[allow(non_snake_case)]

  pub_con.publish(&channel, message)?;

  Ok(())
}