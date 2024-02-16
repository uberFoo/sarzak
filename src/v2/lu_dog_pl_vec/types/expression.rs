// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-use-statements"}}}
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_pl_vec::types::a_wait::AWait;
use crate::v2::lu_dog_pl_vec::types::argument::Argument;
use crate::v2::lu_dog_pl_vec::types::block::Block;
use crate::v2::lu_dog_pl_vec::types::call::Call;
use crate::v2::lu_dog_pl_vec::types::empty_expression::EMPTY_EXPRESSION;
use crate::v2::lu_dog_pl_vec::types::expression_bit::ExpressionBit;
use crate::v2::lu_dog_pl_vec::types::expression_statement::ExpressionStatement;
use crate::v2::lu_dog_pl_vec::types::field_access::FieldAccess;
use crate::v2::lu_dog_pl_vec::types::field_expression::FieldExpression;
use crate::v2::lu_dog_pl_vec::types::for_loop::ForLoop;
use crate::v2::lu_dog_pl_vec::types::grouped::Grouped;
use crate::v2::lu_dog_pl_vec::types::index::Index;
use crate::v2::lu_dog_pl_vec::types::lambda::Lambda;
use crate::v2::lu_dog_pl_vec::types::let_statement::LetStatement;
use crate::v2::lu_dog_pl_vec::types::list_element::ListElement;
use crate::v2::lu_dog_pl_vec::types::list_expression::ListExpression;
use crate::v2::lu_dog_pl_vec::types::literal::Literal;
use crate::v2::lu_dog_pl_vec::types::operator::Operator;
use crate::v2::lu_dog_pl_vec::types::pattern::Pattern;
use crate::v2::lu_dog_pl_vec::types::range_expression::RangeExpression;
use crate::v2::lu_dog_pl_vec::types::result_statement::ResultStatement;
use crate::v2::lu_dog_pl_vec::types::struct_expression::StructExpression;
use crate::v2::lu_dog_pl_vec::types::type_cast::TypeCast;
use crate::v2::lu_dog_pl_vec::types::variable_expression::VariableExpression;
use crate::v2::lu_dog_pl_vec::types::x_debugger::X_DEBUGGER;
use crate::v2::lu_dog_pl_vec::types::x_if::XIf;
use crate::v2::lu_dog_pl_vec::types::x_match::XMatch;
use crate::v2::lu_dog_pl_vec::types::x_path::XPath;
use crate::v2::lu_dog_pl_vec::types::x_print::XPrint;
use crate::v2::lu_dog_pl_vec::types::x_return::XReturn;
use crate::v2::lu_dog_pl_vec::types::x_value::XValue;
use crate::v2::lu_dog_pl_vec::types::x_value::XValueEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_vec::store::ObjectStore as LuDogPlVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-hybrid-documentation"}}}
/// An Expression
///
/// Expressions are calculations that render values.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Expression {
    pub subtype: ExpressionEnum,
    pub bogus: bool,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ExpressionEnum {
    AWait(usize),
    Block(usize),
    Call(usize),
    XDebugger(Uuid),
    EmptyExpression(Uuid),
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
    XMatch(usize),
    Operator(usize),
    XPath(usize),
    XPrint(usize),
    RangeExpression(usize),
    XReturn(usize),
    StructExpression(usize),
    TypeCast(usize),
    VariableExpression(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-implementation"}}}
impl Expression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_a_wait"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_a_wait(
        bogus: bool,
        subtype: &Arc<RwLock<AWait>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::AWait(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_block"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_block(
        bogus: bool,
        subtype: &Arc<RwLock<Block>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::Block(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_call"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_call(
        bogus: bool,
        subtype: &Arc<RwLock<Call>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::Call(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_debugger"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_x_debugger(bogus: bool, store: &mut LuDogPlVecStore) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::XDebugger(X_DEBUGGER),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_empty_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_empty_expression(
        bogus: bool,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::EmptyExpression(EMPTY_EXPRESSION),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_field_access"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_field_access(
        bogus: bool,
        subtype: &Arc<RwLock<FieldAccess>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::FieldAccess(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_field_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_field_expression(
        bogus: bool,
        subtype: &Arc<RwLock<FieldExpression>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::FieldExpression(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_for_loop"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_for_loop(
        bogus: bool,
        subtype: &Arc<RwLock<ForLoop>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::ForLoop(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_grouped"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_grouped(
        bogus: bool,
        subtype: &Arc<RwLock<Grouped>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::Grouped(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_if"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_x_if(
        bogus: bool,
        subtype: &Arc<RwLock<XIf>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::XIf(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_index"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_index(
        bogus: bool,
        subtype: &Arc<RwLock<Index>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::Index(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_lambda"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_lambda(
        bogus: bool,
        subtype: &Arc<RwLock<Lambda>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::Lambda(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_list_element"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_list_element(
        bogus: bool,
        subtype: &Arc<RwLock<ListElement>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::ListElement(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_list_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_list_expression(
        bogus: bool,
        subtype: &Arc<RwLock<ListExpression>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::ListExpression(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_literal"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_literal(
        bogus: bool,
        subtype: &Arc<RwLock<Literal>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::Literal(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_match"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_x_match(
        bogus: bool,
        subtype: &Arc<RwLock<XMatch>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::XMatch(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_operator"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_operator(
        bogus: bool,
        subtype: &Arc<RwLock<Operator>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::Operator(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_path"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_x_path(
        bogus: bool,
        subtype: &Arc<RwLock<XPath>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::XPath(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_print"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_x_print(
        bogus: bool,
        subtype: &Arc<RwLock<XPrint>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::XPrint(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_range_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_range_expression(
        bogus: bool,
        subtype: &Arc<RwLock<RangeExpression>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::RangeExpression(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_return"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_x_return(
        bogus: bool,
        subtype: &Arc<RwLock<XReturn>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::XReturn(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_struct_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_struct_expression(
        bogus: bool,
        subtype: &Arc<RwLock<StructExpression>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::StructExpression(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_type_cast"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_type_cast(
        bogus: bool,
        subtype: &Arc<RwLock<TypeCast>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::TypeCast(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_variable_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_variable_expression(
        bogus: bool,
        subtype: &Arc<RwLock<VariableExpression>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Expression>> {
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::VariableExpression(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-argument"}}}
    /// Navigate to [`Argument`] across R37(1-M)
    pub fn r37_argument<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<Argument>>> {
        store
            .iter_argument()
            .filter(|argument| argument.read().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-cond-to-a_wait"}}}
    /// Navigate to [`AWait`] across R98(1-1c)
    pub fn r98c_a_wait<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<AWait>>> {
        let a_wait = store
            .iter_a_wait()
            .find(|a_wait| a_wait.read().x_future == self.id);
        match a_wait {
            Some(ref a_wait) => vec![a_wait.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-call"}}}
    /// Navigate to [`Call`] across R29(1-Mc)
    pub fn r29_call<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<Call>>> {
        store
            .iter_call()
            .filter(|call| call.read().expression == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-expression_bit"}}}
    /// Navigate to [`ExpressionBit`] across R109(1-M)
    pub fn r109_expression_bit<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<ExpressionBit>>> {
        store
            .iter_expression_bit()
            .filter(|expression_bit| expression_bit.read().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-expression_statement"}}}
    /// Navigate to [`ExpressionStatement`] across R31(1-M)
    pub fn r31_expression_statement<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<ExpressionStatement>>> {
        store
            .iter_expression_statement()
            .filter(|expression_statement| expression_statement.read().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-field_access"}}}
    /// Navigate to [`FieldAccess`] across R27(1-M)
    pub fn r27_field_access<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<FieldAccess>>> {
        store
            .iter_field_access()
            .filter(|field_access| field_access.read().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-field_expression"}}}
    /// Navigate to [`FieldExpression`] across R38(1-M)
    pub fn r38_field_expression<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<FieldExpression>>> {
        store
            .iter_field_expression()
            .filter(|field_expression| field_expression.read().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-for_loop"}}}
    /// Navigate to [`ForLoop`] across R43(1-M)
    pub fn r43_for_loop<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<ForLoop>>> {
        store
            .iter_for_loop()
            .filter(|for_loop| for_loop.read().block == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-for_loop"}}}
    /// Navigate to [`ForLoop`] across R42(1-M)
    pub fn r42_for_loop<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<ForLoop>>> {
        store
            .iter_for_loop()
            .filter(|for_loop| for_loop.read().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-grouped"}}}
    /// Navigate to [`Grouped`] across R61(1-M)
    pub fn r61_grouped<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<Grouped>>> {
        store
            .iter_grouped()
            .filter(|grouped| grouped.read().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_if"}}}
    /// Navigate to [`XIf`] across R44(1-M)
    pub fn r44_x_if<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<XIf>>> {
        store
            .iter_x_if()
            .filter(|x_if| x_if.read().test == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-x_if"}}}
    /// Navigate to [`XIf`] across R52(1-Mc)
    pub fn r52_x_if<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<XIf>>> {
        store
            .iter_x_if()
            .filter(|x_if| x_if.read().false_block == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-index"}}}
    /// Navigate to [`Index`] across R56(1-M)
    pub fn r56_index<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<Index>>> {
        store
            .iter_index()
            .filter(|index| index.read().index == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-index"}}}
    /// Navigate to [`Index`] across R57(1-M)
    pub fn r57_index<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<Index>>> {
        store
            .iter_index()
            .filter(|index| index.read().target == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-cond-to-let_statement"}}}
    /// Navigate to [`LetStatement`] across R20(1-1c)
    pub fn r20c_let_statement<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<LetStatement>>> {
        let let_statement = store
            .iter_let_statement()
            .find(|let_statement| let_statement.read().expression == self.id);
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
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<ListElement>>> {
        store
            .iter_list_element()
            .filter(|list_element| list_element.read().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_match"}}}
    /// Navigate to [`XMatch`] across R91(1-M)
    pub fn r91_x_match<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<XMatch>>> {
        store
            .iter_x_match()
            .filter(|x_match| x_match.read().scrutinee == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-operator"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
    /// Navigate to [`Operator`] across R51(1-Mc)
    pub fn r51_operator<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<Operator>>> {
        store
            .iter_operator()
            .filter(|operator| operator.read().rhs == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-operator"}}}
    /// Navigate to [`Operator`] across R50(1-M)
    pub fn r50_operator<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<Operator>>> {
        store
            .iter_operator()
            .filter(|operator| operator.read().lhs == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-pattern"}}}
    /// Navigate to [`Pattern`] across R92(1-M)
    pub fn r92_pattern<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<Pattern>>> {
        store
            .iter_pattern()
            .filter(|pattern| pattern.read().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_print"}}}
    /// Navigate to [`XPrint`] across R32(1-M)
    pub fn r32_x_print<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<XPrint>>> {
        store
            .iter_x_print()
            .filter(|x_print| x_print.read().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-range_expression"}}}
    /// Navigate to [`RangeExpression`] across R58(1-Mc)
    pub fn r58_range_expression<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<RangeExpression>>> {
        store
            .iter_range_expression()
            .filter(|range_expression| range_expression.read().lhs == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-range_expression"}}}
    /// Navigate to [`RangeExpression`] across R59(1-Mc)
    pub fn r59_range_expression<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<RangeExpression>>> {
        store
            .iter_range_expression()
            .filter(|range_expression| range_expression.read().rhs == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-result_statement"}}}
    /// Navigate to [`ResultStatement`] across R41(1-M)
    pub fn r41_result_statement<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<ResultStatement>>> {
        store
            .iter_result_statement()
            .filter(|result_statement| result_statement.read().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_return"}}}
    /// Navigate to [`XReturn`] across R45(1-M)
    pub fn r45_x_return<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<XReturn>>> {
        store
            .iter_x_return()
            .filter(|x_return| x_return.read().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-type_cast"}}}
    /// Navigate to [`TypeCast`] across R68(1-M)
    pub fn r68_type_cast<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<TypeCast>>> {
        store
            .iter_type_cast()
            .filter(|type_cast| type_cast.read().lhs == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-assoc-many-to-pattern"}}}
    /// Navigate to [`Pattern`] across R87(1-M)
    pub fn r87_pattern<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<Pattern>>> {
        store
            .iter_pattern()
            .filter(|pattern| pattern.read().match_expr == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-impl-nav-subtype-to-supertype-x_value"}}}
    // Navigate to [`XValue`] across R11(isa)
    pub fn r11_x_value<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<XValue>>> {
        vec![store
            .iter_x_value()
            .find(|x_value| {
                if let XValueEnum::Expression(id) = x_value.read().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-implementation"}}}
impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.bogus == other.bogus
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
