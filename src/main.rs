use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::str::FromStr;
use std::io::prelude::*;
use std::fs::File;
use std::fs;

use lazy_static::lazy_static;

use teloxide::prelude::*;
use tokio_stream::wrappers::UnboundedReceiverStream;
use teloxide::utils::command::BotCommand;
use teloxide::types::{MessageKind, MediaKind};

use crate::trapper::adauga::Expression;
use crate::trapper::Trapper;
use bimap::BiMap;
use crate::constants::*;

mod trapper;
mod constants;

lazy_static! {
    static ref ALIASES: Arc<Mutex<BiMap<String, i64>>> = {
        Arc::new(Mutex::new(load_aliases()))
    };
    static ref STATEMAP: Arc<Mutex<HashMap<i64, trapper::Trapper> > > = {
        Arc::new(Mutex::new(load_bot_data()))
    };
    static ref BOT_NAME: String = {
        std::env::var("BOT_NAME")
            .expect("BOT_NAME environment variable not set")
    };
    static ref ADMIN_ID: String = {
        std::env::var("ADMIN_ID")
            .expect("ADMIN_ID environment variable not set")
    };
}


fn save_bot_data() {
    log::info!("Saving all bot data");

    let mut statemap = STATEMAP.lock().unwrap();
    let mut all_commands: Vec<Expression> = Vec::new();
    let mut all_thoughts: Vec<(String, i64)> = Vec::new();
    let hash_keys: Vec<i64> = statemap.keys().map(|x| { *x } ).collect();

    for k in hash_keys {
        let trapper = statemap.remove(&k);

        if let Some(mut trapper) = trapper {
            while trapper.commands.len() > 0 {
                all_commands.push(trapper.commands.pop().unwrap());
            }
            while trapper.thoughts.len() > 0 {
                all_thoughts.push((trapper.thoughts.pop().unwrap(), k));
            }
        }
    }

    let serialized_comm = serde_json::to_string(&all_commands).unwrap();
    let serialized_thoughts = serde_json::to_string(&all_thoughts).unwrap();

    let file = File::create("data.JSON");
    match file {
    Ok(mut file) => {
        file.write_all(serialized_comm.as_bytes())
            .expect("Failed to write all the data in the file");
        
    }
    Err(x) => {
        log::info!("Failed to create data.JSON: {}", x);
    }
    }
    
    let file = File::create("thoughts.JSON");
    match file {
    Ok(mut file) => {
        file.write_all(serialized_thoughts.as_bytes())
            .expect("Failed to write all the data in the file");
    }
    Err(x) => {
        log::info!("Failed to create thoughts.JSON: {}", x);
    }
    }

    log::info!("Saved all bot data");
}

fn save_aliases() {
    log::info!("Saving all aliases");

    let aliases = ALIASES.lock().unwrap();
    let all_aliases: Vec<(String, i64)> = aliases.iter().map(|x| { (x.0.clone(), *x.1) } )
        .collect();

    let serialized = serde_json::to_string(&all_aliases).unwrap();

    let file = File::create("aliases.JSON");
    match file {
    Ok(mut file) => {
        file.write_all(serialized.as_bytes())
            .expect("Failed to write all the data in the file");
        log::info!("Saved all aliases");
    }
    Err(x) => {
        log::info!("Failed to create aliases.JSON: {}", x);
    }
    }
}

fn exit_program() {
    save_bot_data();
    save_aliases();

    log::info!("Shutting down bot...");
    std::process::exit(0);
}

fn load_bot_data() -> HashMap<i64, trapper::Trapper> {
    log::info!("Loading all commands");
    let deserialized = fs::read_to_string("data.JSON");

    let deserialized = match deserialized {
    Ok(x)  => { x }
    Err(x) => {
        log::error!("Failed to read data from data.JSON: {}", x);
        String::new()
    }
    };

    let all_commands = serde_json::from_str(&deserialized);
    let mut all_commands: Vec<Expression> = match all_commands {
    Ok(x) => { x }
    Err(x) => {
        log::error!("Failed deserializing data.JSON: {}", x);
        Vec::new()
    }
    };

    let mut statemap: HashMap<i64, trapper::Trapper> = HashMap::new();

    while !all_commands.is_empty() {
        let command = all_commands.pop().unwrap();
        
        let mut trapper = if let Some(x) = statemap.remove(&command.group_id) {
            x
        } else {
            Trapper::new()
        };
        
        let chat_id = command.group_id;
        trapper.commands.push(command);

        statemap.insert(chat_id, trapper);
    }
    
    log::info!("Loaded all commands");

    log::info!("Loading all thoughts");
    let deserialized = fs::read_to_string("thoughts.JSON");

    let deserialized = match deserialized {
    Ok(x) => { x }
    Err(x) => {
        log::info!("Failed to read data from thoughts.JSON: {}", x);
        String::new()
    }
    };

    let all_thoughts = serde_json::from_str(&deserialized);
    let mut all_thoughts: Vec<(String, i64)> = match all_thoughts {
    Ok(x) => { x }
    Err(x) => {
        log::info!("Failed deserializing thoughts.JSON: {}", x);
        Vec::new()
    }
    };

    while !all_thoughts.is_empty() {
        // this shouldn't panic
        let (thought, chat_id) = all_thoughts.pop().unwrap();
        let mut trapper = match statemap.remove(&chat_id) { 
            Some(x) => { x } 
            None => {trapper::Trapper::new()} 
        };
    
        trapper.thoughts.push(thought);
        statemap.insert(chat_id, trapper);
    }

    statemap
}

