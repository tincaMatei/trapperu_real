use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use rand::prelude::*;

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, Clone)]
pub struct Markov {
    chain: HashMap<String, Vec<(String, i64)>>,
}

impl Markov {
    pub fn new() -> Markov {
        Markov {
            chain: HashMap::new()
        }
    }

    pub fn add_sequence(&mut self, message: String) {
        let mut words: Vec<String> = message.split(" ")
            .filter(|x| { x != &"" })
            .map(|x| { String::from(x) } )
            .collect();
        
        words.insert(0, String::from(""));
        words.push(String::from(""));
    
        for i in 0..words.len() - 1 {
            let str1 = words[i].clone();
            let str2 = words[i + 1].clone();

            let mut id = 0;
            let mut edge = self.chain.remove(&str1);
            
            let to_insert = match edge {
            Some(mut x) => {
                let mut id = 0;
                while id < x.len() && x[id].0 != str2 {
                    id += 1;
                }
                
                if id == x.len() {
                    x.push((str2, 1));
                } else {
                    x[id].1 += 1;
                }

                x
            }
            None => {
                vec![(str2, 1)]
            }
            };

            self.chain.insert(str1, to_insert);
        }
    }

    fn get_random_edge(&mut self, from: String) -> Option<String> {
        let edges = self.chain.remove(&from);

        match edges {
        None => {
            None
        }
        Some(x) => {
            let mut sum: i64 = 0;
            for edge in &x {
                sum = sum + edge.1;
            }
            let mut id: i64 = rand::random::<i64>() % sum;
            let mut res_id = 0;

            while id >= x[res_id].1 {
                id = id - x[res_id].1;
                res_id += 1;
            }

            let res_string = x[res_id].0.clone();
            self.chain.insert(from, x);

            Some(res_string)
        }
        }
    }

    pub fn get_random(&mut self) -> Option<String> {
        let last_string = self.get_random_edge(String::new());
        let mut last_string = match last_string {
        None => { return None; }
        Some(x) => { x }
        };
        
        let mut final_message = last_string.clone();

        while last_string != String::new() {
            last_string = match self.get_random_edge(last_string) {
            None => { return None; }
            Some(x) => { x }
            };
            
            if last_string != String::new() {
                final_message = final_message + &" " + &last_string;
            }
        }

        Some(final_message)
    }
}

#[cfg(test)]
mod tests {
    use crate::Markov;
    use std::collections::HashMap;
    #[test]
    fn building_markov() {
        let mut markov = Markov::new();
        
        markov.add_sequence("a b".to_string());
        markov.add_sequence("a c".to_string());
        markov.add_sequence("a b".to_string());
    
        let mut hashmap: HashMap<String, Vec<(String, i64)>> = HashMap::new();
        hashmap.insert("".to_string(), vec![("a".to_string(), 3)]);
        hashmap.insert("a".to_string(), vec![("b".to_string(), 2), 
                                             ("c".to_string(), 1)]);
        hashmap.insert("b".to_string(), vec![("".to_string(), 2)]);
        hashmap.insert("c".to_string(), vec![("".to_string(), 1)]);
    
        assert_eq!(Markov {
            chain: hashmap
        }, markov);
    }
    
    #[test]
    fn generate_test() {
        let mut markov = Markov::new();

        markov.add_sequence("a b c".to_string());
        assert_eq!(markov.get_random(), Some("a b c".to_string()));
    }

    #[test]
    fn null_test() {
        let mut markov = Markov::new();
        assert_eq!(markov.get_random(), None);
    }
}

