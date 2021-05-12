use std::str::FromStr;
use std::collections::HashMap;

const BAD_SEPARATORS: &str = "N-ai pus '~' sau ai pus prea multi bombardiere";
const BAD_PARANTHESES: &str = "Ai belit parantezele la expresie bombardiere";
const BAD_CHARACTERS: &str = "Nush ce plm ai facut dar nu era corect";
const BAD_OPERATOR: &str = "Wtf is this";

#[derive(Debug, PartialEq)]
pub struct Expression {
    added_by: i64,
    expr: ExpressionTree,
    pub response: String,
}

#[derive(Debug, PartialEq)]
enum ExpressionTree {
    Variable(String),
    OrSign(Box<(ExpressionTree, ExpressionTree)>),
    AndSign(Box<(ExpressionTree, ExpressionTree)>),
}

impl ExpressionTree {
    fn eval(&self, words: &HashMap<&str, ()>) -> bool {
        match self {
        ExpressionTree::Variable(word) => { words.get(word.as_str()).is_some() }
        ExpressionTree::OrSign(children) => {
            let res_left = children.0.eval(words);

            if res_left {
                true
            } else {
                children.1.eval(words)
            }
        }
        ExpressionTree::AndSign(children) => {
            let res_left = children.0.eval(words);

            if !res_left {
                false
            } else {
                children.1.eval(words)
            }
        }
        }
    }
}

// Expression Id~ExpressionTree~message
// ExpressionTree = Var &| Var &| ... &| Var
// Var = String
//     = (Expr)

fn parse_variable(mut token: &[u8]) -> Result<(ExpressionTree, &[u8]), &'static str> {
    if !token.is_empty() && token[0] == b'(' {
        token = &token[1..];

        let (expression, remainder) = parse_expr(token)?;
        token = &remainder;

        if !token.is_empty() && token[0] == b')' {
            token = &token[1..];
            Ok((expression, token))
        } else {
            Err(BAD_PARANTHESES)
        }
    } else if !token.is_empty() && (token[0] as char).is_alphanumeric() {
        let mut output = String::new();
        while !token.is_empty() && (token[0] as char).is_alphanumeric() {
            let mut letter = token[0] as char;
            letter.make_ascii_lowercase();
            output.push(letter);
            token = &token[1..];
        }
        Ok((ExpressionTree::Variable(output), token))
    } else {
        Err(BAD_CHARACTERS)
    }
}

fn parse_expr(mut token: &[u8]) -> Result<(ExpressionTree, &[u8]), &'static str> {
    let (mut expression, remainder) = parse_variable(token)?;
    token = &remainder;

    while !token.is_empty() && (token[0] == b'&' || token[0] == b'|') {
        let operation = token[0];
        token = &token[1..];
        
        let (additional_expression, remainder) = parse_variable(token)?;
        token = &remainder;
    
        expression = match operation {
        b'&' => { ExpressionTree::AndSign(Box::new((expression, additional_expression))) }
        b'|' => { ExpressionTree::OrSign(Box::new((expression, additional_expression))) }
        _    => { return Err(BAD_OPERATOR); } // lmao this never happens
        }
    }

    Ok((expression, token))
}

fn parse(token: String) -> Result<ExpressionTree, &'static str> {
    let (expression, remainder) = parse_expr(token.as_bytes())?;
    
    if !remainder.is_empty() {
        Err(BAD_PARANTHESES)
    } else {
        Ok(expression)
    }
}

impl Expression {
    pub fn eval(&self, words: &HashMap<&str, ()>) -> bool {
        self.expr.eval(words)
    }
}

impl FromStr for Expression {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        let tokens: Vec<&str> = s.split('~').collect();
        
        if tokens.len() != 3 {
            Err(BAD_SEPARATORS)
        } else {
            Ok(Expression {
                added_by: i64::from_str(tokens[0]).unwrap(),
                expr: {
                    let mut cpy = tokens[1].to_string();
                    cpy.retain(|x| { x != ' ' });
                    parse(cpy)?
                },
                response: tokens[2].to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_test() {
        assert_eq!(
            Expression::from_str("1256262~asDf    |    milsugi     & (coaie | pula)~test"), 
            Ok(Expression {
                added_by: 1256262,
                expr: 
                ExpressionTree::AndSign(Box::new((
                    ExpressionTree::OrSign(Box::new((
                        ExpressionTree::Variable("asdf".to_string()),
                        ExpressionTree::Variable("milsugi".to_string())
                    ))),
                    ExpressionTree::OrSign(Box::new((
                        ExpressionTree::Variable("coaie".to_string()),
                        ExpressionTree::Variable("pula".to_string())
                    )))
                ))),
                response: "test".to_string(),
            })
        )
    }

    #[test]
    fn bad_separators() {
        assert_eq!(Expression::from_str("125~asdf|milsugi|(coaie|pula)~test~test"),
            Err(BAD_SEPARATORS));
    }

    #[test]
    fn not_enough_parantheses() {
        assert_eq!(Expression::from_str("1262~asdf|milsugi|(coaie|pula~test"),
            Err(BAD_PARANTHESES));
    }
    
    #[test]
    fn too_many_parantheses() {
        assert_eq!(Expression::from_str("1262~asdf|milsugi|(coaie|pula))~test"),
            Err(BAD_PARANTHESES));
    }

    #[test]
    fn bad_operator() {
        assert_eq!(Expression::from_str("1262~asdf|milsugi^(coaie|pula)~test"),
            Err(BAD_PARANTHESES));
    }

    #[test]
    fn illegal_characters() {
        assert_eq!(Expression::from_str("1262~asdf|.milsugi|(coaie|pula)~test"),
            Err(BAD_CHARACTERS));
    }
}