fn load_aliases() -> BiMap<String, i64> {
    log::info!("Loading all aliases");
    
    let deserialized = fs::read_to_string("aliases.JSON").unwrap();
    let mut all_aliases: Vec<(String, i64)> = serde_json::from_str(&deserialized).unwrap();
    let mut aliases: BiMap<String, i64> = BiMap::new();

    while !all_aliases.is_empty() {
        // this shouldn't panic
        let (alias, id) = all_aliases.pop().unwrap();
        aliases.insert(alias, id);
    }

    log::info!("Loaded all aliases");

    aliases
}

#[derive(BotCommand, Debug)]
#[command(rename="lowercase", parse_with="default")]
enum BotCommands {
    #[command(description = "Iane, joaco")]
    Joaco,
    #[command(description = "Taci in pula mea de bot handicapat")]
    Taci,
    #[command(description = "Adauga o comanda blana")]
    Adauga(String),
    #[command(description = "cu ce te ajut sacale?")]
    Help(String),
    #[command(description = "Cum ma cunoaste lumea in cartier")]
    Alias(String),
    #[command(description = "Ati spun un gind frumos de la altii")]
    Gind,
    #[command(description = "Gandesc")]
    Gindeste(String),

    #[command(description = "Noi fumam cioate in timp ce o dam")]
    Dao,
    #[command(description = "idk, fa ceva")]
    Ceva(String),
}

async fn add_command(command: String) -> Result<String, String> {
    let expression = Expression::from_str(&command);
    
    match expression {
    Err(error) => {
        Err(error)
    }  
    Ok(expression) => {
        let targeted_chat_id = expression.group_id;
        let mut statemap = STATEMAP.lock().unwrap();
        let mut trapper = match statemap.remove(&targeted_chat_id) {
        Some(x) => { x }
        None    => { Trapper::new() }
        };

        trapper.commands.push(expression);
        
        statemap.insert(targeted_chat_id, trapper);
        Ok("Hai ca am adaugat sacale".to_string())
    }
    }
}

