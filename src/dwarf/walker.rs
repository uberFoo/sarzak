use ansi_term::{Colour, Style};
use fnv::FnvHashMap as HashMap;
use heck::ToUpperCamelCase;
use log;
use uuid::Uuid;

use crate::{
    dwarf::{
        Expression as ParserExpression, Item, ItemKind, Result, Spanned,
        Statement as ParserStatement, Token, Type,
    },
    lu_dog::{
        store::ObjectStore as LuDogStore,
        types::{
            Block, Call, Error, ErrorExpression, Expression, ExpressionStatement, Field, Function,
            FunctionCall, Implementation, Import, IntegerLiteral, LetStatement, Literal,
            LocalVariable, Parameter, Print, Statement, StringLiteral, Value, ValueEnum, ValueType,
            Variable, VariableExpression, WoogOption, WoogStruct,
        },
    },
    sarzak::{
        store::ObjectStore as SarzakStore,
        types::{Object, Ty},
    },
};

macro_rules! link_parameter {
    ($last:expr, $next:expr, $store:expr) => {{
        if let Some(last) = $last {
            let mut last = $store.exhume_parameter(&last).unwrap().clone();
            last.next = Some($next.id);
            $store.inter_parameter(last);
        }

        Some($next.id)
    }};
}

macro_rules! link_statement {
    ($last:expr, $next:expr, $store:expr) => {{
        if let Some(last) = $last {
            let mut last = $store.exhume_statement(&last).unwrap().clone();
            last.next = Some($next.id);
            $store.inter_statement(last);
        }

        Some($next.id)
    }};
}

macro_rules! debug {
    ($arg:expr) => {
        log::debug!("{:?}\n  --> {}:{}:{}", $arg, file!(), line!(), column!());
    };
    ($msg:expr, $arg:expr) => {
        log::debug!(
            "{} --> {:?}\n  --> {}:{}:{}",
            Colour::Yellow.paint($msg),
            $arg,
            file!(),
            line!(),
            column!()
        );
    };
}
// These below are just to avoid cloning things.
struct ConveyFunc<'a> {
    name: &'a str,
    params: &'a [(Spanned<String>, Spanned<Type>)],
    return_type: &'a Spanned<Type>,
    statements: &'a [Spanned<ParserStatement>],
}

impl<'a> ConveyFunc<'a> {
    fn new(
        name: &'a str,
        params: &'a [(Spanned<String>, Spanned<Type>)],
        return_type: &'a Spanned<Type>,
        statements: &'a [Spanned<ParserStatement>],
    ) -> Self {
        Self {
            name,
            params,
            return_type,
            statements,
        }
    }
}

struct ConveyStruct<'a> {
    name: &'a str,
    fields: &'a [(Spanned<String>, Spanned<Type>)],
}

impl<'a> ConveyStruct<'a> {
    fn new(name: &'a str, fields: &'a [(Spanned<String>, Spanned<Type>)]) -> Self {
        Self { name, fields }
    }
}

struct ConveyImpl<'a> {
    name: &'a str,
    funcs: &'a [(Spanned<String>, Box<Item>)],
}

impl<'a> ConveyImpl<'a> {
    fn new(name: &'a str, funcs: &'a [(Spanned<String>, Box<Item>)]) -> Self {
        Self { name, funcs }
    }
}

pub fn populate_lu_dog(
    ast: &HashMap<(String, ItemKind), Item>,
    model: &SarzakStore,
    sarzak: &SarzakStore,
) -> Result<LuDogStore> {
    let mut lu_dog = LuDogStore::new();

    walk_tree(ast, &mut lu_dog, model, sarzak)?;

    Ok(lu_dog)
}

