#[derive(Debug)]
pub enum Ast {
    Program {
        statements: Vec<Ast>,
    },
    VariableDeclaration {
        constant: bool,
        identifier: String,
        value: Box<Ast>,
    },
    BinaryExpr {
        left: Box<Ast>,
        right: Box<Ast>,
        operator: char,
    },
    AssignmentExpr {
        assignee: Box<Ast>,
        value: Box<Ast>,
    },
    MemberExpr {
        object: Box<Ast>,
        property: Box<Ast>,
        computed: bool,
    },
    CallExpr {
        caller: Box<Ast>,
        args: Vec<Ast>,
    },
    Identifier(String),

    NumericLiteral(isize),
    PropertyLiteral {
        key: String,
        value: Option<Box<Ast>>,
    },
    ObjectLiteral {
        properties: Vec<(String, Option<Box<Ast>>)>,
    },
}
