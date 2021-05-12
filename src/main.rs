use std::collections::HashMap;

use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use teloxide::prelude::*;
use tokio_stream::wrappers::UnboundedReceiverStream;

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

    let bot = Bot::from_env().auto_send();
    
    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<AutoSend<Bot>, Message>| {
            UnboundedReceiverStream::new(rx)
                .for_each_concurrent(None, |message| async move {
                    let chat = message.update.chat_id();
                    let mut trapper = if let Some(x) = STATEMAP.lock().unwrap().remove(&chat) {
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

    log::info!("Coaie");
}