fn walk_tree(
    ast: &HashMap<(String, ItemKind), Item>,
    lu_dog: &mut LuDogStore,
    model: &SarzakStore,
    sarzak: &SarzakStore,
) -> Result<()> {
    let mut funcs = Vec::new();
    let mut implementations = Vec::new();
    let mut structs = Vec::new();

    // We need the structs before the impls, so we do this.
    for ((ref name, _), item) in ast {
        match item {
            Item::Function(params, return_type, stmts) => {
                funcs.push(ConveyFunc::new(name, params, return_type, stmts))
            }
            Item::Implementation(funcs) => implementations.push(ConveyImpl::new(name, funcs)),
            // Imports can happen any time, I think.
            Item::Import(path) => inter_import(&path.0, lu_dog),
            Item::Struct(fields) => structs.push(ConveyStruct::new(name, fields)),
        }
    }

    // Put the type information in first.
    for ConveyStruct { name, fields } in structs {
        inter_struct(&name, &fields, lu_dog, model, sarzak);
    }

    // Using the type information, and the input, inter the implementation blocks.
    for ConveyImpl { name, funcs } in implementations {
        inter_implementation(&name, &funcs, lu_dog, model, sarzak);
    }

    // Finally, inter the loose functions.
    for ConveyFunc {
        name,
        params,
        return_type,
        statements,
    } in funcs
    {
        inter_func(
            &name,
            &params,
            &return_type,
            &statements,
            None,
            None,
            lu_dog,
            model,
            sarzak,
        );
    }

    Ok(())
}

fn inter_func(
    name: &str,
    params: &[(Spanned<String>, Spanned<Type>)],
    return_type: &Spanned<Type>,
    stmts: &[Spanned<ParserStatement>],
    impl_block: Option<&Implementation>,
    impl_ty: Option<&ValueType>,
    lu_dog: &mut LuDogStore,
    model: &SarzakStore,
    sarzak: &SarzakStore,
) {
    debug!("inter_func", name);
    let block = Block::new(Uuid::new_v4(), lu_dog);

    let name = if let Some(name) = de_sanitize(&name) {
        name
    } else {
        name
    };

    let ret_ty = get_value_type(&return_type.0, impl_ty, lu_dog, model, sarzak);
    let func = Function::new(name.to_owned(), &block, impl_block, &ret_ty, lu_dog);
    // Create a type for our function
    ValueType::new_function(&func, lu_dog);

    let mut last_param_uuid: Option<Uuid> = None;
    for ((param_name, _), (param_ty, _)) in params {
        let param = Parameter::new(&func, None, lu_dog);
        let var = Variable::new_parameter(name.to_owned(), &param, lu_dog);
        // 🚧 We'll need to do something about this soon. Actually, it never belonged
        // here. It only makes sense that you can only have values is block. Now the
        // model enforces that.
        // let param_ty = get_value_type(&param_ty, impl_ty, lu_dog, model, sarzak);
        // let _value = Value::new_variable(&block, &param_ty, &var, lu_dog);
        last_param_uuid = link_parameter!(last_param_uuid, param, lu_dog);
    }

    let stmts: Vec<&ParserStatement> = stmts.iter().map(|stmt| &stmt.0).collect();
    inter_statements(&stmts, &block, lu_dog);
}

pub fn inter_statement(
    stmt: &ParserStatement,
    block: &Block,
    lu_dog: &mut LuDogStore,
) -> (Statement, ValueType) {
    debug!("inter_statement", stmt);

    match stmt {
        ParserStatement::Let((var, _), (expr, _)) => {
            // Shit. Type inference happens here. Never done this before...
            let local = LocalVariable::new(Uuid::new_v4(), lu_dog);
            let var = Variable::new_local_variable(var.to_owned(), &local, lu_dog);
            let (expr, ty) = inter_expression(expr, &block, lu_dog);
            let _value = Value::new_variable(&block, &ty, &var, lu_dog);
            // let _value = Value::new_expression(&ty, &expr, lu_dog);
            let stmt = LetStatement::new(&expr, &local, lu_dog);
            let stmt = Statement::new_let_statement(&block, None, &stmt, lu_dog);

            (stmt, ValueType::new_empty())
        }
        ParserStatement::Result((ref expr, _)) => {
            let (expr, ty) = inter_expression(expr, &block, lu_dog);
            let stmt = ExpressionStatement::new(&expr, lu_dog);
            let stmt = Statement::new_expression_statement(&block, None, &stmt, lu_dog);

            (stmt, ty)
        }
        ParserStatement::Expression((expr, _)) => {
            let (expr, _) = inter_expression(expr, &block, lu_dog);
            let stmt = ExpressionStatement::new(&expr, lu_dog);
            let stmt = Statement::new_expression_statement(&block, None, &stmt, lu_dog);

            (stmt, ValueType::new_empty())
        }
        道 => todo!("{:?}", 道),
    }
}

