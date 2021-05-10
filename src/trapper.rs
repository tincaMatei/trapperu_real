use teloxide::prelude::*;
use teloxide::utils::command::BotCommand;
use teloxide::types::{MessageKind, MediaKind};

use lazy_static::lazy_static;

lazy_static! {
    static ref BOT_NAME: String = {
        let bot_name = std::env::var("BOT_NAME")
            .expect("BOT_NAME environment variable not set");
        println!("lazy_static: {:?}", bot_name);
        "asdf".to_string()
    };
}

#[derive(BotCommand, Debug)]
#[command(rename="lowercase")]
enum BotCommands {
    #[command(description = "Iane, joaco")]
    Joaco,
    #[command(description = "idk, fa ceva")]
    Ceva(String),
}

#[derive(Debug)]
pub struct Trapper {
    cnt: i32
}

impl Trapper {
    pub fn new() -> Trapper {
        Trapper {
            cnt: 0
        }
    }

    pub fn _increment(&mut self) {
        self.cnt += 1;
    }

    pub fn _get_value(&self) -> i32 {
        self.cnt
    }
    
    async fn run_command(&self, command: BotCommands, message: UpdateWithCx<Message>) {
        match command {
        BotCommands::Joaco => {
            message.answer_str("https://www.youtube.com/watch?v=uMUaqROInGk")
                .await
                .log_on_error()
                .await;
        }
        _ => {

        }
        };
    }

    pub async fn process_message(&self, message: UpdateWithCx<Message>) {
        let message_text = if let MessageKind::Common(ref message) = message.update.kind {
            if let MediaKind::Text(ref message_text) = message.media_kind {
                message_text.text.clone()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let command = BotCommands::parse(&message_text, BOT_NAME.clone());
        log::info!("{:?}", command);

        if let Ok(command) = command {
            self.run_command(command, message).await;
        } else {
            message.answer_str(message_text)
                .await
                .log_on_error()
                .await;
        }
    }
}

