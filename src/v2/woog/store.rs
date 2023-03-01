//! v2::woog Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Access`]
//! * [`Block`]
//! * [`Call`]
//! * [`Expression`]
//! * [`GraceType`]
//! * [`XLet`]
//! * [`Local`]
//! * [`ObjectMethod`]
//! * [`WoogOption`]
//! * [`Ownership`]
//! * [`Parameter`]
//! * [`Reference`]
//! * [`Statement`]
//! * [`Value`]
//! * [`Variable`]
//! * [`Visibility`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-object-store-definition"}}}
use std::collections::HashMap;
use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::woog::types::{
    Access, Block, Call, Expression, GraceType, Local, ObjectMethod, Ownership, Parameter,
    Reference, Statement, Value, Variable, Visibility, WoogOption, XLet, BORROWED, KRATE, LITERAL,
    MUTABLE, OWNED, PRIVATE, PUBLIC,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    access: HashMap<Uuid, Access>,
    block: HashMap<Uuid, Block>,
    call: HashMap<Uuid, Call>,
    expression: HashMap<Uuid, Expression>,
    grace_type: HashMap<Uuid, GraceType>,
    x_let: HashMap<Uuid, XLet>,
    local: HashMap<Uuid, Local>,
    object_method: HashMap<Uuid, ObjectMethod>,
    woog_option: HashMap<Uuid, WoogOption>,
    ownership: HashMap<Uuid, Ownership>,
    parameter: HashMap<Uuid, Parameter>,
    reference: HashMap<Uuid, Reference>,
    statement: HashMap<Uuid, Statement>,
    value: HashMap<Uuid, Value>,
    variable: HashMap<Uuid, Variable>,
    visibility: HashMap<Uuid, Visibility>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            access: HashMap::new(),
            block: HashMap::new(),
            call: HashMap::new(),
            expression: HashMap::new(),
            grace_type: HashMap::new(),
            x_let: HashMap::new(),
            local: HashMap::new(),
            object_method: HashMap::new(),
            woog_option: HashMap::new(),
            ownership: HashMap::new(),
            parameter: HashMap::new(),
            reference: HashMap::new(),
            statement: HashMap::new(),
            value: HashMap::new(),
            variable: HashMap::new(),
            visibility: HashMap::new(),
        };

        // Initialize Singleton Subtypes
        store.inter_expression(Expression::Literal(LITERAL));
        store.inter_ownership(Ownership::Borrowed(BORROWED));
        store.inter_ownership(Ownership::Mutable(MUTABLE));
        store.inter_ownership(Ownership::Owned(OWNED));
        store.inter_visibility(Visibility::Krate(KRATE));
        store.inter_visibility(Visibility::Private(PRIVATE));
        store.inter_visibility(Visibility::Public(PUBLIC));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-object-store-methods"}}}
    /// Inter [`Access`] into the store.
    ///
    pub fn inter_access(&mut self, access: Access) {
        self.access.insert(access.id, access);
    }

    /// Exhume [`Access`] from the store.
    ///
    pub fn exhume_access(&self, id: &Uuid) -> Option<&Access> {
        self.access.get(id)
    }

    /// Exhume [`Access`] from the store — mutably.
    ///
    pub fn exhume_access_mut(&mut self, id: &Uuid) -> Option<&mut Access> {
        self.access.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Access>`.
    ///
    pub fn iter_access(&self) -> impl Iterator<Item = &Access> {
        self.access.values()
    }

    /// Inter [`Block`] into the store.
    ///
    pub fn inter_block(&mut self, block: Block) {
        self.block.insert(block.id, block);
    }

    /// Exhume [`Block`] from the store.
    ///
    pub fn exhume_block(&self, id: &Uuid) -> Option<&Block> {
        self.block.get(id)
    }

    /// Exhume [`Block`] from the store — mutably.
    ///
    pub fn exhume_block_mut(&mut self, id: &Uuid) -> Option<&mut Block> {
        self.block.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Block>`.
    ///
    pub fn iter_block(&self) -> impl Iterator<Item = &Block> {
        self.block.values()
    }

    /// Inter [`Call`] into the store.
    ///
    pub fn inter_call(&mut self, call: Call) {
        self.call.insert(call.id, call);
    }

    /// Exhume [`Call`] from the store.
    ///
    pub fn exhume_call(&self, id: &Uuid) -> Option<&Call> {
        self.call.get(id)
    }

    /// Exhume [`Call`] from the store — mutably.
    ///
    pub fn exhume_call_mut(&mut self, id: &Uuid) -> Option<&mut Call> {
        self.call.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Call>`.
    ///
    pub fn iter_call(&self) -> impl Iterator<Item = &Call> {
        self.call.values()
    }

    /// Inter [`Expression`] into the store.
    ///
    pub fn inter_expression(&mut self, expression: Expression) {
        self.expression.insert(expression.id(), expression);
    }

    /// Exhume [`Expression`] from the store.
    ///
    pub fn exhume_expression(&self, id: &Uuid) -> Option<&Expression> {
        self.expression.get(id)
    }

    /// Exhume [`Expression`] from the store — mutably.
    ///
    pub fn exhume_expression_mut(&mut self, id: &Uuid) -> Option<&mut Expression> {
        self.expression.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Expression>`.
    ///
    pub fn iter_expression(&self) -> impl Iterator<Item = &Expression> {
        self.expression.values()
    }

    /// Inter [`GraceType`] into the store.
    ///
    pub fn inter_grace_type(&mut self, grace_type: GraceType) {
        self.grace_type.insert(grace_type.id(), grace_type);
    }

    /// Exhume [`GraceType`] from the store.
    ///
    pub fn exhume_grace_type(&self, id: &Uuid) -> Option<&GraceType> {
        self.grace_type.get(id)
    }

    /// Exhume [`GraceType`] from the store — mutably.
    ///
    pub fn exhume_grace_type_mut(&mut self, id: &Uuid) -> Option<&mut GraceType> {
        self.grace_type.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, GraceType>`.
    ///
    pub fn iter_grace_type(&self) -> impl Iterator<Item = &GraceType> {
        self.grace_type.values()
    }

    /// Inter [`XLet`] into the store.
    ///
    pub fn inter_x_let(&mut self, x_let: XLet) {
        self.x_let.insert(x_let.id, x_let);
    }

    /// Exhume [`XLet`] from the store.
    ///
    pub fn exhume_x_let(&self, id: &Uuid) -> Option<&XLet> {
        self.x_let.get(id)
    }

    /// Exhume [`XLet`] from the store — mutably.
    ///
    pub fn exhume_x_let_mut(&mut self, id: &Uuid) -> Option<&mut XLet> {
        self.x_let.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XLet>`.
    ///
    pub fn iter_x_let(&self) -> impl Iterator<Item = &XLet> {
        self.x_let.values()
    }

    /// Inter [`Local`] into the store.
    ///
    pub fn inter_local(&mut self, local: Local) {
        self.local.insert(local.id, local);
    }

    /// Exhume [`Local`] from the store.
    ///
    pub fn exhume_local(&self, id: &Uuid) -> Option<&Local> {
        self.local.get(id)
    }

    /// Exhume [`Local`] from the store — mutably.
    ///
    pub fn exhume_local_mut(&mut self, id: &Uuid) -> Option<&mut Local> {
        self.local.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Local>`.
    ///
    pub fn iter_local(&self) -> impl Iterator<Item = &Local> {
        self.local.values()
    }

    /// Inter [`ObjectMethod`] into the store.
    ///
    pub fn inter_object_method(&mut self, object_method: ObjectMethod) {
        self.object_method.insert(object_method.id, object_method);
    }

    /// Exhume [`ObjectMethod`] from the store.
    ///
    pub fn exhume_object_method(&self, id: &Uuid) -> Option<&ObjectMethod> {
        self.object_method.get(id)
    }

    /// Exhume [`ObjectMethod`] from the store — mutably.
    ///
    pub fn exhume_object_method_mut(&mut self, id: &Uuid) -> Option<&mut ObjectMethod> {
        self.object_method.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectMethod>`.
    ///
    pub fn iter_object_method(&self) -> impl Iterator<Item = &ObjectMethod> {
        self.object_method.values()
    }

    /// Inter [`WoogOption`] into the store.
    ///
    pub fn inter_woog_option(&mut self, woog_option: WoogOption) {
        self.woog_option.insert(woog_option.id, woog_option);
    }

    /// Exhume [`WoogOption`] from the store.
    ///
    pub fn exhume_woog_option(&self, id: &Uuid) -> Option<&WoogOption> {
        self.woog_option.get(id)
    }

    /// Exhume [`WoogOption`] from the store — mutably.
    ///
    pub fn exhume_woog_option_mut(&mut self, id: &Uuid) -> Option<&mut WoogOption> {
        self.woog_option.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogOption>`.
    ///
    pub fn iter_woog_option(&self) -> impl Iterator<Item = &WoogOption> {
        self.woog_option.values()
    }

    /// Inter [`Ownership`] into the store.
    ///
    pub fn inter_ownership(&mut self, ownership: Ownership) {
        self.ownership.insert(ownership.id(), ownership);
    }

    /// Exhume [`Ownership`] from the store.
    ///
    pub fn exhume_ownership(&self, id: &Uuid) -> Option<&Ownership> {
        self.ownership.get(id)
    }

    /// Exhume [`Ownership`] from the store — mutably.
    ///
    pub fn exhume_ownership_mut(&mut self, id: &Uuid) -> Option<&mut Ownership> {
        self.ownership.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Ownership>`.
    ///
    pub fn iter_ownership(&self) -> impl Iterator<Item = &Ownership> {
        self.ownership.values()
    }

    /// Inter [`Parameter`] into the store.
    ///
    pub fn inter_parameter(&mut self, parameter: Parameter) {
        self.parameter.insert(parameter.id, parameter);
    }

    /// Exhume [`Parameter`] from the store.
    ///
    pub fn exhume_parameter(&self, id: &Uuid) -> Option<&Parameter> {
        self.parameter.get(id)
    }

    /// Exhume [`Parameter`] from the store — mutably.
    ///
    pub fn exhume_parameter_mut(&mut self, id: &Uuid) -> Option<&mut Parameter> {
        self.parameter.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    pub fn iter_parameter(&self) -> impl Iterator<Item = &Parameter> {
        self.parameter.values()
    }

    /// Inter [`Reference`] into the store.
    ///
    pub fn inter_reference(&mut self, reference: Reference) {
        self.reference.insert(reference.id, reference);
    }

    /// Exhume [`Reference`] from the store.
    ///
    pub fn exhume_reference(&self, id: &Uuid) -> Option<&Reference> {
        self.reference.get(id)
    }

    /// Exhume [`Reference`] from the store — mutably.
    ///
    pub fn exhume_reference_mut(&mut self, id: &Uuid) -> Option<&mut Reference> {
        self.reference.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Reference>`.
    ///
    pub fn iter_reference(&self) -> impl Iterator<Item = &Reference> {
        self.reference.values()
    }

    /// Inter [`Statement`] into the store.
    ///
    pub fn inter_statement(&mut self, statement: Statement) {
        self.statement.insert(statement.id, statement);
    }

    /// Exhume [`Statement`] from the store.
    ///
    pub fn exhume_statement(&self, id: &Uuid) -> Option<&Statement> {
        self.statement.get(id)
    }

    /// Exhume [`Statement`] from the store — mutably.
    ///
    pub fn exhume_statement_mut(&mut self, id: &Uuid) -> Option<&mut Statement> {
        self.statement.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Statement>`.
    ///
    pub fn iter_statement(&self) -> impl Iterator<Item = &Statement> {
        self.statement.values()
    }

    /// Inter [`Value`] into the store.
    ///
    pub fn inter_value(&mut self, value: Value) {
        self.value.insert(value.id, value);
    }

    /// Exhume [`Value`] from the store.
    ///
    pub fn exhume_value(&self, id: &Uuid) -> Option<&Value> {
        self.value.get(id)
    }

    /// Exhume [`Value`] from the store — mutably.
    ///
    pub fn exhume_value_mut(&mut self, id: &Uuid) -> Option<&mut Value> {
        self.value.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Value>`.
    ///
    pub fn iter_value(&self) -> impl Iterator<Item = &Value> {
        self.value.values()
    }

    /// Inter [`Variable`] into the store.
    ///
    pub fn inter_variable(&mut self, variable: Variable) {
        self.variable.insert(variable.id(), variable);
    }

    /// Exhume [`Variable`] from the store.
    ///
    pub fn exhume_variable(&self, id: &Uuid) -> Option<&Variable> {
        self.variable.get(id)
    }

    /// Exhume [`Variable`] from the store — mutably.
    ///
    pub fn exhume_variable_mut(&mut self, id: &Uuid) -> Option<&mut Variable> {
        self.variable.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Variable>`.
    ///
    pub fn iter_variable(&self) -> impl Iterator<Item = &Variable> {
        self.variable.values()
    }

    /// Inter [`Visibility`] into the store.
    ///
    pub fn inter_visibility(&mut self, visibility: Visibility) {
        self.visibility.insert(visibility.id(), visibility);
    }

    /// Exhume [`Visibility`] from the store.
    ///
    pub fn exhume_visibility(&self, id: &Uuid) -> Option<&Visibility> {
        self.visibility.get(id)
    }

    /// Exhume [`Visibility`] from the store — mutably.
    ///
    pub fn exhume_visibility_mut(&mut self, id: &Uuid) -> Option<&mut Visibility> {
        self.visibility.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Visibility>`.
    ///
    pub fn iter_visibility(&self) -> impl Iterator<Item = &Visibility> {
        self.visibility.values()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let path = path.join("woog.json");
        fs::create_dir_all(&path)?;

        // Persist Access.
        {
            let path = path.join("access.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.access.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Block.
        {
            let path = path.join("block.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.block.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Call.
        {
            let path = path.join("call.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.call.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Expression.
        {
            let path = path.join("expression.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.expression.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Grace Type.
        {
            let path = path.join("grace_type.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.grace_type.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Let.
        {
            let path = path.join("x_let.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.x_let.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Local.
        {
            let path = path.join("local.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.local.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Object Method.
        {
            let path = path.join("object_method.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.object_method.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Option.
        {
            let path = path.join("woog_option.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.woog_option.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Ownership.
        {
            let path = path.join("ownership.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.ownership.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Parameter.
        {
            let path = path.join("parameter.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.parameter.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Reference.
        {
            let path = path.join("reference.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.reference.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Statement.
        {
            let path = path.join("statement.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.statement.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Value.
        {
            let path = path.join("value.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.value.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Variable.
        {
            let path = path.join("variable.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.variable.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Visibility.
        {
            let path = path.join("visibility.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.visibility.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        Ok(())
    }

    /// Load the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let path = path.join("woog.json");

        let mut store = Self::new();

        // Load Access.
        {
            let path = path.join("access.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let access: Vec<Access> = serde_json::from_reader(reader)?;
            store.access = access.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Block.
        {
            let path = path.join("block.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let block: Vec<Block> = serde_json::from_reader(reader)?;
            store.block = block.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Call.
        {
            let path = path.join("call.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let call: Vec<Call> = serde_json::from_reader(reader)?;
            store.call = call.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Expression.
        {
            let path = path.join("expression.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let expression: Vec<Expression> = serde_json::from_reader(reader)?;
            store.expression = expression.into_iter().map(|道| (道.id(), 道)).collect();
        }
        // Load Grace Type.
        {
            let path = path.join("grace_type.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let grace_type: Vec<GraceType> = serde_json::from_reader(reader)?;
            store.grace_type = grace_type.into_iter().map(|道| (道.id(), 道)).collect();
        }
        // Load Let.
        {
            let path = path.join("x_let.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let x_let: Vec<XLet> = serde_json::from_reader(reader)?;
            store.x_let = x_let.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Local.
        {
            let path = path.join("local.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let local: Vec<Local> = serde_json::from_reader(reader)?;
            store.local = local.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Object Method.
        {
            let path = path.join("object_method.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let object_method: Vec<ObjectMethod> = serde_json::from_reader(reader)?;
            store.object_method = object_method.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Option.
        {
            let path = path.join("woog_option.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let woog_option: Vec<WoogOption> = serde_json::from_reader(reader)?;
            store.woog_option = woog_option.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Ownership.
        {
            let path = path.join("ownership.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let ownership: Vec<Ownership> = serde_json::from_reader(reader)?;
            store.ownership = ownership.into_iter().map(|道| (道.id(), 道)).collect();
        }
        // Load Parameter.
        {
            let path = path.join("parameter.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let parameter: Vec<Parameter> = serde_json::from_reader(reader)?;
            store.parameter = parameter.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Reference.
        {
            let path = path.join("reference.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let reference: Vec<Reference> = serde_json::from_reader(reader)?;
            store.reference = reference.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Statement.
        {
            let path = path.join("statement.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let statement: Vec<Statement> = serde_json::from_reader(reader)?;
            store.statement = statement.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Value.
        {
            let path = path.join("value.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let value: Vec<Value> = serde_json::from_reader(reader)?;
            store.value = value.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Variable.
        {
            let path = path.join("variable.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let variable: Vec<Variable> = serde_json::from_reader(reader)?;
            store.variable = variable.into_iter().map(|道| (道.id(), 道)).collect();
        }
        // Load Visibility.
        {
            let path = path.join("visibility.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let visibility: Vec<Visibility> = serde_json::from_reader(reader)?;
            store.visibility = visibility.into_iter().map(|道| (道.id(), 道)).collect();
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
