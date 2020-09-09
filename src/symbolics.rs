//!
//!
//!

// NOTE: cannot preserve the symbolic for constraints since PartialOrd, PartialEq return
// cannot be customized
use super::constraint::{Constraint, RelationalOperator};
use super::expression::Expression;
use super::strength::{MEDIUM, REQUIRED, STRONG, WEAK};
use super::term::Term;
use super::variable::Variable;
use std::iter;
use std::ops;

// XXX The following could be optimized to reduce cloning when we get owned value
// as input.

// Variable *, /, - operation

impl_op_ex_commutative!(*|lhs: &Variable, rhs: &f64| -> Term { Term::new(lhs.clone(), *rhs) });
impl_op_ex_commutative!(/ |lhs: &Variable, rhs: &f64| -> Term {lhs * 1.0/(*rhs) });
impl_op_ex!(-|lhs: &Variable| -> Term { lhs * -1.0 });

// Term *, /, - operation
impl_op_ex_commutative!(*|lhs: &Term, rhs: &f64| -> Term {
    Term::new(lhs.variable().clone(), lhs.coefficient() * rhs)
});
impl_op_ex_commutative!(/ |lhs: &Term, rhs: &f64| -> Term { lhs * (1.0/rhs) });
impl_op_ex!(-|lhs: &Term| -> Term { lhs * -1.0 });

// Expression *, /, - operation
impl_op_ex_commutative!(*|lhs: &Expression, rhs: &f64| -> Expression {
    Expression::new(
        lhs.terms()
            .iter()
            .map(|item| Term::new(item.variable().clone(), item.coefficient() * rhs))
            .collect(),
        lhs.constant() * rhs,
    )
});
impl_op_ex_commutative!(/ |lhs: &Expression, rhs: &f64| -> Expression { lhs * (1.0/rhs) });
impl_op_ex!(-|lhs: &Expression| -> Expression { lhs * -1.0 });

// Expression add and subtract
impl_op_ex!(+ |lhs: &Expression, rhs: &Expression| -> Expression {
    Expression::new(
        lhs.terms().iter().cloned().chain(
         rhs.terms().iter().cloned()).collect(),
         lhs.constant() + rhs.constant())});
impl_op_ex!(-|lhs: &Expression, rhs: &Expression| -> Expression { lhs + (-rhs) });

impl_op_ex_commutative!(+ |lhs: &Expression, rhs: &Term| -> Expression {
    Expression::new(
        lhs.terms().iter().cloned().chain(
         iter::once(rhs.clone())).collect(),
         lhs.constant())});
impl_op_ex!(-|lhs: &Expression, rhs: &Term| -> Expression { lhs + (-rhs) });
impl_op_ex!(-|lhs: &Term, rhs: &Expression| -> Expression { lhs + (-rhs) });

impl_op_ex_commutative!(+ |lhs: &Expression, rhs: &Variable| -> Expression {
    Expression::new(
        lhs.terms().iter().cloned().chain(
         iter::once(Term::new( rhs.clone(), 1.0))).collect(),
         lhs.constant())});
impl_op_ex!(-|lhs: &Expression, rhs: &Variable| -> Expression { lhs + (-rhs) });
impl_op_ex!(-|lhs: &Variable, rhs: &Expression| -> Expression { lhs + (-rhs) });

impl_op_ex_commutative!(+ |lhs: &Expression, rhs: &f64| -> Expression {
    Expression::new(
        lhs.terms().clone(),
         lhs.constant() + rhs)});
impl_op_ex!(-|lhs: &Expression, rhs: &f64| -> Expression { lhs + (-rhs) });
impl_op_ex!(-|lhs: &f64, rhs: &Expression| -> Expression { lhs + (-rhs) });

// Term add and subtract
impl_op_ex!(+ |lhs: &Term, rhs: &Term| -> Expression {
    Expression::new(
        vec!(lhs.clone(), rhs.clone()),
        0.0)});
impl_op_ex!(-|lhs: &Term, rhs: &Term| -> Expression { lhs + (-rhs) });

impl_op_ex_commutative!(+ |lhs: &Term, rhs: &Variable| -> Expression {
    Expression::new(
        vec!(lhs.clone(), Term::new(rhs.clone(), 1.0)),
        0.0)});
impl_op_ex!(-|lhs: &Term, rhs: &Variable| -> Expression { lhs + (-rhs) });
impl_op_ex!(-|lhs: &Variable, rhs: &Term| -> Expression { lhs + (-rhs) });