fn inter_statements(
    statements: &[&ParserStatement],
    block: &Block,
    lu_dog: &mut LuDogStore,
) -> ValueType {
    let mut value_type = ValueType::new_empty();

    let mut last_stmt_uuid: Option<Uuid> = None;
    for stmt in statements {
        let (stmt, ty) = inter_statement(stmt, block, lu_dog);
        last_stmt_uuid = link_statement!(last_stmt_uuid, stmt, lu_dog);
        value_type = ty;
    }

    value_type
}

/// I have a feeling that this one is going to be intense...
/// Actually, maybe not. There's not much happening just when we are populating.
/// It should get intense when we are evaluating for the SVM.
/// I may have spoken too soon. We need to be populating the store, in addition
/// to returning the type. Duh. And we should return the expression so that we
/// can create a value from it.
fn inter_expression(
    expr: &ParserExpression,
    block: &Block,
    lu_dog: &mut LuDogStore,
) -> (Expression, ValueType) {
    debug!("inter_expression", expr);

    match expr {
        ParserExpression::Block(ref stmts) => {
            let block = Block::new(Uuid::new_v4(), lu_dog);
            debug!("ParserExpression::Block", block);
            let stmts: Vec<&ParserStatement> = stmts.iter().map(|stmt| &stmt.0).collect();
            (
                Expression::new_block(&block, lu_dog),
                inter_statements(&stmts, &block, lu_dog),
            )
        }
        ParserExpression::FunctionCall(func, params) => {
            debug!("FunctionCall", func);
            let func = &func.0;
            let params: Vec<&ParserExpression> = params.iter().map(|param| &param.0).collect();
            let (func_expr, ret_ty) = inter_expression(&func, &block, lu_dog);
            let func = FunctionCall::new();
            let func = Call::new_function_call(Some(&func_expr), lu_dog);
            let func = Expression::new_call(&func, lu_dog);

            (func, ret_ty)
        }
        ParserExpression::IntegerLiteral(literal) => (
            Expression::new_literal(
                &Literal::new_integer_literal(&IntegerLiteral::new(*literal, lu_dog), lu_dog),
                lu_dog,
            ),
            ValueType::new_ty(&Ty::new_integer(), lu_dog),
        ),
        // There is nothing to inter here. The literals are consts.
        //  ParserExpression::BooleanLiteral(literal) => ValueType::new_empty(),
        // I literally have no idea what to do with this. I just haven't given
        // it any thought. IT came from the parser I looked at when building
        // this one. It (could) contain(s) the span of whatever didn't parse, whilst
        // parsing an expression. I did wonder about adding an Error object
        // to the domain. So, maybe this is where that fits in? I just don't
        // recall what prompted me to consider an Error Object, and just as
        // important, why I dismissed the notion.
        //  ParserExpression::Error => ValueType::new_empty(),
        ParserExpression::LocalVariable(name) => {
            debug!("LocalVariable", name);
            // We need to return an expression and a type.
            // We look for a values matching the in the current block.
            let expr_ty: Vec<Value> = lu_dog
                .iter_value()
                .filter(|value| value.block == block.id)
                .cloned()
                .collect();

            let expr_ty: Vec<Option<(Expression, ValueType)>> = expr_ty
                .iter()
                .map(|value| {
                    debug!("ParserExpression::LocalVariable: value", value);

                    match value.subtype {
                        ValueEnum::Expression(ref expr) => {
                            let expr = lu_dog.exhume_expression(expr).unwrap();
                            panic!(
                                "I don't think that we should ever see an expression here: {:?}",
                                expr
                            );
                        }
                        ValueEnum::Variable(ref var) => {
                            let var = lu_dog.exhume_variable(var).unwrap();
                            // Check the name
                            if var.name == *name {
                                let value = var.r11_value(lu_dog)[0];
                                let ty = value.r24_value_type(lu_dog)[0].clone();

                                // Ok, so I parsed a local variable expression. We need to create
                                // a VariableExpression, and it in turn needs an Expression, which
                                // needs a Value, and finally a ValueType.
                                // Except that I don't think we want to create values in the walker.
                                // Doing so wreaks havoc downstream in the interpreter, because
                                // It sees that value and expects that it's been evaluated.
                                let expr = VariableExpression::new(name.to_owned(), lu_dog);
                                debug!("ParserExpression::LocalVariable: created expr", expr);
                                let expr = Expression::new_variable_expression(&expr, lu_dog);
                                debug!("ParserExpression::LocalVariable: created expr", expr);

                                // This is the value that we need to give the expression a type when
                                // we later parse a local variable access in the interpreter.
                                // let _value = Value::new_expression(&block, &ty, &expr, lu_dog);

                                Some((expr, ty))
                            } else {
                                None
                            }
                        }
                    }
                })
                .collect();

            if expr_ty.len() > 0 {
                if expr_ty.len() > 1 {
                    panic!(
                        "I don't think that we should ever see more than one value here: {:?}",
                        &expr_ty
                    );
                }
                expr_ty[0].clone().unwrap()
            } else {
                let expr = ErrorExpression::new("Variable not found".to_owned(), lu_dog);
                let expr = Expression::new_error_expression(&expr, lu_dog);
                (
                    expr,
                    ValueType::new_error(&Error::new_unknown_variable(), lu_dog),
                )
            }
        }
        //  ParserExpression::StaticMethodCall(_, _, _) => ValueType::new_empty(),
        ParserExpression::StringLiteral(literal) => (
            Expression::new_literal(
                &Literal::new_string_literal(
                    &StringLiteral::new(literal.to_owned(), lu_dog),
                    lu_dog,
                ),
                lu_dog,
            ),
            ValueType::new_ty(&Ty::new_s_string(), lu_dog),
        ),
        //  ParserExpression::Struct((name, _), fields) => ValueType::new_empty(),
        ParserExpression::Print(expr) => {
            let (expr, ty) = inter_expression(&(**expr).0, &block, lu_dog);
            let print = Print::new(&expr, lu_dog);

            (Expression::new_print(&print, lu_dog), ty)
        }
        道 => todo!("{:?}", 道),
    }
}

