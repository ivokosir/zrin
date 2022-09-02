use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Identifier {
    name: String,
    #[serde(default)]
    id: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "tag", rename_all = "snake_case")]
pub enum Expression {
    Body {
        body: Vec<Expression>,
    },
    Name {
        name: Identifier,
        body: Box<Expression>,
    },
    Function {
        param: Identifier,
        body: Box<Expression>,
    },
    Op {
        op: String,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Reference(Identifier),
    Literal {
        value: i32,
    },
    Call {
        caller: Box<Expression>,
        arg: Box<Expression>,
    },
    If {
        cond: Box<Expression>,
        #[serde(rename = "then")]
        then_: Box<Expression>,
        #[serde(rename = "else")]
        else_: Box<Expression>,
    },
}