impl_op_ex_commutative!(+ |lhs: &Term, rhs: &f64| -> Expression {
    Expression::new(
        vec!(lhs.clone()),
        *rhs)});
impl_op_ex!(-|lhs: &Term, rhs: &f64| -> Expression { lhs + (-rhs) });
impl_op_ex!(-|lhs: &f64, rhs: &Term| -> Expression { lhs + (-rhs) });

// Variable add and subtract
impl_op_ex!(+ |lhs: &Variable, rhs: &Variable| -> Expression {
    Expression::new(
        vec!(Term::new(lhs.clone(), 1.0), Term::new(rhs.clone(), 1.0)),
        0.0)});
impl_op_ex!(-|lhs: &Variable, rhs: &Variable| -> Expression { lhs + (-rhs) });

impl_op_ex_commutative!(+ |lhs: &Variable, rhs: &f64| -> Expression {
    Expression::new(
        vec!(Term::new(lhs.clone(), 1.0)),
        *rhs)});
impl_op_ex!(-|lhs: &Variable, rhs: &f64| -> Expression { lhs + (-rhs) });
impl_op_ex!(-|lhs: &f64, rhs: &Variable| -> Expression { lhs + (-rhs) });

// Constraints creation

trait IntoConstraint: Sized {
    ///
    fn into_expr(self) -> Expression;

    fn eq(self, strength: f64) -> Constraint {
        Constraint::new(self.into_expr(), RelationalOperator::Equal, strength)
    }

    ///
    fn weak_eq(self) -> Constraint {
        Constraint::new(self.into_expr(), RelationalOperator::Equal, WEAK)
    }

    ///
    fn medium_eq(self) -> Constraint {
        Constraint::new(self.into_expr(), RelationalOperator::Equal, MEDIUM)
    }

    ///
    fn strong_eq(self) -> Constraint {
        Constraint::new(self.into_expr(), RelationalOperator::Equal, STRONG)
    }

    ///
    fn required_eq(self) -> Constraint {
        Constraint::new(self.into_expr(), RelationalOperator::Equal, REQUIRED)
    }

    ///
    fn le(self, strength: f64) -> Constraint {
        Constraint::new(self.into_expr(), RelationalOperator::LessEqual, strength)
    }

    ///
    fn weak_le(self) -> Constraint {
        Constraint::new(self.into_expr(), RelationalOperator::LessEqual, WEAK)
    }

    ///
    fn medium_le(self) -> Constraint {
        Constraint::new(self.into_expr(), RelationalOperator::LessEqual, MEDIUM)
    }

    ///
    fn strong_le(self) -> Constraint {
        Constraint::new(self.into_expr(), RelationalOperator::LessEqual, STRONG)
    }

    ///
    fn required_le(self) -> Constraint {
        Constraint::new(self.into_expr(), RelationalOperator::LessEqual, REQUIRED)
    }

    ///
    fn ge(self, strength: f64) -> Constraint {
        Constraint::new(self.into_expr(), RelationalOperator::GreaterEqual, strength)
    }

    ///
    fn weak_ge(self) -> Constraint {
        Constraint::new(self.into_expr(), RelationalOperator::GreaterEqual, WEAK)
    }

    ///
    fn medium_ge(self) -> Constraint {
        Constraint::new(self.into_expr(), RelationalOperator::GreaterEqual, MEDIUM)
    }

    ///
    fn strong_ge(self) -> Constraint {
        Constraint::new(self.into_expr(), RelationalOperator::GreaterEqual, STRONG)
    }

    ///
    fn required_ge(self) -> Constraint {
        Constraint::new(self.into_expr(), RelationalOperator::GreaterEqual, REQUIRED)
    }
}

impl IntoConstraint for Expression {
    ///
    #[inline]
    fn into_expr(self) -> Expression {
        self
    }
}

impl IntoConstraint for Term {
    ///
    #[inline]
    fn into_expr(self) -> Expression {
        Expression::new(vec![self], 0.0)
    }
}

impl IntoConstraint for Variable {
    ///
    #[inline]
    fn into_expr(self) -> Expression {
        Expression::new(vec![Term::new(self, 1.0)], 0.0)
    }
}

// Constraint strength modifier
impl_op_ex_commutative!(| |lhs: &Constraint, rhs: &f64| -> Constraint { Constraint::from(lhs, *rhs) });
