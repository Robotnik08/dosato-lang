use dosato_lexer::TokenKind;

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

    TernaryExpression {
        condition: Box<Node>,
        true_expression: Box<Node>,
        false_expression: Box<Node>,
    },

    VariableDeclaration {
        type_annotation: dosato_lexer::KeyWord,
        constant: bool,
        uses_array_unwrapping: bool,
        identifiers: Vec<Node>, // ids based on the identifier table
        values: Vec<Node>, // parallel to identifiers
    },

    FunctionDeclaration {
        id: u16, // id based on the identifier table
        return_type: dosato_lexer::KeyWord,
        parameters: Vec<Node>, // declaration nodes for parameters
        body: Box<Node>,
    },

    FunctionParameter {
        id: u16, // id based on the identifier table
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

    LambdaExpression {
        return_type: dosato_lexer::KeyWord,
        parameters: Vec<Node>, // declaration nodes for parameters
        body: Box<Node>,
    },

    TypeCastExpression {
        expression: Box<Node>,
        target_type: dosato_lexer::KeyWord,
    },

    Set { // Inner body of a set statement
        variable_expressions: Vec<Node>,
        operator: dosato_lexer::Operator,
        value_expressions: Vec<Node>,
    },

    Return {
        argument: Option<Box<Node>>,
    },

    Break,
    Continue,

    Inherit {
        expression: Box<Node>,
    },

    Import {
        string_literal: TokenKind,
    },

    Include {
        string_literal: TokenKind,
    },

    Loop {
        body: Box<Node>,
    },

    If {
        inverse: bool,
        expression: Box<Node>,
        body: Box<Node>,
    },

    While {
        inverse: bool,
        expression: Box<Node>,
        body: Box<Node>,
    },

    Else {
        body: Option<Box<Node>>,
    },

    For {
        loop_expression: Box<Node>,
        body: Box<Node>,
    },
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
    TernaryExpression,

    VariableDeclaration,
    FunctionDeclaration,
    FunctionParameter,
    ArrayExpression,
    ObjectExpression,
    ObjectProperty,
    LambdaExpression,

    Do,
    Set,
    Return,
    Inherit,
    Import,
    Include,
    Loop,
    If,
    While,
    Else,
    For
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
            Node::TernaryExpression { condition, true_expression, false_expression } => f.debug_struct("TernaryExpression").field("condition", condition).field("true_expression", true_expression).field("false_expression", false_expression).finish(),

            Node::VariableDeclaration { type_annotation, constant, uses_array_unwrapping, identifiers, values } => f.debug_struct("VariableDeclaration").field("type_annotation", type_annotation).field("constant", constant).field("uses_array_unwrapping", uses_array_unwrapping).field("identifiers", identifiers).field("values", values).finish(),
            Node::FunctionDeclaration { id, return_type, parameters, body } => f.debug_struct("FunctionDeclaration").field("id", id).field("return_type", return_type).field("parameters", parameters).field("body", body).finish(),
            Node::FunctionParameter { id, type_annotation, default_value } => f.debug_struct("FunctionParameter").field("id", id).field("type_annotation", type_annotation).field("default_value", default_value).finish(),
            Node::ArrayExpression { elements } => f.debug_struct("ArrayExpression").field("elements", elements).finish(),
            Node::ObjectExpression { properties } => f.debug_struct("ObjectExpression").field("properties", properties).finish(),
            Node::ObjectProperty { key, value } => f.debug_struct("ObjectProperty").field("key", key).field("value", value).finish(),
            Node::LambdaExpression { return_type, parameters, body } => f.debug_struct("LambdaExpression").field("return_type", return_type).field("parameters", parameters).field("body", body).finish(),
            Node::TypeCastExpression { expression, target_type } => f.debug_struct("TypeCastExpression").field("expression", expression).field("target_type", target_type).finish(),

            Node::Set { variable_expressions, operator, value_expressions } => f.debug_struct("Set").field("variable_expressions", variable_expressions).field("operator", operator).field("value_expressions", value_expressions).finish(),
            Node::Return { argument } => f.debug_struct("Return").field("argument", argument).finish(),
        
            Node::Break => write!(f, "Break"),
            Node::Continue => write!(f, "Continue"),

            Node::Inherit { expression } => f.debug_struct("Inherit").field("expression", expression).finish(),
            Node::Import { string_literal } => f.debug_struct("Import").field("string_literal", string_literal).finish(),
            Node::Include { string_literal } => f.debug_struct("Include").field("string_literal", string_literal).finish(),

            Node::Loop { body } => f.debug_struct("Loop").field("body", body).finish(),

            Node::If { inverse, expression, body } => f.debug_struct("If").field("inverse", inverse).field("expression", expression).field("body", body).finish(),
            Node::Else { body } => f.debug_struct("Else").field("body", body).finish(),
            Node::While { inverse, expression, body } => f.debug_struct("While").field("inverse", inverse).field("expression", expression).field("body", body).finish(),
            Node::For { loop_expression, body } => f.debug_struct("For").field("loop_expression", loop_expression).field("body", body).finish(),
        }
    }
}