fn inter_import(path: &String, lu_dog: &mut LuDogStore) {
    Import::new(path.clone(), lu_dog);
}

fn inter_implementation(
    name: &str,
    funcs: &[(Spanned<String>, Box<Item>)],
    lu_dog: &mut LuDogStore,
    model: &SarzakStore,
    sarzak: &SarzakStore,
) {
    let name = if let Some(name) = de_sanitize(&name) {
        name
    } else {
        name
    };

    let impl_ty = get_value_type(
        &Type::UserType(Box::new(Token::Object(name.to_owned()))),
        None,
        lu_dog,
        model,
        sarzak,
    );

    let obj_id = model
        .exhume_object_id_by_name(name)
        .expect(&format!("Object {} not found", name));

    let obj = model
        .exhume_object(obj_id)
        .expect(&format!("Object {} not found", name));

    let mt = lu_dog
        .iter_woog_struct()
        .find(|mt| mt.object == obj.id)
        .expect(&format!("Struct for {} not found", name))
        .clone();
    let implementation = Implementation::new(&mt, lu_dog);

    for ((name, _), func) in funcs {
        match **func {
            Item::Function(ref params, ref return_type, ref stmts) => {
                inter_func(&name, &params, &return_type, &stmts, Some(&implementation), Some(&impl_ty), lu_dog, model, sarzak)
            }
            _ => panic!("Implementation can only contain functions -- this is actually wrong, but it's good enough for a temporary failure message"),
        }
    }
}

