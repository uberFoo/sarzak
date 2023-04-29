use std::{ops::Range, sync::Arc, sync::RwLock};

use ansi_term::Colour;
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
            Block, Call, Error, ErrorExpression, Expression, ExpressionStatement, Field,
            FieldExpression, Function, Implementation, Import, IntegerLiteral, LetStatement,
            Literal, LocalVariable, Parameter, Print, Statement, StaticMethodCall, StringLiteral,
            StructExpression, Value, ValueEnum, ValueType, Variable, VariableExpression,
            WoogOption, WoogStruct,
        },
        Argument, FloatLiteral, List, MethodCall, Reference, ResultStatement,
    },
    sarzak::{store::ObjectStore as SarzakStore, types::Ty},
};

macro_rules! link_parameter {
    ($last:expr, $next:expr, $store:expr) => {{
        let next = $next.read().unwrap();
        if let Some(last) = $last {
            let last = $store.exhume_parameter(&last).unwrap().clone();
            let mut last = last.write().unwrap();
            last.next = Some(next.id);
        }

        Some(next.id)
    }};
}

macro_rules! link_argument {
    ($last:expr, $next:expr, $store:expr) => {{
        let next = $next.read().unwrap();
        if let Some(last) = $last {
            let last = $store.exhume_argument(&last).unwrap().clone();
            let mut last = last.write().unwrap();
            last.next = Some(next.id);
        }

        Some(next.id)
    }};
}

macro_rules! link_statement {
    ($last:expr, $next:expr, $store:expr) => {{
        let next = $next.read().unwrap();
        if let Some(last) = $last {
            let last = $store.exhume_statement(&last).unwrap().clone();
            let mut last = last.write().unwrap();
            last.next = Some(next.id);
        }

        Some(next.id)
    }};
}

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
        log::debug!("{:?}\n  --> {}:{}:{}", $arg, file!(), line!(), column!());
    };
}

macro_rules! error {
    ($msg:literal, $($arg:expr),*) => {
        $(
            log::error!(
                "{} --> {:?}\n  --> {}:{}:{}",
                Colour::Red.paint($msg),
                $arg,
                file!(),
                line!(),
                column!()
            );
        )*
    };
    ($arg:expr) => {
        log::error!("{:?}\n  --> {}:{}:{}", $arg, file!(), line!(), column!());
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

/// The main entry point
///
/// This is where we go to populate the model from the parsed AST.
///
/// 🚧 Return a result!
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
            Item::Import(path, alias) => inter_import(&path.0, alias, lu_dog),
            Item::Struct(fields) => structs.push(ConveyStruct::new(name, fields)),
        }
    }

    // Put the type information in first.
    for ConveyStruct { name, fields } in structs {
        debug!("Intering struct {}", name);
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
    impl_block: Option<Arc<RwLock<Implementation>>>,
    impl_ty: Option<Arc<RwLock<ValueType>>>,
    lu_dog: &mut LuDogStore,
    model: &SarzakStore,
    sarzak: &SarzakStore,
) {
    debug!("inter_func", name);
    let block = Block::new(Uuid::new_v4(), lu_dog);

    let name = name.de_sanitize();

    let ret_ty = get_value_type(&return_type.0, impl_ty.clone(), lu_dog, model, sarzak);
    let func = Function::new(name.to_owned(), block.clone(), impl_block, ret_ty, lu_dog);
    // Create a type for our function
    ValueType::new_function(func.clone(), lu_dog);

    let mut last_param_uuid: Option<Uuid> = None;
    for ((param_name, _), (param_ty, _)) in params {
        debug!("inter_func param name", param_name);
        debug!("inter_func param ty", param_ty);
        let param = Parameter::new(func.clone(), None, lu_dog);
        debug!("inter_func param param", param);
        let var = Variable::new_parameter(param_name.to_owned(), param.clone(), lu_dog);
        debug!("inter_func param var", var);
        // 🚧 We'll need to do something about this soon. Actually, it never belonged
        // here. It only makes sense that you can only have values in a block. Now the
        // model enforces that.
        //
        // That said, we need to introduce the values into the block, so that we don't
        // error out when parsing the statements.
        //
        let param_ty = get_value_type(&param_ty, impl_ty.clone(), lu_dog, model, sarzak);
        debug!("inter_func param param_ty", param_ty);
        let _value = Value::new_variable(block.clone(), param_ty, var, lu_dog);
        last_param_uuid = link_parameter!(last_param_uuid, param, lu_dog);
        debug!("inter_func param last_param_uuid", last_param_uuid);
    }

    let stmts: Vec<Arc<RwLock<ParserStatement>>> = stmts
        .iter()
        .map(|stmt| Arc::new(RwLock::new(stmt.0.clone())))
        .collect();
    inter_statements(&stmts, &block, lu_dog, model, sarzak);
}

