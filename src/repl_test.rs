use std::collections::HashMap;
use std::fs;

use crate::repl;
use crate::runtime::environment;
use crate::runtime::types::RuntimeVal;

#[test]
fn test() {
    let string = "let x = 10 * ( 10 /10 ) - 1;x".to_string();
    let mut env = environment::global_env();
    let runtime_val = repl::execute(&mut env, string);
    assert_eq!(runtime_val, RuntimeVal::Number(9))
}

#[test]
fn test_with_file() {
    let string = fs::read_to_string("src/test.v").unwrap();
    let mut env = environment::global_env();
    let runtime_val = repl::execute(&mut env, string);
    let properties_map = HashMap::from([
        ("x".to_string(), RuntimeVal::Number(10)),
        ("y".to_string(), RuntimeVal::Number(32)),
        ("foo".to_string(), RuntimeVal::Number(100)),
        (
            "complex".to_string(),
            RuntimeVal::Object(HashMap::from([("bar".to_string(), RuntimeVal::Bool(true))])),
        ),
    ]);
    assert_eq!(runtime_val, RuntimeVal::Object(properties_map));
}

pub mod test_file {
    use std::fs;

    use crate::repl;
    use crate::runtime::environment;

    pub fn test_with_file() {
        let string = fs::read_to_string("src/test.v").unwrap();
        let mut env = environment::global_env();
        let runtime_val = repl::execute(&mut env, string);
        println!("{:#?}", runtime_val)
    }
}
