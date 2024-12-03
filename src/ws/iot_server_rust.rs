use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use axum_extra::TypedHeader;
use uuid::Uuid;

use std::net::SocketAddr;

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;

use std::sync::Arc;
use tokio::sync::Mutex;
//allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};
use futures_util::stream::SplitSink;
use serde::{Serialize, Deserialize};
use serde_json;
use crate::lib::infos_system::InfoSystem;


#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct PingKP007 {
    sockID: String,
    process: String,
    sParam: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct PongKP009 {
    sParam: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct MsgChannel {
  sockID: String,
  sCode: String,
  ip: String,
  sParam: String,
  flag: String,
  from: String,
  to: String,
  data: String,
}

#[derive(Serialize, Deserialize)]
struct WsSession {
    received_pong: bool,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct InfoSystemData  {
    sParam: String,
    sData: InfoSystem,
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
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

/// helper to print contents of messages to stdout. Has special treatment for Close.
async fn handle_socket(socket: WebSocket, who: SocketAddr) {
    let uid = Uuid::new_v4();
    let sock_id = uid.to_string();

    let n_nb_tent_kp007 = 2;
    let mut current_nb_tent_kp007 = 0;
    let kp007 = "KP007".to_string();
    let kp007bo = "KP007bo".to_string();
    let struct_session = WsSession {
        received_pong: false,
    };
    let struct_session_bo = WsSession {
        received_pong: false,
    };
    // crate::redis_manager::utils::set_redis_value::function(&kp007, &serde_json::to_string(&struct_session).unwrap()).unwrap();
    // crate::redis_manager::utils::set_redis_value::function(&kp007bo, &serde_json::to_string(&struct_session_bo).unwrap()).unwrap();


    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (sender, mut receiver) = socket.split();

    let o_socket: Arc<Mutex<SplitSink<WebSocket, axum::extract::ws::Message>>> = Arc::new(Mutex::new(sender));

    // SENDER FOR SECSRV CHANNEL
    let sender1: Arc<Mutex<SplitSink<WebSocket, axum::extract::ws::Message>>> = Arc::clone(&o_socket);
    // SENDER FOR PING INTERVAL
    let sender3: Arc<Mutex<SplitSink<WebSocket, axum::extract::ws::Message>>> = Arc::clone(&o_socket);
    let sender4: Arc<Mutex<SplitSink<WebSocket, axum::extract::ws::Message>>> = Arc::clone(&o_socket);
    let sender5: Arc<Mutex<SplitSink<WebSocket, axum::extract::ws::Message>>> = Arc::clone(&o_socket);

    // CLONE SOCK ID FOR PING INTERVAL
    // CLONE SOCK ID FOR STANDARD WEBSOCKET CHANNEL
    let sock_id_clone4 = sock_id.clone();
    let sock_id_clone5 = sock_id.clone();
    let sock_id_clone6 = sock_id.clone();
    let sock_id_clone7 = sock_id.clone();

    let mut infos_system_data = DataExchange {
        sParam: String::from("996"),
        sCode: String::new(),
        B0: sock_id_clone7.to_string(),
        B1: String::new(),
        B2: String::new(),
        B3: String::new(),
        B4: String::new(),
        B5: String::new(),
        B6: String::new(),
        B7: String::new(),
        B8: String::new(),
        B9: String::new(),
    };
    match crate::lib::infos_system::gather_system_info() {
        Ok(infos_system) => {
            infos_system_data.B1 = serde_json::to_string(&infos_system).unwrap();
        },
        Err(error) => eprintln!("Failed to gather system info: {}", error),
    }
    let _ = sender1.lock().await.send(Message::Text(serde_json::to_string(&infos_system_data).unwrap())).await;

    // SUBSCRIBER FOR SECSRV CHANNEL
    // let mut recv_msg_channel = tokio::spawn(async move {
    //     let cnt = 0;
    //     crate::redis_manager::channels::rpmgnt::subscribe(sock_id_clone5, "rpmgnt-channel", sender4).await.unwrap();
    //     cnt
    // });

    // PING INTERVAL
    let mut loop_ping = tokio::spawn(async move {
        let ping_msg = DataExchange {
            sParam: String::from("97"),
            sCode: String::new(),
            B0: sock_id_clone6.to_string(),
            B1: String::from("KP007"),
            B2: String::new(),
            B3: String::new(),
            B4: String::new(),
            B5: String::new(),
            B6: String::new(),
            B7: String::new(),
            B8: String::new(),
            B9: String::new(),
        };
        let mut _cnt = 0;
        loop {
            let kp007 = "KP007".to_string();
            // let get_ws_session = crate::redis_manager::utils::get_redis_value::function(&kp007).unwrap();
            // tracing::info!("GET WS SESSION IN RPMGNT: {}", get_ws_session);
            // let struct_session: WsSession = serde_json::from_str(&get_ws_session).unwrap();
            let received_pong_gard = struct_session.received_pong;

            if received_pong_gard {
                current_nb_tent_kp007 = 0;
                tracing::debug!("RPMGNT PING NBR ATTEMPT REDUCED: {}", current_nb_tent_kp007);
            }
            if !received_pong_gard {
                current_nb_tent_kp007 += 1;
                tracing::debug!("RPMGNT PING ATTEMPT: {}", current_nb_tent_kp007);
            }
            let struct_session = WsSession {
                received_pong: false,
            };
            // crate::redis_manager::utils::set_redis_value::function(&kp007, &serde_json::to_string(&struct_session).unwrap()).unwrap();

            tokio::time::sleep(std::time::Duration::from_millis(27000)).await;

            if !received_pong_gard && current_nb_tent_kp007 > n_nb_tent_kp007 {
                let _ = sender3.lock().await.send(Message::Close(None)).await;
                break;
            }

            let _ = sender3.lock().await.send(Message::Text(serde_json::to_string(&ping_msg).unwrap())).await;
            _cnt += 1;
            tokio::time::sleep(std::time::Duration::from_millis(3000)).await;
        }
    });

    // RECEIVE MESSAGE FROM WEBSOCKET
    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;
        
        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;
            match msg {
                Message::Text(text) => {
                    // Handle text message
                    tracing::info!("Received text message: {}", text);
                    // crate::redis_manager::utils::set_redis_value::function("test", &text).unwrap();
                    let received_data: Result<DataExchange, _> = serde_json::from_str(&text);
                    match received_data {
                        Ok(received_data) => {
                            if received_data.sParam == "99" {
                                let struct_session = WsSession {
                                    received_pong: true,
                                };
                                let struct_session_bo = WsSession {
                                    received_pong: true,
                                };
                                let kp007 = "KP007";
                                let kp007bo = "KP007bo";
                                // crate::redis_manager::utils::set_redis_value::function(kp007, &serde_json::to_string(&struct_session).unwrap()).unwrap();
                                // crate::redis_manager::utils::set_redis_value::function(kp007bo, &serde_json::to_string(&struct_session_bo).unwrap()).unwrap();
                            }
                            if received_data.sParam == "999" {
                              tracing::info!("Enrôlement: Renvoi ACK_TO_AUTH sinon Alerte pour faux SecNav ou /et Faux Rproxy ou association non attendue (999)");
                              let send_to_channel = MsgChannel {
                                sockID: sock_id_clone4.clone(),
                                sCode: String::new(),
                                ip: String::new(),
                                sParam: "999".to_string(),
                                flag: "".to_string(),
                                from: "secSrv".to_string(),
                                to: "".to_string(),
                                data: text.clone(),
                              };
                              let send_data = serde_json::to_string(&send_to_channel).unwrap();
                            //   crate::redis_manager::channels::secmgnt::publish("secmgnt-channel", &send_data).await.unwrap();
                            }
                            if received_data.sParam == "777" {
                                crate::lib::reset_websocket::reset_websocket();
                                let _ = sender5.lock().await.send(Message::Close(None)).await;
                            }
                        }
                        Err(err) => {
                            tracing::error!("Error parsing JSON: {}", err);
                        }
                    }
                },
                Message::Binary(bin) => {
                    println!("Received binary message: {:?}", bin); // Handle binary message
                },
                Message::Close(_) => {
                    let _ = sender5.lock().await.send(Message::Close(None)).await;
                    println!("Received close message"); // Handle close message
                },
                Message::Ping(ping) => {
                    println!("Received ping message: {:?}", ping); // Handle ping message
                },
                Message::Pong(pong) => {
                    println!("Received pong message: {:?}", pong); // Handle pong message
                },
            }
        }
        cnt
        
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => println!("Received {b:?} messages"),
                Err(b) => println!("Error receiving messages {b:?}")
            }
            loop_ping.abort();
            // recv_msg_channel.abort();
        },
        // rv_pub = (&mut recv_msg_channel) => {
        //     match rv_pub {
        //         Ok(b) => println!("Received {b:?} messages"),
        //         Err(b) => println!("Error receiving messages {b:?}")
        //     }
        //     recv_task.abort();
        //     loop_ping.abort();
        // },
        rv_cnt_ping = (&mut loop_ping) => {
            match rv_cnt_ping {
                Ok(b) => println!("Received {b:?} messages"),
                Err(b) => println!("Error receiving messages {b:?}")
            }
            recv_task.abort();
            // recv_msg_channel.abort();
        }
    }

    // returning from the handler closes the websocket connection
    println!("Websocket context {who} destroyed");
}
