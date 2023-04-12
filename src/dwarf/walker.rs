use chumsky::prelude::*;
use fnv::FnvHashMap as HashMap;
use heck::ToUpperCamelCase;
use snafu::prelude::*;

use crate::dwarf::{Expression, Item, ItemKind, Result, Spanned, Token, Type};
use crate::lu_dog::{
    store::ObjectStore as LuDogStore,
    types::{Field, Implementation, Import, ModelType, Some, ValueType, WoogOption},
};
use crate::sarzak::{
    store::ObjectStore as SarzakStore,
    types::{Object, Ty},
};

struct Function<'a> {
    name: &'a str,
}

impl<'a> Function<'a> {
    fn new(name: &'a str) -> Self {
        Self { name }
    }
}

pub fn populate_lu_dog(
    ast: &HashMap<(String, ItemKind), Item>,
    model: &SarzakStore,
    sarzak: &SarzakStore,
) -> Result<LuDogStore> {
    let mut lu_dog = LuDogStore::new();

    walk_tree(ast, &mut lu_dog, model, sarzak)?;

    dbg!(&lu_dog);

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
    // 🚧 I really should define some data structures to hold the temporary
    // stuff with lifetime annotations so that I don't need to clone everything.
    for ((ref name, _), item) in ast {
        match item {
            Item::Function(_, _, _) => funcs.push(Function::new(name)),
            Item::Implementation(funcs) => implementations.push((name.clone(), funcs.clone())),
            Item::Import(path) => inter_import(&path.0, lu_dog),
            Item::Struct(fields) => structs.push((name.clone(), fields.clone())),
        }
    }

    for (name, fields) in structs {
        inter_struct(&name, &fields, lu_dog, model, sarzak);
    }

    for (name, funcs) in implementations {
        inter_implementation(&name, &funcs, lu_dog, model, sarzak);
    }

    Ok(())
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

    let obj = model
        .exhume_object_by_name(name)
        .expect(&format!("Object {} not found", name));

    let mt = lu_dog
        .iter_model_type()
        .find(|mt| mt.object == obj.id)
        .expect(&format!("ModelType for {} not found", name))
        .clone();
    let implementation = Implementation::new(&mt, lu_dog);
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

    let obj = model
        .exhume_object_by_name(name)
        .expect(&format!("Object {} not found", name));

    let mt = ModelType::new(&obj, lu_dog);
    for ((name, _), (type_, _)) in fields {
        let name = if let Some(name) = de_sanitize(&name) {
            name
        } else {
            name
        };

        let ty = get_value_type(type_, lu_dog, model, sarzak);
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
            let ty = Ty::new_string();
            ValueType::new_ty(&ty, lu_dog)
        }
        Type::UserType(tok) => {
            let name = match **tok {
                Token::Object(ref name) => name,
                _ => panic!("Expected ident, found {:?}", tok),
            };

            let name = if let Some(name) = de_sanitize(&name) {
                name
            } else {
                name
            };

            let ty = if let Some(ty) = model.iter_ty().find(|ty| match ty {
                Ty::Object(ref obj) => {
                    let obj = model.exhume_object(obj).unwrap();
                    obj.name.to_upper_camel_case() == *name
                }
                _ => false,
            }) {
                ty
            } else {
                // Unlikely to have to reach back this far, except of course for
                // the Uuid. So, it's not unlikely, t's the least likely.
                sarzak
                    .iter_ty()
                    .find(|ty| match ty {
                        Ty::Object(ref obj) => {
                            let obj = sarzak.exhume_object(obj).unwrap();
                            obj.name.to_upper_camel_case() == *name
                        }
                        _ => false,
                    })
                    .expect(&format!("Type not found for object {}.", name))
            };

            ValueType::new_ty(ty, lu_dog)
        }
        Type::Option(ref type_) => {
            let inner = get_value_type(type_, lu_dog, model, sarzak);
            let some = Some::new(&inner, lu_dog);
            let option = WoogOption::new_some(&some, lu_dog);
            ValueType::new_woog_option(&option, lu_dog)
        }
        道 => todo!("{:?}", 道),
    }
}

fn de_sanitize(string: &str) -> Option<&str> {
    match string {
        "Ty" => Some("Type"),
        "WoogOption" => Some("Option"),
        _ => None,
    }
}
