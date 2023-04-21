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
            Block, Call, Error, ErrorExpression, Expression, ExpressionStatement, Field,
            FieldExpression, Function, FunctionCall, Implementation, Import, IntegerLiteral,
            LetStatement, Literal, LocalVariable, Parameter, Print, Statement, StaticMethodCall,
            StringLiteral, StructExpression, Value, ValueEnum, ValueType, Variable,
            VariableExpression, WoogOption, WoogStruct,
        },
        Argument, List, MethodCall, Reference,
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

macro_rules! link_argument {
    ($last:expr, $next:expr, $store:expr) => {{
        if let Some(last) = $last {
            let mut last = $store.exhume_argument(&last).unwrap().clone();
            last.next = Some($next.id);
            $store.inter_argument(last);
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

macro_rules! link_field_expr {
    ($last:expr, $next:expr, $store:expr) => {{
        if let Some(last) = $last {
            let mut last = $store.exhume_field_expression(&last).unwrap().clone();
            last.next = Some($next.id);
            $store.inter_field_expression(last);
        }

        Some($next.id)
    }};
}

macro_rules! debug {
    ($msg:literal, $($arg:expr),*) => {
        $(
            log::debug!(
                "{} --> {:?}\n  --> {}:{}:{}",
                Colour::Yellow.paint($msg),
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
    impl_block: Option<&Implementation>,
    impl_ty: Option<&ValueType>,
    lu_dog: &mut LuDogStore,
    model: &SarzakStore,
    sarzak: &SarzakStore,
) {
    debug!("inter_func", name);
    let block = Block::new(Uuid::new_v4(), lu_dog);

    let name = name.de_sanitize();

    let ret_ty = get_value_type(&return_type.0, impl_ty, lu_dog, model, sarzak);
    let func = Function::new(name.to_owned(), &block, impl_block, &ret_ty, lu_dog);
    // Create a type for our function
    ValueType::new_function(&func, lu_dog);

    let mut last_param_uuid: Option<Uuid> = None;
    for ((param_name, _), (param_ty, _)) in params {
        debug!("inter_func name", param_name);
        debug!("inter_func ty", param_ty);
        let param = Parameter::new(&func, None, lu_dog);
        let var = Variable::new_parameter(param_name.to_owned(), &param, lu_dog);
        // 🚧 We'll need to do something about this soon. Actually, it never belonged
        // here. It only makes sense that you can only have values in a block. Now the
        // model enforces that.
        //
        // That said, we need to introduce the values into the block, so that we don't
        // error out when parsing the statements.
        //
        let param_ty = get_value_type(&param_ty, impl_ty, lu_dog, model, sarzak);
        let _value = Value::new_variable(&block, &param_ty, &var, lu_dog);
        last_param_uuid = link_parameter!(last_param_uuid, param, lu_dog);
    }

    let stmts: Vec<&ParserStatement> = stmts.iter().map(|stmt| &stmt.0).collect();
    inter_statements(&stmts, &block, lu_dog, model, sarzak);
}

pub fn inter_statement(
    stmt: &ParserStatement,
    block: &Block,
    lu_dog: &mut LuDogStore,
    model: &SarzakStore,
    sarzak: &SarzakStore,
) -> (Statement, ValueType) {
    debug!("inter_statement", stmt);

    match stmt {
        ParserStatement::Let((var_name, _), type_, (expr, _)) => {
            // Setup the local variable that is the LHS of the statement.
            let local = LocalVariable::new(Uuid::new_v4(), lu_dog);
            let var = Variable::new_local_variable(var_name.to_owned(), &local, lu_dog);

            // Now parse the RHS, which is an expression.
            let (expr, ty) = inter_expression(expr, &block, lu_dog, model, sarzak);

            let ty = if let Some((type_, _)) = type_ {
                type_.into_value_type(lu_dog, model, sarzak)
            } else {
                ty
            };

            if let ValueType::Unknown(_) = ty {
                error!("Unknown type for variable", var_name);
            }

            // Create a variable, now that we (hopefully) have a type from the expression.
            let _value = Value::new_variable(&block, &ty, &var, lu_dog);

            // Setup the let statement itself.
            let stmt = LetStatement::new(&expr, &local, lu_dog);
            let stmt = Statement::new_let_statement(&block, None, &stmt, lu_dog);

            (stmt, ValueType::new_empty())
        }
        ParserStatement::Result((ref expr, _)) => {
            let (expr, ty) = inter_expression(expr, &block, lu_dog, model, sarzak);
            let stmt = ExpressionStatement::new(&expr, lu_dog);
            let stmt = Statement::new_expression_statement(&block, None, &stmt, lu_dog);

            (stmt, ty)
        }
        ParserStatement::Expression((expr, _)) => {
            let (expr, _) = inter_expression(expr, &block, lu_dog, model, sarzak);
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
    model: &SarzakStore,
    sarzak: &SarzakStore,
) -> ValueType {
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
    expr: &ParserExpression,
    block: &Block,
    lu_dog: &mut LuDogStore,
    model: &SarzakStore,
    sarzak: &SarzakStore,
) -> (Expression, ValueType) {
    debug!("inter_expression", expr);

    match expr {
        ParserExpression::Block(ref stmts) => {
            let block = Block::new(Uuid::new_v4(), lu_dog);
            debug!("ParserExpression::Block", block);
            let stmts: Vec<&ParserStatement> = stmts.iter().map(|stmt| &stmt.0).collect();
            (
                Expression::new_block(&block, lu_dog),
                inter_statements(&stmts, &block, lu_dog, model, sarzak),
            )
        }
        ParserExpression::FunctionCall(func, params) => {
            debug!("ParserExpression::FunctionCall", func);
            let func = &func.0;
            let params: Vec<&ParserExpression> = params.iter().map(|param| &param.0).collect();
            debug!("ParserExpression::FunctionCall", params);
            let (func_expr, ret_ty) = inter_expression(&func, &block, lu_dog, model, sarzak);
            let func_call = Call::new_function_call(Some(&func_expr), lu_dog);
            let func = Expression::new_call(&func_call, lu_dog);

            let mut last_arg_uuid: Option<Uuid> = None;

            for param in params {
                let (arg_expr, ty) = inter_expression(param, &block, lu_dog, model, sarzak);
                let _value = Value::new_expression(&block, &ty, &arg_expr, lu_dog);
                let arg = Argument::new(None, &func_call, &arg_expr, lu_dog);
                last_arg_uuid = link_argument!(last_arg_uuid, arg, lu_dog);
            }

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
            debug!("ParserExpression::LocalVariable", name);
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
                            debug!("ParserExpression::LocalVariable: var", var);
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
                .filter(|value| value.is_some())
                .collect();

            debug!("ParserExpression::LocalVariable: expr_ty", expr_ty);

            if expr_ty.len() > 0 {
                if expr_ty.len() > 1 {
                    panic!(
                        "I don't think that we should ever see more than one value here: {:?}",
                        &expr_ty
                    );
                }

                debug!("ParserExpression::LocalVariable: returning", expr_ty[0]);
                expr_ty[0].clone().unwrap()
            } else {
                // 🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧
                // As neat as it is that I'm compiling this into the binary, we should actually
                // bail here, and make the user do something about it. The issue is that this
                // may get run from the repl, and in that case we want to return the expression.
                // So, we need a flag...
                let expr = ErrorExpression::new(
                    format!(
                        "\n  ──➤  variable: `{}` not found\n",
                        Colour::Red.paint(name)
                    ),
                    lu_dog,
                );
                let expr = Expression::new_error_expression(&expr, lu_dog);
                (
                    expr,
                    ValueType::new_error(&Error::new_unknown_variable(), lu_dog),
                )
            }
        }
        ParserExpression::MethodCall(instance, (method, _), params) => {
            debug!("ParserExpression::MethodCall", instance);
            let (instance, instance_ty) =
                inter_expression(&(**instance).0, &block, lu_dog, model, sarzak);
            let meth = MethodCall::new(method.to_owned(), lu_dog);
            let call = Call::new_method_call(Some(&instance), &meth, lu_dog);
            let expr = Expression::new_call(&call, lu_dog);

            (expr, instance_ty)
        }
        ParserExpression::Print(expr) => {
            let (expr, ty) = inter_expression(&(**expr).0, &block, lu_dog, model, sarzak);
            let print = Print::new(&expr, lu_dog);

            (Expression::new_print(&print, lu_dog), ty)
        }
        ParserExpression::StaticMethodCall((obj, _), (method, _), params) => {
            debug!("ParserExpression::StaticMethodCall", obj);
            let type_name = if let Token::Object(obj) = obj {
                obj.de_sanitize()
            } else {
                panic!("I don't think that we should ever see anything other than an object here: {:?}", obj);
            };

            let meth = StaticMethodCall::new(method.to_owned(), type_name.to_owned(), lu_dog);
            let call = Call::new_static_method_call(None, &meth, lu_dog);
            let expr = Expression::new_call(&call, lu_dog);

            // 🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧
            // So we are down to this. I suppose that we can check the obj against
            // what's been entered thus far. Really this should be a second pass
            // then. For now, I'm going to hack something in...
            // We could do something with the imports...
            // 🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧
            let ty = if type_name == "Uuid" && method == "new_v4" {
                ValueType::new_ty(&Ty::new_s_uuid(), lu_dog)
            } else {
                // How do we look up a type? Oh, duh.
                let obj = model.exhume_object_id_by_name(type_name).unwrap();
                let ty = model.exhume_ty(&obj).unwrap();
                lu_dog.exhume_value_type(&ty.id()).unwrap().clone()

                // lu_dog
                //     .iter_woog_struct()
                //     .find(|woog_struct| {
                //         let woog_struct = model.exhume_object(&woog_struct.object).unwrap();
                //         woog_struct.name == type_name
                //     })
                //     .map(|woog_struct| {
                //         let obj = model.exhume_object(&woog_struct.object).unwrap();
                //         let ty = model.exhume_ty(&obj.id).unwrap();
                //         // ValueType::new_ty(&ty, lu_dog)
                //         lu_dog.exhume_value_type(&ty.id()).unwrap().clone()
                //     })
                //     .unwrap_or_else(|| {
                //         error!("Could not find type for", type_name, method);
                //         ValueType::new_unknown()
                //     })
            };

            (expr, ty)
        }
        ParserExpression::StringLiteral(literal) => {
            debug!("ParserExpression::StringLiteral", literal);
            (
                Expression::new_literal(
                    &Literal::new_string_literal(
                        &StringLiteral::new(literal.to_owned(), lu_dog),
                        lu_dog,
                    ),
                    lu_dog,
                ),
                ValueType::new_ty(&Ty::new_s_string(), lu_dog),
            )
        }
        ParserExpression::Struct((name, _), fields) => {
            let name = if let Token::Object(name) = name {
                // name.de_sanitize()
                name
            } else {
                panic!("I don't think that we should ever see anything other than an object here: {:?}", name);
            };
            debug!("ParserExpression::Struct", name);

            // dbg!(&lu_dog.iter_woog_struct().collect::<Vec<_>>());

            // Here we don't de_sanitize the name, and we are looking it up in the
            // dwarf model.
            let struct_id = lu_dog.exhume_woog_struct_id_by_name(name).unwrap();
            let woog_struct = lu_dog.exhume_woog_struct(&struct_id).unwrap().clone();

            let expr = StructExpression::new(Uuid::new_v4(), &woog_struct, lu_dog);
            let mut last_field_uuid: Option<Uuid> = None;
            fields
                .iter()
                .map(|((name, _), (field_expr, _))| (name, field_expr))
                .for_each(|(name, field_expr)| {
                    // 🚧 Do type checking here? I don't think that I have what I need.
                    let (field_expr, _) =
                        inter_expression(field_expr, &block, lu_dog, model, sarzak);
                    let field =
                        FieldExpression::new(name.to_owned(), &field_expr, None, &expr, lu_dog);
                    last_field_uuid = link_field_expr!(last_field_uuid, field, lu_dog);
                });

            // model.iter_object().for_each(|o| debug!("object", o.name));

            // Same name, de_sanitized, in a different model. Oh, right, this is
            // the source model. What's going on above?
            let obj = model.exhume_object_id_by_name(name.de_sanitize()).unwrap();
            let ty = model.exhume_ty(&obj).unwrap();

            (
                Expression::new_struct_expression(&expr, lu_dog),
                ValueType::new_ty(&ty, lu_dog),
            )
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
    let name = name.de_sanitize();

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
        .find(|mt| mt.object == Some(obj.id))
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
    let s_name = name.de_sanitize();

    let obj_id = model
        .exhume_object_id_by_name(s_name)
        .expect(&format!("Object {} not found", s_name));

    let obj = model
        .exhume_object(obj_id)
        .expect(&format!("Object {} not found", s_name));

    let mt = WoogStruct::new(name.to_owned(), Some(&obj), lu_dog);
    let _ty = ValueType::new_woog_struct(&mt, lu_dog);
    for ((name, _), (type_, _)) in fields {
        let name = name.de_sanitize();

        let ty = get_value_type(type_, None, lu_dog, model, sarzak);
        let _field = Field::new(name.to_owned(), &mt, &ty, lu_dog);
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
        Type::Empty => ValueType::new_empty(),
        Type::Float => {
            let ty = Ty::new_float();
            ValueType::new_ty(&ty, lu_dog)
        }
        Type::Integer => {
            let ty = Ty::new_integer();
            ValueType::new_ty(&ty, lu_dog)
        }
        Type::List(ref type_) => {
            let inner_type = get_value_type(type_, enclosing_type, lu_dog, model, sarzak);
            let list = List::new(&inner_type, lu_dog);
            ValueType::new_list(&list, lu_dog)
        }
        Type::Option(ref type_) => {
            let inner_type = get_value_type(type_, enclosing_type, lu_dog, model, sarzak);
            let option = WoogOption::new_none(&inner_type, lu_dog);
            ValueType::new_woog_option(&option, lu_dog)
        }
        Type::Reference(ref type_) => {
            let inner_type = get_value_type(type_, enclosing_type, lu_dog, model, sarzak);
            // We don't know the address yet -- we'll fix it in the interpreter.
            let reference = Reference::new(Uuid::new_v4(), false, &inner_type, lu_dog);
            ValueType::new_reference(&reference, lu_dog)
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

            let name = name.de_sanitize();

            // Need to deal with Self.
            if name == "Self" {
                match enclosing_type {
                    Some(ty) => ty.clone(),
                    None => panic!("Self must be used inside an impl block."),
                }
            } else if name == "ObjectStore" {
                ValueType::new_empty()
            } else if name == "String" {
                ValueType::new_ty(&Ty::new_s_string(), lu_dog)
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
                // the Uuid. So, it's not unlikely; it's the least likely.
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
        _ => None,
    }
}
