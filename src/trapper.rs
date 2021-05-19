use teloxide::prelude::*;

pub mod adauga;

use crate::trapper::adauga::Expression;
use rand::thread_rng;
use rand::prelude::SliceRandom;

#[derive(Debug)]
pub struct Trapper {
    pub commands: Vec<Expression>,
    pub thoughts: Vec<String>,
}

impl Trapper {
    pub fn new() -> Trapper {
        Trapper {
            commands: vec![],
            thoughts: vec![],
        }
    }

    pub fn shuffle_commands(&mut self) {
        self.commands.shuffle(&mut thread_rng());
    }

    pub fn shuffle_thoughts(&mut self) {
        self.commands.shuffle(&mut thread_rng());
    }
}