pub fn inter_statement(
    stmt: &Arc<RwLock<ParserStatement>>,
    block: &Arc<RwLock<Block>>,
    lu_dog: &mut LuDogStore,
    model: &SarzakStore,
    sarzak: &SarzakStore,
) -> (Arc<RwLock<Statement>>, Arc<RwLock<ValueType>>) {
    debug!("inter_statement", stmt);

    match &*stmt.read().unwrap() {
        //
        // Expression
        //
        ParserStatement::Expression((expr, _)) => {
            let (expr, _) = inter_expression(
                &Arc::new(RwLock::new(expr.to_owned())),
                block,
                lu_dog,
                model,
                sarzak,
            );
            let stmt = ExpressionStatement::new(expr, lu_dog);
            let stmt = Statement::new_expression_statement(block.clone(), None, stmt, lu_dog);

            (stmt, ValueType::new_empty())
        }
        //
        // Let
        //
        ParserStatement::Let((var_name, _), type_, (expr, _)) => {
            // Setup the local variable that is the LHS of the statement.
            let local = LocalVariable::new(Uuid::new_v4(), lu_dog);
            let var = Variable::new_local_variable(var_name.to_owned(), local.clone(), lu_dog);

            // Now parse the RHS, which is an expression.
            let (expr, ty) = inter_expression(
                &Arc::new(RwLock::new(expr.to_owned())),
                block,
                lu_dog,
                model,
                sarzak,
            );

            let ty = if let Some((type_, _)) = type_ {
                type_.into_value_type(lu_dog, model, sarzak)
            } else {
                ty
            };

            // let ty = ty.read().unwrap().to_owned();
            if let ValueType::Unknown(_) = &*ty.read().unwrap() {
                error!("Unknown type for variable", var_name);
            }

            // Create a variable, now that we (hopefully) have a type from the expression.
            let _value = Value::new_variable(block.clone(), ty, var, lu_dog);

            // Setup the let statement itself.
            let stmt = LetStatement::new(expr, local, lu_dog);
            let stmt = Statement::new_let_statement(block.clone(), None, stmt, lu_dog);

            (stmt, ValueType::new_empty())
        }
        //
        // Result
        //
        ParserStatement::Result((ref expr, _)) => {
            let (expr, ty) = inter_expression(
                &Arc::new(RwLock::new(expr.to_owned())),
                block,
                lu_dog,
                model,
                sarzak,
            );
            let stmt = ResultStatement::new(expr, lu_dog);
            let stmt = Statement::new_result_statement(block.clone(), None, stmt, lu_dog);

            (stmt, ty)
        }
        道 => todo!("{:?}", 道),
    }
}

