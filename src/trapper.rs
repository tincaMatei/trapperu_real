use teloxide::prelude::*;
use teloxide::utils::command::BotCommand;
use teloxide::types::{MessageKind, MediaKind};

use lazy_static::lazy_static;

mod adauga;
use crate::trapper::adauga::Expression;
use std::str::FromStr;

use std::collections::HashMap;

use rand::seq::SliceRandom;
use rand::thread_rng;

lazy_static! {
    static ref BOT_NAME: String = {
        std::env::var("BOT_NAME")
            .expect("BOT_NAME environment variable not set")
    };
    static ref ADMIN_ID: String = {
        std::env::var("ADMIN_ID")
            .expect("ADMIN_ID environment variable not set")
    };
}

#[derive(BotCommand, Debug)]
#[command(rename="lowercase")]
enum BotCommands {
    #[command(description = "Iane, joaco")]
    Joaco,
    #[command(description = "Taci in pula mea de bot handicapat")]
    Taci,
    #[command(description = "Adauga o comanda blana")]
    Adauga(String),
    
    #[command(description = "Adauga un gind frumos")]
    Gind,
    #[command(description = "cu ce te ajut sacale?")]
    Help,
    #[command(description = "Noi fumam cioate in timp ce o dam")]
    Dao,
    #[command(description = "idk, fa ceva")]
    Ceva(String),
}

#[derive(Debug)]
pub struct Trapper {
    cnt: i32,
    commands: Vec<Expression>,
}

impl Trapper {
    pub fn new() -> Trapper {
        Trapper {
            cnt: 0,
            commands: vec![],
        }
    }

    pub fn _increment(&mut self) {
        self.cnt += 1;
    }

    pub fn _get_value(&self) -> i32 {
        self.cnt
    }
    
    async fn run_command(&mut self, command: BotCommands, message: UpdateWithCx<AutoSend<Bot>, Message>) {
        match command {
        BotCommands::Joaco => {
            message.answer("https://www.youtube.com/watch?v=uMUaqROInGk")
                .await
                .log_on_error()
                .await;
        }
        BotCommands::Taci => {
            let chat = message.update.chat_id();

            if chat.to_string() == ADMIN_ID.to_string() {
                message.answer("Bine coaie")
                    .await
                    .log_on_error()
                    .await;
                std::process::exit(0);
            } else {
                message.answer("Da nu vrei sa-mi sugi tu pula ca sa taci tu?")
                    .await
                    .log_on_error()
                    .await;
            }
        }
        BotCommands::Adauga(command) => {
            let mut command = command.to_string();
            if let MessageKind::Common(ref message) = message.update.kind {
                let user_id = message.from.as_ref().unwrap().id; // If this panics, fuck
                command = user_id.to_string() + &"~".to_string() + &command.to_string();
            }
            
            let expression = Expression::from_str(&command);
            
            if let Err(error) = expression {
                message.answer(error)
                    .await
                    .log_on_error()
                    .await;
            } else if let Ok(expression) = expression {
                self.commands.push(expression);
                message.answer(format!("{:?}", self.commands))
                    .await
                    .log_on_error()
                    .await;
            }
        }
        _ => {

        }
        };
    }

    pub async fn process_message(&mut self, message: UpdateWithCx<AutoSend<Bot>, Message>) {
        let mut message_text = if let MessageKind::Common(ref message) = message.update.kind {
            if let MediaKind::Text(ref message_text) = message.media_kind {
                message_text.text.clone()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let command = BotCommands::parse(&message_text, BOT_NAME.clone());

        if let Ok(command) = command {
            self.run_command(command, message).await;
        } else {
            message_text.make_ascii_lowercase();
            let words: Vec<&str> = message_text
                .split(|x: char| { !x.is_alphanumeric() })
                .collect();
            
            let mut words_map: HashMap<&str, ()> = HashMap::new();

            for i in words { words_map.insert(i, ()); }
            
            self.commands.shuffle(&mut thread_rng());

            if let Some(command) = self.commands.iter().find(|x| { x.eval(&words_map) } ) {
                message.answer(command.response.clone())
                    .await
                    .log_on_error()
                    .await;
            }
        }
    }
}

