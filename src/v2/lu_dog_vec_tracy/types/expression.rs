// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::a_wait::AWait;
use crate::v2::lu_dog_vec_tracy::types::argument::Argument;
use crate::v2::lu_dog_vec_tracy::types::block::Block;
use crate::v2::lu_dog_vec_tracy::types::call::Call;
use crate::v2::lu_dog_vec_tracy::types::empty_expression::EMPTY_EXPRESSION;
use crate::v2::lu_dog_vec_tracy::types::expression_statement::ExpressionStatement;
use crate::v2::lu_dog_vec_tracy::types::field_access::FieldAccess;
use crate::v2::lu_dog_vec_tracy::types::field_expression::FieldExpression;
use crate::v2::lu_dog_vec_tracy::types::for_loop::ForLoop;
use crate::v2::lu_dog_vec_tracy::types::grouped::Grouped;
use crate::v2::lu_dog_vec_tracy::types::index::Index;
use crate::v2::lu_dog_vec_tracy::types::lambda::Lambda;
use crate::v2::lu_dog_vec_tracy::types::let_statement::LetStatement;
use crate::v2::lu_dog_vec_tracy::types::list_element::ListElement;
use crate::v2::lu_dog_vec_tracy::types::list_expression::ListExpression;
use crate::v2::lu_dog_vec_tracy::types::literal::Literal;
use crate::v2::lu_dog_vec_tracy::types::operator::Operator;
use crate::v2::lu_dog_vec_tracy::types::pattern::Pattern;
use crate::v2::lu_dog_vec_tracy::types::range_expression::RangeExpression;
use crate::v2::lu_dog_vec_tracy::types::result_statement::ResultStatement;
use crate::v2::lu_dog_vec_tracy::types::struct_expression::StructExpression;
use crate::v2::lu_dog_vec_tracy::types::type_cast::TypeCast;
use crate::v2::lu_dog_vec_tracy::types::variable_expression::VariableExpression;
use crate::v2::lu_dog_vec_tracy::types::x_debugger::X_DEBUGGER;
use crate::v2::lu_dog_vec_tracy::types::x_if::XIf;
use crate::v2::lu_dog_vec_tracy::types::x_match::XMatch;
use crate::v2::lu_dog_vec_tracy::types::x_path::XPath;
use crate::v2::lu_dog_vec_tracy::types::x_print::XPrint;
use crate::v2::lu_dog_vec_tracy::types::x_return::XReturn;
use crate::v2::lu_dog_vec_tracy::types::x_value::XValue;
use crate::v2::lu_dog_vec_tracy::types::x_value::XValueEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
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
        subtype: &Rc<RefCell<AWait>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::AWait(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_block"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_block(
        bogus: bool,
        subtype: &Rc<RefCell<Block>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::Block(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_call"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_call(
        bogus: bool,
        subtype: &Rc<RefCell<Call>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::Call(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_debugger"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_debugger"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_x_debugger(bogus: bool, store: &mut LuDogVecTracyStore) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
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
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
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
        subtype: &Rc<RefCell<FieldAccess>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::FieldAccess(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_field_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_field_expression(
        bogus: bool,
        subtype: &Rc<RefCell<FieldExpression>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::FieldExpression(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_for_loop"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_for_loop(
        bogus: bool,
        subtype: &Rc<RefCell<ForLoop>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::ForLoop(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_grouped"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_grouped(
        bogus: bool,
        subtype: &Rc<RefCell<Grouped>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::Grouped(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_if"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_x_if(
        bogus: bool,
        subtype: &Rc<RefCell<XIf>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::XIf(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_index"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_index(
        bogus: bool,
        subtype: &Rc<RefCell<Index>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::Index(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_lambda"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_lambda(
        bogus: bool,
        subtype: &Rc<RefCell<Lambda>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::Lambda(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_list_element"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_list_element(
        bogus: bool,
        subtype: &Rc<RefCell<ListElement>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::ListElement(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_list_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_list_expression(
        bogus: bool,
        subtype: &Rc<RefCell<ListExpression>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::ListExpression(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_literal"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_literal(
        bogus: bool,
        subtype: &Rc<RefCell<Literal>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::Literal(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_match"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_x_match(
        bogus: bool,
        subtype: &Rc<RefCell<XMatch>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::XMatch(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_operator"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_operator(
        bogus: bool,
        subtype: &Rc<RefCell<Operator>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::Operator(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_path"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_x_path(
        bogus: bool,
        subtype: &Rc<RefCell<XPath>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::XPath(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_print"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_x_print(
        bogus: bool,
        subtype: &Rc<RefCell<XPrint>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::XPrint(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_range_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_range_expression(
        bogus: bool,
        subtype: &Rc<RefCell<RangeExpression>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::RangeExpression(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_return"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_x_return(
        bogus: bool,
        subtype: &Rc<RefCell<XReturn>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::XReturn(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_struct_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_struct_expression(
        bogus: bool,
        subtype: &Rc<RefCell<StructExpression>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::StructExpression(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_type_cast"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_type_cast(
        bogus: bool,
        subtype: &Rc<RefCell<TypeCast>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::TypeCast(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_variable_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_variable_expression(
        bogus: bool,
        subtype: &Rc<RefCell<VariableExpression>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Expression>> {
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                bogus: bogus,
                subtype: ExpressionEnum::VariableExpression(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-argument"}}}
    /// Navigate to [`Argument`] across R37(1-M)
    pub fn r37_argument<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Argument>>> {
        span!("r37_argument");
        store
            .iter_argument()
            .filter(|argument| argument.borrow().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-cond-to-a_wait"}}}
    /// Navigate to [`AWait`] across R98(1-1c)
    pub fn r98c_a_wait<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<AWait>>> {
        span!("r98_a_wait");
        let a_wait = store
            .iter_a_wait()
            .find(|a_wait| a_wait.borrow().x_future == self.id);
        match a_wait {
            Some(ref a_wait) => vec![a_wait.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-call"}}}
    /// Navigate to [`Call`] across R29(1-Mc)
    pub fn r29_call<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Call>>> {
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
        store: &'a LuDogVecTracyStore,
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
        store: &'a LuDogVecTracyStore,
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
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<FieldExpression>>> {
        span!("r38_field_expression");
        store
            .iter_field_expression()
            .filter(|field_expression| field_expression.borrow().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-for_loop"}}}
    /// Navigate to [`ForLoop`] across R43(1-M)
    pub fn r43_for_loop<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<ForLoop>>> {
        span!("r43_for_loop");
        store
            .iter_for_loop()
            .filter(|for_loop| for_loop.borrow().block == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-for_loop"}}}
    /// Navigate to [`ForLoop`] across R42(1-M)
    pub fn r42_for_loop<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<ForLoop>>> {
        span!("r42_for_loop");
        store
            .iter_for_loop()
            .filter(|for_loop| for_loop.borrow().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-grouped"}}}
    /// Navigate to [`Grouped`] across R61(1-M)
    pub fn r61_grouped<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Grouped>>> {
        span!("r61_grouped");
        store
            .iter_grouped()
            .filter(|grouped| grouped.borrow().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_if"}}}
    /// Navigate to [`XIf`] across R44(1-M)
    pub fn r44_x_if<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<XIf>>> {
        span!("r44_x_if");
        store
            .iter_x_if()
            .filter(|x_if| x_if.borrow().test == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-x_if"}}}
    /// Navigate to [`XIf`] across R52(1-Mc)
    pub fn r52_x_if<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<XIf>>> {
        span!("r52_x_if");
        store
            .iter_x_if()
            .filter(|x_if| x_if.borrow().false_block == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-index"}}}
    /// Navigate to [`Index`] across R56(1-M)
    pub fn r56_index<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Index>>> {
        span!("r56_index");
        store
            .iter_index()
            .filter(|index| index.borrow().index == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-index"}}}
    /// Navigate to [`Index`] across R57(1-M)
    pub fn r57_index<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Index>>> {
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
        store: &'a LuDogVecTracyStore,
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
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<ListElement>>> {
        span!("r55_list_element");
        store
            .iter_list_element()
            .filter(|list_element| list_element.borrow().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_match"}}}
    /// Navigate to [`XMatch`] across R91(1-M)
    pub fn r91_x_match<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<XMatch>>> {
        span!("r91_x_match");
        store
            .iter_x_match()
            .filter(|x_match| x_match.borrow().scrutinee == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-operator"}}}
    /// Navigate to [`Operator`] across R51(1-Mc)
    pub fn r51_operator<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Operator>>> {
        span!("r51_operator");
        store
            .iter_operator()
            .filter(|operator| operator.borrow().rhs == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
    /// Navigate to [`Operator`] across R50(1-M)
    pub fn r50_operator<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Operator>>> {
        span!("r50_operator");
        store
            .iter_operator()
            .filter(|operator| operator.borrow().lhs == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-pattern"}}}
    /// Navigate to [`Pattern`] across R92(1-M)
    pub fn r92_pattern<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Pattern>>> {
        span!("r92_pattern");
        store
            .iter_pattern()
            .filter(|pattern| pattern.borrow().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_print"}}}
    /// Navigate to [`XPrint`] across R32(1-M)
    pub fn r32_x_print<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<XPrint>>> {
        span!("r32_x_print");
        store
            .iter_x_print()
            .filter(|x_print| x_print.borrow().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-range_expression"}}}
    /// Navigate to [`RangeExpression`] across R58(1-Mc)
    pub fn r58_range_expression<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<RangeExpression>>> {
        span!("r58_range_expression");
        store
            .iter_range_expression()
            .filter(|range_expression| range_expression.borrow().lhs == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-range_expression"}}}
    /// Navigate to [`RangeExpression`] across R59(1-Mc)
    pub fn r59_range_expression<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<RangeExpression>>> {
        span!("r59_range_expression");
        store
            .iter_range_expression()
            .filter(|range_expression| range_expression.borrow().rhs == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-result_statement"}}}
    /// Navigate to [`ResultStatement`] across R41(1-M)
    pub fn r41_result_statement<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
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
    pub fn r45_x_return<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<XReturn>>> {
        span!("r45_x_return");
        store
            .iter_x_return()
            .filter(|x_return| x_return.borrow().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-type_cast"}}}
    /// Navigate to [`TypeCast`] across R68(1-M)
    pub fn r68_type_cast<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<TypeCast>>> {
        span!("r68_type_cast");
        store
            .iter_type_cast()
            .filter(|type_cast| type_cast.borrow().lhs == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-assoc-many-to-pattern"}}}
    /// Navigate to [`Pattern`] across R87(1-M)
    pub fn r87_pattern<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Pattern>>> {
        span!("r87_pattern");
        store
            .iter_pattern()
            .filter(|pattern| pattern.borrow().match_expr == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-impl-nav-subtype-to-supertype-x_value"}}}
    // Navigate to [`XValue`] across R11(isa)
    pub fn r11_x_value<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<XValue>>> {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-implementation"}}}
impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.bogus == other.bogus
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
