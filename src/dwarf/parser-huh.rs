use std::fmt;

use chumsky::prelude::*;
use fnv::FnvHashMap as HashMap;

pub type Span = std::ops::Range<usize>;

#[derive(Debug)]
enum Token {
    Num(String),
    Str(String),
    Ident(String),
    Punct(char),
    Struct,
    Enum,
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
            Token::Struct => write!(f, "struct"),
            Token::Enum => write!(f, "enum"),
            Token::Fn => write!(f, "fn"),
            Token::Impl => write!(f, "impl"),
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
    let punct = one_of("()[]{};,").map(|c| Token::Punct(c));

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
        "struct" => Token::Struct,
        "enum" => Token::Enum,
        "impl" => Token::Impl,
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
enum Item {
    Struct,
    Enumeration,
    Function,
    Implementation,
}

fn item_parser() -> impl Parser<Token, HashMap<String, Item>, Error = Simple<Token>> + Clone {
    let ident 
}

// pub fn parse(src: &str) -> Option<HashMap<String, Item>> {
//     let (tokens, mut errs) = lexer().parse_recovery(src);
//     let (ast, parse_errs) = if let Some(tokens) = tokens {
//         let len = src.chars().count();
//         let (ast, parse_errs) =
//             funcs_parser().parse_recovery(Stream::from_iter(len..len + 1, tokens.into_iter()));

//         (ast, parse_errs)
//     } else {
//         (None, Vec::new())
//     };

//     ast
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let src = r#"
            // This is a comment
            struct Foo {}
            enum Bar {}
            enum Baz
        "#;

        let (tokens, errs) = lexer().parse_recovery(src);

        dbg!(&tokens, &errs);

        assert!(tokens.is_some());
        assert!(errs.is_empty());
    }
}