fn inter_struct(
    name: &str,
    fields: &[(Spanned<String>, Spanned<Type>)],
    lu_dog: &mut LuDogStore,
    model: &SarzakStore,
    sarzak: &SarzakStore,
) {
    let name = if let Some(name) = de_sanitize(&name) {
        name
    } else {
        name
    };

    let obj_id = model
        .exhume_object_id_by_name(name)
        .expect(&format!("Object {} not found", name));

    let obj = model
        .exhume_object(obj_id)
        .expect(&format!("Object {} not found", name));

    let mt = WoogStruct::new(&obj, lu_dog);
    for ((name, _), (type_, _)) in fields {
        let name = if let Some(name) = de_sanitize(&name) {
            name
        } else {
            name
        };

        let ty = get_value_type(type_, None, lu_dog, model, sarzak);
        let field = Field::new(name.to_owned(), &mt, &ty, lu_dog);
    }
}

/// Get a Lu-Dog ValueType from a Dwarf Type
///
/// Note that the `new_*` methods on `Ty` just return `const`s. Also, the
/// `ValueType::new_ty` method takes on the id of it's subtype, so neither do
/// those take much space.
fn get_value_type(
    type_: &Type,
    enclosing_type: Option<&ValueType>,
    lu_dog: &mut LuDogStore,
    model: &SarzakStore,
    sarzak: &SarzakStore,
) -> ValueType {
    match type_ {
        Type::Boolean => {
            let ty = Ty::new_boolean();
            ValueType::new_ty(&ty, lu_dog)
        }
        Type::Integer => {
            let ty = Ty::new_integer();
            ValueType::new_ty(&ty, lu_dog)
        }
        Type::String => {
            let ty = Ty::new_s_string();
            ValueType::new_ty(&ty, lu_dog)
        }
        Type::UserType(tok) => {
            let name = match **tok {
                Token::Object(ref name) => name,
                _ => panic!("Expected object, found {:?}", tok),
            };

            let name = if let Some(name) = de_sanitize(&name) {
                name
            } else {
                name
            };

            // Need to deal with Self.
            if name == "Self" {
                match enclosing_type {
                    Some(ty) => ty.clone(),
                    None => panic!("Self must be used inside an impl block."),
                }
            } else
            // Look for the Object in the model domain first.
            if let Some(ty) = model.iter_ty().find(|ty| match ty {
                Ty::Object(ref obj) => {
                    let obj = model.exhume_object(obj).unwrap();
                    obj.name.to_upper_camel_case() == *name
                }
                _ => false,
            }) {
                ValueType::new_ty(ty, lu_dog)
            } else {
                // Unlikely to have to reach back this far, except of course for
                // the Uuid. So, it's not unlikely, it's the least likely.
                if let Some(ty) = sarzak.iter_ty().find(|ty| match ty {
                    Ty::Object(ref obj) => {
                        let obj = sarzak.exhume_object(obj).unwrap();
                        obj.name.to_upper_camel_case() == *name
                    }
                    _ => false,
                }) {
                    ValueType::new_ty(ty, lu_dog)
                } else {
                    panic!("Type not found for object {}.", name)
                }
            }
        }
        Type::Option(ref type_) => {
            let inner_type = get_value_type(type_, enclosing_type, lu_dog, model, sarzak);
            let option = WoogOption::new_none(&inner_type, lu_dog);
            ValueType::new_woog_option(&option, lu_dog)
        }
        Type::Empty => ValueType::new_empty(),
        道 => todo!("{:?}", 道),
    }
}

fn de_sanitize(string: &str) -> Option<&str> {
    match string {
        "Ty" => Some("Type"),
        "WoogOption" => Some("Option"),
        "WoogStruct" => Some("Struct"),
        "False Literal" => Some("False"),
        "FalseLiteral" => Some("False"),
        "True Literal" => Some("True"),
        "TrueLiteral" => Some("True"),
        _ => None,
    }
}
