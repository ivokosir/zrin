use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Name {
    IName(u32),
    Name(String),
}

#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(transparent)]
pub struct Label {
    pub id: u32,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Constant {
    CInt(i32),
}

#[derive(Deserialize, Serialize)]
pub enum Type {
    #[serde(rename = "i32")]
    TInt,
    #[serde(rename = "void")]
    TVoid,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Value {
    Constant(Constant),
    Reference { name: Name },
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "tag", rename_all = "snake_case")]
pub enum InstructionBase {
    Binary {
        op: String,
        lhs: Value,
        rhs: Value,
    },
    Call {
        caller: Value,
        arg: Value,
    },
    Phi {
        #[serde(rename = "then")]
        then_: Value,
        then_label: Label,
        #[serde(rename = "else")]
        else_: Value,
        else_label: Label,
    },
    Struct {
        values: Vec<Value>,
    },
}

#[derive(Deserialize, Serialize)]
pub struct Instruction {
    pub name: Name,
    #[serde(flatten)]
    pub instruction_base: InstructionBase,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "tag", rename_all = "snake_case")]
pub enum Terminator {
    Ret {
        value: Value,
    },
    CondBr {
        cond: Value,
        #[serde(rename = "then")]
        then_: Label,
        #[serde(rename = "else")]
        else_: Label,
    },
    Br {
        label: Label,
    },
}

#[derive(Deserialize, Serialize)]
pub struct Block {
    pub label: Label,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

#[derive(Deserialize, Serialize)]
pub struct Function {
    pub name: Name,
    pub ret_type: Type,
    pub param: Name,
    pub param_type: Type,
    pub blocks: Vec<Block>,
}

#[derive(Deserialize, Serialize)]
pub struct Module {
    pub name: String,
    pub functions: Vec<Function>,
}
