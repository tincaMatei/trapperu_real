use teloxide::prelude::*;

pub mod adauga;
pub mod dao;

use crate::trapper::adauga::Expression;
use crate::trapper::dao::Markov;
use rand::thread_rng;
use rand::prelude::SliceRandom;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trapper {
    pub commands: Vec<Expression>,
    pub thoughts: Vec<String>,
    pub markov: Markov,
}

impl Trapper {
    pub fn new() -> Trapper {
        Trapper {
            commands: vec![],
            thoughts: vec![],
            markov: Markov::new(),
        }
    }

    pub fn shuffle_commands(&mut self) {
        self.commands.shuffle(&mut thread_rng());
    }

    pub fn shuffle_thoughts(&mut self) {
        self.commands.shuffle(&mut thread_rng());
    }
}

