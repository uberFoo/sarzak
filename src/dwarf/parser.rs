use std::fmt;

use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::{prelude::*, stream::Stream};
use fnv::FnvHashMap as HashMap;

pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Token {
    Bool(bool),
    Float(String),
    Fn,
    Ident(String),
    Impl,
    Integer(String),
    Let,
    // This is a type that starts with a capital letter. It's special. 💩
    Object(String),
    Op(String),
    Option,
    Print,
    Punct(char),
    Self_,
    Str(String),
    Struct,
    Type(Type),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bool(bool_) => write!(f, "{}", bool_),
            Self::Float(num) => write!(f, "{}", num),
            Self::Fn => write!(f, "fn"),
            Self::Ident(ident) => write!(f, "{}", ident),
            Self::Impl => write!(f, "impl"),
            Self::Integer(num) => write!(f, "{}", num),
            Self::Let => write!(f, "let"),
            Self::Object(object) => write!(f, "{}", object),
            Self::Op(op) => write!(f, "{}", op),
            Self::Option => write!(f, "Option"),
            Self::Print => write!(f, "print"),
            Self::Punct(punct) => write!(f, "{}", punct),
            Self::Self_ => write!(f, "Self"),
            Self::Str(str_) => write!(f, "{}", str_),
            Self::Struct => write!(f, "type"),
            Self::Type(type_) => write!(f, "{}", type_),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Boolean,
    Float,
    Integer,
    Option(Box<Self>),
    Self_(Box<Token>),
    String,
    UserType(Box<Token>),
    Uuid,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean => write!(f, "bool"),
            Self::Float => write!(f, "float"),
            Self::Integer => write!(f, "int"),
            Self::Option(type_) => write!(f, "Option<{}>", type_),
            Self::Self_(type_) => write!(f, "{}", type_),
            Self::String => write!(f, "string"),
            Self::UserType(type_) => write!(f, "{}", type_),
            Self::Uuid => write!(f, "Uuid"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    Expression(Spanned<Expression>),
    Item(Item),
    Let(Spanned<String>, Spanned<Expression>),
    Result(Spanned<Expression>),
}

#[derive(Clone, Debug)]
pub enum Expression {
    Block(Vec<Statement>),
    BooleanLiteral(bool),
    Error,
    FieldAccess(Box<Spanned<Self>>, Spanned<String>),
    FloatLiteral(f64),
    // The first element is the function being called, the second is the list of
    // arguments.
    FunctionCall(Box<Spanned<Self>>, Vec<Spanned<Self>>),
    IntegerLiteral(i64),
    LocalVariable(String),
    MethodCall(Box<Spanned<Self>>, Spanned<String>, Vec<Spanned<Self>>),
    Print(Box<Spanned<Self>>),
    StaticMethodCall(Spanned<Token>, Spanned<String>, Vec<Spanned<Self>>),
    StringLiteral(String),
    // Where the fuck did this come from? Copilot insert it when I wasn't watching?
    // Maybe I did it. It's starting to look familiar. It's describing a list of
    // field assignments, e.g., `foo: 42`.
    Struct(Vec<(Spanned<String>, Self)>),
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ItemKind {
    Function,
    Implementation,
    Struct,
}

#[derive(Clone, Debug)]
pub enum Item {
    Function {
        params: Vec<(Spanned<String>, Spanned<Type>)>,
        return_type: Spanned<Type>,
        body: Vec<Spanned<Statement>>,
    },
    Implementation(Vec<(Spanned<String>, Box<Item>)>),
    Struct(Vec<(Spanned<String>, Spanned<Type>)>),
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
    let punct = one_of("()[]{}:;,.<>").map(|c| Token::Punct(c));

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
        "print" => Token::Print,
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
        "Self" => Token::Self_,
        _ => Token::Ident(ident),
    });

    let option = just("Option").map(|_| Token::Option);

    // A single token can be one of the above
    let token = float
        .or(int)
        .or(str_)
        .or(dagger)
        .or(double_colon)
        .or(op)
        .or(punct)
        .or(option)
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

    let expr = expr_parser()
        .then_ignore(just(Token::Punct(';')))
        .map(Statement::Expression);

    let result = expr_parser().map(Statement::Result);

    let_.or(expr).or(result)
}

