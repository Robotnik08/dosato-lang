mod structures;

// Re-exporting for easier access
pub use structures::ast::*;
use dosato_runtime::*;
use dosato_lexer::*;

macro_rules! skip_block {
    ($self:ident, $index:ident, $span:ident) => {
        if let TokenKind::BracketOpen(bracket_type) = &$self.tokens[$index].kind {
            while $index < $span.1 {
                $index += 1;
                if let TokenKind::BracketClose(close_bracket_type) = &$self.tokens[$index].kind {
                    if close_bracket_type == bracket_type {
                        break;
                    }
                }
            }

            if $index >= $span.1 {
                return Err(
                    error::Error::new(
                        error::ErrorKind::SyntaxError,
                        "Unclosed bracket".to_string(),
                        $self.source_name.clone().unwrap_or("".to_string()),
                        $self.get_line_column_len($index - 1, $index - 1),
                        false
                    )
                )
            }
        }
    };
}

macro_rules! skip_block_backwards {
    ($self:ident, $index:ident, $span:ident) => {
        if let TokenKind::BracketClose(bracket_type) = &$self.tokens[$index].kind {
            while $index > $span.0 {
                $index -= 1;
                if let TokenKind::BracketOpen(open_bracket_type) = &$self.tokens[$index].kind {
                    if open_bracket_type == bracket_type {
                        break;
                    }
                }
            }

            if $index <= $span.0 {
                return Err(
                    error::Error::new(
                        error::ErrorKind::SyntaxError,
                        "Unclosed bracket".to_string(),
                        $self.source_name.clone().unwrap_or("".to_string()),
                        $self.get_line_column_len($index + 1, $index + 1),
                        false
                    )
                )
            }
        }
    };
}

pub struct Parser {
    tokens: Vec<dosato_lexer::Token>,
    source_name: Option<String>,
}

impl Parser {
    pub fn new(tokens: Vec<dosato_lexer::Token>, source_name: Option<String>) -> Self {
        // remove comments
        let tokens = tokens.into_iter().filter(|token| {
            !matches!(token.kind, dosato_lexer::TokenKind::Comment(_))
        }).collect();
        Self { tokens, source_name }
    }

    pub fn parse(&self) -> Result<Node, error::Error> {
        self.parse_node(NodeType::Program, (0, self.tokens.len()))
    }


    fn get_line_column_len(&self, index1: usize, index2: usize) -> (usize, usize, usize) {
        let token_length = self.tokens.len();
        let start = &self.tokens[std::cmp::min(index1, token_length - 1)];
        let end = &self.tokens[std::cmp::min(index2, token_length - 1)];
        (
            start.position.line,
            start.position.column,
            end.span.1 - start.span.0
        )
    }

    pub fn encased_in_parentheses(&self, span: (usize, usize)) -> bool {
        if span.1 - span.0 < 2 {
            return false;
        }
        
        if let TokenKind::BracketOpen(BracketType::Parenthesis(depth_start)) = &self.tokens[span.0].kind {
            for i in (span.0 + 1)..span.1 {
                if let TokenKind::BracketClose(BracketType::Parenthesis(depth_end)) = &self.tokens[i].kind {
                    if depth_start == depth_end {
                        return i == span.1 - 1;
                    }
                }
            }
        }

        false
    }

    pub fn encased_in_square_brackets(&self, span: (usize, usize)) -> bool {
        if span.1 - span.0 < 2 {
            return false;
        }

        if let TokenKind::BracketOpen(BracketType::Bracket(depth_start)) = &self.tokens[span.0].kind {
            for i in (span.0 + 1)..span.1 {
                if let TokenKind::BracketClose(BracketType::Bracket(depth_end)) = &self.tokens[i].kind {
                    if depth_start == depth_end {
                        return i == span.1 - 1;
                    }
                }
            }
        }

        false
    }

    pub fn encased_in_curly_braces(&self, span: (usize, usize)) -> bool {
        if span.1 - span.0 < 2 {
            return false;
        }

        if let TokenKind::BracketOpen(BracketType::Brace(depth_start)) = &self.tokens[span.0].kind {
            for i in (span.0 + 1)..span.1 {
                if let TokenKind::BracketClose(BracketType::Brace(depth_end)) = &self.tokens[i].kind {
                    if depth_start == depth_end {
                        return i == span.1 - 1;
                    }
                }
            }
        }

        false
    }

