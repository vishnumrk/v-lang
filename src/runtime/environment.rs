use std::collections::{HashMap, HashSet};
use std::ops::Deref;

use crate::runtime::types::RuntimeVal;

#[derive(Debug, Clone)]
pub struct Environment {
    parent: Option<Box<Environment>>,
    variables: HashMap<String, RuntimeVal>,
    constants: HashSet<String>,
}

impl Environment {
    pub fn new(parent: Option<Environment>) -> Self {
        Environment {
            parent: parent.map(Box::new),
            variables: HashMap::new(),
            constants: HashSet::new(),
        }
    }

    pub(crate) fn declare_variable(
        &mut self,
        variable: &str,
        value: RuntimeVal,
        constant: bool,
    ) -> RuntimeVal {
        if self.variables.contains_key(variable.deref()) {
            panic!("variable is already defined; {} = {:?}", variable, value)
        }
        if constant {
            self.constants.insert(variable.to_string());
        }

        self.variables.insert(variable.to_string(), value.clone());
        value
    }

    pub(crate) fn assign_variable(&mut self, variable: &str, value: RuntimeVal) -> RuntimeVal {
        let environment = self.resolve(variable);
        let is_constant = environment.constants.contains(variable);
        if is_constant {
            panic!(
                "Cannot reassign variable {} as it is already defined as a constant",
                variable
            );
        }
        environment
            .variables
            .insert(variable.to_string(), value.clone());
        value
    }

    pub(crate) fn lookup_variable(&mut self, variable: &str) -> Option<RuntimeVal> {
        let environment = self.resolve(variable);
        environment.variables.get(variable).cloned()
    }

    fn resolve(&mut self, variable: &str) -> &mut Environment {
        if self.variables.contains_key(variable) {
            return self;
        }
        if self.parent.is_some() {
            let parent = self.parent.as_mut().unwrap();
            return parent.as_mut().resolve(variable);
        }
        panic!("{} variable is not defined", variable);
    }
}

pub fn global_env() -> Environment {
    let mut environment = Environment::new(None);
    environment.declare_variable("null", RuntimeVal::Null, true);
    environment.declare_variable("true", RuntimeVal::Bool(true), true);
    environment.declare_variable("false", RuntimeVal::Bool(false), true);
    environment
}

