pub enum Node {
    Blank,

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

    VariableDeclaration {
        type_annotation: dosato_lexer::KeyWord,
        constant: bool,
        uses_array_unwrapping: bool,
        identifiers: Vec<Node>, // ids based on the identifier table
        values: Vec<Node>, // parallel to identifiers
    },

    FunctionDeclaration {
        name: u16, // id based on the identifier table
        return_type: dosato_lexer::KeyWord,
        parameters: Vec<Node>, // declaration nodes for parameters
        body: Box<Node>,
    },

    FunctionParameter {
        name: u16, // id based on the identifier table
        type_annotation: dosato_lexer::KeyWord,
        default_value: Box<Node>,
    },

    ArrayExpression {
        elements: Vec<Node>,
    },

    ObjectExpression {
        properties: Vec<Node>,
    },

    ObjectProperty {
        key: Box<Node>,
        value: Box<Node>,
    },

    TypeCastExpression {
        expression: Box<Node>,
        target_type: dosato_lexer::KeyWord,
    },

    // Master bodies
    Do { // Outer do, encapsulates the body and any potential extensions
        body: Vec<Node>
    },
    DoBody { // Inner body of a do statement
        body: Box<Node>
    },

    Set { // Outer set, encapsulates the body and any potential extensions
        body: Vec<Node>
    },
    SetBody { // Inner body of a set statement
        variable_expressions: Vec<Node>,
        operator: dosato_lexer::Operator,
        value_expressions: Vec<Node>,
    },

    Make { // Outer make, encapsulates the body and any potential extensions
        body: Vec<Node>
    },
    Const { // Outer const, encapsulates the body and any potential extensions
        body: Vec<Node>
    }
}

pub enum NodeType {
    Program,
    Block,
    Statement,
    Expression,
    BinaryExpression,
    UnaryExpressionPrefix,
    UnaryExpressionPostfix,
    Literal,
    Identifier,
    CallExpression,
    VariableDeclaration,
    FunctionDeclaration,
    FunctionParameter,
    ArrayExpression,
    ObjectExpression,
    ObjectProperty,

    Do,
    DoBody,
    Set,
    SetBody,
    Make,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Blank => write!(f, "Blank"),

            Node::Program(body) => f.debug_struct("Program").field("body", body).finish(),
            Node::Block(body) => f.debug_struct("Block").field("body", body).finish(),
            Node::Statement(statement) => f.debug_struct("Statement").field("statement", statement).finish(),
            Node::Expression(expression) => f.debug_struct("Expression").field("expression", expression).finish(),
            Node::BinaryExpression { left, operator, right } => f.debug_struct("BinaryExpression").field("left", left).field("operator", operator).field("right", right).finish(),
            Node::UnaryExpressionPrefix { operator, argument } => f.debug_struct("UnaryExpressionPrefix").field("operator", operator).field("argument", argument).finish(),
            Node::UnaryExpressionPostfix { argument, operator } => f.debug_struct("UnaryExpressionPostfix").field("argument", argument).field("operator", operator).finish(),
            Node::Literal(token) => f.debug_struct("Literal").field("token", token).finish(),
            Node::Identifier(id) => f.debug_struct("Identifier").field("id", id).finish(),
            Node::CallExpression { callee, arguments } => f.debug_struct("CallExpression").field("callee", callee).field("arguments", arguments).finish(),
            Node::VariableDeclaration { type_annotation, constant, uses_array_unwrapping, identifiers, values } => f.debug_struct("VariableDeclaration").field("type_annotation", type_annotation).field("constant", constant).field("uses_array_unwrapping", uses_array_unwrapping).field("identifiers", identifiers).field("values", values).finish(),
            Node::FunctionDeclaration { name, return_type, parameters, body } => f.debug_struct("FunctionDeclaration").field("name", name).field("return_type", return_type).field("parameters", parameters).field("body", body).finish(),
            Node::FunctionParameter { name, type_annotation, default_value } => f.debug_struct("FunctionParameter").field("name", name).field("type_annotation", type_annotation).field("default_value", default_value).finish(),
            Node::ArrayExpression { elements } => f.debug_struct("ArrayExpression").field("elements", elements).finish(),
            Node::ObjectExpression { properties } => f.debug_struct("ObjectExpression").field("properties", properties).finish(),
            Node::ObjectProperty { key, value } => f.debug_struct("ObjectProperty").field("key", key).field("value", value).finish(),
            Node::TypeCastExpression { expression, target_type } => f.debug_struct("TypeCastExpression").field("expression", expression).field("target_type", target_type).finish(),

            Node::Do { body } => f.debug_struct("Do").field("body", body).finish(),
            Node::DoBody { body } => f.debug_struct("DoBody").field("expression", body).finish(),
            Node::Set { body } => f.debug_struct("Set").field("body", body).finish(),
            Node::SetBody { variable_expressions, operator, value_expressions } => f.debug_struct("SetBody").field("variable_expressions", variable_expressions).field("operator", operator).field("value_expressions", value_expressions).finish(),
            Node::Make { body } => f.debug_struct("Make").field("body", body).finish(),
            Node::Const { body } => f.debug_struct("Const").field("body", body).finish(),
        }
    }
}