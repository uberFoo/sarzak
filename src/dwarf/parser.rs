use std::fmt;

use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::{prelude::*, stream::Stream};
use fnv::FnvHashMap as HashMap;

pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Token {
    Num(String),
    Str(String),
    Ident(String),
    Punct(char),
    Type(Type),
    Struct,
    Option,
    Fn,
    Impl,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Num(num) => write!(f, "{}", num),
            Token::Str(str_) => write!(f, "{}", str_),
            Token::Ident(ident) => write!(f, "{}", ident),
            Token::Punct(punct) => write!(f, "{}", punct),
            Token::Type(type_) => write!(f, "{}", type_),
            Token::Struct => write!(f, "type"),
            Token::Option => write!(f, "Option"),
            Token::Fn => write!(f, "fn"),
            Token::Impl => write!(f, "impl"),
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
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Boolean => write!(f, "bool"),
            Type::Float => write!(f, "float"),
            Type::Integer => write!(f, "int"),
            Type::String => write!(f, "string"),
            Type::Uuid => write!(f, "uuid"),
            Type::Option(type_) => write!(f, "option<{}>", type_),
        }
    }
}

fn lexer() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    // A parser for numbers
    let num = text::int(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
        .collect::<String>()
        .map(Token::Num);

    // A parser for strings
    let str_ = just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Token::Str);

    // A parser for operators
    // let op = one_of("+-*/!=")
    //     .repeated()
    //     .at_least(1)
    //     .collect::<String>()
    //     .map(Token::Op);

    // A parser for punctuation (delimiters, semicolons, etc.)
    let punct = one_of("()[]{}:;,<>").map(|c| Token::Punct(c));

    // A parser for identifiers and keywords
    let ident = text::ident().map(|ident: String| match ident.as_str() {
        "fn" => Token::Fn,
        // "let" => Token::Let,
        // "print" => Token::Print,
        // "if" => Token::If,
        // "else" => Token::Else,
        // "true" => Token::Bool(true),
        // "false" => Token::Bool(false),
        // "null" => Token::Null,
        "type" => Token::Struct,
        "impl" => Token::Impl,
        "int" => Token::Type(Type::Integer),
        "string" => Token::Type(Type::String),
        "bool" => Token::Type(Type::Boolean),
        "float" => Token::Type(Type::Float),
        "uuid" => Token::Type(Type::Uuid),
        "Option" => Token::Option,
        _ => Token::Ident(ident),
    });

    // A single token can be one of the above
    let token = num
        .or(str_)
        // .or(op)
        .or(punct)
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
pub enum Item {
    Struct {
        fields: Vec<(Spanned<String>, Spanned<Type>)>,
    },
    Function,
    Implementation,
}

fn type_parser() -> impl Parser<Token, Type, Error = Simple<Token>> + Clone {
    let type_ = recursive(|type_| {
        let type_ = filter_map(|span: Span, tok| match tok {
            Token::Type(type_) => Ok(type_.clone()),
            _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
        });

        let option = just(Token::Option)
            .ignore_then(just(Token::Punct('<')))
            .ignore_then(type_.clone())
            .then_ignore(just(Token::Punct('>')))
            .map(|type_| Type::Option(Box::new(type_)));

        option.or(type_)
    });

    type_
}

fn structs_parser() -> impl Parser<Token, HashMap<String, Item>, Error = Simple<Token>> + Clone {
    let ident = filter_map(|span: Span, tok| match tok {
        Token::Ident(ident) => Ok(ident.clone()),
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
    });

    let type_ = type_parser();

    let field = ident
        .clone()
        .map_with_span(|type_, span| (type_, span))
        .labelled("field type")
        .then_ignore(just(Token::Punct(':')))
        .then(
            type_
                .map_with_span(|name, span| (name, span))
                .labelled("field name"),
        )
        .map(|(name, type_)| (name, type_));

    let fields = field
        .separated_by(just(Token::Punct(',')))
        .allow_trailing()
        .delimited_by(just(Token::Punct('{')), just(Token::Punct('}')))
        .map(|fields| fields.into_iter().collect::<Vec<_>>());

    let struct_ = just(Token::Struct)
        .ignore_then(
            ident
                .map_with_span(|name, span| (name, span))
                .labelled("struct name"),
        )
        .then(fields)
        .map(|(name, fields)| (name, Item::Struct { fields }))
        .labelled("struct");

    struct_
        .repeated()
        .try_map(|ss, _| {
            let mut structs = HashMap::default();
            for ((name, name_span), s) in ss {
                if structs.insert(name.clone(), s).is_some() {
                    return Err(Simple::custom(
                        name_span.clone(),
                        format!("Struct `{}` already defined", name),
                    ));
                }
            }
            Ok(structs)
        })
        .then_ignore(end())
}

// fn item_parser() -> impl Parser<Token, HashMap<String, Item>, Error = Simple<Token>> + Clone {
//     let ident = filter_map(|span, tok| match tok {
//         Token::Ident(ident) => Ok(ident.clone()),
//         _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
//     });

//     let struct_ = just(Token::Struct).ignore_then(
//         ident
//             .map_with_span(|name, span| (name, span))
//             .labelled("struct name"),
//     ).then

// }

// pub struct Error {
// span: Span,
// msg: String,
// }

pub fn parse(src: &str) -> Option<HashMap<String, Item>> {
    let (tokens, errs) = lexer().parse_recovery(src);
    let (ast, parse_errs) = if let Some(tokens) = tokens {
        let len = src.chars().count();
        let (ast, parse_errs) =
            structs_parser().parse_recovery(Stream::from_iter(len..len + 1, tokens.into_iter()));

        (ast, parse_errs)
    } else {
        (None, Vec::new())
    };

    dbg!(&errs, &parse_errs);
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
            dbg!("bar");
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
}
