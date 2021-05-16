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

mod trapper;

lazy_static! {
    static ref STATEMAP: Arc<Mutex<HashMap<i64, trapper::Trapper> > > = {
        Arc::new(Mutex::new(load_commands()))
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

const HELP_DEFAULT: &str = "Cel mai adevarat bot, va arat cum se face smecherie.

Lectii in smecherie, pe capitole (scrie randurile alea complet ca sa vezi capitolul):
/help joaco
/help adauga
/help taci
/help alias
/help help

Celalalte comenzi de pe acolo care mai apar momentan sunt la harneala, mai aveti rabdare
";

const HELP_ALIAS_TAB: &str = "alias";
const HELP_ALIAS: &str = "Schimb aicea numele si chestii pe aici:

/alias
Aici iti zic id-ul grupului si aliasul grupului.

/alias [nume]    (**NEIMPLEMENTAT**)
Tin minte ca daca zici de acuma [nume], te referi defapt la id, poti sa faci direct in loc de:
/adauga [ceva numar random]~[Expresie]~[Mesaj]
poti sa faci
/alias coaie
/adauga coaie~[Expresie]~[Mesaj]
";

const HELP_HELP_TAB: &str = "help";
const HELP_HELP: &str = "Te ajut in pula mea calmeaza-te";

const HELP_TACI_TAB: &str = "taci";
const HELP_TACI: &str = "Incearca sa vezi ce face /taci, hai coaie te provoc, nu te tine";

const HELP_JOACO_TAB: &str = "joaco";
const HELP_JOACO: &str = "[LYRICS]
(Azteca)
Sa moara familia mea
(Ian)
lasama
(Azteca)
Sa moara familia mea
(Ian)
lasama ba lasama
(???)
BA BAAA
(Ian)
Ce-i cu figurile astea pa tine?
(Oscar)
Ba, ba! Joaco!
(Ian)
Leilaaa! Leilalala leila leilaa aaaaaaaaaaaaaaaaaaaaaaaa";

const HELP_ADAUGA_TAB: &str = "adauga";
const HELP_ADAUGA: &str = "Fii atent coaie te arat cum se face smecherie cu adaugatul.

Ai urmatoarele variante:
/adauga [Expresie]~[Mesaj]
/adauga [Id]~[Expresie]~[Mesaj]

[Mesaj] este un mesaj oarecare cu care raspund daca expresia [Expresie] este adevarata.

Unde la [Expresie] ai ceva de genul \"a&b|(c|d&e)\", unde a, b, c, d si e sunt chestii cu \
litere si cifre, doar astea, sa nu aiba altele dintre care si spatii (daca bagi spatii \
vezi ca le ignor direct si daca ai o expresie \"a b\", atunci se triggereste daca zici ab). \
Daca expresia este corecta dpdv gramatical, atunci cand cineva trimite un mesaj, eu o sa \
inlocuiesc fiecare cuvant din expresia aia cu adevarat daca apare sau fals daca nu apare \
si daca la sfarsit expresia este adevarata, atunci o sa zic mesajul de mai sus.

De exemplu, daca dau comanda \"/adauga a&(b|c)~test\", o sa raspunc tu \"test\" daca cineva \
zice cuvantul a si unul din cuvintele b si c absolut oriunde in propozitie.

[Id] este id-ul unui grup. Daca vrei sa bagi o comanda, dar sa nu o vada ceilalti din grup, \
bagi acolo id-ul la inceput si trimiti mesajul ala la mine in privat. Cum afli id-ul grupului? \
Idk sincer, asteapta sa apara comanda /alias.";

fn exit_program() {
    let mut statemap = STATEMAP.lock().unwrap();
    let mut all_commands: Vec<Expression> = Vec::new();

    let hash_keys: Vec<i64> = statemap.keys().map(|x| { *x } ).collect();

    for k in hash_keys {
        let trapper = statemap.remove(&k);

        if let Some(mut trapper) = trapper {
            while trapper.commands.len() > 0 {
                all_commands.push(trapper.commands.pop().unwrap());
            }
        }
    }

    let serialized = serde_json::to_string(&all_commands).unwrap();

    let mut file = File::create("data.JSON").unwrap();
    file.write_all(serialized.as_bytes())
        .expect("Failed to write all the data in the file");

    log::info!("Shutting down bot...");
    std::process::exit(0);
}

fn load_commands() -> HashMap<i64, trapper::Trapper> {
    let deserialized = fs::read_to_string("data.JSON").unwrap();
    let mut all_commands: Vec<Expression> = serde_json::from_str(&deserialized).unwrap();
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
    
    statemap
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

    #[command(description = "Adauga un gind frumos")]
    Gind,
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
        let mut command = command.to_string();
        let chat_id = message.update.chat_id();
        if let MessageKind::Common(ref message) = message.update.kind {
            let user_id = message.from.as_ref().unwrap().id; // If this panics, fuck
            
            if command.rmatches("~").count() == 1 {
                command = chat_id.to_string() + &"~".to_string() + &command;
            }

            if command.rmatches("~").count() == 2 {
                command = user_id.to_string() + &"~".to_string() + &command;
            }
        }
        
        let result_command = match add_command(command).await { Ok(x) => {x} Err(x) => {x} } ;
        
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
        match alias.as_str() {
        "" => {
            let chat_id = message.update.chat_id();
            message.answer(format!("Uite aici id-ul chatului sacale: {}", chat_id))
                .await
                .log_on_error()
                .await;
        }
        _ => {

        }
        }
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