async fn run_command(command: BotCommands, message: UpdateWithCx<AutoSend<Bot>, Message>) {
    match command {
    BotCommands::Joaco => {
        message.answer("https://www.youtube.com/watch?v=uMUaqROInGk")
            .await
            .log_on_error()
            .await;
    }
    BotCommands::Taci => {
        let user_id = if let MessageKind::Common(ref message) = message.update.kind {
            message.from.as_ref().unwrap().id // if this panics, fuck
        } else {
            0
        };

        if user_id.to_string() == ADMIN_ID.to_string() {
            message.answer("Bine coaie")
                .await
                .log_on_error()
                .await;
            exit_program();
        } else {
            message.answer("Da nu vrei sa-mi sugi tu pula ca sa taci tu?")
                .await
                .log_on_error()
                .await;
        }
    }
    BotCommands::Adauga(command) => {
        let chat_id = message.update.chat_id();
        let tokens: Vec<String> = command.split("~").map(|x| { x.to_string() }).collect();
        let command = if let MessageKind::Common(ref message) = message.update.kind {
            let user_id = message.from.as_ref().unwrap().id; // If this panics, fuck
            if tokens.len() == 3 {
                Ok(user_id.to_string() + &"~" + &command)
            } else if tokens.len() == 2 {
                match ALIASES.lock().unwrap().get_by_right(&chat_id) {
                Some(x) => { Ok(user_id.to_string() + &"~" + x + &"~" + &tokens[0] + 
                                                                 &"~" + &tokens[1]) }
                None    => { Err(WRONG_ALIAS.to_string()) }
                }
            } else {
                Err(BAD_SEPARATORS.to_string())
            }
        } else {
            Err("Ce plm mi-ai trimis aici".to_string())
        };
        
        let result_command = match command {
        Ok(command) => {
            match add_command(command).await { Ok(x) => {x} Err(x) => {x} }
        }
        Err(x) => { x }
        };
        
        message.answer(result_command)
            .await
            .log_on_error()
            .await;
    }
    BotCommands::Help(with_what) => {
        let help_message = match with_what.trim().to_lowercase().as_str() {
        HELP_ADAUGA_TAB => {
            HELP_ADAUGA
        }
        HELP_TACI_TAB => {
            HELP_TACI
        }
        HELP_JOACO_TAB => {
            HELP_JOACO
        }
        HELP_HELP_TAB => {
            HELP_HELP
        }
        HELP_ALIAS_TAB => {
            HELP_ALIAS
        }
        HELP_GIND_TAB => {
            HELP_GIND
        }
        _ => {
            HELP_DEFAULT
        }
        };

        message.answer(help_message)
            .await
            .log_on_error()
            .await;
    }
    BotCommands::Alias(alias) => {
        let response = match alias.as_str() {
        "" => {
            let chat_id = message.update.chat_id();
            let mut response = format!("Uite aici id-ul chatului sacale: {}\n", chat_id);
            response = response + &match ALIASES.lock().unwrap().get_by_right(&chat_id) {
            None => { String::new() }
            Some(x) => { format!("Uite aici aliasul chatului sacale: {}", x) }
            };
            response
        }
        _ => {
            let chat_id = message.update.chat_id();
            let mut aliases = ALIASES.lock().unwrap();
            
            match { aliases.get_by_left(&alias) } {
            Some(id) => {
                if *id != chat_id {
                    "Ce faci sacale, vrei sa furi clout?".to_string()
                } else {
                    "Ce faci ma, ai pus deja aliasul asta esti prajit?".to_string()
                }
            }
            None => {
                let ans = format!("Ti-am schimbat aliasul in: {}", alias);
                aliases.remove_by_right(&chat_id);
                aliases.insert(alias, chat_id);
                ans
            }
            }
        }
        };
        message.answer(response)
            .await
            .log_on_error()
            .await;
    }
    BotCommands::Gindeste(gind) => {
        let tokens: Vec<String> = gind.split("~").map(|x| {x.to_string()}).collect();
        
        let result: Result<(i64, String), String> = match tokens.len() {
        0 => {
            Ok((message.update.chat_id(), String::new()))
        }
        1 => {
            Ok((message.update.chat_id(), tokens[0].clone()))
        }
        _ => { 
            let mut concatenated = tokens[1].clone();
            for i in 2..tokens.len() {
                concatenated = concatenated + &"~" + &tokens[i];
            }
            

            match ALIASES.lock().unwrap().get_by_left(&tokens[0]) {
            Some(x) => { Ok((*x, concatenated)) }
            None    => {
                let result = i64::from_str(&tokens[0]);
                match result {
                Ok(x)  => { Ok((x, concatenated)) }
                Err(x) => { Err(x.to_string()) }
                }
            }
            }
        }
        };

        let response = match result {
        Ok((chat_id, gind)) => {
            let mut statemap = STATEMAP.lock().unwrap();
            let mut trapper = match statemap.remove(&chat_id) {
            None => {
                trapper::Trapper::new()
            }
            Some(x) => { x }
            };
        
            trapper.thoughts.push(gind);
            statemap.insert(chat_id, trapper);
            "Am bagat un gind frumos".to_string()
        }
        Err(x) => {
            x
        }
        };

        message.answer(response)
            .await
            .log_on_error()
            .await;
    }
    BotCommands::Gind => {
        let chat_id = message.update.chat_id();
        let response = {
            let mut statemap = STATEMAP.lock().unwrap();
            let mut trapper = match statemap.remove(&chat_id) {
                None => { trapper::Trapper::new() }
                Some(x) => { x }
            };
        
            trapper.shuffle_thoughts();
            let gind = trapper.thoughts.pop();
            statemap.insert(chat_id, trapper);
            
            match gind {
            Some(x) => {x}
            None => { "Nu gindesc, deci nu exist".to_string() }
            }
            
        };

        message.answer(response)
            .await
            .log_on_error()
            .await;
    }
    _ => {
    }
    };
}

pub async fn process_message(message: UpdateWithCx<AutoSend<Bot>, Message>) {
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
        run_command(command, message).await;
    } else {
        message_text.make_ascii_lowercase();
        let words: Vec<&str> = message_text
            .split(|x: char| { !x.is_alphanumeric() })
            .collect();
        
        let mut words_map: HashMap<&str, ()> = HashMap::new();

        for i in words { words_map.insert(i, ()); }
        
        let response = {
            let mut statemap = STATEMAP.lock().unwrap();
            let trapper = statemap.remove(&message.update.chat_id());

            if let Some(mut trapper) = trapper {
                trapper.shuffle_commands();
                let response = if let Some(command) = trapper.commands.iter().find(|x| { x.eval(&words_map) } ) {
                    command.response.clone()
                } else {
                    "".to_string()
                };
                statemap.insert(message.update.chat_id(), trapper);
                response
            } else {
                "".to_string()
            }
        };
        
        if response != "".to_string() {
            message.answer(response)
                .await
                .log_on_error()
                .await;
        }
    }
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting shared_state_bot...");
    
    ctrlc::set_handler(exit_program).expect("Failed to set handler");

    let bot = Bot::from_env().auto_send();
    
    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<AutoSend<Bot>, Message>| {
            UnboundedReceiverStream::new(rx)
                .for_each_concurrent(None, |message| async move {
                    process_message(message).await;
                })
        })
        .dispatch()
        .await;
}
