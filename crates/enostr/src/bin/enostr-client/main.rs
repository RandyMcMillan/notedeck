use enostr::{ClientMessage, RelayMessage};
use ewebsock::{Options, WsEvent, WsMessage, WsReceiver, WsSender};
use nostrdb::Filter;
use std::collections::VecDeque;
use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mut args = std::env::args();
    let _ = args.next(); // skip program name
    let relay_url = args.next().unwrap_or("ws://localhost:8080".into());

    let (ws_sender, ws_receiver) = ewebsock::connect(relay_url, Options::default()).unwrap();

    let mut messages = VecDeque::new();
    messages.push_back(
        ClientMessage::req("sub-1".into(), vec![Filter::new().build()])
            .to_json()
            .unwrap(),
    );

    let mut sub_id = String::new();
    let mut filters = Vec::new();

    event_loop(
        ws_sender,
        ws_receiver,
        &mut messages,
        &mut sub_id,
        &mut filters,
    );
}

fn event_loop(
    mut ws_sender: WsSender,
    ws_receiver: WsReceiver,
    messages: &mut VecDeque<String>,
    sub_id: &mut String,
    filters: &mut Vec<Filter>,
) {
    loop {
        if let Some(msg) = messages.pop_front() {
            ws_sender.send(WsMessage::Text(msg));
        }
        if let Some(event) = ws_receiver.try_recv() {
            match event {
                WsEvent::Message(msg) => {
                    if let WsMessage::Text(text) = msg {
                        match RelayMessage::from_json(&text) {
                            Ok(relay_msg) => {
                                //                                println!(
                                //                                  "Received: {}",
                                //                                serde_json::to_string(&relay_msg).unwrap()
                                //                          );
                            }
                            Err(e) => {
                                tracing::error!("Failed to parse relay message: {}", e);
                            }
                        }
                    }
                }
                WsEvent::Opened => {
                    tracing::info!("WebSocket connection opened.");
                    let req_msg = ClientMessage::req(sub_id.clone(), filters.clone());
                    ws_sender.send(WsMessage::Text(req_msg.to_json().unwrap()));
                }
                WsEvent::Closed => {
                    tracing::info!("WebSocket connection closed.");
                    break;
                }
                WsEvent::Error(e) => {
                    tracing::error!("WebSocket error: {}", e);
                }
            }
        }
    }
}
