pub enum Node {
    Program(Vec<Node>),
    Block(Vec<Node>),
    Statement(Box<Node>),

    Expression(Box<Node>),
    BinaryExpression {
        left: Box<Node>,
        operator: dosato_lexer::Operator,
        right: Box<Node>,
    },
    UnaryExpressionPrefix {
        operator: dosato_lexer::Operator,
        argument: Box<Node>,
    },
    UnaryExpressionPostfix {
        argument: Box<Node>,
        operator: dosato_lexer::Operator,
    },
    Literal(dosato_lexer::Token),
    Identifier(u16), // id based on the identifier table
    CallExpression {
        callee: Box<Node>,
        arguments: Vec<Node>,
    },
    
    MemberExpression {
        object: Box<Node>,
        operator: dosato_lexer::Operator,
        property: Box<Node>,
    },

    VariableDeclaration {
        type_annotation: Option<dosato_lexer::KeyWord>,
        constant: bool,
        uses_array_unwrapping: bool,
        identifiers: Vec<u16>, // ids based on the identifier table
        values: Vec<Box<Node>>, // parallel to identifiers
    },

    FunctionDeclaration {
        name: u16, // id based on the identifier table
        return_type: Option<dosato_lexer::KeyWord>,
        parameters: Vec<Node>, // declaration nodes for parameters
        body: Box<Node>,
    },
    
    FunctionParameter {
        name: u16, // id based on the identifier table
        type_annotation: Option<dosato_lexer::KeyWord>,
        default_value: Option<Box<Node>>,
    },
}