    pub fn split_on_commas(&self, span: (usize, usize)) -> Result<Vec<(usize, usize)>, error::Error> {
        if span.1 - span.0 == 0 {
            return Ok(vec![]);
        }

        let mut segments: Vec<(usize, usize)> = Vec::new();

        let mut index = span.0;
        let mut current_start = span.0;
        while index < span.1 {
            let token = &self.tokens[index];
            skip_block!(self, index, span);
            if let TokenKind::Operator(op) = &token.kind {
                if let Operator::Comma = op {
                    segments.push((current_start, index));
                    current_start = index + 1;
                }
            }
            index += 1;
        }

        segments.push((current_start, index));

        Ok(segments)
    }
    
    pub fn parse_node(&self, node_type: NodeType, span: (usize, usize)) -> Result<Node, error::Error> {
        match node_type {
            NodeType::Expression => {
                if self.encased_in_parentheses(span) {
                    return self.parse_node(NodeType::Expression, (span.0 + 1, span.1 - 1));
                }

                if self.encased_in_square_brackets(span) {
                    return self.parse_node(NodeType::ArrayExpression, (span.0 + 1, span.1 - 1));
                }

                if self.encased_in_curly_braces(span) {
                    return self.parse_node(NodeType::ObjectExpression, (span.0 + 1, span.1 - 1));
                }

                if span.1 - span.0 == 1 {
                    let token = &self.tokens[span.0];
                    match &token.kind {
                        TokenKind::Identifier(_) => {
                            return Ok(Node::Identifier(0));
                        }
                        TokenKind::IntegerLiteral(_) | 
                        TokenKind::NumberLiteral(_) | 
                        TokenKind::StringLiteral(_) | 
                        TokenKind::BooleanLiteral(_) | 
                        TokenKind::CharLiteral(_) |
                        TokenKind::Infinity |
                        TokenKind::NaN |
                        TokenKind::Null => {
                            return Ok(Node::Literal(token.clone()));
                        }

                        _ => {
                            return Err(
                                error::Error::new(error::ErrorKind::SyntaxError, "Expected an expression".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(span.0, span.0), false)
                            );
                        }
                    }
                } else if span.1 - span.0 == 0 {
                    return Err(
                        error::Error::new(error::ErrorKind::SyntaxError, "Empty expression".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(span.0, span.0), false)
                    );
                }
                
                let mut index = span.0;
                let mut found_operator = false;
                let mut operator_index = span.0;
                let mut operator_precedence = 0; // lowest precedence
                let mut last_operator_was_unary_postfix = false;
                let mut found_lambda = false;
                let mut arrow_index = span.0;
                let mut second_operator_index_for_ternary = span.0;
                while index < span.1 {
                    let token = &self.tokens[index];
                    skip_block!(self, index, span);

                    if let TokenKind::Operator(op) = &token.kind {
                        if index + 1 < span.1 && matches!(op, Operator::FatArrow) {
                            arrow_index = index;
                            index += 1;
                            found_lambda = true;
                            // check next token, if open curly brace, skip till end of block, if not, break, the rest of the expression is the body
                            if let Some(next_token) = self.tokens.get(index) {
                                if let TokenKind::BracketOpen(BracketType::Brace(start_depth)) = &next_token.kind {
                                    // skip till end of block
                                    while let Some(token) = self.tokens.get(index) {
                                        if let TokenKind::BracketClose(BracketType::Brace(end_depth)) = &token.kind {
                                            if start_depth == end_depth {
                                                index += 1;
                                                break;
                                            }    
                                        }
                                        index += 1;
                                        continue;
                                    }
                                } else {
                                    // not a block, break
                                    break;
                                }
                            }

                            continue;
                        } else if matches!(op, Operator::Question) {
                            let question_index = index;
                            let mut question_count = 0;

                            let mut index = question_index;
                            while index < span.1 {
                                let token = &self.tokens[index];
                                skip_block!(self, index, span);
                                if let TokenKind::Operator(op) = &token.kind {
                                    match op {
                                        Operator::Question => {
                                            question_count += 1;
                                        }
                                        Operator::Colon => {
                                            question_count -= 1;
                                            if question_count == 0 {
                                                second_operator_index_for_ternary = index;
                                                break;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                index += 1;
                            }

                            if question_count != 0 {
                                return Err(
                                    error::Error::new(error::ErrorKind::SyntaxError, "Unmatched '?' in ternary expression".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(question_index, span.1 - 1), false)
                                );
                            }

                            operator_index = question_index;
                            found_operator = true;
                            break; // ternary operator is lowest precedence, stop here
                        }
                        


                        let precedence = op.precedence();
                        if precedence >= operator_precedence {
                            if found_operator && op.is_unary_prefix() && !last_operator_was_unary_postfix && operator_index == index - 1 {
                                index += 1;
                                continue; // skip unary prefix operators in a row
                            }
                            found_operator = true;
                            last_operator_was_unary_postfix = op.is_unary_postfix();
                            operator_index = index;
                            operator_precedence = if index == span.0 && op.is_unary_prefix() { UNARY_PREFIX_PRECEDENCE } else { precedence };
                        }
                    }
                    index += 1;
                }

                if operator_precedence == 0 {
                    // if last token is a closing parenthesis, it might be a call expression
                    let last_token = &self.tokens[span.1 - 1];
                    if let TokenKind::BracketClose(BracketType::Parenthesis(_)) = &last_token.kind {
                        return self.parse_node(NodeType::CallExpression, span);
                    }
                }

                if found_operator {
                    if second_operator_index_for_ternary > span.0 {
                        // ternary operator
                        let condition = self.parse_node(NodeType::Expression, (span.0, operator_index))?;
                        let true_expr = self.parse_node(NodeType::Expression, (operator_index + 1, second_operator_index_for_ternary))?;
                        let false_expr = self.parse_node(NodeType::Expression, (second_operator_index_for_ternary + 1, span.1))?;
                        return Ok(Node::TernaryExpression {
                            condition: Box::new(condition),
                            true_expression: Box::new(true_expr),
                            false_expression: Box::new(false_expr),
                        });
                    }
                    if operator_index == span.0 { // unary prefix
                        let right = self.parse_node(NodeType::Expression, (operator_index + 1, span.1))?;
                        let operator_token = &self.tokens[operator_index];
                        let operator = match &operator_token.kind {
                            TokenKind::Operator(op) => *op,
                            _ => {
                                return Err(
                                    error::Error::new(error::ErrorKind::SyntaxError, "Expected operator".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(operator_index, operator_index), false)
                                );
                            }
                        };
                        if !operator.is_unary_prefix() {
                            return Err(
                                error::Error::new(error::ErrorKind::SyntaxError, "Operator is not a unary prefix operator".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(operator_index, operator_index), false)
                            );
                        }
                        return Ok(Node::UnaryExpressionPrefix {
                            operator: operator,
                            argument: Box::new(right),
                        });
                    } else if operator_index == span.1 - 1 { // unary postfix
                        let left = self.parse_node(NodeType::Expression, (span.0, operator_index))?;
                        let operator_token = &self.tokens[operator_index];
                        let operator = match &operator_token.kind {
                            TokenKind::Operator(op) => *op,
                            _ => {
                                return Err(
                                    error::Error::new(error::ErrorKind::SyntaxError, "Expected operator".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(operator_index, operator_index), false)
                                );
                            }
                        };
                        if !operator.is_unary_postfix() {
                            return Err(
                                error::Error::new(error::ErrorKind::SyntaxError, "Operator is not a unary postfix operator".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(operator_index, operator_index), false)
                            );
                        }
                        return Ok(Node::UnaryExpressionPostfix {
                            operator: operator,
                            argument: Box::new(left),
                        });
                    } else { // binary
                        let operator_token = &self.tokens[operator_index];
                        let operator = match &operator_token.kind {
                            TokenKind::Operator(op) => *op,
                            _ => {
                                return Err(
                                    error::Error::new(error::ErrorKind::SyntaxError, "Expected operator".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(operator_index, operator_index), false)
                                );
                            }
                        };

                        if let Operator::TypeCast = operator {
                            // type cast operator
                            // check if right has 1 token and is a type keyword
                            if operator_index + 1 != span.1 - 1 {
                                return Err(
                                    error::Error::new(error::ErrorKind::SyntaxError, "Type cast operator must be followed by a single type keyword".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(operator_index + 1, span.1 - 1), false)
                                );
                            }

                            let right_token = &self.tokens[operator_index + 1];
                            let right = match &right_token.kind {
                                TokenKind::KeyWord(kw) => if kw.is_type_keyword() { *kw } else {
                                    return Err(
                                        error::Error::new(error::ErrorKind::SyntaxError, "Type cast operator must be followed by a type keyword".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(operator_index + 1, operator_index + 1), false)
                                    );
                                },
                                _ => {
                                    return Err(
                                        error::Error::new(error::ErrorKind::SyntaxError, "Type cast operator must be followed by a type keyword".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(operator_index + 1, operator_index + 1), false)
                                    );
                                }
                            };

                            let left = self.parse_node(NodeType::Expression, (span.0, operator_index))?;
                            return Ok(Node::TypeCastExpression {
                                expression: Box::new(left),
                                target_type: right,
                            });
                        }

                        let left = self.parse_node(NodeType::Expression, (span.0, operator_index))?;
                        let right = self.parse_node(NodeType::Expression, (operator_index + 1, span.1))?;
                        return Ok(Node::BinaryExpression {
                            left: Box::new(left),
                            operator: operator,
                            right: Box::new(right),
                        });
                    }
                } else if found_lambda {
                    // if first token is type annotation, use that and the rest is parameters
                    let mut first_index = span.0;
                    let first_token = &self.tokens[first_index];
                    let return_type = match &first_token.kind {
                        TokenKind::KeyWord(kw) => if kw.is_type_keyword() { 
                                first_index += 1;
                                *kw
                            } else { 
                                KeyWord::Any
                            },
                        _ => {
                            KeyWord::Any
                        }
                    };

                    let parameters = if self.encased_in_parentheses((first_index, arrow_index)) {
                        // split on comma
                        let segments = self.split_on_commas((first_index + 1, arrow_index - 1))?;
                        let mut params: Vec<Node> = Vec::new();
                        for segment in segments {
                            let param = self.parse_node(NodeType::FunctionParameter, segment)?;
                            params.push(param);
                        }
                        params
                    } else if first_index == arrow_index - 1 {
                        vec![self.parse_node(NodeType::FunctionParameter, (first_index, arrow_index - 1))?]
                    } else {
                        return Err(
                            error::Error::new(error::ErrorKind::SyntaxError, "Invalid lambda parameters".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(first_index, arrow_index - 1), false)
                        );
                    };

                    let body = if self.encased_in_curly_braces((arrow_index + 1, span.1)) {
                        self.parse_node(NodeType::Block, (arrow_index + 2, span.1 - 1))?
                    } else {
                        self.parse_node(NodeType::Expression, (arrow_index + 1, span.1))?
                    };
                    return Ok(Node::LambdaExpression {
                        return_type,
                        parameters,
                        body: Box::new(body),
                    });
                }

                Err(
                    error::Error::new(error::ErrorKind::SyntaxError, "Invalid expression".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(span.0, span.1 - 1), false)
                )
            }

            NodeType::ArrayExpression => {
                let mut elements: Vec<Node> = Vec::new();

                if span.0 == span.1 {
                    return Ok(Node::ArrayExpression { elements });
                }

                let mut index = span.0;
                let mut current_element_start = index;
                while index < span.1 {
                    let token = &self.tokens[index];
                    skip_block!(self, index, span);
                    if let TokenKind::Operator(op) = &token.kind {
                        if let Operator::Comma = op {
                            let element = self.parse_node(NodeType::Expression, (current_element_start, index))?;
                            elements.push(element);
                            current_element_start = index + 1;
                        }
                    }
                    index += 1;
                }

                if index > current_element_start {
                    let element = self.parse_node(NodeType::Expression, (current_element_start, index))?;
                    elements.push(element);
                }

                Ok(Node::ArrayExpression { elements })
            }
            
            NodeType::ObjectExpression => {
                let mut properties: Vec<Node> = Vec::new();
                if span.0 == span.1 {
                    return Ok(Node::ObjectExpression { properties });
                }

                let mut index = span.0;
                let mut current_property_start = index;
                while index < span.1 {
                    let token = &self.tokens[index];
                    skip_block!(self, index, span);
                    if let TokenKind::Operator(op) = &token.kind {
                        if let Operator::Comma = op {
                            let property = self.parse_node(NodeType::ObjectProperty, (current_property_start, index))?;
                            properties.push(property);
                            current_property_start = index + 1;
                        }
                    }
                    index += 1;
                }

                // remaining property
                if index > current_property_start {
                    let property = self.parse_node(NodeType::ObjectProperty, (current_property_start, index))?;
                    properties.push(property);
                }

                Ok(Node::ObjectExpression{ properties })
            }

            NodeType::ObjectProperty => {
                let mut index = span.0;
                let mut found_colon = false;
                let mut colon_index = span.0;
                while index < span.1 {
                    let token = &self.tokens[index];
                    skip_block!(self, index, span);
                    if let TokenKind::Operator(op) = &token.kind {
                        if let Operator::Colon = op {
                            found_colon = true;
                            colon_index = index;
                            break;
                        }
                    }
                    index += 1;
                }

                if found_colon {
                    return Ok(Node::ObjectProperty {
                        key: Box::new(self.parse_node(NodeType::Expression, (span.0, colon_index))?),
                        value: Box::new(self.parse_node(NodeType::Expression, (colon_index + 1, span.1))?),
                    })
                }

                Ok(Node::ObjectProperty {
                    key: Box::new(Node::Blank),
                    value: Box::new(self.parse_node(NodeType::Expression, span)?),
                })
            }

            NodeType::CallExpression => {
                let end_token = &self.tokens[span.1 - 1];
                // must end with a closing parenthesis, everything inside until the first opening parenthesis are the arguments
                // the rest up front is the callee AKA expression
                if let TokenKind::BracketClose(BracketType::Parenthesis(_)) = &end_token.kind {
                    let mut index = span.1 - 1;
                    skip_block_backwards!(self, index, span);
                    if let TokenKind::BracketOpen(BracketType::Parenthesis(_)) = &self.tokens[index].kind {
                        let arguments = self.parse_node(NodeType::ArrayExpression, (index + 1, span.1 - 1))?;
                        let arguments = if let Node::ArrayExpression { elements } = arguments { elements } else { unreachable!() };

                        let callee = self.parse_node(NodeType::Expression, (span.0, index))?;
                        Ok(Node::CallExpression {
                            callee: Box::new(callee),
                            arguments: arguments,
                        })
                    } else {
                        Err(
                            error::Error::new(error::ErrorKind::SyntaxError, "Expected opening parenthesis for call expression".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(index, index), false)
                        )
                    }
                } else {
                    Err(
                        error::Error::new(error::ErrorKind::SyntaxError, "Expected closing parenthesis for call expression".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(span.1 - 1, span.1 - 1), false)
                    )
                }
            }


            NodeType::Program | NodeType::Block => {
                let mut statements: Vec<Node> = Vec::new();

                let mut index = span.0;
                // if master keyword is found, it's a statement, until the next master keyword or end of span
                let mut current_statement_start = index;
                while index < span.1 {
                    let token = &self.tokens[index];
                    skip_block!(self, index, span);
                    if let TokenKind::KeyWord(keyword_type) = token.kind {
                        if !keyword_type.is_master_keyword() {
                            index += 1;
                            continue;
                        }

                        if index > current_statement_start {
                            let statement = self.parse_node(NodeType::Statement, (current_statement_start, index))?;
                            statements.push(statement);
                        }
                        current_statement_start = index;
                    }
                    index += 1;
                }

                // parse the last statement
                if index > current_statement_start {
                    let statement = self.parse_node(NodeType::Statement, (current_statement_start, index))?;
                    statements.push(statement);
                }

                match node_type {
                    NodeType::Program => Ok(Node::Program(statements)),
                    NodeType::Block => Ok(Node::Block(statements)),
                    _ => unreachable!(),
                }
            }
            
            NodeType::Statement => {
                // Get first keyword
                let first_token = &self.tokens[span.0];
                let key_word_type = match &first_token.kind {
                    TokenKind::KeyWord(kw) => kw,
                    _ => {
                        return Err(
                            error::Error::new(error::ErrorKind::SyntaxError, "Expected a master keyword at the start of statement".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(span.0, span.0), false)
                        );
                    }
                };

                if key_word_type.is_master_keyword() {
                    match key_word_type {
                        KeyWord::Do => {
                            let node = self.parse_node(NodeType::Do, (span.0 + 1, span.1))?;
                            Ok(node)
                        }
                        KeyWord::Set => {
                            let node = self.parse_node(NodeType::Set, (span.0 + 1, span.1))?;
                            Ok(node)
                        }
                        KeyWord::Make => {
                            let body = self.parse_node(NodeType::VariableDeclaration, (span.0 + 1, span.1))?;
                            if let Node::VariableDeclaration { type_annotation, constant: _, uses_array_unwrapping, identifiers, values } = body {
                                return Ok(Node::VariableDeclaration {
                                    type_annotation,
                                    constant: false,
                                    uses_array_unwrapping,
                                    identifiers,
                                    values,
                                })
                            }
                            Err(
                                error::Error::new(error::ErrorKind::SyntaxError, "Expected variable declaration after make keyword".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(span.0 + 1, span.1 - 1), false)
                            )
                        }
                        KeyWord::Const => {
                            let body = self.parse_node(NodeType::VariableDeclaration, (span.0 + 1, span.1))?;
                            if let Node::VariableDeclaration { type_annotation, constant: _, uses_array_unwrapping, identifiers, values } = body {
                                return Ok(Node::VariableDeclaration {
                                    type_annotation,
                                    constant: true,
                                    uses_array_unwrapping,
                                    identifiers,
                                    values,
                                })
                            }
                            Err(
                                error::Error::new(error::ErrorKind::SyntaxError, "Expected variable declaration after const keyword".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(span.0 + 1, span.1 - 1), false)
                            )
                        }
                        KeyWord::Define | KeyWord::Implement => {
                            let node = self.parse_node(NodeType::FunctionDeclaration, (span.0 + 1, span.1))?;
                            Ok(node)
                        }
                        KeyWord::Return => {
                            let expression = if span.1 - span.0 > 1 {
                                Some(self.parse_node(NodeType::Expression, (span.0 + 1, span.1))?)
                            } else {
                                None
                            };
                            Ok(Node::Return {
                                argument: expression.map(Box::new),
                            })
                        }
                        KeyWord::Break => {
                            if span.1 - span.0 > 1 {
                                return Err(
                                    error::Error::new(error::ErrorKind::SyntaxError, "Break statement does not take any arguments".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(span.0 + 1, span.1 - 1), false)
                                )
                            }
                            Ok(Node::Break)
                        },
                        KeyWord::Continue => {
                            if span.1 - span.0 > 1 {
                                return Err(
                                    error::Error::new(error::ErrorKind::SyntaxError, "Continue statement does not take any arguments".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(span.0 + 1, span.1 - 1), false)
                                )
                            }
                            Ok(Node::Continue)
                        },
                        _ => Ok(Node::Blank)
                    }
                } else {
                    Err(
                        error::Error::new(error::ErrorKind::SyntaxError, "Expected a master keyword at the start of statement".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(span.0, span.0), false)
                    )
                }
            }


            NodeType::Do => {
                if self.encased_in_curly_braces(span) {
                    return self.parse_node(NodeType::Block, (span.0 + 1, span.1 - 1));
                }

                let expression = self.parse_node(NodeType::Expression, span)?;
                if let Node::CallExpression { .. } = expression {
                    Ok(expression)
                } else {
                    Err(error::Error::new(error::ErrorKind::SyntaxError, "Expected function call or block".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(span.0, span.1 - 1), false))
                }
            }

            NodeType::Set => {
                // Split by the first assignment operator found
                let mut index = span.0;
                let mut found_operator = false;
                let mut operator_index = span.0;
                while index < span.1 {
                    let token = &self.tokens[index];
                    skip_block!(self, index, span);
                    if let TokenKind::Operator(op) = &token.kind {
                        if op.is_assignment() {
                            found_operator = true;
                            operator_index = index;
                            break;
                        }
                    }
                    index += 1;
                }

                // Increment Decrement
                if operator_index == span.0 {
                    return match found_operator {
                        true => Err(
                            error::Error::new(error::ErrorKind::SyntaxError, "Expected variable expression before assignment operator".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(span.0, operator_index), false)
                        ),
                        false => {
                            if let TokenKind::Operator(op) = &self.tokens[span.1 - 1].kind {
                                match op {
                                    Operator::Decrement | Operator::Increment => {
                                        // expression on the comma
                                        let mut expressions: Vec<Node> = vec![];

                                        let mut expression_start = span.0;
                                        let mut index = span.0;
                                        while index < span.1 - 1 {
                                            let token = &self.tokens[index];
                                            skip_block!(self, index, span);
                                            if let TokenKind::Operator(op) = &token.kind {
                                                if let Operator::Comma = op {
                                                    let expr = self.parse_node(NodeType::Expression, (expression_start, index))?;
                                                    expressions.push(expr);
                                                    expression_start = index + 1;
                                                }
                                            }
                                            index += 1;
                                        }

                                        let expr = self.parse_node(NodeType::Expression, (expression_start, index))?;
                                        expressions.push(expr);


                                        Ok(Node::Set {
                                            variable_expressions: expressions,
                                            operator: *op,
                                            value_expressions: vec![],
                                        })
                                    }
                                    _ => {
                                        Err(
                                            error::Error::new(error::ErrorKind::SyntaxError, "Expected assignment operator in set statement".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(span.0, span.1 - 1), false)
                                        )
                                    }
                                }
                            } else {
                                Err(
                                    error::Error::new(error::ErrorKind::SyntaxError, "Expected assignment operator in set statement".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(span.0, span.1 - 1), false)
                                )
                            }
                        },
                    }
                }

                // pre operator expressions
                let mut index = span.0;
                let mut variable_expressions: Vec<Node> = vec![];
                let mut expression_start = span.0;
                while index < operator_index {
                    let token = &self.tokens[index];
                    skip_block!(self, index, span);
                    if let TokenKind::Operator(op) = &token.kind {
                        if let Operator::Comma = op {
                            let expr = self.parse_node(NodeType::Expression, (expression_start, index))?;
                            variable_expressions.push(expr);
                            expression_start = index + 1;
                        }
                    }
                    index += 1;
                }

                let expr = self.parse_node(NodeType::Expression, (expression_start, operator_index))?;
                variable_expressions.push(expr);

                // post operator expressions
                let mut value_expressions: Vec<Node> = vec![];
                index = operator_index + 1;
                expression_start = operator_index + 1;
                while index < span.1 {
                    let token = &self.tokens[index];
                    skip_block!(self, index, span);
                    if let TokenKind::Operator(op) = &token.kind {
                        if let Operator::Comma = op {
                            let expr = self.parse_node(NodeType::Expression, (expression_start, index))?;
                            value_expressions.push(expr);
                            expression_start = index + 1;
                        }
                    }
                    index += 1;
                }

                let expr = self.parse_node(NodeType::Expression, (expression_start, span.1))?;
                value_expressions.push(expr);

                Ok(Node::Set {
                    variable_expressions,
                    operator: match &self.tokens[operator_index].kind {
                        TokenKind::Operator(op) => *op,
                        _ => unreachable!()
                    },
                    value_expressions,
                })
            }

            NodeType::VariableDeclaration => {
                // split on the first assignment operator
                let mut index = span.0;
                let mut found_operator = false;
                let mut operator_index = span.0;
                let mut uses_array_unwrapping = false;
                while index < span.1 {
                    let token = &self.tokens[index];
                    skip_block!(self, index, span);
                    if let TokenKind::Operator(op) = &token.kind {
                        if op.is_assigment_pure() {
                            found_operator = true;
                            operator_index = index;
                            uses_array_unwrapping = matches!(op, Operator::ArrayUnwrapAssign);
                            break;
                        }
                    }
                    index += 1;
                }

                if !found_operator {
                    return Err(
                        error::Error::new(error::ErrorKind::SyntaxError, "Expected assignment operator in variable declaration".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(span.0, span.1 - 1), false)
                    )
                } else if operator_index == span.0 {
                    return Err(
                        error::Error::new(error::ErrorKind::SyntaxError, "Expected variable declaration before assignment operator".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(span.0, operator_index), false)
                    )
                }

                let first_token = &self.tokens[span.0];

                let mut start_of_variables = span.0;
                let type_annotation = if let TokenKind::KeyWord(kw) = &first_token.kind {
                    if kw.is_type_keyword() {
                        start_of_variables += 1;
                        *kw
                    } else {
                        KeyWord::Any
                    }
                } else {
                    KeyWord::Any
                };

                let mut variable_declarations: Vec<Node> = vec![];
                index = start_of_variables;
                let mut current_variable_start = index;
                while index < operator_index {
                    let token = &self.tokens[index];
                    skip_block!(self, index, span);
                    if let TokenKind::Operator(op) = &token.kind {
                        if let Operator::Comma = op {
                            let var_decl = self.parse_node(NodeType::Expression, (current_variable_start, index))?;
                            variable_declarations.push(var_decl);
                            current_variable_start = index + 1;
                        }
                    }
                    index += 1;
                }
                let var_decl = self.parse_node(NodeType::Expression, (current_variable_start, operator_index))?;
                variable_declarations.push(var_decl);
                
                let mut value_expressions: Vec<Node> = vec![];
                index = operator_index + 1;
                let mut current_value_start = index;

                while index < span.1 {
                    let token = &self.tokens[index];
                    skip_block!(self, index, span);
                    if let TokenKind::Operator(op) = &token.kind {
                        if let Operator::Comma = op {
                            let value_expr = self.parse_node(NodeType::Expression, (current_value_start, index))?;
                            value_expressions.push(value_expr);
                            current_value_start = index + 1;
                        }
                    }
                    index += 1;
                }
                let value_expr = self.parse_node(NodeType::Expression, (current_value_start, span.1))?;
                value_expressions.push(value_expr);

                Ok(Node::VariableDeclaration { 
                    type_annotation,
                    constant: false,
                    uses_array_unwrapping,
                    identifiers: variable_declarations,
                    values: value_expressions,
                })
            }

            NodeType::FunctionDeclaration => {
                // If first token is a type, we know the return type, else it's Any
                let first_token = &self.tokens[span.0];
                let mut start_of_function = span.0;
                let return_type = if let TokenKind::KeyWord(kw) = &first_token.kind {
                    if kw.is_type_keyword() {
                        start_of_function += 1;
                        *kw
                    } else {
                        KeyWord::Any
                    }
                } else {
                    KeyWord::Any
                };


                // Next token must be an identifier (function name)
                let name_token = &self.tokens[start_of_function];
                let _name = if let TokenKind::Identifier(id) = &name_token.kind {
                    id
                } else {
                    return Err(
                        error::Error::new(error::ErrorKind::SyntaxError, "Expected function name identifier".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(start_of_function, start_of_function), false)
                    )
                };

                // Next token must be opening parenthesis
                let open_paren_token = &self.tokens[start_of_function + 1];
                if let TokenKind::BracketOpen(BracketType::Parenthesis(_)) = &open_paren_token.kind {
                    // find closing parenthesis
                    let mut index = start_of_function + 1;
                    skip_block!(self, index, span);

                    if let TokenKind::BracketClose(BracketType::Parenthesis(_)) = &self.tokens[index].kind {
                        // parse parameters
                        let parameters = self.split_on_commas((start_of_function + 2, index))?;
                        let mut parameter_nodes: Vec<Node> = vec![];
                        for parameter_span in &parameters {
                            let param_node = self.parse_node(NodeType::FunctionParameter, *parameter_span)?;
                            parameter_nodes.push(param_node);
                        }

                        // the rest is the function body
                        let body = self.parse_node(NodeType::Block, (index + 2, span.1 - 1))?;

                        Ok(Node::FunctionDeclaration {
                            id: 0,
                            parameters: parameter_nodes,
                            return_type,
                            body: Box::new(body),
                        })
                    } else {
                        Err(
                            error::Error::new(error::ErrorKind::SyntaxError, "Expected closing parenthesis for function parameters".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(index, index), false)
                        )
                    }
                } else {
                    Err(
                        error::Error::new(error::ErrorKind::SyntaxError, "Expected opening parenthesis for function parameters".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(start_of_function + 1, start_of_function + 1), false)
                    )
                }
            }

            NodeType::FunctionParameter => {
                // if first keyword is a type, it's the type annotation, otherwise Any, then identifier, then optional default value after equals
                let first_token = &self.tokens[span.0];
                let mut current_index = span.0;
                let type_annotation = if let TokenKind::KeyWord(kw) = &first_token.kind {
                    if kw.is_type_keyword() {
                        current_index += 1;
                        *kw
                    } else {
                        KeyWord::Any
                    }
                } else {
                    KeyWord::Any
                };

                let name_token = &self.tokens[current_index];
                let _name = if let TokenKind::Identifier(id) = &name_token.kind {
                    current_index += 1;
                    id
                } else {
                    return Err(
                        error::Error::new(error::ErrorKind::SyntaxError, "Expected parameter name identifier".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(current_index, current_index), false)
                    )
                };

                let default_value = if current_index < span.1 {
                    let equals_token = &self.tokens[current_index];
                    if let TokenKind::Operator(op) = &equals_token.kind {
                        if let Operator::Assign = op {
                            current_index += 1;
                            let default_value_node = self.parse_node(NodeType::Expression, (current_index, span.1))?;
                            Box::new(default_value_node)
                        } else {
                            return Err(
                                error::Error::new(error::ErrorKind::SyntaxError, "Expected assignment operator for default parameter value".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(current_index, current_index), false)
                            )
                        }
                    } else {
                        return Err(
                            error::Error::new(error::ErrorKind::SyntaxError, "Expected assignment operator for default parameter value".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(current_index, current_index), false)
                        )
                    }
                } else {
                    Box::new(Node::Blank)
                };

                Ok(Node::FunctionParameter {
                    id: 0,
                    type_annotation,
                    default_value,
                })
            }

            _ => {
                Err(
                    error::Error::new(error::ErrorKind::SyntaxError, "Unknown node type".to_string(), self.source_name.clone().unwrap_or("".to_string()), self.get_line_column_len(span.0, span.1 - 1), false)
                )
            }
        }
    }
}