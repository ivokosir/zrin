use std::{collections::HashMap, mem};

use crate::{code::*, expression::Expression};

struct State {
    scope: HashMap<String, Value>,
    requested_name: Option<String>,
    next_label: Label,
    next_iname: u32,
    functions: Vec<Function>,
    current_label: Label,
    current_blocks: Vec<Block>,
    current_instructions: Vec<Instruction>,
}

fn get_initial_state() -> State {
    State {
        scope: HashMap::new(),
        requested_name: None,
        next_label: Label { id: 1 },
        next_iname: 0,
        functions: Vec::new(),
        current_label: Label { id: 0 },
        current_blocks: Vec::new(),
        current_instructions: Vec::new(),
    }
}

impl State {
    fn generate_name(&mut self, name: Option<String>) -> Name {
        match name {
            Some(name) => Name::Name(name),
            None => {
                let i = self.next_iname;
                self.next_iname = i + 1;
                Name::IName(i)
            }
        }
    }

    fn generate_label(&mut self) -> Label {
        let label = self.next_label;
        self.next_label = Label { id: label.id + 1 };
        return label;
    }

    fn add_instruction(&mut self, instruction: Instruction) -> Value {
        let name = instruction.name.clone();
        self.current_instructions.push(instruction);
        return Value::Reference { name };
    }

    fn terminate_block(&mut self, terminator: Terminator, next_label: Label) -> () {
        let label = self.current_label;
        self.current_label = next_label;

        let instructions = mem::replace(&mut self.current_instructions, Vec::new());

        let block = Block {
            label,
            instructions,
            terminator,
        };
        self.current_blocks.push(block)
    }

    fn set_name(&mut self, name: String, value: Value) -> () {
        self.scope.insert(name, value);
    }

    fn generate(&mut self, expression: Expression) -> Value {
        match expression {
            Expression::Body { mut body } => {
                let old_scope = self.scope.clone();

                let requested_name = self.requested_name.take();
                let last = body.pop().expect("Unexpected empty body");

                for expression in body {
                    self.generate(expression);
                }

                self.requested_name = requested_name;
                let value = self.generate(last);

                self.scope = old_scope;

                value
            }
            Expression::Name { name, body } => {
                let parent_name = self.requested_name.take();
                let name = parent_name.unwrap_or(name);
                self.requested_name = Some(name.clone());

                let old_scope = self.scope.clone();
                let value = self.generate(*body);
                self.scope = old_scope;

                self.requested_name.take();
                self.set_name(name, value.clone());

                value
            }
            Expression::Function { param, body } => {
                let requested_name = self.requested_name.take();
                let name = self.generate_name(requested_name);
                let mut functions = generate_function(name.clone(), Type::TInt, param, *body);
                self.functions.append(&mut functions);
                Value::Reference { name }
            }
            Expression::Op { op, lhs, rhs } => {
                let requested_name = self.requested_name.take();

                let old_scope = self.scope.clone();
                let lhs_value = self.generate(*lhs);
                self.scope = old_scope.clone();

                let rhs_value = self.generate(*rhs);
                self.scope = old_scope;

                let name = self.generate_name(requested_name);
                self.add_instruction(Instruction {
                    name,
                    instruction_base: InstructionBase::Binary {
                        op,
                        lhs: lhs_value,
                        rhs: rhs_value,
                    },
                })
            }
            Expression::Reference(Name { name, id: _ }) => self
                .scope
                .get(&name)
                .expect(format!("Can't find name {name}").as_str())
                .clone(),
            Expression::Literal { value } => Value::Constant(Constant::CInt(value)),
            Expression::Call { caller, arg } => {
                let requested_name = self.requested_name.take();

                let old_scope = self.scope.clone();
                let arg_value = self.generate(*arg);
                self.scope = old_scope.clone();

                let caller_value = self.generate(*caller);
                self.scope = old_scope;

                let name = self.generate_name(requested_name);
                self.add_instruction(Instruction {
                    name,
                    instruction_base: InstructionBase::Call {
                        caller: caller_value,
                        arg: arg_value,
                    },
                })
            }
            Expression::If { cond, then_, else_ } => {
                let requested_name = self.requested_name.take();

                let old_scope = self.scope.clone();
                let cond_value = self.generate(*cond);
                self.scope = old_scope;
                let then_label = self.generate_label();
                let else_label = self.generate_label();
                self.terminate_block(
                    Terminator::CondBr {
                        cond: cond_value,
                        then_: then_label,
                        else_: else_label,
                    },
                    then_label,
                );

                let old_scope = self.scope.clone();
                let then_value = self.generate(*then_);
                self.scope = old_scope;
                let then_label = self.current_label;
                let end_label = self.generate_label();
                self.terminate_block(Terminator::Br { label: end_label }, else_label);

                let old_scope = self.scope.clone();
                let else_value = self.generate(*else_);
                self.scope = old_scope;
                let else_label = self.current_label;
                self.terminate_block(Terminator::Br { label: end_label }, end_label);

                let phi_name = self.generate_name(requested_name);
                self.add_instruction(Instruction {
                    name: phi_name,
                    instruction_base: InstructionBase::Phi {
                        then_: then_value,
                        then_label,
                        else_: else_value,
                        else_label,
                    },
                })
            }
        }
    }
}

fn generate_function(name: Name, _type_: Type, param: String, body: Expression) -> Vec<Function> {
    let mut state = get_initial_state();
    state.scope.insert(
        param.clone(),
        Value::Reference {
            name: Name::Name(param.clone()),
        },
    );
    let value = state.generate(body);
    let last_block = Block {
        label: state.current_label,
        instructions: state.current_instructions,
        terminator: Terminator::Ret { value },
    };
    let mut blocks = state.current_blocks;
    blocks.push(last_block);
    let function = Function {
        name,
        ret_type: Type::TInt,
        param_type: if param.is_empty() {
            Type::TVoid
        } else {
            Type::TInt
        },
        param: Name::Name(param),
        blocks,
    };
    let mut functions = state.functions;
    functions.push(function);
    return functions;
}

pub fn generate_code(expression: Expression) -> Module {
    let functions = generate_function(
        Name::Name("main".to_string()),
        Type::TInt,
        "".to_string(),
        expression,
    );
    Module {
        name: "module".to_string(),
        functions,
    }
}
