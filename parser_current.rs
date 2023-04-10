use std::fmt;

use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::{prelude::*, stream::Stream};
use fnv::FnvHashMap as HashMap;

pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Token {
    Integer(String),
    Float(String),
    Str(String),
    Bool(bool),
    Ident(String),
    Punct(char),
    Type(Type),
    Op(String),
    Object(String),
    Struct,
    Option,
    Fn,
    Impl,
    Self_,
    Let,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Integer(num) => write!(f, "{}", num),
            Token::Float(num) => write!(f, "{}", num),
            Token::Str(str_) => write!(f, "{}", str_),
            Token::Bool(bool_) => write!(f, "{}", bool_),
            Token::Ident(ident) => write!(f, "{}", ident),
            Token::Punct(punct) => write!(f, "{}", punct),
            Token::Type(type_) => write!(f, "{}", type_),
            Token::Op(op) => write!(f, "{}", op),
            Token::Object(object) => write!(f, "{}", object),
            Token::Struct => write!(f, "type"),
            Token::Option => write!(f, "Option"),
            Token::Fn => write!(f, "fn"),
            Token::Impl => write!(f, "impl"),
            Token::Self_ => write!(f, "Self"),
            Token::Let => write!(f, "let"),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Boolean,
    Float,
    Integer,
    String,
    Uuid,
    Option(Box<Type>),
    UserType(Box<Token>),
    Self_(Box<Token>),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Boolean => write!(f, "bool"),
            Type::Float => write!(f, "float"),
            Type::Integer => write!(f, "int"),
            Type::String => write!(f, "string"),
            Type::Uuid => write!(f, "Uuid"),
            Type::Option(type_) => write!(f, "Option<{}>", type_),
            Type::UserType(type_) => write!(f, "{}", type_),
            Type::Self_(type_) => write!(f, "{}", type_),
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    Let(Spanned<String>, Expression),
    Expression(Expression),
    Result(Expression),
    Item(Item),
}