fn inter_statements(
    statements: &[Arc<RwLock<ParserStatement>>],
    block: &Arc<RwLock<Block>>,
    lu_dog: &mut LuDogStore,
    model: &SarzakStore,
    sarzak: &SarzakStore,
) -> Arc<RwLock<ValueType>> {
    let mut value_type = ValueType::new_empty();

    let mut last_stmt_uuid: Option<Uuid> = None;
    for stmt in statements {
        let (stmt, ty) = inter_statement(stmt, block, lu_dog, model, sarzak);
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
    expr: &Arc<RwLock<ParserExpression>>,
    block: &Arc<RwLock<Block>>,
    lu_dog: &mut LuDogStore,
    model: &SarzakStore,
    sarzak: &SarzakStore,
) -> (Arc<RwLock<Expression>>, Arc<RwLock<ValueType>>) {
    debug!("inter_expression", expr);

    match &*expr.read().unwrap() {
        //
        // Block
        //
        ParserExpression::Block(ref stmts) => {
            let block = Block::new(Uuid::new_v4(), lu_dog);
            debug!("ParserExpression::Block", block);
            let stmts: Vec<Arc<RwLock<ParserStatement>>> = stmts
                .iter()
                .map(|stmt| Arc::new(RwLock::new(stmt.0.to_owned())))
                .collect();
            (
                Expression::new_block(block.clone(), lu_dog),
                inter_statements(&stmts, &block, lu_dog, model, sarzak),
            )
        }
        //
        // BooleanLiteral
        //
        // There is nothing to inter here. The literals are consts.
        //  ParserExpression::BooleanLiteral(literal) => ValueType::new_empty(),
        //
        // Error
        //
        ParserExpression::Error => {
            // 🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧
            //
            // This error stuff really needs fleshing out:
            //
            //  💥 The parser needs to capture the span of the error, and we need to use it
            //    to look up the text in the source code. That, or we need to capture the
            //    source in the error.
            //  💥 If we look it up in the source, then we need to capture that. I'm part-
            //    way to having that done, as there is a place to do that in the model now.
            //  💥 We need to return an `ErrorExpression`. Right now it's got an attribute
            //     called `span` that's a String. Probably rename that, but leave it otherwise.
            //  💥 We need to figure out what's up with `ValueType::Error`. I plugged some
            //     shit in a while back when stubbing something out, but I didn't put much
            //     thought into it.
            //
            // 🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧
            let error = ErrorExpression::new(
                "🚧 Under Construction 🚧\n💥 Input is fucked up for some reason 💥\n".to_owned(),
                lu_dog,
            );
            let expr = Expression::new_error_expression(error, lu_dog);
            // Returning an empty, because the error stuff in ValueType is fucked.
            (expr, ValueType::new_empty())
        }
        //
        // FloatLiteral
        //
        ParserExpression::FloatLiteral(literal) => (
            Expression::new_literal(
                Literal::new_float_literal(FloatLiteral::new(*literal, lu_dog), lu_dog),
                lu_dog,
            ),
            ValueType::new_ty(Arc::new(RwLock::new(Ty::new_integer())), lu_dog),
        ),
        //
        // FunctionCall
        //
        ParserExpression::FunctionCall(func, params) => {
            debug!("ParserExpression::FunctionCall", func);
            let func = &func.0;
            let params: Vec<&ParserExpression> = params.iter().map(|param| &param.0).collect();
            debug!("ParserExpression::FunctionCall", params);
            let (func_expr, ret_ty) = inter_expression(
                &Arc::new(RwLock::new(func.to_owned())),
                block,
                lu_dog,
                model,
                sarzak,
            );
            let func_call = Call::new_function_call(Some(func_expr), lu_dog);
            let func = Expression::new_call(func_call.clone(), lu_dog);

            let mut last_arg_uuid: Option<Uuid> = None;
            for param in params {
                let (arg_expr, ty) = inter_expression(
                    &Arc::new(RwLock::new(param.to_owned())),
                    block,
                    lu_dog,
                    model,
                    sarzak,
                );
                let _value = Value::new_expression(block.clone(), ty, arg_expr.clone(), lu_dog);
                let arg = Argument::new(None, func_call.clone(), arg_expr, lu_dog);
                last_arg_uuid = link_argument!(last_arg_uuid, arg, lu_dog);
            }

            (func, ret_ty)
        }
        //
        // IntegerLiteral
        //
        ParserExpression::IntegerLiteral(literal) => (
            Expression::new_literal(
                Literal::new_integer_literal(IntegerLiteral::new(*literal, lu_dog), lu_dog),
                lu_dog,
            ),
            ValueType::new_ty(Arc::new(RwLock::new(Ty::new_integer())), lu_dog),
        ),
        //
        // LocalVarialbe
        //
        ParserExpression::LocalVariable(name) => {
            debug!("ParserExpression::LocalVariable", name);
            // We need to return an expression and a type.
            // We look for a value in the current block. We need to clone them
            // to be able to modify lu_dog below.
            //
            // So, multiple let statements will result in multiple values. We only
            // need one -- and it needs to be the right one...
            // To expound, there are likely to be multiple values in this block,
            // and we need to find the one that matches the variable name.
            //
            // ⚡️ Oh shit -- I'm in the compiler!!!
            //
            // So what's happening? We hit a local variable node in the ast. We need
            // to create
            //
            let values = lu_dog
                .iter_value()
                // This feels like deadlock...
                .filter(|value| value.read().unwrap().block == block.read().unwrap().id)
                .collect::<Vec<Arc<RwLock<Value>>>>();

            debug!("ParserExpression::LocalVariable values", values);

            // Now search for a value that's a Variable, and see if the access matches
            // the variable.
            let mut expr_type_tuples = values
                .iter()
                .filter_map(|value| {
                    debug!("ParserExpression::LocalVariable: value", value);

                    match value.read().unwrap().subtype {
                        ValueEnum::Expression(ref expr) => {
                            let expr = lu_dog.exhume_expression(expr).unwrap();
                            // error!("we don't expect to be here", expr);
                            // So we get here after all.
                            // Must. Remember. In. Compiler.
                            // So we need to create some nodes here. And return an expression
                            // and a type.
                            //
                            // Still wondering how we get here. Debugging is showing that we've
                            // got a Literal expression. But why's it showing up as a LocalVariable?
                            // I got here by entering `Point::new(5, a)` in the interpreter. `a` is
                            // an Inflection instance. I need to turn on logging and sort this out.
                            //
                            // Fuck me. I've been debugging something that's completely normal. I'm
                            // stoned now, so don't blame yourself later for it being that. What's
                            // going on is that there are a bunch of values in the block --
                            // especially when running the interpreter. So we are iterating over
                            // them all, and we are bound to find some that aren't, variable expressions
                            // even though we are parsing a LocalVariable. Remember these are all of
                            // the values -- not just the ones that have something to do with finding
                            // ourselves here.
                            //
                            // Hopefully this is concluded.
                            //

                            None
                        }
                        ValueEnum::Variable(ref var) => {
                            let var = lu_dog.exhume_variable(var).unwrap().read().unwrap().clone();
                            debug!("ParserExpression::LocalVariable: var", var);
                            // Check the name
                            if var.name == *name {
                                let value = var.r11_value(lu_dog)[0].read().unwrap().clone();
                                let ty = value.r24_value_type(lu_dog)[0].clone();

                                // Ok, so I parsed a local variable expression. We need to create
                                // a VariableExpression, and it in turn needs an Expression, which
                                // needs a Value, and finally a ValueType.
                                // Except that I don't think we want to create values in the walker.
                                // Doing so wreaks havoc downstream in the interpreter, because
                                // It sees that value and expects that it's been evaluated.
                                // And we got here by searching for a value anyway.
                                //
                                // We don't want to create more than one of these.
                                let expr = lu_dog
                                    .iter_variable_expression()
                                    .find(|expr| expr.read().unwrap().name == *name);

                                let expr = if let Some(expr) = expr {
                                    expr.read().unwrap().r15_expression(lu_dog)[0].clone()
                                } else {
                                    let expr = VariableExpression::new(name.to_owned(), lu_dog);
                                    debug!("created a new variable expression", expr);
                                    Expression::new_variable_expression(expr, lu_dog)
                                };

                                Some((expr, ty))
                            } else {
                                None
                            }
                        }
                    }
                })
                .collect::<Vec<(Arc<RwLock<Expression>>, Arc<RwLock<ValueType>>)>>();
            // There should be zero or 1 results.
            debug_assert!(expr_type_tuples.len() <= 1);

            debug!("ParserExpression::LocalVariable: expr_ty", expr_type_tuples);

            // Why are we taking the last one? -- Oh, read above.
            if let Some(expr_ty_tuple) = expr_type_tuples.pop() {
                debug!("ParserExpression::LocalVariable: returning", expr_ty_tuple);
                expr_ty_tuple
            } else {
                // 🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧
                // As neat as it is that I'm compiling this into the binary, we should actually
                // bail here, and make the user do something about it. The issue is that this
                // may get run from the repl, and in that case we want to return the expression.
                // So, we need a flag...
                let expr = ErrorExpression::new(
                    format!(
                        "\n  ──➤  variable: `{}` not found\n",
                        Colour::Red.paint(name.to_owned())
                    ),
                    lu_dog,
                );
                let expr = Expression::new_error_expression(expr, lu_dog);
                (
                    expr,
                    ValueType::new_error(Error::new_unknown_variable(), lu_dog),
                );
                let expr = VariableExpression::new(name.to_owned(), lu_dog);
                let expr = Expression::new_variable_expression(expr, lu_dog);
                (expr, ValueType::new_unknown())
            }
        }
        //
        // MethodCall
        //
        ParserExpression::MethodCall(instance, (method, _), params) => {
            debug!("ParserExpression::MethodCall", instance);

            let (instance, instance_ty) = inter_expression(
                &Arc::new(RwLock::new((*instance).0.clone())),
                block,
                lu_dog,
                model,
                sarzak,
            );
            let meth = MethodCall::new(method.to_owned(), lu_dog);
            let call = Call::new_method_call(Some(instance), meth, lu_dog);
            let expr = Expression::new_call(call.clone(), lu_dog);

            let mut last_arg_uuid: Option<Uuid> = None;
            let params: Vec<&ParserExpression> = params.iter().map(|param| &param.0).collect();
            for param in params {
                let (arg_expr, ty) = inter_expression(
                    &Arc::new(RwLock::new(param.to_owned())),
                    block,
                    lu_dog,
                    model,
                    sarzak,
                );
                let _value = Value::new_expression(block.clone(), ty, arg_expr.clone(), lu_dog);
                let arg = Argument::new(None, call.clone(), arg_expr.clone(), lu_dog);
                last_arg_uuid = link_argument!(last_arg_uuid, arg, lu_dog);
            }

            (expr, instance_ty)
        }
        ParserExpression::Print(expr) => {
            let (expr, ty) = inter_expression(
                &Arc::new(RwLock::new((*expr).0.clone())),
                block,
                lu_dog,
                model,
                sarzak,
            );
            let print = Print::new(expr, lu_dog);

            (Expression::new_print(print, lu_dog), ty)
        }
        //
        // StaticMethodCall
        //
        ParserExpression::StaticMethodCall((obj, _), (method, _), params) => {
            debug!("ParserExpression::StaticMethodCall", obj);
            let type_name = if let Token::Ident(obj) = obj {
                obj.de_sanitize().to_owned()
            } else if obj == &Token::Uuid {
                "Uuid".to_owned()
            } else {
                panic!("I don't think that we should ever see anything other than an object or Uuid here: {:?}", obj);
            };

            let meth = StaticMethodCall::new(method.to_owned(), type_name.to_owned(), lu_dog);
            let call = Call::new_static_method_call(None, meth, lu_dog);
            let expr = Expression::new_call(call.clone(), lu_dog);

            debug!("ParserExpression::StaticMethodCall: name", type_name);
            debug!("ParserExpression::StaticMethodCall: method", method);

            // 🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧
            // So we are down to this. I suppose that we can check the obj against
            // what's been entered thus far. Really this should be a second pass
            // then. For now, I'm going to hack something in...
            // We could do something with the imports...
            // 🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧
            let ty = if type_name == "Uuid" && method == "new" {
                ValueType::new_ty(Arc::new(RwLock::new(Ty::new_s_uuid())), lu_dog)
            } else {
                debug!(
                    "ParserExpression::StaticMethodCall: looking up type",
                    type_name
                );
                // How do we look up a type? Oh, duh.
                if let Some(obj) = model.exhume_object_id_by_name(&type_name) {
                    let ty = model.exhume_ty(&obj).unwrap();
                    lu_dog.exhume_value_type(&ty.id()).unwrap().clone()
                } else {
                    ValueType::new_unknown()
                }
            };

            let mut last_arg_uuid: Option<Uuid> = None;
            let params: Vec<&ParserExpression> = params.iter().map(|param| &param.0).collect();
            for param in params {
                let (arg_expr, ty) = inter_expression(
                    &Arc::new(RwLock::new(param.to_owned())),
                    block,
                    lu_dog,
                    model,
                    sarzak,
                );
                let _value = Value::new_expression(block.clone(), ty, arg_expr.clone(), lu_dog);
                let arg = Argument::new(None, call.clone(), arg_expr.clone(), lu_dog);
                last_arg_uuid = link_argument!(last_arg_uuid, arg, lu_dog);
            }

            (expr, ty)
        }
        //
        // StringLiteral
        //
        ParserExpression::StringLiteral(literal) => {
            debug!("ParserExpression::StringLiteral", literal);
            (
                Expression::new_literal(
                    Literal::new_string_literal(
                        StringLiteral::new(literal.to_owned(), lu_dog),
                        lu_dog,
                    ),
                    lu_dog,
                ),
                ValueType::new_ty(Arc::new(RwLock::new(Ty::new_s_string())), lu_dog),
            )
        }
        //
        // Struct
        //
        ParserExpression::Struct((name, _), fields) => {
            let name = if let Token::Ident(name) = name {
                // name.de_sanitize()
                name
            } else {
                panic!("I don't think that we should ever see anything other than an object here: {:?}", name);
            };
            debug!("ParserExpression::Struct", name);

            // dbg!(&lu_dog.iter_woog_struct().collect::<Vec<_>>());

            // Here we don't de_sanitize the name, and we are looking it up in the
            // dwarf model.
            let struct_id = lu_dog.exhume_woog_struct_id_by_name(&name).unwrap();
            let woog_struct = lu_dog.exhume_woog_struct(&struct_id).unwrap().clone();

            let expr = StructExpression::new(Uuid::new_v4(), woog_struct, lu_dog);
            fields
                .iter()
                .map(|((name, _), (field_expr, _))| (name, field_expr))
                .for_each(|(name, field_expr)| {
                    // 🚧 Do type checking here? I don't think that I have what I need.
                    let (field_expr, _) = inter_expression(
                        &Arc::new(RwLock::new(field_expr.to_owned())),
                        block,
                        lu_dog,
                        model,
                        sarzak,
                    );
                    let _field =
                        FieldExpression::new(name.to_owned(), field_expr, expr.clone(), lu_dog);
                });

            // model.iter_object().for_each(|o| debug!("object", o.name));

            // Same name, de_sanitized, in a different model. Oh, right, this is
            // the source model. What's going on above?
            let obj = model.exhume_object_id_by_name(name.de_sanitize()).unwrap();
            let ty = model.exhume_ty(&obj).unwrap();

            (
                Expression::new_struct_expression(expr, lu_dog),
                ValueType::new_ty(Arc::new(RwLock::new(ty.clone())), lu_dog),
            )
        }
        道 => todo!("{:?}", 道),
    }
}

fn inter_import(path: &String, alias: &Option<(String, Range<usize>)>, lu_dog: &mut LuDogStore) {
    let mut path_root = path.split("::").collect::<Vec<_>>();
    path_root.pop().expect("Path root not found");
    let path_root = path_root.join("::");
    let obj_name = path.split("::").last().unwrap();
    let (has_alias, alias) = if let Some((alias, _)) = alias {
        (true, alias.to_owned())
    } else {
        (false, "".to_owned())
    };

    let import = Import::new(
        alias,
        has_alias,
        obj_name.to_owned(),
        path_root,
        None,
        lu_dog,
    );
    debug!("import", import);
}

fn inter_implementation(
    name: &str,
    funcs: &[(Spanned<String>, Box<Item>)],
    lu_dog: &mut LuDogStore,
    model: &SarzakStore,
    sarzak: &SarzakStore,
) {
    let name = name.de_sanitize();

    let impl_ty = get_value_type(
        &Type::UserType(Box::new(Token::Ident(name.to_owned()))),
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
        .find(|mt| mt.read().unwrap().object == Some(obj.id))
        .expect(&format!("Struct for {} not found", name))
        .clone();
    let implementation = Implementation::new(mt, lu_dog);

    for ((name, _), func) in funcs {
        match **func {
            Item::Function(ref params, ref return_type, ref stmts) => {
                inter_func(&name, &params, &return_type, &stmts, Some(implementation.clone()), Some(impl_ty.clone()), lu_dog, model, sarzak)
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
    debug!("inter_struct", name);
    let s_name = name.de_sanitize();

    let obj_id = model
        .exhume_object_id_by_name(s_name)
        .expect(&format!("Object {} not found", s_name));

    let obj = model
        .exhume_object(obj_id)
        .expect(&format!("Object {} not found", s_name));

    let mt = WoogStruct::new(name.to_owned(), Some(obj.clone()), lu_dog);
    let _ty = ValueType::new_woog_struct(mt.clone(), lu_dog);
    for ((name, _), (type_, _)) in fields {
        let name = name.de_sanitize();

        let ty = get_value_type(type_, None, lu_dog, model, sarzak);
        let _field = Field::new(name.to_owned(), mt.clone(), ty, lu_dog);
    }
}

/// Get a Lu-Dog ValueType from a Dwarf Type
///
/// Note that the `new_*` methods on `Ty` just return `const`s. Also, the
/// `ValueType::new_ty` method takes on the id of it's subtype, so neither do
/// those take much space.
///
/// 🚧 This should return a result...
fn get_value_type(
    type_: &Type,
    enclosing_type: Option<Arc<RwLock<ValueType>>>,
    lu_dog: &mut LuDogStore,
    model: &SarzakStore,
    sarzak: &SarzakStore,
) -> Arc<RwLock<ValueType>> {
    match type_ {
        Type::Boolean => {
            let ty = Ty::new_boolean();
            ValueType::new_ty(Arc::new(RwLock::new(ty)), lu_dog)
        }
        Type::Empty => ValueType::new_empty(),
        Type::Float => {
            let ty = Ty::new_float();
            ValueType::new_ty(Arc::new(RwLock::new(ty)), lu_dog)
        }
        Type::Integer => {
            let ty = Ty::new_integer();
            ValueType::new_ty(Arc::new(RwLock::new(ty)), lu_dog)
        }
        Type::List(ref type_) => {
            let inner_type = get_value_type(type_, enclosing_type, lu_dog, model, sarzak);
            let list = List::new(inner_type, lu_dog);
            ValueType::new_list(list, lu_dog)
        }
        Type::Option(ref type_) => {
            let inner_type = get_value_type(type_, enclosing_type, lu_dog, model, sarzak);
            let option = WoogOption::new_z_none(inner_type, lu_dog);
            ValueType::new_woog_option(option, lu_dog)
        }
        Type::Reference(ref type_) => {
            let inner_type = get_value_type(type_, enclosing_type, lu_dog, model, sarzak);
            // We don't know the address yet -- we'll fix it in the interpreter.
            let reference = Reference::new(Uuid::new_v4(), false, inner_type, lu_dog);
            ValueType::new_reference(reference, lu_dog)
        }
        Type::String => {
            let ty = Ty::new_s_string();
            ValueType::new_ty(Arc::new(RwLock::new(ty)), lu_dog)
        }
        Type::UserType(tok) => {
            let name = match **tok {
                Token::Ident(ref name) => name,
                _ => panic!("Expected object, found {:?}", tok),
            };

            let name = name.de_sanitize();

            // Deal with imports
            let import = lu_dog.iter_import().find(|import| {
                let import = import.read().unwrap();
                import.name == name || (import.has_alias && import.alias == name)
            });

            if let Some(import) = import {
                // Now what do we do with it? If it's something that we generated,
                // then we could load up the store, if we knew where it was, and
                // but we don't.
                //
                // I don't think that we can actually connect the dots until we've
                // generated the imports themselves. So the best we can do is leave
                // behind a breadcrumb.
                //
                // It might be sort of cool to make `Import` a `ValueType`, and then
                // just return this. `Function` and `Struct`, two out of the other
                // three `Item`s are already `ValueType`s, so it's not a stretch.
                ValueType::new_import(import, lu_dog)
            } else if name == "Self" {
                match enclosing_type {
                    Some(ty) => ty.clone(),
                    None => panic!("Self must be used inside an impl block."),
                }
            } else if name == "String" {
                ValueType::new_ty(Arc::new(RwLock::new(Ty::new_s_string())), lu_dog)
            } else
            // Look for the Object in the model domain first.
            if let Some(ty) = model.iter_ty().find(|ty| match ty {
                Ty::Object(ref obj) => {
                    let obj = model.exhume_object(obj).unwrap();
                    obj.name.to_upper_camel_case() == *name
                }
                _ => false,
            }) {
                ValueType::new_ty(Arc::new(RwLock::new(ty.clone())), lu_dog)
            } else {
                // Unlikely to have to reach back this far, except of course for
                // the Uuid. So, it's not unlikely; it's the least likely.
                if let Some(ty) = sarzak.iter_ty().find(|ty| match ty {
                    Ty::Object(ref obj) => {
                        let obj = sarzak.exhume_object(obj).unwrap();
                        obj.name.to_upper_camel_case() == *name
                    }
                    _ => false,
                }) {
                    ValueType::new_ty(Arc::new(RwLock::new(ty.clone())), lu_dog)
                } else {
                    panic!("Type not found for object {}.", name)
                }
            }
        }
        Type::Uuid => {
            let ty = Ty::new_s_uuid();
            ValueType::new_ty(Arc::new(RwLock::new(ty)), lu_dog)
        }
        道 => todo!("{:?}", 道),
    }
}

trait DeSanitize {
    fn de_sanitize(&self) -> &str;
}

impl DeSanitize for String {
    fn de_sanitize(&self) -> &str {
        if let Some(str) = de_sanitize(self) {
            str
        } else {
            self
        }
    }
}

impl DeSanitize for &str {
    fn de_sanitize(&self) -> &str {
        if let Some(str) = de_sanitize(self) {
            str
        } else {
            self
        }
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
        "XSuper" => Some("Super"),
        "XBox" => Some("Box"),
        "ZObjectStore" => Some("ObjectStore"),
        "ZSome" => Some("Some"),
        "ZNone" => Some("None"),
        _ => None,
    }
}
