// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-use-statements"}}}
use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::argument::Argument;
use crate::v2::lu_dog::types::block::Block;
use crate::v2::lu_dog::types::call::Call;
use crate::v2::lu_dog::types::debugger::DEBUGGER;
use crate::v2::lu_dog::types::error_expression::ErrorExpression;
use crate::v2::lu_dog::types::expression_statement::ExpressionStatement;
use crate::v2::lu_dog::types::field_access::FieldAccess;
use crate::v2::lu_dog::types::field_expression::FieldExpression;
use crate::v2::lu_dog::types::for_loop::ForLoop;
use crate::v2::lu_dog::types::grouped::Grouped;
use crate::v2::lu_dog::types::index::Index;
use crate::v2::lu_dog::types::let_statement::LetStatement;
use crate::v2::lu_dog::types::list_element::ListElement;
use crate::v2::lu_dog::types::list_expression::ListExpression;
use crate::v2::lu_dog::types::literal::Literal;
use crate::v2::lu_dog::types::operator::Operator;
use crate::v2::lu_dog::types::print::Print;
use crate::v2::lu_dog::types::range_expression::RangeExpression;
use crate::v2::lu_dog::types::result_statement::ResultStatement;
use crate::v2::lu_dog::types::struct_expression::StructExpression;
use crate::v2::lu_dog::types::type_cast::TypeCast;
use crate::v2::lu_dog::types::variable_expression::VariableExpression;
use crate::v2::lu_dog::types::x_if::XIf;
use crate::v2::lu_dog::types::x_return::XReturn;
use crate::v2::lu_dog::types::x_value::XValue;
use crate::v2::lu_dog::types::x_value::XValueEnum;
use crate::v2::lu_dog::types::z_none::Z_NONE;
use crate::v2::lu_dog::types::z_some::ZSome;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-enum-documentation"}}}
/// An Expression
///
/// Expressions are calculations that render values.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-enum-definition"}}}
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Expression {
    Block(Uuid),
    Call(Uuid),
    Debugger(Uuid),
    ErrorExpression(Uuid),
    FieldAccess(Uuid),
    FieldExpression(Uuid),
    ForLoop(Uuid),
    Grouped(Uuid),
    XIf(Uuid),
    Index(Uuid),
    ListElement(Uuid),
    ListExpression(Uuid),
    Literal(Uuid),
    ZNone(Uuid),
    Operator(Uuid),
    Print(Uuid),
    RangeExpression(Uuid),
    XReturn(Uuid),
    ZSome(Uuid),
    StructExpression(Uuid),
    TypeCast(Uuid),
    VariableExpression(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-implementation"}}}
impl Expression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-new-impl"}}}
    /// Create a new instance of Expression::Block
    pub fn new_block(block: &Rc<RefCell<Block>>, store: &mut LuDogStore) -> Rc<RefCell<Self>> {
        let id = block.borrow().id;
        if let Some(block) = store.exhume_expression(&id) {
            block
        } else {
            let new = Rc::new(RefCell::new(Self::Block(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::Call
    pub fn new_call(call: &Rc<RefCell<Call>>, store: &mut LuDogStore) -> Rc<RefCell<Self>> {
        let id = call.borrow().id;
        if let Some(call) = store.exhume_expression(&id) {
            call
        } else {
            let new = Rc::new(RefCell::new(Self::Call(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::Debugger
    pub fn new_debugger(store: &LuDogStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_expression(&DEBUGGER).unwrap()
    }

    /// Create a new instance of Expression::ErrorExpression
    pub fn new_error_expression(
        error_expression: &Rc<RefCell<ErrorExpression>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = error_expression.borrow().id;
        if let Some(error_expression) = store.exhume_expression(&id) {
            error_expression
        } else {
            let new = Rc::new(RefCell::new(Self::ErrorExpression(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::FieldAccess
    pub fn new_field_access(
        field_access: &Rc<RefCell<FieldAccess>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = field_access.borrow().id;
        if let Some(field_access) = store.exhume_expression(&id) {
            field_access
        } else {
            let new = Rc::new(RefCell::new(Self::FieldAccess(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::FieldExpression
    pub fn new_field_expression(
        field_expression: &Rc<RefCell<FieldExpression>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = field_expression.borrow().id;
        if let Some(field_expression) = store.exhume_expression(&id) {
            field_expression
        } else {
            let new = Rc::new(RefCell::new(Self::FieldExpression(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::ForLoop
    pub fn new_for_loop(
        for_loop: &Rc<RefCell<ForLoop>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = for_loop.borrow().id;
        if let Some(for_loop) = store.exhume_expression(&id) {
            for_loop
        } else {
            let new = Rc::new(RefCell::new(Self::ForLoop(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::Grouped
    pub fn new_grouped(
        grouped: &Rc<RefCell<Grouped>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = grouped.borrow().id;
        if let Some(grouped) = store.exhume_expression(&id) {
            grouped
        } else {
            let new = Rc::new(RefCell::new(Self::Grouped(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::XIf
    pub fn new_x_if(x_if: &Rc<RefCell<XIf>>, store: &mut LuDogStore) -> Rc<RefCell<Self>> {
        let id = x_if.borrow().id;
        if let Some(x_if) = store.exhume_expression(&id) {
            x_if
        } else {
            let new = Rc::new(RefCell::new(Self::XIf(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::Index
    pub fn new_index(index: &Rc<RefCell<Index>>, store: &mut LuDogStore) -> Rc<RefCell<Self>> {
        let id = index.borrow().id;
        if let Some(index) = store.exhume_expression(&id) {
            index
        } else {
            let new = Rc::new(RefCell::new(Self::Index(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::ListElement
    pub fn new_list_element(
        list_element: &Rc<RefCell<ListElement>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = list_element.borrow().id;
        if let Some(list_element) = store.exhume_expression(&id) {
            list_element
        } else {
            let new = Rc::new(RefCell::new(Self::ListElement(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::ListExpression
    pub fn new_list_expression(
        list_expression: &Rc<RefCell<ListExpression>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = list_expression.borrow().id;
        if let Some(list_expression) = store.exhume_expression(&id) {
            list_expression
        } else {
            let new = Rc::new(RefCell::new(Self::ListExpression(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::Literal
    pub fn new_literal(
        literal: &Rc<RefCell<Literal>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = literal.borrow().id();
        if let Some(literal) = store.exhume_expression(&id) {
            literal
        } else {
            let new = Rc::new(RefCell::new(Self::Literal(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::ZNone
    pub fn new_z_none(store: &LuDogStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_expression(&Z_NONE).unwrap()
    }

    /// Create a new instance of Expression::Operator
    pub fn new_operator(
        operator: &Rc<RefCell<Operator>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = operator.borrow().id;
        if let Some(operator) = store.exhume_expression(&id) {
            operator
        } else {
            let new = Rc::new(RefCell::new(Self::Operator(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::Print
    pub fn new_print(print: &Rc<RefCell<Print>>, store: &mut LuDogStore) -> Rc<RefCell<Self>> {
        let id = print.borrow().id;
        if let Some(print) = store.exhume_expression(&id) {
            print
        } else {
            let new = Rc::new(RefCell::new(Self::Print(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::RangeExpression
    pub fn new_range_expression(
        range_expression: &Rc<RefCell<RangeExpression>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = range_expression.borrow().id;
        if let Some(range_expression) = store.exhume_expression(&id) {
            range_expression
        } else {
            let new = Rc::new(RefCell::new(Self::RangeExpression(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::XReturn
    pub fn new_x_return(
        x_return: &Rc<RefCell<XReturn>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = x_return.borrow().id;
        if let Some(x_return) = store.exhume_expression(&id) {
            x_return
        } else {
            let new = Rc::new(RefCell::new(Self::XReturn(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::ZSome
    pub fn new_z_some(z_some: &Rc<RefCell<ZSome>>, store: &mut LuDogStore) -> Rc<RefCell<Self>> {
        let id = z_some.borrow().id;
        if let Some(z_some) = store.exhume_expression(&id) {
            z_some
        } else {
            let new = Rc::new(RefCell::new(Self::ZSome(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::StructExpression
    pub fn new_struct_expression(
        struct_expression: &Rc<RefCell<StructExpression>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = struct_expression.borrow().id;
        if let Some(struct_expression) = store.exhume_expression(&id) {
            struct_expression
        } else {
            let new = Rc::new(RefCell::new(Self::StructExpression(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::TypeCast
    pub fn new_type_cast(
        type_cast: &Rc<RefCell<TypeCast>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = type_cast.borrow().id;
        if let Some(type_cast) = store.exhume_expression(&id) {
            type_cast
        } else {
            let new = Rc::new(RefCell::new(Self::TypeCast(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::VariableExpression
    pub fn new_variable_expression(
        variable_expression: &Rc<RefCell<VariableExpression>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = variable_expression.borrow().id;
        if let Some(variable_expression) = store.exhume_expression(&id) {
            variable_expression
        } else {
            let new = Rc::new(RefCell::new(Self::VariableExpression(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Expression::Block(id) => *id,
            Expression::Call(id) => *id,
            Expression::Debugger(id) => *id,
            Expression::ErrorExpression(id) => *id,
            Expression::FieldAccess(id) => *id,
            Expression::FieldExpression(id) => *id,
            Expression::ForLoop(id) => *id,
            Expression::Grouped(id) => *id,
            Expression::XIf(id) => *id,
            Expression::Index(id) => *id,
            Expression::ListElement(id) => *id,
            Expression::ListExpression(id) => *id,
            Expression::Literal(id) => *id,
            Expression::ZNone(id) => *id,
            Expression::Operator(id) => *id,
            Expression::Print(id) => *id,
            Expression::RangeExpression(id) => *id,
            Expression::XReturn(id) => *id,
            Expression::ZSome(id) => *id,
            Expression::StructExpression(id) => *id,
            Expression::TypeCast(id) => *id,
            Expression::VariableExpression(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-argument"}}}
    /// Navigate to [`Argument`] across R37(1-M)
    pub fn r37_argument<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Argument>>> {
        span!("r37_argument");
        store
            .iter_argument()
            .filter(|argument| argument.borrow().expression == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-call"}}}
    /// Navigate to [`Call`] across R29(1-Mc)
    pub fn r29_call<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Call>>> {
        span!("r29_call");
        store
            .iter_call()
            .filter(|call| call.borrow().expression == Some(self.id()))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-expression_statement"}}}
    /// Navigate to [`ExpressionStatement`] across R31(1-M)
    pub fn r31_expression_statement<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<ExpressionStatement>>> {
        span!("r31_expression_statement");
        store
            .iter_expression_statement()
            .filter(|expression_statement| expression_statement.borrow().expression == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-field_access"}}}
    /// Navigate to [`FieldAccess`] across R27(1-M)
    pub fn r27_field_access<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<FieldAccess>>> {
        span!("r27_field_access");
        store
            .iter_field_access()
            .filter(|field_access| field_access.borrow().expression == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-field_expression"}}}
    /// Navigate to [`FieldExpression`] across R38(1-M)
    pub fn r38_field_expression<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<FieldExpression>>> {
        span!("r38_field_expression");
        store
            .iter_field_expression()
            .filter(|field_expression| field_expression.borrow().expression == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-for_loop"}}}
    /// Navigate to [`ForLoop`] across R42(1-M)
    pub fn r42_for_loop<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<ForLoop>>> {
        span!("r42_for_loop");
        store
            .iter_for_loop()
            .filter(|for_loop| for_loop.borrow().expression == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-grouped"}}}
    /// Navigate to [`Grouped`] across R61(1-M)
    pub fn r61_grouped<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Grouped>>> {
        span!("r61_grouped");
        store
            .iter_grouped()
            .filter(|grouped| grouped.borrow().expression == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_if"}}}
    /// Navigate to [`XIf`] across R44(1-M)
    pub fn r44_x_if<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<XIf>>> {
        span!("r44_x_if");
        store
            .iter_x_if()
            .filter(|x_if| x_if.borrow().test == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-index"}}}
    /// Navigate to [`Index`] across R56(1-M)
    pub fn r56_index<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Index>>> {
        span!("r56_index");
        store
            .iter_index()
            .filter(|index| index.borrow().index == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-index"}}}
    /// Navigate to [`Index`] across R57(1-M)
    pub fn r57_index<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Index>>> {
        span!("r57_index");
        store
            .iter_index()
            .filter(|index| index.borrow().target == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-cond-to-let_statement"}}}
    /// Navigate to [`LetStatement`] across R20(1-1c)
    pub fn r20c_let_statement<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<LetStatement>>> {
        span!("r20_let_statement");
        let let_statement = store
            .iter_let_statement()
            .find(|let_statement| let_statement.borrow().expression == self.id());
        match let_statement {
            Some(ref let_statement) => vec![let_statement.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-list_element"}}}
    /// Navigate to [`ListElement`] across R55(1-M)
    pub fn r55_list_element<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<ListElement>>> {
        span!("r55_list_element");
        store
            .iter_list_element()
            .filter(|list_element| list_element.borrow().expression == self.id())
            // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
            // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-negation"}}}
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
    /// Navigate to [`Operator`] across R50(1-M)
    pub fn r50_operator<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Operator>>> {
        span!("r50_operator");
        store
            .iter_operator()
            .filter(|operator| operator.borrow().lhs == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-operator"}}}
    /// Navigate to [`Operator`] across R51(1-Mc)
    pub fn r51_operator<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Operator>>> {
        span!("r51_operator");
        store
            .iter_operator()
            // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
            // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
            .filter(|operator| operator.borrow().rhs == Some(self.id()))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-print"}}}
    /// Navigate to [`Print`] across R32(1-M)
    pub fn r32_print<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Print>>> {
        span!("r32_print");
        store
            .iter_print()
            .filter(|print| print.borrow().expression == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-range_expression"}}}
    /// Navigate to [`RangeExpression`] across R59(1-Mc)
    pub fn r59_range_expression<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<RangeExpression>>> {
        span!("r59_range_expression");
        store
            .iter_range_expression()
            .filter(|range_expression| range_expression.borrow().rhs == Some(self.id()))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-range_expression"}}}
    /// Navigate to [`RangeExpression`] across R58(1-Mc)
    pub fn r58_range_expression<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<RangeExpression>>> {
        span!("r58_range_expression");
        store
            .iter_range_expression()
            .filter(|range_expression| range_expression.borrow().lhs == Some(self.id()))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-result_statement"}}}
    /// Navigate to [`ResultStatement`] across R41(1-M)
    pub fn r41_result_statement<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<ResultStatement>>> {
        span!("r41_result_statement");
        store
            .iter_result_statement()
            .filter(|result_statement| result_statement.borrow().expression == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_return"}}}
    /// Navigate to [`XReturn`] across R45(1-M)
    pub fn r45_x_return<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<XReturn>>> {
        span!("r45_x_return");
        store
            .iter_x_return()
            .filter(|x_return| x_return.borrow().expression == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-type_cast"}}}
    /// Navigate to [`TypeCast`] across R68(1-M)
    pub fn r68_type_cast<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<TypeCast>>> {
        span!("r68_type_cast");
        store
            .iter_type_cast()
            .filter(|type_cast| type_cast.borrow().lhs == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-impl-nav-subtype-to-supertype-x_value"}}}
    // Navigate to [`XValue`] across R11(isa)
    pub fn r11_x_value<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<XValue>>> {
        span!("r11_x_value");
        vec![store
            .iter_x_value()
            .find(|x_value| {
                if let XValueEnum::Expression(id) = x_value.borrow().subtype {
                    id == self.id()
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
