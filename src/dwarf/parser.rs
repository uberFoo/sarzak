use ansi_term::Colour;
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::{error::SimpleReason, prelude::*};
use log;

use crate::dwarf::{DwarfFloat, Expression, Item, Spanned, Statement, Token, Type};

use super::DwarfInteger;

macro_rules! debug {
    ($msg:literal, $($arg:expr),*) => {
        $(
            log::debug!(
                "{} --> {:?}\n  --> {}:{}:{}",
                Colour::Yellow.underline().paint($msg),
                $arg,
                file!(),
                line!(),
                column!()
            );
        )*
    };
    ($arg:expr) => {
        log::debug!("{:?}\n  --> {}:{}:{}", $arg, file!(), line!(), column!())
    };
}

macro_rules! error {
    ($msg:literal, $($arg:expr),*) => {
        $(
            log::error!(
                "{} --> {:?}\n  --> {}:{}:{}",
                Colour::Red.underline().paint($msg),
                $arg,
                file!(),
                line!(),
                column!()
            );
        )*
    };
    ($arg:literal) => {
        log::error!("{}\n  --> {}:{}:{}", Colour::Red.underline().paint($arg), file!(), line!(), column!())
    };
    ($arg:expr) => {
        log::error!("{:?}\n  --> {}:{}:{}", $arg, file!(), line!(), column!())
    };
}

type Result<T, E = Simple<String>> = std::result::Result<T, E>;

fn lexer() -> impl Parser<char, Vec<Spanned<Token>>, Error = Simple<char>> {
    // A parser for numbers
    let int = text::int(10).map(Token::Integer);

    let float = text::int(10)
        .chain::<char, _, _>(just('.').chain::<char, _, _>(text::digits(10)))
        .collect::<String>()
        .map(Token::Float);

    // A parser for strings
    let str_ = just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Token::String);

    let dagger = just::<char, char, Simple<char>>('-')
        .ignore_then(just('>'))
        .map(|_| Token::Punct('→'));

    let double_colon = just::<char, char, Simple<char>>(':')
        .ignore_then(just(':'))
        .padded_by(filter(|c: &char| c.is_whitespace()).repeated())
        .map(|_| Token::Punct('∷'));

    // A parser for operators
    // let op = one_of("+-*/!=")
    //     .repeated()
    //     .at_least(1)
    //     .collect::<String>()
    //     .map(Token::Op);

    // A parser for punctuation (delimiters, semicolons, etc.)
    let punct = one_of("=-()[]{}:;,.&<>").map(|c| Token::Punct(c));

    // A "Object type" parser. Basically I'm asserting that if an identifier starts
    // with a capital letter, then it's an object.
    // let object = filter::<_, _, Simple<char>>(|c: &char| c.is_uppercase())
    //     .chain::<char, Vec<_>, _>(filter(|c: &char| c.is_alphanumeric()).repeated())
    //     .collect::<String>()
    //     .map(Token::Object);

    // let some = just::<char, &str, Simple<char>>("Some")
    //     .then_ignore(just('('))
    //     .then(filter(|c: &char| *c != ')').repeated().collect::<String>())
    //     .map(|inner| Token::Some(inner.1));

    // A parser for identifiers and keywords
    let ident = text::ident().map(|ident: String| match ident.as_str() {
        "as" => Token::As,
        "bool" => Token::Type(Type::Boolean),
        // "else" => Token::Else,
        "false" => Token::Bool(false),
        "float" => Token::Type(Type::Float),
        "fn" => Token::Fn,
        "for" => Token::For,
        // "global" => Token::Global,
        // "if" => Token::If,
        "impl" => Token::Impl,
        "int" => Token::Type(Type::Integer),
        "in" => Token::In,
        "let" => Token::Let,
        "None" => Token::None,
        "print" => Token::Print,
        // "Self" => Token::Self_,
        // "self" => Token::SmallSelf,
        "return" => Token::Return,
        "Some" => Token::Some,
        "string" => Token::Type(Type::String),
        "struct" => Token::Struct,
        "true" => Token::Bool(true),
        "use" => Token::Import,
        // "Uuid" => Token::Uuid,
        _ => Token::Ident(ident),
    });

    let option = just("Option").map(|_| Token::Option);

    // A single token can be one of the above
    let token = float
        .or(int)
        .or(str_)
        // .or(dagger)
        // .or(double_colon)
        // .or(op)
        .or(punct)
        .or(option)
        // .or(object)
        // .or(some)
        .or(ident)
        .recover_with(skip_then_retry_until([]));

    let comment = just("//").then(take_until(just('\n'))).padded();

    token
        .map_with_span(|tok, span| (tok, span))
        .padded_by(comment.repeated())
        .padded()
        .repeated()
}

#[derive(Debug)]
struct DwarfParser {
    tokens: Vec<Spanned<Token>>,
    current: usize,
    errors: Vec<Simple<String>>,
}

