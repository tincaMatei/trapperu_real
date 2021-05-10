// This bot answers how many messages it received in total on every message.

use std::collections::HashMap;

use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use teloxide::prelude::*;

mod trapper;

lazy_static! {
    static ref STATEMAP: Arc<Mutex<HashMap<i64, trapper::Trapper> > > = {
        Arc::new(Mutex::new(HashMap::new()))
    };
}


#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting shared_state_bot...");

    let bot = Bot::from_env();
    
    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.for_each_concurrent(None, |message| async move {
                let chat = message.update.chat_id();
                let trapper = if let Some(x) = STATEMAP.lock().unwrap().remove(&chat) {
                    x
                } else {
                    trapper::Trapper::new()
                };

                trapper.process_message(message).await;
                STATEMAP.lock().unwrap().insert(chat, trapper);
            })
        })
        .dispatch()
        .await;
}