#[derive(Debug)]
pub enum Expression {
    Block(Vec<Statement>),
    StringLiteral(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    BooleanLiteral(bool),
    Struct(Vec<(Spanned<String>, Expression)>),
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ItemKind {
    Struct,
    Function,
    Implementation,
}

#[derive(Debug)]
pub enum Item {
    Struct(Vec<(Spanned<String>, Spanned<Type>)>),
    Function {
        params: Vec<(Spanned<String>, Spanned<Type>)>,
        return_type: Spanned<Type>,
        body: Vec<Spanned<Statement>>,
    },
    Implementation(Vec<(Spanned<String>, Box<Item>)>),
}

fn lexer() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
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
        .map(Token::Str);

    let dagger = just::<char, char, Simple<char>>('-')
        .ignore_then(just('>'))
        .map(|_| Token::Punct('→'));

    let double_colon = just::<char, char, Simple<char>>(':')
        .ignore_then(just(':'))
        .map(|_| Token::Punct('⊥'));

    // A parser for operators
    let op = one_of("+-*/!=")
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(Token::Op);

    // A parser for punctuation (delimiters, semicolons, etc.)
    let punct = one_of("()[]{}:;,<>").map(|c| Token::Punct(c));

    // A "Object type" parser. Basically I'm asserting that if an identifier starts
    // with a capital letter, then it's an object.
    let object = filter::<_, _, Simple<char>>(|c: &char| c.is_uppercase())
        .chain::<char, Vec<_>, _>(filter(|c: &char| c.is_alphanumeric() || *c == '_').repeated())
        .collect::<String>()
        .map(Token::Object);

    // A parser for identifiers and keywords
    let ident = text::ident().map(|ident: String| match ident.as_str() {
        "fn" => Token::Fn,
        "let" => Token::Let,
        // "print" => Token::Print,
        // "if" => Token::If,
        // "else" => Token::Else,
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        "type" => Token::Struct,
        "impl" => Token::Impl,
        "int" => Token::Type(Type::Integer),
        "string" => Token::Type(Type::String),
        "bool" => Token::Type(Type::Boolean),
        "float" => Token::Type(Type::Float),
        "Uuid" => Token::Type(Type::Uuid),
        "Option" => Token::Option,
        "Self" => Token::Self_,
        _ => Token::Ident(ident),
    });

    // A single token can be one of the above
    let token = float
        .or(int)
        .or(str_)
        .or(dagger)
        .or(double_colon)
        .or(op)
        .or(punct)
        .or(object)
        .or(ident)
        .recover_with(skip_then_retry_until([]));

    let comment = just("//").then(take_until(just('\n'))).padded();

    token
        .map_with_span(|tok, span| (tok, span))
        .padded_by(comment.repeated())
        .padded()
        .repeated()
}

fn type_parser() -> impl Parser<Token, Type, Error = Simple<Token>> + Clone {
    recursive(|type_| {
        let type_ = filter_map(|span: Span, tok| match tok {
            Token::Type(type_) => Ok(type_.clone()),
            Token::Object(ident) => Ok(Type::UserType(Box::new(Token::Object(ident.clone())))),
            _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
        });

        let option = just(Token::Option)
            .ignore_then(just(Token::Punct('<')))
            .ignore_then(type_.clone())
            .then_ignore(just(Token::Punct('>')))
            .map(|type_| Type::Option(Box::new(type_)));

        // let self_ = just(Token::Self_).map(|_| Type::Self_(Box::new(Token::Self_)));

        // self_.or(option).or(type_)
        option.or(type_)
    })
}

fn stmt_parser() -> impl Parser<Token, Statement, Error = Simple<Token>> + Clone {
    let ident = filter_map(|span: Span, tok| match tok {
        Token::Ident(ident) => Ok(ident.clone()),
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
    });

    let let_ = just(Token::Let)
        .ignore_then(
            ident
                .map_with_span(|ident, span| (ident, span))
                .labelled("variable name"),
        )
        .then_ignore(just(Token::Op("=".to_string())))
        .then(expr_parser())
        .then_ignore(just(Token::Punct(';')))
        .map(|(name, expr)| Statement::Let(name, expr));

    let_
}

fn expr_parser() -> impl Parser<Token, Expression, Error = Simple<Token>> + Clone {
    let val = select! {
        Token::Bool(x) => Expression::BooleanLiteral(x),
        Token::Integer(n) => Expression::IntegerLiteral(n.parse().unwrap()),
        Token::Float(n) => Expression::FloatLiteral(n.parse().unwrap()),
        Token::Str(s) => Expression::StringLiteral(s),
    }
    .labelled("value");

    val
}

fn impl_block_parser(
) -> impl Parser<Token, ((Spanned<String>, ItemKind), Item), Error = Simple<Token>> + Clone {
    let ident = filter_map(|span: Span, tok| match tok {
        Token::Object(ident) => Ok(ident.clone()),
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
    });

    let functions = fun_parser()
        .map_with_span(|name, span| (name, span))
        .repeated()
        .delimited_by(just(Token::Punct('{')), just(Token::Punct('}')))
        .map(|functions| functions.into_iter().collect::<Vec<_>>());

    just(Token::Impl)
        .ignore_then(
            ident
                .map_with_span(|name, span| (name, span))
                .labelled("implementation name"),
        )
        .then(functions)
        .map(|(name, functions)| {
            let functions = functions
                .into_iter()
                .map(|(((name, _), function), _)| (name, Box::new(function)))
                .collect::<Vec<_>>();
            (
                (name, ItemKind::Implementation),
                Item::Implementation(functions),
            )
        })
        .labelled("implementation block")
}

fn fun_parser(
) -> impl Parser<Token, ((Spanned<String>, ItemKind), Item), Error = Simple<Token>> + Clone {
    let ident = filter_map(|span: Span, tok| match tok {
        Token::Ident(ident) => Ok(ident.clone()),
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
    });

    let param = ident
        .clone()
        .map_with_span(|type_, span| (type_, span))
        .labelled("param type")
        .then_ignore(just(Token::Punct(':')))
        .then(
            type_parser()
                .map_with_span(|name, span| (name, span))
                .labelled("param name"),
        )
        .map(|(name, type_)| (name, type_));

    let params = param
        .separated_by(just(Token::Punct(',')))
        .allow_trailing()
        .delimited_by(just(Token::Punct('(')), just(Token::Punct(')')))
        .labelled("function parameters")
        .map(|params| params.into_iter().collect::<Vec<_>>());

    just(Token::Fn)
        .ignore_then(
            ident
                .map_with_span(|name, span| (name, span))
                .labelled("function name"),
        )
        .then(params)
        .map_with_span(|name, span| (name, span))
        .labelled("function parameters")
        .then_ignore(just(Token::Punct('→')))
        .then(type_parser())
        .map_with_span(|ty, span| (ty, span))
        .labelled("return type")
        .then(
            stmt_parser()
                .map_with_span(|stmt, span| (stmt, span))
                .repeated()
                .delimited_by(just(Token::Punct('{')), just(Token::Punct('}')))
                .labelled("function body"),
        )
        // This got away from me...
        .map(
            |(((((name, params), _span_2), return_type), span_3), body)| {
                (
                    (name, ItemKind::Function),
                    Item::Function {
                        params,
                        return_type: (return_type, span_3),
                        body,
                    },
                )
            },
        )
        .labelled("function")
}

fn struct_parser(
) -> impl Parser<Token, ((Spanned<String>, ItemKind), Item), Error = Simple<Token>> + Clone {
    let obj = filter_map(|span: Span, tok| match tok {
        Token::Object(ident) => Ok(ident.clone()),
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
    });

    let ident = filter_map(|span: Span, tok| match tok {
        Token::Ident(ident) => Ok(ident.clone()),
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
    });

    let field = ident
        .map_with_span(|type_, span| (type_, span))
        .labelled("field type")
        .then_ignore(just(Token::Punct(':')))
        .then(
            type_parser()
                .map_with_span(|name, span| (name, span))
                .labelled("field name"),
        )
        .map(|(name, type_)| (name, type_));

    let fields = field
        .separated_by(just(Token::Punct(',')))
        .allow_trailing()
        .delimited_by(just(Token::Punct('{')), just(Token::Punct('}')))
        .map(|fields| fields.into_iter().collect::<Vec<_>>());

    just(Token::Struct)
        .ignore_then(
            obj.map_with_span(|name, span| (name, span))
                .labelled("struct name"),
        )
        .then(fields)
        .map(|(name, fields)| ((name, ItemKind::Struct), Item::Struct(fields)))
        .labelled("struct")
}

fn item_parser(
) -> impl Parser<Token, HashMap<(String, ItemKind), Item>, Error = Simple<Token>> + Clone {
    let item = struct_parser().or(impl_block_parser());

    item.repeated()
        .try_map(|items, _| {
            let mut result = HashMap::default();
            for (((name, span), kind), item) in items {
                if result.insert((name.clone(), kind), item).is_some() {
                    return Err(Simple::custom(
                        span.clone(),
                        format!("Item `{}` already defined", name),
                    ));
                }
            }
            Ok(result)
        })
        .then_ignore(end())
}

// pub struct Error {
// span: Span,
// msg: String,
// }

pub fn parse(src: &str) -> Option<HashMap<(String, ItemKind), Item>> {
    let (tokens, errs) = lexer().parse_recovery(src);
    let (ast, parse_errs) = if let Some(tokens) = tokens {
        let len = src.chars().count();
        let (ast, parse_errs) =
            item_parser().parse_recovery(Stream::from_iter(len..len + 1, tokens.into_iter()));

        (ast, parse_errs)
    } else {
        (None, Vec::new())
    };

    dbg!(&ast, &errs, &parse_errs);
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

    ast
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
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

        // dbg!(&tokens, &errs);

        assert!(tokens.is_some());
        assert!(errs.is_empty());
    }

    #[test]
    fn test_struct() {
        let src = r#"
            type Foo {
                bar: Option<int>,
                baz: string,
                uber: uuid
            }

            type Bar {}
        "#;

        let ast = parse(src);

        dbg!(&ast);

        assert!(ast.is_some());
    }

    #[test]
    fn test_impl() {
        let src = r#"
            impl Foo {
                fn bar(baz: Bar, id: Uuid) -> int {
                    let a = 1;
                    let b = true;
                    let c = "Hello, World!";
                    let d = 3.14;
                }

                fn baz() -> Bar {
                    // let id = uuid::new();
                    // Bar {}
                }
            }
        "#;

        let ast = parse(src);

        dbg!(&ast);

        assert!(ast.is_some());
    }
}