impl DwarfParser {
    pub fn new(tokens: Vec<Spanned<Token>>) -> Self {
        Self {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    /// Parse a program
    ///
    /// A proram is a list of items
    ///
    /// program -> item*
    fn parse_program(&mut self) -> (Vec<Spanned<Item>>, Vec<Simple<String>>) {
        debug!("enter parse_program");

        let mut result = Vec::new();

        while !self.at_end() {
            debug!("parse_program: parsse_item");
            if let Some(item) = self.parse_item() {
                debug!("parse_program: item:", item);
                result.push(item);
            } else {
                let tok = if let Some(tok) = self.peek() {
                    tok
                } else {
                    self.previous().unwrap()
                };

                let err = Simple::expected_input_found(
                    tok.1.clone(),
                    [
                        Some("use".to_owned()),
                        Some("impl".to_owned()),
                        Some("struct".to_owned()),
                        Some("Fn".to_owned()),
                    ],
                    Some(tok.0.to_string()),
                );
                let err = err.with_label("expected item");

                self.errors.push(err);

                error!("parse_program: resynchronize looking for '}'");
                while !self.at_end() && !self.match_(&[Token::Punct('}')]) {
                    self.advance();
                }
                error!("parse_program: resynchronized");
            }
        }

        debug!("exit parse_program: ", result);

        (result, self.errors.clone())
    }

    /// Parse a Statement
    ///
    ///  statement -> ; | Item | LetStatement | ExpressionStatement
    fn parse_statement(&mut self) -> Result<Option<Spanned<Statement>>> {
        debug!("enter parse_statement");

        let start = if let Some(tok) = self.peek() {
            tok.1.start
        } else {
            return Ok(None);
        };

        if self.match_(&[Token::Punct(';')]) {
            debug!("parse_statement: empty statement");
            return Ok(Some((
                Statement::Empty,
                start..self.previous().unwrap().1.end,
            )));
        }

        if let Some(item) = self.parse_item() {
            debug!("parse_statement: item:", item);
            return Ok(Some((
                Statement::Item(item),
                start..self.previous().unwrap().1.end,
            )));
        }

        // OK. Time to work this out.I want to try to parse a let statemnet. If
        // I get an error I want to append the error to our vec, and then continue
        // on as if nothing happened. Basically, we need to go up one more level
        // and hove them start over. So maybe we just pass this up the stack.

        if let Some(statement) = self.parse_let_statement()? {
            if self.match_(&[Token::Punct(';')]) {
                debug!("parse_statement: let statement:", statement);
                return Ok(Some(statement));
            } else {
                let tok = self.peek().unwrap();
                let err = Simple::expected_input_found(
                    tok.1.clone(),
                    [Some("';'".to_owned())],
                    Some(tok.0.to_string()),
                );
                let err = err.with_label("expected ;");
                return Err(err);
            }
        }

        if let Some(expr) = self.parse_expression_without_block()? {
            debug!("parse_statement: expression:", expr);
            if self.match_(&[Token::Punct(';')]) {
                debug!("parse_statement: expression statement:", expr);
                return Ok(Some((
                    Statement::Expression(expr),
                    start..self.previous().unwrap().1.end,
                )));
                // Don't consume the '}'.
            } else if self.check(&Token::Punct('}')) {
                debug!("parse_statement: result statement:", expr);
                return Ok(Some((
                    Statement::Result(expr),
                    start..self.previous().unwrap().1.end,
                )));
            } else {
                let tok = if let Some(tok) = self.peek() {
                    tok
                } else {
                    self.previous().unwrap()
                };
                let err = Simple::expected_input_found(
                    tok.1.clone(),
                    [Some("'}'".to_owned()), Some("';'".to_owned())],
                    Some(tok.0.to_string()),
                );

                debug!("exit parse_statement: error:", err);
                return Err(err);
            }
        }

        if let Some(expr) = self.parse_expression_with_block()? {
            debug!("parse_statement: expression:", expr);
            return Ok(Some((
                Statement::Expression(expr),
                start..self.previous().unwrap().1.end,
            )));
        }

        Ok(None)
    }

    /// Parse an Item
    ///
    /// This should probably just return an error...
    ///  item -> Struct | ImplBlock | Import | Function
    fn parse_item(&mut self) -> Option<Spanned<Item>> {
        debug!("enter parse_item");

        // Try to parse a struct
        match self.parse_struct() {
            Ok(Some(item)) => {
                debug!("parse_item: struct:", item);
                return Some(item);
            }
            Ok(None) => {}
            Err(err) => {
                error!("parse_item: error:", err);
                self.errors.push(err);
            }
        }

        if let Some(item) = self.parse_impl_block() {
            return Some(item);
        }

        if let Some(item) = self.parse_import() {
            return Some(item);
        }

        // Try to parse a function
        match self.parse_function() {
            Ok(Some(func)) => {
                debug!("parse_item: function:", func);
                return Some(func);
            }
            Ok(None) => {}
            Err(err) => {
                self.errors.push(err);
            }
        }

        None
    }

    /// Parse a path
    ///
    /// path -> IDENTIFIER (:: IDENTIFIER)*
    fn parse_path(&mut self) -> Option<Spanned<Vec<Spanned<String>>>> {
        debug!("enter parse_path");

        let mut path = Vec::new();
        while let Some(ident) = self.parse_ident() {
            path.push(ident);
            if !self.match_(&[Token::Punct(':')]) || !self.match_(&[Token::Punct(':')]) {
                debug!("exit parse_path snarf: ", path);
                break;
            }
        }

        if path.is_empty() {
            return None;
        }

        debug!("exit parse_path: ", path);
        let span = path[0].1.start..path[path.len() - 1].1.end;
        Some((path, span))
    }

    /// Parse an Import
    ///
    /// import -> USE IDENTIFIER (:: IDENTIFIER)* ( AS IDENTIFIER )? ;
    fn parse_import(&mut self) -> Option<Spanned<Item>> {
        debug!("enter parse_import");

        let start = if let Some(tok) = self.peek() {
            tok.1.start
        } else {
            return None;
        };

        if !self.match_(&[Token::Import]) {
            return None;
        }

        let name = if let Some(ident) = self.parse_path() {
            debug!("parse_import: path:", ident);
            ident
        } else {
            let tok = self.peek().unwrap();
            let err = Simple::expected_input_found(
                tok.1.clone(),
                [Some("<path -> IDENTIFIER (:: IDENTIFIER)*>".to_owned())],
                Some(tok.0.to_string()),
            );
            let err = err.with_label("expected path");
            self.errors.push(err);
            return None;
        };

        let alias = if self.match_(&[Token::As]) {
            if let Some(ident) = self.parse_ident() {
                debug!("parse_import: alias:", ident);
                Some(ident)
            } else {
                let tok = self.peek().unwrap();
                let err = Simple::expected_input_found(
                    tok.1.clone(),
                    [Some("identifier".to_owned())],
                    Some(tok.0.to_string()),
                );
                let err = err.with_label("expected identifier");
                self.errors.push(err);
                return None;
            }
        } else {
            None
        };

        if !self.match_(&[Token::Punct(';')]) {
            let tok = self.peek().unwrap();
            let err = Simple::expected_input_found(
                tok.1.clone(),
                [Some("';'".to_owned())],
                Some(tok.0.to_string()),
            );
            let err = err.with_label("expected ;");
            self.errors.push(err);
            return None;
        }

        debug!("exit parse_import");

        Some((
            Item::Import(name, alias),
            start..self.previous().unwrap().1.end,
        ))
    }

    /// Parse an impl block
    ///
    /// impl_block -> impl IDENTIFIER  { impl_block_body }
    fn parse_impl_block(&mut self) -> Option<Spanned<Item>> {
        debug!("enter parse_impl_block");

        let start = if let Some(tok) = self.peek() {
            tok.1.start
        } else {
            return None;
        };

        if !self.match_(&[Token::Impl]) {
            return None;
        }

        let name = if let Some(ident) = self.parse_ident() {
            ident
        } else {
            let tok = self.peek().unwrap();
            let err = Simple::expected_input_found(
                tok.1.clone(),
                [Some("identifier".to_owned())],
                Some(tok.0.to_string()),
            );
            let err = err.with_label("expected identifier");
            self.errors.push(err);
            return None;
        };

        if !self.match_(&[Token::Punct('{')]) {
            let tok = self.peek().unwrap();
            let err = Simple::expected_input_found(
                tok.1.clone(),
                [Some("'{'".to_owned())],
                Some(tok.0.to_string()),
            );
            let err = err.with_label("expected '{'");
            self.errors.push(err);
            return None;
        }

        let mut body = Vec::new();
        while !self.match_(&[Token::Punct('}')]) {
            if let Some(item) = self.parse_item() {
                debug!("parse_impl_block: item:", item);
                body.push(item);
            } else {
                let tok = self.peek().unwrap();
                let err = Simple::expected_input_found(
                    tok.1.clone(),
                    [Some("'}'".to_owned())],
                    Some(tok.0.to_string()),
                );
                let err = err.with_label("expected '}'");
                self.errors.push(err);
                return None;
            }
        }

        debug!("exit parse_impl_block: ", (&name, &body));

        Some((
            Item::Implementation(name, body),
            start..self.previous().unwrap().1.end,
        ))
    }

    /// Parse a Let Statement
    ///
    /// let_statement -> let IDENTIFIER(:TYPE)? = expression
    fn parse_let_statement(&mut self) -> Result<Option<Spanned<Statement>>> {
        debug!("enter parse_let_statement");

        let start = if let Some(tok) = self.peek() {
            tok.1.start
        } else {
            return Ok(None);
        };

        if !self.match_(&[Token::Let]) {
            return Ok(None);
        }

        let name = if let Some(ident) = self.parse_ident() {
            ident
        } else {
            let tok = self.peek().unwrap();
            let err = Simple::expected_input_found(
                tok.1.clone(),
                [Some("identifier".to_owned())],
                Some(tok.0.to_string()),
            );
            let err = err.with_label("expected identifier");
            return Err(err);
        };

        let type_ = if self.match_(&[Token::Punct(':')]) {
            if let Some(ty) = self.parse_type()? {
                Some(ty)
            } else {
                let tok = self.peek().unwrap();
                let err = Simple::expected_input_found(
                    tok.1.clone(),
                    [
                        Some("'bool'".to_owned()),
                        Some("'float'".to_owned()),
                        Some("'int'".to_owned()),
                        Some("'string'".to_owned()),
                        Some("'Uuid'".to_owned()),
                        Some("'Option<T>'".to_owned()),
                        Some("'[T]'".to_owned()),
                    ],
                    Some(tok.0.to_string()),
                );
                let err = err.with_label("expected type");
                return Err(err);
            }
        } else {
            None
        };

        if !self.match_(&[Token::Punct('=')]) {
            let tok = self.peek().unwrap();
            let err = Simple::expected_input_found(
                tok.1.clone(),
                [Some("'='".to_owned())],
                Some(tok.0.to_string()),
            );
            let err = err.with_label("expected equals");
            return Err(err);
        }

        let value = if let Some(expr) = self.parse_expression()? {
            expr
        } else {
            let tok = self.peek().unwrap();
            let err = Simple::expected_input_found(
                tok.1.clone(),
                [Some("expression".to_owned())],
                Some(tok.0.to_string()),
            );
            let err = err.with_label("expected expression");
            return Err(err);
        };

        debug!("exit parse_let_statement");

        Ok(Some((
            Statement::Let(name, type_, value),
            start..self.previous().unwrap().1.end,
        )))
    }

    /// Parse a "simple" expression
    ///
    /// I broke this out for a reason that no longer applies. I'm not sure that
    /// it's great.
    ///
    /// expression -> boolean_literal | float_literal | integer_literal |
    ///               local_variable | none | some | string_literal
    fn parse_simple_expression(&mut self) -> Result<Option<Spanned<Expression>>> {
        debug!("enter parse_simple_expression");

        // parse a boolean literal
        if let Some(expression) = self.parse_boolean_literal() {
            debug!("parse_simple_expression: boolean literal:", expression);
            return Ok(Some(expression));
        }

        // parse a float literal
        if let Some(expression) = self.parse_float_literal() {
            debug!("parse_simple_expression: float literal:", expression);
            return Ok(Some(expression));
        }

        // parse an interger literal
        if let Some(expression) = self.parse_integer_literal() {
            debug!("parse_simple_expression: integer literal:", expression);
            return Ok(Some(expression));
        }

        // parse a none literal
        if let Some(expression) = self.parse_none_literal() {
            debug!("parse_simple_expression: none literal:", expression);
            return Ok(Some(expression));
        }

        // parse a print expression
        if let Some(expression) = self.parse_print_expression()? {
            debug!(
                "parse_expression_without_block: print expression:",
                expression
            );
            return Ok(Some(expression));
        }

        // parse a return expression
        if let Some(expression) = self.parse_return_expression()? {
            debug!(
                "parse_expression_without_block: return expression:",
                expression
            );
            return Ok(Some(expression));
        }

        // parse a some literal
        if let Some(expression) = self.parse_some_literal()? {
            debug!("parse_simple_expression: some literal:", expression);
            return Ok(Some(expression));
        }

        // parse a string literal
        if let Some(expression) = self.parse_string_literal() {
            debug!("parse_simple_expression: string literal:", expression);
            return Ok(Some(expression));
        }

        // parse a list literal
        if let Some(expression) = self.parse_list_literal()? {
            debug!("parse_simple_expression: list literal:", expression);
            return Ok(Some(expression));
        }

        // parse a local variable
        if let Some(expression) = self.parse_local_variable() {
            debug!("parse_expression: local variable:", expression);
            return Ok(Some(expression));
        }

        Ok(None)
    }

    fn parse_expression(&mut self) -> Result<Option<Spanned<Expression>>> {
        debug!("enter parse_expression");
        if let Some(expression) = self.parse_expression_without_block()? {
            debug!("parse_expression: expression without block:", expression);
            return Ok(Some(expression));
        }

        if let Some(expression) = self.parse_expression_with_block()? {
            debug!("parse_expression: expression with block:", expression);
            return Ok(Some(expression));
        }

        debug!("exit parse_expression None");
        Ok(None)
    }

    /// Parse an expression without a block
    ///
    /// expression -> assignment | block | Error | field_access |
    ///               for | function_call | if |
    ///               list | method_call |print |
    ///               static_method_call | struct
    fn parse_expression_without_block(&mut self) -> Result<Option<Spanned<Expression>>> {
        debug!("enter parse_expression_without_block");

        let expr = self.parse_simple_expression()?;
        debug!("parse_expression_without_block: simple expression:", expr);

        if let Some(expr) = &expr {
            // parse a function call
            if let Some(expression) = self.parse_function_call(expr)? {
                debug!("parse_expression_without_block: function call:", expression);
                return Ok(Some(expression));
            }

            // parse a field access
            if let Some(expression) = self.parse_field_access(expr)? {
                debug!("parse_expression_without_block: field access:", expression);
                return Ok(Some(expression));
            }

            // parse a static method call
            if let Some(expression) = self.parse_static_method_call(expr)? {
                debug!(
                    "parse_expression_without_block: static method call:",
                    expression
                );
                return Ok(Some(expression));
            }

            // Parse a struct expression
            if let Some(expression) = self.parse_struct_expression(expr)? {
                debug!(
                    "parse_expression_without_block: struct expression:",
                    expression
                );
                return Ok(Some(expression));
            }
        }

        debug!(
            "exit parse_expression_without_block with simple expression",
            expr
        );
        Ok(expr)
    }

    /// Parse an expression with a block
    ///
    /// expression -> assignment | block | Error | field_access |
    ///               for | function_call | if |
    ///               list | method_call |print |
    ///               static_method_call | struct
    fn parse_expression_with_block(&mut self) -> Result<Option<Spanned<Expression>>> {
        debug!("enter parse_expression_with_block");

        // parse a block expression
        if let Some(expression) = self.parse_block_expression()? {
            debug!("parse_expression_with_block: block expression:", expression);
            return Ok(Some(expression));
        }

        // parse a for loop expression
        if let Some(expression) = self.parse_for_loop_expression()? {
            debug!(
                "parse_expression_with_block: for loop expression:",
                expression
            );
            return Ok(Some(expression));
        }

        debug!("exit parse_expression_with_block with None");
        Ok(None)
    }

    /// Parse a Boolean Litearal
    ///
    /// boolean_literal -> true | false
    fn parse_boolean_literal(&mut self) -> Option<Spanned<Expression>> {
        debug!("enter parse_boolean_literal");

        let token = self.peek()?.clone();

        if let (Token::Bool(bool), span) = token {
            self.advance();
            Some((Expression::BooleanLiteral(bool), span.to_owned()))
        } else {
            None
        }
    }

    /// Parse field access
    ///
    /// field _access -> expression ('.' expression)+
    fn parse_field_access(
        &mut self,
        name: &Spanned<Expression>,
    ) -> Result<Option<Spanned<Expression>>> {
        debug!("enter parse_field_access");

        let start = name.1.start;

        if !self.match_(&[Token::Punct('.')]) {
            return Ok(None);
        }

        let expr = if let Some(expr) = self.parse_expression()? {
            expr
        } else {
            let token = &self.previous().unwrap();
            let err = Simple::expected_input_found(
                token.1.clone(),
                [Some("<expression>".to_owned())],
                Some(token.0.to_string()),
            );
            return Err(err);
        };

        let end = expr.1.end;

        debug!("exit parse_field_access");

        Ok(Some((
            Expression::FieldAccess(Box::new(name.clone()), Box::new(expr)),
            start..end,
        )))
    }

    /// Parse a Float Literal
    ///
    /// float_litearl -> FLOAT
    fn parse_float_literal(&mut self) -> Option<Spanned<Expression>> {
        debug!("enter parse_float_literal");

        let token = self.peek()?.clone();

        if let (Token::Float(float), span) = token {
            if let Ok(float) = float.parse::<DwarfFloat>() {
                self.advance();
                Some((Expression::FloatLiteral(float), span.to_owned()))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Parse a for loop expression
    ///
    /// for_loop --> FOR expression IN expression expression
    fn parse_for_loop_expression(&mut self) -> Result<Option<Spanned<Expression>>> {
        debug!("enter parse_for_loop_expression");

        let start = if let Some(tok) = self.peek() {
            tok.1.start
        } else {
            debug!("exit parse_for_loop_expression");
            return Ok(None);
        };

        if !self.match_(&[Token::For]) {
            debug!("exit parse_for_loop_expression no token");
            return Ok(None);
        }

        debug!("parse_for_loop_expression getting iterator");
        let iterator = if let Some(ident) = self.parse_ident() {
            debug!("parse_for_loop_expression iterator", ident);
            ident
        } else {
            let token = &self.previous().unwrap();
            let err = Simple::expected_input_found(
                token.1.clone(),
                [Some("IDENT".to_owned())],
                Some(token.0.to_string()),
            );
            debug!("exit parse_for_loop_expression no iterator");
            return Err(err);
        };

        if !self.match_(&[Token::In]) {
            let token = &self.previous().unwrap();
            let err = Simple::expected_input_found(
                token.1.clone(),
                [Some("in".to_owned())],
                Some(token.0.to_string()),
            );
            debug!("exit parse_for_loop_expression no in");
            return Err(err);
        }

        debug!("parse_for_loop_expression calling getting collection");
        let collection = if let Some(expr) = self.parse_simple_expression()? {
            debug!("parse_for_loop_expression collection", expr);
            expr
        } else {
            let token = &self.previous().unwrap();
            let err = Simple::expected_input_found(
                token.1.clone(),
                [Some("<expression>".to_owned())],
                Some(token.0.to_string()),
            );
            debug!("exit parse_for_loop_expression no collection");
            return Err(err);
        };

        debug!("parse_for_loop_expression getting body");
        let body = if let Some(expr) = self.parse_block_expression()? {
            expr
        } else {
            let token = &self.previous().unwrap();
            let err = Simple::expected_input_found(
                token.1.clone(),
                [Some("<expression>".to_owned())],
                Some(token.0.to_string()),
            );
            debug!("exit parse_for_loop_expression no body");
            return Err(err);
        };

        debug!("exit parse_for_loop_expression");

        Ok(Some((
            Expression::For(iterator, Box::new(collection), Box::new(body)),
            start..self.previous().unwrap().1.end,
        )))
    }

    /// Parse a function call
    ///
    /// function_call -> expression '(' expression,* ')'
    fn parse_function_call(
        &mut self,
        name: &Spanned<Expression>,
    ) -> Result<Option<Spanned<Expression>>> {
        debug!("enter parse_function_call");

        let start = name.1.start;

        if !self.match_(&[Token::Punct('(')]) {
            return Ok(None);
        }

        let mut arguments = Vec::new();

        while !self.at_end() && !self.match_(&[Token::Punct(')')]) {
            if let Some(expr) = self.parse_expression()? {
                arguments.push(expr);
                if self.peek().unwrap().0 == Token::Punct(',') {
                    self.advance();
                }
            } else {
                let tok = self.peek().unwrap();
                let err = Simple::expected_input_found(
                    tok.1.clone(),
                    [Some("expression".to_owned())],
                    Some(tok.0.to_string()),
                );
                let err = err.with_label("expected expression");
                return Err(err);
            }
        }

        debug!("exit parse_function_call");

        Ok(Some((
            Expression::FunctionCall(Box::new(name.clone()), arguments),
            start..self.previous().unwrap().1.end,
        )))
    }

    /// Parse an Integer Literal
    ///
    /// integer_literal -> INTEGER
    fn parse_integer_literal(&mut self) -> Option<Spanned<Expression>> {
        debug!("enter parse_integer_literal");

        let token = self.peek()?.clone();

        if let (Token::Integer(int), span) = token {
            if let Ok(int) = int.parse::<DwarfInteger>() {
                self.advance();
                Some((Expression::IntegerLiteral(int), span.to_owned()))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Parse a list expression
    ///
    /// Each element in the expression should be the same type, and I think that
    /// needs to be checked in the compiler, and not the parser, since the parser
    /// doesn't really grok types.
    ///
    /// list -> '[' expression,* ']'
    fn parse_list_literal(&mut self) -> Result<Option<Spanned<Expression>>> {
        debug!("enter parse_list_expression");

        let start = if let Some(tok) = self.peek() {
            tok.1.start
        } else {
            debug!("exit parse_list_expression");
            return Ok(None);
        };

        if !self.match_(&[Token::Punct('[')]) {
            debug!("exit parse_list_expression no token");
            return Ok(None);
        }

        let mut elements = Vec::new();

        while !self.at_end() && !self.match_(&[Token::Punct(']')]) {
            if let Some(expr) = self.parse_expression()? {
                elements.push(expr);
                if self.peek().unwrap().0 == Token::Punct(',') {
                    self.advance();
                }
            } else {
                let tok = self.peek().unwrap();
                let err = Simple::expected_input_found(
                    tok.1.clone(),
                    [Some("expression".to_owned())],
                    Some(tok.0.to_string()),
                );
                let err = err.with_label("expected expression");
                return Err(err);
            }
        }

        debug!("exit parse_list_expression");

        Ok(Some((
            Expression::List(elements),
            start..self.previous().unwrap().1.end,
        )))
    }

    /// Parse a local variable
    ///
    /// var -> IDENTIFIER
    fn parse_local_variable(&mut self) -> Option<Spanned<Expression>> {
        debug!("enter parse_local_variable");

        let token = self.peek()?.clone();

        if let (Token::Ident(ident), span) = token {
            self.advance();
            debug!("exit parse_local_variable", ident);
            Some((Expression::LocalVariable(ident), span.to_owned()))
        } else {
            None
        }
    }

    /// Parse a None expression
    ///
    /// none -> NONE
    fn parse_none_literal(&mut self) -> Option<Spanned<Expression>> {
        debug!("enter parse_none_expression");

        let token = self.peek()?.clone();

        if let (Token::None, span) = token {
            self.advance();
            Some((Expression::None, span.to_owned()))
        } else {
            None
        }
    }

    /// Parse a print expression
    ///
    /// print_expression -> PRINT
    fn parse_print_expression(&mut self) -> Result<Option<Spanned<Expression>>> {
        debug!("enter parse_print_expression");

        let start = if let Some(tok) = self.peek() {
            tok.1.start
        } else {
            return Ok(None);
        };

        if !self.match_(&[Token::Print]) {
            return Ok(None);
        }

        if !self.match_(&[Token::Punct('(')]) {
            let token = &self.previous().unwrap();
            let err = Simple::expected_input_found(
                token.1.clone(),
                [Some("(".to_owned())],
                Some(token.0.to_string()),
            );
            return Err(err);
        }

        let expression = if let Some(expr) = self.parse_expression()? {
            expr
        } else {
            let token = &self.previous().unwrap();
            let err = Simple::expected_input_found(
                token.1.clone(),
                [Some("<expression>".to_owned())],
                Some(token.0.to_string()),
            );
            return Err(err);
        };

        if !self.match_(&[Token::Punct(')')]) {
            let token = &self.previous().unwrap();
            let err = Simple::expected_input_found(
                token.1.clone(),
                [Some(")".to_owned())],
                Some(token.0.to_string()),
            );
            return Err(err);
        }

        debug!("exit parse_print_expression");

        Ok(Some((
            Expression::Print(Box::new(expression)),
            start..self.previous().unwrap().1.end,
        )))
    }

    /// Parse a static method call
    ///
    /// This is a bit goofy. Rust calls locals and the static method syntax a
    /// path. I guess the discriminate all the bits in the compiler when they
    /// are doing something with the AST. I don't think that I need that sort
    /// of flexibility.
    ///
    /// Anyway, the type part of the static method call has already been parsed
    /// as a local variable. If the next two tokens (yes, I'm pasing that as two
    /// tokens and not one. I may pay for that later.) are ':' and ':' then I
    /// am going to change the type of the passed in thing.
    ///
    /// static_method_call -> Type '::' IDENTIFIER '(' expression,* ')'
    fn parse_static_method_call(
        &mut self,
        name: &Spanned<Expression>,
    ) -> Result<Option<Spanned<Expression>>> {
        debug!("enter parse_static_method_call");

        let start = name.1.start;

        if !self.match_(&[Token::Punct(':')]) {
            return Ok(None);
        }

        if !self.match_(&[Token::Punct(':')]) {
            return Ok(None);
        }

        let name = if let (Expression::LocalVariable(name), span) = &name {
            Type::UserType((name.to_owned(), span.to_owned()))
        } else {
            let err = Simple::expected_input_found(
                name.1.clone(),
                [Some("<local_variable>".to_owned())],
                Some(format!("{:?}", name.0)),
            );
            return Err(err);
        };

        let method_name = if let Some(ident) = self.parse_ident() {
            ident
        } else {
            let token = &self.previous().unwrap();
            let err = Simple::expected_input_found(
                token.1.clone(),
                [Some("<ident>".to_owned())],
                Some(token.0.to_string()),
            );
            return Err(err);
        };

        if !self.match_(&[Token::Punct('(')]) {
            let token = &self.previous().unwrap();
            let err = Simple::expected_input_found(
                token.1.clone(),
                [Some("(".to_owned())],
                Some(token.0.to_string()),
            );
            return Err(err);
        }

        let mut arguments = Vec::new();

        while !self.at_end() && !self.match_(&[Token::Punct(')')]) {
            if let Some(expr) = self.parse_expression()? {
                arguments.push(expr);

                if self.peek().unwrap().0 == Token::Punct(',') {
                    self.advance();
                }
            } else {
                let tok = self.peek().unwrap();
                let err = Simple::expected_input_found(
                    tok.1.clone(),
                    [Some("expression".to_owned())],
                    Some(tok.0.to_string()),
                );
                let err = err.with_label("expected expression");

                return Err(err);
            }
        }

        debug!("exit parse_static_method_call");

        Ok(Some((
            Expression::StaticMethodCall(name, method_name, arguments),
            start..self.previous().unwrap().1.end,
        )))
    }

    /// Parse a struct expression
    ///
    /// struct_expression -> IDENTIFIER '{' (IDENTIFIER ':' expression,)* '}'
    fn parse_struct_expression(
        &mut self,
        name: &Spanned<Expression>,
    ) -> Result<Option<Spanned<Expression>>> {
        debug!("enter parse_struct_expression");

        let start = if let Some(tok) = self.peek() {
            tok.1.start
        } else {
            return Ok(None);
        };

        if !self.match_(&[Token::Punct('{')]) {
            return Ok(None);
        }

        let name = if let (Expression::LocalVariable(name), span) = &name {
            Type::UserType((name.to_owned(), span.to_owned()))
        } else {
            let err = Simple::expected_input_found(
                name.1.clone(),
                [Some("<local_variable>".to_owned())],
                Some(format!("{:?}", name.0)),
            );
            return Err(err);
        };

        let mut fields = Vec::new();

        while !self.at_end() && !self.match_(&[Token::Punct('}')]) {
            let field_name = if let Some(ident) = self.parse_ident() {
                ident
            } else {
                let token = &self.previous().unwrap();
                let err = Simple::expected_input_found(
                    token.1.clone(),
                    [Some("<ident>".to_owned())],
                    Some(token.0.to_string()),
                );
                return Err(err);
            };

            if !self.match_(&[Token::Punct(':')]) {
                let token = &self.previous().unwrap();
                let err = Simple::expected_input_found(
                    token.1.clone(),
                    [Some(":".to_owned())],
                    Some(token.0.to_string()),
                );
                return Err(err);
            }

            let expression = if let Some(expr) = self.parse_expression()? {
                expr
            } else {
                let token = &self.previous().unwrap();
                let err = Simple::expected_input_found(
                    token.1.clone(),
                    [Some("<expression>".to_owned())],
                    Some(token.0.to_string()),
                );
                return Err(err);
            };

            fields.push((field_name, expression));

            if self.peek().unwrap().0 == Token::Punct(',') {
                self.advance();
            }
        }

        debug!("exit parse_struct_expression");

        Ok(Some((
            Expression::Struct(name, fields),
            start..self.previous().unwrap().1.end,
        )))
    }

    /// Parse a return expression
    ///
    /// return -> RETURN expression
    fn parse_return_expression(&mut self) -> Result<Option<Spanned<Expression>>> {
        debug!("enter parse_return_expression");

        if !self.match_(&[Token::Return]) {
            return Ok(None);
        }

        let start = if let Some(tok) = self.peek() {
            tok.1.start
        } else {
            return Ok(None);
        };

        let expression = if let Some(expr) = self.parse_expression()? {
            expr
        } else {
            let token = &self.previous().unwrap();
            let err = Simple::expected_input_found(
                token.1.clone(),
                [Some("<expression -> there's a lot of them...>".to_owned())],
                Some(token.0.to_string()),
            );
            return Err(err);
        };

        debug!("exit parse_return_expression");

        Ok(Some((
            Expression::Return(Box::new(expression)),
            start..self.previous().unwrap().1.end,
        )))
    }

    /// Parse a Some expression
    ///
    /// some -> SOME
    fn parse_some_literal(&mut self) -> Result<Option<Spanned<Expression>>> {
        debug!("enter parse_some_expression");

        if !self.match_(&[Token::Some]) {
            return Ok(None);
        }

        let start = if let Some(tok) = self.peek() {
            tok.1.start
        } else {
            return Ok(None);
        };

        if !self.match_(&[Token::Punct('(')]) {
            let token = &self.previous().unwrap();
            let err = Simple::expected_input_found(
                token.1.clone(),
                [Some("(".to_owned())],
                Some(token.0.to_string()),
            );
            return Err(err);
        }

        let expression = if let Some(expr) = self.parse_expression()? {
            expr
        } else {
            let token = &self.previous().unwrap();
            let err = Simple::expected_input_found(
                token.1.clone(),
                [Some("<expression>".to_owned())],
                Some(token.0.to_string()),
            );
            return Err(err);
        };

        if !self.match_(&[Token::Punct(')')]) {
            let token = &self.previous().unwrap();
            let err = Simple::expected_input_found(
                token.1.clone(),
                [Some(")".to_owned())],
                Some(token.0.to_string()),
            );
            return Err(err);
        }

        debug!("exit parse_some_expression");

        Ok(Some((
            Expression::Some(Box::new(expression)),
            start..self.previous().unwrap().1.end,
        )))
    }

    /// Parse a String Literal
    ///
    /// string_siteral -> STRING
    fn parse_string_literal(&mut self) -> Option<Spanned<Expression>> {
        debug!("enter parse_string_literal");

        let token = self.peek()?.clone();

        if let (Token::String(string), span) = token {
            self.advance();
            Some((Expression::StringLiteral(string), span.to_owned()))
        } else {
            None
        }
    }

    fn parse_block_expression(&mut self) -> Result<Option<Spanned<Expression>>> {
        debug!("enter parse_block_expression");

        let start = if let Some(tok) = self.peek() {
            tok.1.start
        } else {
            debug!("parse_block_expression: no tokens");
            return Ok(None);
        };

        if !self.match_(&[Token::Punct('{')]) {
            debug!("parse_block_expression: no opening brace");
            return Ok(None);
        }

        let mut statements = Vec::new();

        while !self.at_end() && !self.match_(&[Token::Punct('}')]) {
            // Ok, this is where I want to log, and then ignore errors. Now, how
            // do I do that?
            match self.parse_statement() {
                Ok(Some(statement)) => {
                    debug!("parse_block_expression: statement:", statement);
                    statements.push(statement);
                }
                // 🚧 I have a feeling that we should log this as an error, and
                // maybe even resyrchronize, but I'm not sure, and there's other
                // places to work where I do know what I'm doing. I'll tackle
                // this later. Hopefully it'll pop up.
                Ok(None) => {
                    error!("parse_block_expression: no statement");
                    let token = if let Some(token) = self.peek() {
                        token
                    } else {
                        self.previous().unwrap()
                    };
                    let err = Simple::expected_input_found(
                        token.1.clone(),
                        [Some(
                            "<statement -> [(LET | EXPRESSION); | EXPRESSION]>".to_owned(),
                        )],
                        Some(token.0.to_string()),
                    );

                    debug!("parse_block_expression: no statement");
                    return Err(err);
                }

                Err(error) => {
                    debug!("parse_block_expression: error:", error);
                    return Err(error);
                }
            }
        }

        debug!("exit parse_block_expression");

        Ok(Some((
            Expression::Block(statements),
            start..self.previous().unwrap().1.end,
        )))
    }

    fn parse_function(&mut self) -> Result<Option<Spanned<Item>>> {
        debug!("enter parse_function");

        let start = if let Some(tok) = self.peek() {
            tok.1.start
        } else {
            debug!("exit parse_function: no token");
            return Ok(None);
        };

        if !self.match_(&[Token::Fn]) {
            debug!("exit parse_function: no fn");
            return Ok(None);
        }

        let name = if let Some(ident) = self.parse_ident() {
            ident
        } else {
            let token = self.peek().unwrap().clone();
            let err = Simple::expected_input_found(
                token.1.clone(),
                [Some("identifier".to_owned())],
                Some(token.0.to_string()),
            );
            debug!("exit parse_function: no ident");
            return Err(err);
        };

        if !self.match_(&[Token::Punct('(')]) {
            let token = self.peek().unwrap().clone();
            let err = Simple::expected_input_found(
                token.1.clone(),
                [Some("'('".to_owned())],
                Some(token.0.to_string()),
            );
            debug!("exit parse_function: no '('");
            return Err(err);
        }

        let mut params = Vec::new();

        while !self.at_end() && !self.match_(&[Token::Punct(')')]) {
            match self.parse_param() {
                Ok(Some(param)) => {
                    params.push(param);
                    if self.peek().unwrap().0 == Token::Punct(',') {
                        self.advance();
                    }
                }
                Ok(None) => error!("parse_function: no param"),
                Err(error) => {
                    self.errors.push(error);

                    error!("parse_function: resynchronize looking for ')'");
                    while !self.at_end() && !self.match_(&[Token::Punct(')')]) {
                        self.advance();
                    }
                    error!("parse_function: resynchronized");
                }
            }
        }

        let return_type = if self.match_(&[Token::Punct('-')]) {
            if !self.match_(&[Token::Punct('>')]) {
                let token = self.peek().unwrap().clone();
                let err = Simple::expected_input_found(
                    token.1.clone(),
                    [Some("'>'".to_owned())],
                    Some(token.0.to_string()),
                );
                debug!("exit parse_function: got '-', but no '>'");
                return Err(err);
            }

            match self.parse_type() {
                Ok(Some(ty)) => ty,
                Ok(None) => {
                    let start = self.previous().unwrap().1.end;
                    let end = self.peek().unwrap().1.start;
                    debug!("exit parse_function: no type");
                    return Err(Simple::custom(start..end, "missing type"));
                }
                Err(error) => {
                    self.errors.push(error);

                    error!("parse_function: resynchronize looking for '{'");
                    while !self.at_end() && !self.match_(&[Token::Punct('{')]) {
                        self.advance();
                    }
                    error!("parse_function: resynchronized");

                    let start = self.previous().unwrap().1.end;
                    let end = self.peek().unwrap().1.start;
                    (Type::Empty, start..end)
                }
            }
        } else {
            let start = self.previous().unwrap().1.end;
            let end = self.peek().unwrap().1.start;
            (Type::Empty, start..end)
        };

        let body = if let Some(body) = self.parse_block_expression()? {
            body
        } else {
            let start = self.previous().unwrap().1.end;
            let end = self.peek().unwrap().1.start;
            let err = Simple::custom(start..end, "missing body");
            debug!("exit parse_function: no body");
            return Err(err);
        };

        let end = body.1.end;

        debug!("exit parse_function");

        Ok(Some((
            Item::Function(name, params, return_type, body),
            start..end,
        )))
    }

    /// Parse a parameter
    ///
    /// param -> ident : type
    fn parse_param(&mut self) -> Result<Option<(Spanned<String>, Spanned<Type>)>> {
        debug!("enter parse_param");

        let name = if let Some(ident) = self.parse_ident() {
            ident
        } else {
            return Ok(None);
        };

        if !self.match_(&[Token::Punct(':')]) {
            let token = self.peek().unwrap().clone();
            let err = Simple::expected_input_found(
                token.1.clone(),
                [Some("':'".to_owned())],
                Some(token.0.to_string()),
            );
            return Err(err);
        }

        let ty = if let Some(ty) = self.parse_type()? {
            ty
        } else {
            let start = self.previous().unwrap().1.end;
            let end = self.peek().unwrap().1.start;
            let err = Simple::custom(start..end, "missing type");
            return Err(err);
        };

        debug!("exit parse_param: ", (&name, &ty));

        Ok(Some((name, ty)))
    }

    /// Parse a Type
    ///
    /// type -> boolean | empty | float | integer | option | string | UDT | uuid
    fn parse_type(&mut self) -> Result<Option<Spanned<Type>>> {
        debug!("enter parse_type");

        let start = if let Some(tok) = self.peek() {
            tok.1.start
        } else {
            return Ok(None);
        };

        // Match a boolean
        if self.match_(&[Token::Type(Type::Boolean)]) {
            debug!("exit parse_type: boolean");
            return Ok(Some((Type::Boolean, start..self.peek().unwrap().1.end)));
        }

        // Match emppty
        if self.match_(&[Token::Punct('(')]) {
            if !self.match_(&[Token::Punct(')')]) {
                let token = self.peek().unwrap().clone();
                let err = Simple::expected_input_found(
                    token.1.clone(),
                    [Some("')'".to_owned())],
                    Some(token.0.to_string()),
                );
                return Err(err);
            }

            debug!("exit parse_type: empty");
            return Ok(Some((Type::Empty, start..self.peek().unwrap().1.end)));
        }

        // Match a float
        if self.match_(&[Token::Type(Type::Float)]) {
            debug!("exit parse_type: float");
            return Ok(Some((Type::Float, start..self.peek().unwrap().1.end)));
        }

        // Mtatch an integer
        if self.match_(&[Token::Type(Type::Integer)]) {
            debug!("exit parse_type: integer");
            return Ok(Some((Type::Integer, start..self.peek().unwrap().1.end)));
        }

        // Match a list
        if self.match_(&[Token::Punct('[')]) {
            let ty = if let Some(ty) = self.parse_type()? {
                ty
            } else {
                let start = self.previous().unwrap().1.end;
                let end = self.peek().unwrap().1.start;
                let err = Simple::custom(start..end, "missing type");
                return Err(err);
            };

            if !self.match_(&[Token::Punct(']')]) {
                let token = self.peek().unwrap().clone();
                let err = Simple::expected_input_found(
                    token.1.clone(),
                    [Some("']'".to_owned())],
                    Some(token.0.to_string()),
                );
                return Err(err);
            }

            debug!("exit parse_type: list");
            return Ok(Some((
                Type::List(Box::new(ty)),
                start..self.peek().unwrap().1.end,
            )));
        }

        // Match an option
        if self.match_(&[Token::Option]) {
            if !self.match_(&[Token::Punct('<')]) {
                let token = self.peek().unwrap().clone();
                let err = Simple::expected_input_found(
                    token.1.clone(),
                    [Some("'<'".to_owned())],
                    Some(token.0.to_string()),
                );
                return Err(err);
            }

            let ty = self.parse_type()?;

            if let None = ty {
                let tok = self.peek().unwrap();
                let err = Simple::expected_input_found(
                    tok.1.clone(),
                    [
                        Some("'bool'".to_owned()),
                        Some("'float'".to_owned()),
                        Some("'int'".to_owned()),
                        Some("'string'".to_owned()),
                        Some("'Uuid'".to_owned()),
                        Some("'Option<T>'".to_owned()),
                        Some("'[T]'".to_owned()),
                    ],
                    Some(tok.0.to_string()),
                );
                return Err(err);
            }

            if !self.match_(&[Token::Punct('>')]) {
                let token = self.peek().unwrap().clone();
                let err = Simple::expected_input_found(
                    token.1.clone(),
                    [Some("'>'".to_owned())],
                    Some(token.0.to_string()),
                );
                return Err(err);
            }

            debug!("exit parse_type: option", ty);
            return Ok(Some((
                Type::Option(Box::new(ty.unwrap())),
                start..self.peek().unwrap().1.end,
            )));
        }

        // Match Self
        if self.match_(&[Token::Self_]) {
            debug!("exit parse_type: self");
            return Ok(Some((Type::Self_, start..self.peek().unwrap().1.end)));
        }

        // Match String
        if self.match_(&[Token::Type(Type::String)]) {
            debug!("exit parse_type: string");
            return Ok(Some((Type::String, start..self.peek().unwrap().1.end)));
        }

        // Match User Defined Type
        if let Some(ident) = self.parse_ident() {
            debug!("exit parse_type: user defined", ident);
            return Ok(Some((
                Type::UserType(ident),
                start..self.peek().unwrap().1.end,
            )));
        }

        // Match Uuid
        if self.match_(&[Token::Uuid]) {
            debug!("exit parse_type: uuid");
            return Ok(Some((Type::Uuid, start..self.peek().unwrap().1.end)));
        }

        Ok(None)
    }

    /// Parse a Struct
    ///
    /// struct -> struct IDENT { struct_field* }
    fn parse_struct(&mut self) -> Result<Option<Spanned<Item>>> {
        let start = if let Some(tok) = self.peek() {
            tok.1.start
        } else {
            return Ok(None);
        };

        if !self.match_(&[Token::Struct]) {
            return Ok(None);
        }

        let name = if let Some(ident) = self.parse_ident() {
            ident
        } else {
            let tok = self.peek().unwrap();
            let err = Simple::expected_input_found(
                tok.1.clone(),
                [Some("identifier".to_owned())],
                Some(tok.0.to_string()),
            );
            let err = err.with_label("expected identifier");
            return Err(err);
        };

        if !self.match_(&[Token::Punct('{')]) {
            let tok = self.peek().unwrap();
            return Err(Simple::expected_input_found(
                tok.1.clone(),
                [Some("'{".to_owned())],
                Some(tok.0.to_string()),
            ));
        }

        let mut fields = Vec::new();

        while !self.at_end() && !self.match_(&[Token::Punct('}')]) {
            match self.parse_struct_field() {
                Ok(field) => {
                    fields.push(field);
                }
                Err(err) => {
                    return Err(err);
                }
            }
            self.match_(&[Token::Punct(',')]);
        }

        // 🚧 This isn't right, but maybe it's good enough.
        let end = if let Some(tok) = self.peek() {
            tok.1.end
        } else {
            self.previous().unwrap().1.end
        };

        Ok(Some((Item::Struct(name, fields), start..end)))
    }

    /// Parse an identifier
    ///
    /// ident -> IDENT
    fn parse_ident(&mut self) -> Option<Spanned<String>> {
        debug!("enter parse_ident");

        let next = self.peek()?.clone();

        if let (Token::Ident(ident), span) = next {
            self.advance();
            debug!("exit parse_ident");
            Some((ident.to_owned(), span.to_owned()))
        } else {
            None
        }
    }

    /// Parse a struct field
    ///
    /// field -> IDENT : TYPE
    fn parse_struct_field(&mut self) -> Result<(Spanned<String>, Spanned<Type>)> {
        debug!("enter parse_struct_field");

        let name = if let Some(ident) = self.parse_ident() {
            ident
        } else {
            let tok = self.peek().unwrap();
            let err = Simple::expected_input_found(
                tok.1.clone(),
                [Some("identifier".to_owned())],
                Some(tok.0.to_string()),
            );
            let err = err.with_label("expected identifier");
            return Err(err);
        };

        if !self.match_(&[Token::Punct(':')]) {
            let tok = self.peek().unwrap();
            let err = Simple::expected_input_found(
                tok.1.clone(),
                [Some("':'".to_owned())],
                Some(tok.0.to_string()),
            );
            let err = err.with_label("expected colon");
            return Err(err);
        }

        let ty = if let Some(ty) = self.parse_type()? {
            ty
        } else {
            let tok = self.peek().unwrap();
            let err = Simple::expected_input_found(
                tok.1.clone(),
                [
                    Some("'bool'".to_owned()),
                    Some("'float'".to_owned()),
                    Some("'int'".to_owned()),
                    Some("'string'".to_owned()),
                    Some("'Uuid'".to_owned()),
                    Some("'Option<T>'".to_owned()),
                    Some("'[T]'".to_owned()),
                ],
                Some(tok.0.to_string()),
            );
            let err = err.with_label("expected type");
            return Err(err);
        };

        debug!("exit parse_struct_field: ", (&name, &ty));

        Ok((name, ty))
    }

    fn match_(&mut self, tokens: &[Token]) -> bool {
        for tok in tokens {
            if self.check(tok) {
                self.advance();
                debug!("matched: ", tok);
                return true;
            }
        }

        false
    }

    fn check(&mut self, tok: &Token) -> bool {
        if self.at_end() {
            return false;
        }

        if self.peek().unwrap().0 == *tok {
            true
        } else {
            false
        }
    }

    fn advance(&mut self) -> Option<&Spanned<Token>> {
        if !self.at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn at_end(&self) -> bool {
        self.peek().is_none()
    }

    fn peek(&self) -> Option<&Spanned<Token>> {
        let current = self.tokens.get(self.current);
        debug!("peek:", current);

        current
    }

    fn previous(&self) -> Option<&Spanned<Token>> {
        if self.current == 0 {
            return None;
        }

        self.tokens.get(self.current - 1)
    }
}

pub fn parse_line(src: &str) -> Option<Spanned<Statement>> {
    let (tokens, errs) = lexer().parse_recovery_verbose(src);

    let mut parser = DwarfParser::new(tokens.unwrap());
    let ast = parser.parse_statement();

    match ast {
        Ok(ast) => return ast,
        Err(err) => {
            let mut parser_errs = parser.errors;
            parser_errs.push(err);
            report_errors(errs, parser_errs, src);
            return None;
        }
    }

    //     let (tokens, errs) = lexer().parse_recovery(src);
    //     let (ast, parse_errs) = if let Some(tokens) = tokens {
    //         let len = src.chars().count();
    //         let (ast, parse_errs) =
    //         // stmt_parser()
    //         // .parse_recovery_verbose(Stream::from_iter(len..len + 1, tokens.into_iter()));
    //         stmt_parser().parse_recovery(Stream::from_iter(len..len + 1, tokens.into_iter()));

    //         (ast, parse_errs)
    //     } else {
    //         (None, Vec::new())
    //     };

    //     report_errors(errs, parse_errs, src);

    //     ast
    ast.unwrap()
}

pub fn parse_dwarf(src: &str) -> Vec<Spanned<Item>> {
    let (tokens, errs) = lexer().parse_recovery_verbose(src);

    // dbg!(&errs);

    let mut parser = DwarfParser::new(tokens.unwrap());
    let (ast, parse_errs) = parser.parse_program();

    // let (tokens, errs) = lexer().parse_recovery(src);
    // let (ast, parse_errs) = if let Some(tokens) = tokens {
    //     let len = src.chars().count();
    //     let (ast, parse_errs) = item_parser()
    //         .parse_recovery_verbose(Stream::from_iter(len..len + 1, tokens.into_iter()));
    //     // .parse_recovery(Stream::from_iter(len..len + 1, tokens.into_iter()));

    //     (ast, parse_errs)
    // } else {
    //     (None, Vec::new())
    // };

    report_errors(errs, parse_errs, src);

    // ast

    ast
}

fn report_errors(errs: Vec<Simple<char>>, parse_errs: Vec<Simple<String>>, src: &str) {
    // dbg!(&ast, &errs, &parse_errs);
    errs.into_iter()
        .map(|e| e.map(|c| c.to_string()))
        .chain(parse_errs.into_iter().map(|e| e.map(|tok| tok.to_string())))
        .for_each(|e| {
            let report = Report::build(ReportKind::Error, (), e.span().start);

            let report = match e.reason() {
                chumsky::error::SimpleReason::Unclosed { span, delimiter } => report
                    .with_message(format!(
                        "Unclosed delimiter {}",
                        delimiter.fg(Color::Yellow)
                    ))
                    .with_label(
                        Label::new(span.clone())
                            .with_message(format!(
                                "Unclosed delimiter {}",
                                delimiter.fg(Color::Yellow)
                            ))
                            .with_color(Color::Yellow),
                    )
                    .with_label(
                        Label::new(e.span())
                            .with_message(format!(
                                "Must be closed before this {}",
                                e.found()
                                    .unwrap_or(&"end of file".to_string())
                                    .fg(Color::Red)
                            ))
                            .with_color(Color::Red),
                    ),
                chumsky::error::SimpleReason::Unexpected => report
                    .with_message(format!(
                        "{}, expected {}",
                        if e.found().is_some() {
                            "Unexpected token in input"
                        } else {
                            "Unexpected end of input"
                        },
                        if e.expected().len() == 0 {
                            "something else".to_string()
                        } else {
                            e.expected()
                                .map(|expected| match expected {
                                    Some(expected) => expected.to_string(),
                                    None => "end of input".to_string(),
                                })
                                .collect::<Vec<_>>()
                                .join(", ")
                        }
                    ))
                    .with_label(
                        Label::new(e.span())
                            .with_message(format!(
                                "Unexpected token {}",
                                e.found()
                                    .unwrap_or(&"end of file".to_string())
                                    .fg(Color::Red)
                            ))
                            .with_color(Color::Red),
                    ),
                chumsky::error::SimpleReason::Custom(msg) => report.with_message(msg).with_label(
                    Label::new(e.span())
                        .with_message(format!("{}", msg.fg(Color::Red)))
                        .with_color(Color::Red),
                ),
            };

            report.finish().print(Source::from(&src)).unwrap();
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_statement() {
        let _ = env_logger::builder().is_test(true).try_init();

        let src = r#"
            ;
        "#;

        let ast = parse_line(src);

        assert!(ast.is_some());
        assert_eq!(ast.unwrap(), (Statement::Empty, (13..14)));
    }

    #[test]
    fn test_lexer() {
        let _ = env_logger::builder().is_test(true).try_init();

        let src = r#"
            // This is a comment
            type Foo {
                bar: int,
                baz: str,
            }
            enum Bar {}
            enum Baz
        "#;

        let (tokens, errs) = lexer().parse_recovery(src);

        assert!(tokens.is_some());
        assert!(errs.is_empty());
    }

    #[test]
    fn test_struct() {
        let _ = env_logger::builder().is_test(true).try_init();

        let src = r#"
            struct Foo {
                bar: Option<int>,
                baz: string,
                uber: Uuid
            }

            struct Bar {}
        "#;

        let ast = parse_dwarf(src);

        assert!(!ast.is_empty());
    }

    #[test]
    fn test_import() {
        let _ = env_logger::builder().is_test(true).try_init();

        let src = r#"
            use foo;
            use foo::bar::baz::Baz;
            use foo::bar::Xyzzy as Plugh;
        "#;

        let ast = parse_dwarf(src);

        // dbg!(&ast);

        assert!(!ast.is_empty());
    }

    #[test]
    fn xyzzy() {
        let _ = env_logger::builder().is_test(true).try_init();

        let src = r#"
            fn foo(a: string, b: int, d: Option<Foo>) -> Option<bool> {
                None
            }

            fn bar() -> Option<string> {
                Some("Hello, World!")
            }

            fn xyzzy() -> Bar {
                foo(a, b, c.d(), Foo::new(), 42, 3.14, "Hello, World!", true);
                let a = Foo::new();
                let b = a.b();
                a.collect();
                a.b.c.e;
                a.b.c.d();
                Bar {
                    foo: 42,
                    bar: 3.14,
                    baz: "Hello, World!",
                    uber: Foo::new(),
                    foo: true
                }
            }

            fn yzzyx() -> Self {
                Foo::bar(a, 42, 3.14, true, "Hello, World!", Foo::new(), a.b.c, d.e.f.g(4, 8));
                Uuid::new();
                Self {
                    foo: 42,
                    bar: 3.14,
                    test: func(),
                    baz: "Hello, World!",
                    uber: Foo::new(),
                    foo: true
                }
            }
        "#;

        let ast = parse_dwarf(src);

        // dbg!(&ast);

        assert!(!ast.is_empty());
    }

    #[test]
    fn test_impl() {
        let _ = env_logger::builder().is_test(true).try_init();

        let src = r#"
            impl Foo {
                fn new() -> Self {
                    let empty = Self {a: 42,
                        b: 3.14, c: "Hello, World!"};

                    Uuid::new();
                    Self {
                        bar: 42,
                        pi: 3.14,
                        baz: "Hello, World!",
                        test: func(),
                        uber: Uuid::new(),
                        foo: true
                    }
                }

                fn foo() -> Foo {
                    let empty = Foo {};
                    Foo {
                        bar: 42,
                        pi: 3.14,
                        baz: "Hello, World!",
                        uber: Uuid::new(),
                        foo: false,
                    }
                }

                fn bar(baz: Bar, id: Uuid) -> int {
                    let a = 1;
                    let b = true;
                    let c = "Hello, World!";
                    let d = 3.14;
                    true
                }

                fn baz() -> Bar {
                    let id = id_func(a, foo(), 42, 3.14, "Hello, World!", true);
                    let a = id.name;
                    let bar = id.func();
                    let id = Uuid::new();
                    let hello = Bar::new(42, 3.14, "Hello, World!", true, bar, id, func());
                }
            }
        "#;

        let ast = parse_dwarf(src);

        // dbg!(&ast);

        assert!(!ast.is_empty());
    }

    #[test]
    fn test_item_fn() {
        let _ = env_logger::builder().is_test(true).try_init();

        let src = r#"
            fn foo() -> int {
                let a = 1;
                let b = true;
                let c = "Hello, World!";
                let d = 3.14;
                true
            }

            fn beano() -> bool {
                let a: int = 1;
                let b: bool = true;
                let c: string = "Hello, World!";
                let d: float = 3.14;
                true
            }

            fn bar() -> () {
                print("Hello World");
            }
        "#;

        let ast = parse_dwarf(src);

        assert!(ast.len() == 3);
    }

    // #[test]
    // fn test_reference() {
    //     let _ = env_logger::builder().is_test(true).try_init();

    //     let src = r#"
    //         fn foo() -> () {
    //             let a = &b;
    //         }

    //         fn fun() -> () {}

    //         fn bar(a: &int) -> [&Bar] {}

    //         impl Bar {
    //             fn baz(&self) -> () {}
    //         }
    //     "#;

    //     let ast = parse_dwarf(src);

    //     assert!(!ast.is_empty());
    // }

    #[test]
    fn test_list() {
        let _ = env_logger::builder().is_test(true).try_init();

        let src = r#"
            fn foo() -> [int] {
                let a = [1, 2, 3];
            }
        "#;

        let ast = parse_dwarf(src);

        assert!(!ast.is_empty());
    }

    #[test]
    fn test_option() {
        let _ = env_logger::builder().is_test(true).try_init();

        let src = r#"
            fn foo() -> [int] {
                let a = 123;
                let b = Some (42);
                let a = None;
                call(a, b, None, Some(42));
            }
        "#;

        let ast = parse_dwarf(src);

        assert!(!ast.is_empty());
    }

    #[test]
    fn test_for() {
        let _ = env_logger::builder().is_test(true).try_init();

        let src = r#"
            fn foo() -> () {
                for a in b {
                    print(a);
                }

                for a in [1, 2, 3] {
                    print(a);
                }
            }
        "#;

        let ast = parse_dwarf(src);

        // dbg!(&ast);

        assert!(!ast.is_empty());
    }

    #[test]
    fn test_field_access() {
        let _ = env_logger::builder().is_test(true).try_init();

        let src = "a.id;";

        let ast = parse_line(src);

        // dbg!(&ast);

        assert!(ast.is_some());
    }

    #[test]
    fn test_return() {
        let _ = env_logger::builder().is_test(true).try_init();

        let src = r#"
            fn foo() -> int {
                return 42;
            }

            fn bar() -> int {
                return { 42 };
            }
        "#;

        let ast = parse_dwarf(src);

        dbg!(&ast);

        assert!(!ast.is_empty());
    }
}