fn expr_parser() -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> + Clone {
    recursive(|expr| {
        let raw_expr = recursive(|raw_expr| {
            let literal = select! {
                Token::Bool(x) => Expression::BooleanLiteral(x),
                Token::Integer(n) => Expression::IntegerLiteral(n.parse().unwrap()),
                Token::Float(n) => Expression::FloatLiteral(n.parse().unwrap()),
                Token::Str(s) => Expression::StringLiteral(s),
            }
            .labelled("literal");

            let ident = select! { Token::Ident(ident) => ident.clone() }.labelled("identifier");
            let object = filter_map(|span: Span, tok| match tok {
                Token::Object(ident) => Ok(Token::Object(ident.clone())),
                _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
            });

            // A list of expressions
            let items = expr
                .clone()
                .separated_by(just(Token::Punct(',')))
                .allow_trailing();

            // 'Atoms' are expressions that contain no ambiguity
            let atom = literal
                .or(ident.map(Expression::LocalVariable))
                // In Nano Rust, `print` is just a keyword, just like Python 2, for simplicity
                .or(just(Token::Print)
                    .ignore_then(
                        expr.clone()
                            .delimited_by(just(Token::Punct('(')), just(Token::Punct(')'))),
                    )
                    .map(|expr| Expression::Print(Box::new(expr))))
                .map_with_span(|expr, span| (expr, span))
                // Atoms can also just be normal expressions, but surrounded with parentheses
                .or(expr
                    .clone()
                    .delimited_by(just(Token::Punct('(')), just(Token::Punct(')'))))
                // Attempt to recover anything that looks like a parenthesised expression but contains errors
                .recover_with(nested_delimiters(
                    Token::Punct('('),
                    Token::Punct(')'),
                    [
                        (Token::Punct('['), Token::Punct(']')),
                        (Token::Punct('{'), Token::Punct('}')),
                    ],
                    |span| (Expression::Error, span),
                ));

            let field_access = atom
                .clone()
                .map_with_span(|expr, span| (expr, span))
                .then_ignore(just(Token::Punct('.')))
                .then(ident.map_with_span(|ident, span| (ident, span)))
                .map(|((obj, span), field)| (Expression::FieldAccess(Box::new(obj), field), span));

            let method_call = atom
                .clone()
                .then_ignore(just(Token::Punct('.')))
                .then(ident.map_with_span(|ident, span| (ident, span)))
                .then(
                    items
                        .clone()
                        .delimited_by(just(Token::Punct('(')), just(Token::Punct(')')))
                        .map_with_span(|args, span: Span| (args, span))
                        .repeated(),
                )
                // This bit feels dirty. Like there is an elegant solution that I'm
                // unable to find.
                .foldl(|(f, m), args| {
                    let span = f.1.start..args.1.end;
                    (
                        (Expression::MethodCall(Box::new(f), m.clone(), args.0), span),
                        m,
                    )
                })
                .map(|(expr, _)| expr);

            let static_method_call = object
                .map_with_span(|obj, span| (obj, span))
                .then_ignore(just(Token::Punct('⊥')))
                .then(ident.map_with_span(|ident, span| (ident, span)))
                .then(
                    items
                        .clone()
                        .delimited_by(just(Token::Punct('(')), just(Token::Punct(')')))
                        .map_with_span(|args, span: Span| (args, span))
                        .repeated(),
                )
                .map(|((obj, method), args)| {
                    let args = &args[0];
                    let span = obj.1.start..args.1.end;
                    (
                        Expression::StaticMethodCall(obj, method, args.0.clone()),
                        span,
                    )
                });

            // Function calls have very high precedence so we prioritise them
            let call = atom
                .then(
                    items
                        .delimited_by(just(Token::Punct('(')), just(Token::Punct(')')))
                        .map_with_span(|args, span: Span| (args, span))
                        .repeated(),
                )
                .foldl(|f, args| {
                    let span = f.1.start..args.1.end;
                    (Expression::FunctionCall(Box::new(f), args.0), span)
                });

            static_method_call.or(method_call).or(field_access).or(call)
            // field_access.or(call)

            // // Function calls have very high precedence so we prioritise them
            // let call = atom
            //     .then(
            //         items
            //             .delimited_by(just(Token::Punct('(')), just(Token::Punct(')')))
            //             .map_with_span(|args, span: Span| (args, span))
            //             .repeated(),
            //     )
            //     .foldl(|f, args| {
            //         let span = f.1.start..args.1.end;
            //         (Expression::FunctionCall(Box::new(f), args.0), span)
            //     });

            // call

            // // Product ops (multiply and divide) have equal precedence
            // let op = just(Token::Op("*".to_string()))
            //     .to(BinaryOp::Mul)
            //     .or(just(Token::Op("/".to_string())).to(BinaryOp::Div));
            // let product = call
            //     .clone()
            //     .then(op.then(call).repeated())
            //     .foldl(|a, (op, b)| {
            //         let span = a.1.start..b.1.end;
            //         (Expr::Binary(Box::new(a), op, Box::new(b)), span)
            //     });

            // // Sum ops (add and subtract) have equal precedence
            // let op = just(Token::Op("+".to_string()))
            //     .to(BinaryOp::Add)
            //     .or(just(Token::Op("-".to_string())).to(BinaryOp::Sub));
            // let sum = product
            //     .clone()
            //     .then(op.then(product).repeated())
            //     .foldl(|a, (op, b)| {
            //         let span = a.1.start..b.1.end;
            //         (Expr::Binary(Box::new(a), op, Box::new(b)), span)
            //     });

            // // Comparison ops (equal, not-equal) have equal precedence
            // let op = just(Token::Op("==".to_string()))
            //     .to(BinaryOp::Eq)
            //     .or(just(Token::Op("!=".to_string())).to(BinaryOp::NotEq));
            // let compare = sum
            //     .clone()
            //     .then(op.then(sum).repeated())
            //     .foldl(|a, (op, b)| {
            //         let span = a.1.start..b.1.end;
            //         (Expr::Binary(Box::new(a), op, Box::new(b)), span)
            //     });

            // compare
        });

        raw_expr

        //     // Blocks are expressions but delimited with braces
        //     let block = expr
        //         .clone()
        //         .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}')))
        //         // Attempt to recover anything that looks like a block but contains errors
        //         .recover_with(nested_delimiters(
        //             Token::Ctrl('{'),
        //             Token::Ctrl('}'),
        //             [
        //                 (Token::Ctrl('('), Token::Ctrl(')')),
        //                 (Token::Ctrl('['), Token::Ctrl(']')),
        //             ],
        //             |span| (Expr::Error, span),
        //         ));

        //     let if_ = recursive(|if_| {
        //         just(Token::If)
        //             .ignore_then(expr.clone())
        //             .then(block.clone())
        //             .then(
        //                 just(Token::Else)
        //                     .ignore_then(block.clone().or(if_))
        //                     .or_not(),
        //             )
        //             .map_with_span(|((cond, a), b), span: Span| {
        //                 (
        //                     Expr::If(
        //                         Box::new(cond),
        //                         Box::new(a),
        //                         Box::new(match b {
        //                             Some(b) => b,
        //                             // If an `if` expression has no trailing `else` block, we magic up one that just produces null
        //                             None => (Expr::Value(Value::Null), span.clone()),
        //                         }),
        //                     ),
        //                     span,
        //                 )
        //             })
        //     });

        //     // Both blocks and `if` are 'block expressions' and can appear in the place of statements
        //     let block_expr = block.or(if_).labelled("block");

        //     let block_chain = block_expr
        //         .clone()
        //         .then(block_expr.clone().repeated())
        //         .foldl(|a, b| {
        //             let span = a.1.start..b.1.end;
        //             (Expr::Then(Box::new(a), Box::new(b)), span)
        //         });

        //     block_chain
        //         // Expressions, chained by semicolons, are statements
        //         .or(raw_expr.clone())
        //         .then(just(Token::Ctrl(';')).ignore_then(expr.or_not()).repeated())
        //         .foldl(|a, b| {
        //             // This allows creating a span that covers the entire Then expression.
        //             // b_end is the end of b if it exists, otherwise it is the end of a.
        //             let a_start = a.1.start;
        //             let b_end = b.as_ref().map(|b| b.1.end).unwrap_or(a.1.end);
        //             (
        //                 Expr::Then(
        //                     Box::new(a),
        //                     Box::new(match b {
        //                         Some(b) => b,
        //                         // Since there is no b expression then its span is empty.
        //                         None => (Expr::Value(Value::Null), b_end..b_end),
        //                     }),
        //                 ),
        //                 a_start..b_end,
        //             )
        //         })
    })
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
                uber: Uuid
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
                    true
                }

                fn baz() -> Bar {
                    let id = id_func(a, foo(), 42, 3.14, "Hello, World!", true);
                    let a = id.name;
                    let bar = id.func();
                    let id = Uuid::new();
                    let hello = Bar::new(42, 3.14, "Hello, World!", true, bar, id, func());
                    // Bar {}
                }
            }
        "#;

        let ast = parse(src);

        dbg!(&ast);

        assert!(ast.is_some());
    }
}
