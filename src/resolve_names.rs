use std::collections::HashMap;

use crate::expression::{Expression, Identifier};

struct State {
    scope: HashMap<String, u32>,
}

impl State {
    fn update_identifier(&mut self, Identifier { name, id }: Identifier) -> Identifier {
        let id = *self.scope.get(&name).unwrap_or(&0);
        self.scope.insert(name.clone(), id + 1);
        return Identifier { name, id };
    }

    fn resolve(&mut self, expresison: Expression) -> Expression {
        match expresison {
            Expression::Body { body } => {
                let scope = self.scope.clone();
                let body = body.into_iter().map(|e| self.resolve(e)).collect();
                self.scope = scope;
                Expression::Body { body }
            }
            Expression::Name { name, body } => {
                let name = self.update_identifier(name);
                let scope = self.scope.clone();
                let body = self.resolve(*body);
                self.scope = scope;
                Expression::Name {
                    name,
                    body: Box::new(body),
                }
            }
            Expression::Function { param, body } => {
                let param = self.update_identifier(param);
                let scope = self.scope.clone();
                let body = self.resolve(*body);
                self.scope = scope;
                Expression::Function {
                    param,
                    body: Box::new(body),
                }
            }
            Expression::Op { op, lhs, rhs } => {
                let scope = self.scope.clone();
                let lhs = self.resolve(*lhs);
                self.scope = scope;
                let scope = self.scope.clone();
                let rhs = self.resolve(*rhs);
                self.scope = scope;
                Expression::Op {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }
            }
            Expression::Reference(Identifier { name, id: _ }) => {
                let id = *self
                    .scope
                    .get(&name)
                    .expect(format!("Can't find name {name}").as_str());
                Expression::Reference(Identifier { name, id })
            }
            Expression::Literal { value } => Expression::Literal { value },
            Expression::Call { caller, arg } => {
                let scope = self.scope.clone();
                let caller = self.resolve(*caller);
                self.scope = scope;
                let scope = self.scope.clone();
                let arg = self.resolve(*arg);
                self.scope = scope;
                Expression::Call {
                    caller: Box::new(caller),
                    arg: Box::new(arg),
                }
            }
            Expression::If { cond, then_, else_ } => {
                let scope = self.scope.clone();
                let cond = self.resolve(*cond);
                self.scope = scope;
                let scope = self.scope.clone();
                let then_ = self.resolve(*then_);
                self.scope = scope;
                let scope = self.scope.clone();
                let else_ = self.resolve(*else_);
                self.scope = scope;
                Expression::If {
                    cond: Box::new(cond),
                    then_: Box::new(then_),
                    else_: Box::new(else_),
                }
            }
        }
    }
}

pub fn resolve_names(expresison: Expression) -> Expression {
    let mut state = State {
        scope: HashMap::new(),
    };
    state.resolve(expresison)
}
