// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::argument::Argument;
use crate::v2::lu_dog_vec::types::block::Block;
use crate::v2::lu_dog_vec::types::call::Call;
use crate::v2::lu_dog_vec::types::debugger::DEBUGGER;
use crate::v2::lu_dog_vec::types::error_expression::ErrorExpression;
use crate::v2::lu_dog_vec::types::expression_statement::ExpressionStatement;
use crate::v2::lu_dog_vec::types::field_access::FieldAccess;
use crate::v2::lu_dog_vec::types::field_expression::FieldExpression;
use crate::v2::lu_dog_vec::types::for_loop::ForLoop;
use crate::v2::lu_dog_vec::types::grouped::Grouped;
use crate::v2::lu_dog_vec::types::index::Index;
use crate::v2::lu_dog_vec::types::lambda::Lambda;
use crate::v2::lu_dog_vec::types::let_statement::LetStatement;
use crate::v2::lu_dog_vec::types::list_element::ListElement;
use crate::v2::lu_dog_vec::types::list_expression::ListExpression;
use crate::v2::lu_dog_vec::types::literal::Literal;
use crate::v2::lu_dog_vec::types::operator::Operator;
use crate::v2::lu_dog_vec::types::print::Print;
use crate::v2::lu_dog_vec::types::range_expression::RangeExpression;
use crate::v2::lu_dog_vec::types::result_statement::ResultStatement;
use crate::v2::lu_dog_vec::types::struct_expression::StructExpression;
use crate::v2::lu_dog_vec::types::type_cast::TypeCast;
use crate::v2::lu_dog_vec::types::variable_expression::VariableExpression;
use crate::v2::lu_dog_vec::types::x_if::XIf;
use crate::v2::lu_dog_vec::types::x_return::XReturn;
use crate::v2::lu_dog_vec::types::x_value::XValue;
use crate::v2::lu_dog_vec::types::x_value::XValueEnum;
use crate::v2::lu_dog_vec::types::z_none::Z_NONE;
use crate::v2::lu_dog_vec::types::z_some::ZSome;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-hybrid-documentation"}}}
/// An Expression
///
/// Expressions are calculations that render values.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Expression {
    pub subtype: ExpressionEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ExpressionEnum {
    Block(usize),
    Call(usize),
    Debugger(Uuid),
    ErrorExpression(usize),
    FieldAccess(usize),
    FieldExpression(usize),
    ForLoop(usize),
    Grouped(usize),
    XIf(usize),
    Index(usize),
    Lambda(usize),
    ListElement(usize),
    ListExpression(usize),
    Literal(usize),
    ZNone(Uuid),
    Operator(usize),
    Print(usize),
    RangeExpression(usize),
    XReturn(usize),
    ZSome(usize),
    StructExpression(usize),
    TypeCast(usize),
    VariableExpression(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-implementation"}}}
impl Expression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_block"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_block(
        subtype: &Rc<RefCell<Block>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::Block(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_call"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_call(
        subtype: &Rc<RefCell<Call>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::Call(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_debugger"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_debugger(store: &mut LuDogVecStore) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::Debugger(DEBUGGER),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_error_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_error_expression(
        subtype: &Rc<RefCell<ErrorExpression>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::ErrorExpression(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_field_access"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_field_access(
        subtype: &Rc<RefCell<FieldAccess>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::FieldAccess(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_field_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_field_expression(
        subtype: &Rc<RefCell<FieldExpression>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::FieldExpression(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_for_loop"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_for_loop(
        subtype: &Rc<RefCell<ForLoop>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::ForLoop(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_grouped"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_grouped(
        subtype: &Rc<RefCell<Grouped>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::Grouped(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_if"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_x_if(
        subtype: &Rc<RefCell<XIf>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::XIf(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_index"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_index(
        subtype: &Rc<RefCell<Index>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::Index(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_lambda"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_lambda(
        subtype: &Rc<RefCell<Lambda>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::Lambda(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_list_element"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_list_element(
        subtype: &Rc<RefCell<ListElement>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::ListElement(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_list_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_list_expression(
        subtype: &Rc<RefCell<ListExpression>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::ListExpression(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_literal"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_literal(
        subtype: &Rc<RefCell<Literal>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::Literal(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_z_none"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_z_none(store: &mut LuDogVecStore) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::ZNone(Z_NONE),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_operator"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_operator(
        subtype: &Rc<RefCell<Operator>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::Operator(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_print"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_print(
        subtype: &Rc<RefCell<Print>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::Print(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_range_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_range_expression(
        subtype: &Rc<RefCell<RangeExpression>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::RangeExpression(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_return"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_x_return(
        subtype: &Rc<RefCell<XReturn>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::XReturn(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_z_some"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_z_some(
        subtype: &Rc<RefCell<ZSome>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::ZSome(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_struct_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_struct_expression(
        subtype: &Rc<RefCell<StructExpression>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::StructExpression(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_type_cast"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_type_cast(
        subtype: &Rc<RefCell<TypeCast>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::TypeCast(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_variable_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_variable_expression(
        subtype: &Rc<RefCell<VariableExpression>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: ExpressionEnum::VariableExpression(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-argument"}}}
    /// Navigate to [`Argument`] across R37(1-M)
    pub fn r37_argument<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Argument>>> {
        span!("r37_argument");
        store
            .iter_argument()
            .filter(|argument| argument.borrow().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-call"}}}
    /// Navigate to [`Call`] across R29(1-Mc)
    pub fn r29_call<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Call>>> {
        span!("r29_call");
        store
            .iter_call()
            .filter(|call| call.borrow().expression == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-expression_statement"}}}
    /// Navigate to [`ExpressionStatement`] across R31(1-M)
    pub fn r31_expression_statement<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<ExpressionStatement>>> {
        span!("r31_expression_statement");
        store
            .iter_expression_statement()
            .filter(|expression_statement| expression_statement.borrow().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-field_access"}}}
    /// Navigate to [`FieldAccess`] across R27(1-M)
    pub fn r27_field_access<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<FieldAccess>>> {
        span!("r27_field_access");
        store
            .iter_field_access()
            .filter(|field_access| field_access.borrow().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-field_expression"}}}
    /// Navigate to [`FieldExpression`] across R38(1-M)
    pub fn r38_field_expression<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<FieldExpression>>> {
        span!("r38_field_expression");
        store
            .iter_field_expression()
            .filter(|field_expression| field_expression.borrow().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-for_loop"}}}
    /// Navigate to [`ForLoop`] across R42(1-M)
    pub fn r42_for_loop<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<ForLoop>>> {
        span!("r42_for_loop");
        store
            .iter_for_loop()
            .filter(|for_loop| for_loop.borrow().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-grouped"}}}
    /// Navigate to [`Grouped`] across R61(1-M)
    pub fn r61_grouped<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Grouped>>> {
        span!("r61_grouped");
        store
            .iter_grouped()
            .filter(|grouped| grouped.borrow().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_if"}}}
    /// Navigate to [`XIf`] across R44(1-M)
    pub fn r44_x_if<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<XIf>>> {
        span!("r44_x_if");
        store
            .iter_x_if()
            .filter(|x_if| x_if.borrow().test == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-index"}}}
    /// Navigate to [`Index`] across R56(1-M)
    pub fn r56_index<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Index>>> {
        span!("r56_index");
        store
            .iter_index()
            .filter(|index| index.borrow().index == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-index"}}}
    /// Navigate to [`Index`] across R57(1-M)
    pub fn r57_index<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Index>>> {
        span!("r57_index");
        store
            .iter_index()
            .filter(|index| index.borrow().target == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-cond-to-let_statement"}}}
    /// Navigate to [`LetStatement`] across R20(1-1c)
    pub fn r20c_let_statement<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<LetStatement>>> {
        span!("r20_let_statement");
        let let_statement = store
            .iter_let_statement()
            .find(|let_statement| let_statement.borrow().expression == self.id);
        match let_statement {
            Some(ref let_statement) => vec![let_statement.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-list_element"}}}
    /// Navigate to [`ListElement`] across R55(1-M)
    pub fn r55_list_element<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<ListElement>>> {
        span!("r55_list_element");
        store
            .iter_list_element()
            .filter(|list_element| list_element.borrow().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-operator"}}}
    /// Navigate to [`Operator`] across R51(1-Mc)
    pub fn r51_operator<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Operator>>> {
        span!("r51_operator");
        store
            .iter_operator()
            .filter(|operator| operator.borrow().rhs == Some(self.id))
            // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
            // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
    /// Navigate to [`Operator`] across R50(1-M)
    pub fn r50_operator<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Operator>>> {
        span!("r50_operator");
        store
            .iter_operator()
            .filter(|operator| operator.borrow().lhs == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-print"}}}
    /// Navigate to [`Print`] across R32(1-M)
    pub fn r32_print<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Print>>> {
        span!("r32_print");
        store
            .iter_print()
            .filter(|print| print.borrow().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-range_expression"}}}
    /// Navigate to [`RangeExpression`] across R59(1-Mc)
    pub fn r59_range_expression<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<RangeExpression>>> {
        span!("r59_range_expression");
        store
            .iter_range_expression()
            .filter(|range_expression| range_expression.borrow().rhs == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-range_expression"}}}
    /// Navigate to [`RangeExpression`] across R58(1-Mc)
    pub fn r58_range_expression<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<RangeExpression>>> {
        span!("r58_range_expression");
        store
            .iter_range_expression()
            .filter(|range_expression| range_expression.borrow().lhs == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-result_statement"}}}
    /// Navigate to [`ResultStatement`] across R41(1-M)
    pub fn r41_result_statement<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<ResultStatement>>> {
        span!("r41_result_statement");
        store
            .iter_result_statement()
            .filter(|result_statement| result_statement.borrow().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_return"}}}
    /// Navigate to [`XReturn`] across R45(1-M)
    pub fn r45_x_return<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<XReturn>>> {
        span!("r45_x_return");
        store
            .iter_x_return()
            .filter(|x_return| x_return.borrow().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-type_cast"}}}
    /// Navigate to [`TypeCast`] across R68(1-M)
    pub fn r68_type_cast<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<TypeCast>>> {
        span!("r68_type_cast");
        store
            .iter_type_cast()
            .filter(|type_cast| type_cast.borrow().lhs == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-impl-nav-subtype-to-supertype-x_value"}}}
    // Navigate to [`XValue`] across R11(isa)
    pub fn r11_x_value<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<XValue>>> {
        span!("r11_x_value");
        vec![store
            .iter_x_value()
            .find(|x_value| {
                if let XValueEnum::Expression(id) = x_value.borrow().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
impl Expression {
    pub fn r15_variable_expression<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<VariableExpression>>> {
        span!("r15_variable_expression");
        if let ExpressionEnum::VariableExpression(id) = self.subtype {
            vec![store.exhume_variable_expression(&id).unwrap()]
        } else {
            vec![]
        }
    }
}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
