//!
//!
//!

use super::expression::Expression;
use super::strength::clip;
use super::term::Term;
use std::cell::Ref;
use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationalOperator {
    LessEqual,
    Equal,
    GreaterEqual,
}

///
fn reduce(expr: Expression) -> Expression {
    // FIXME use a cheaper hash algorithm than the default
    // A custom hasher simply inspecting the pointer of the VariableData should be
    // quite efficient
    // Using an AssocVec would also work
    let mut map = HashMap::new();
    for t in expr.terms().iter() {
        *map.entry(t.variable()).or_insert(0.0) += t.coefficient()
    }
    let new_terms = map
        .drain()
        .map(|pair| Term::new(pair.0.clone(), pair.1))
        .collect();
    Expression::new(new_terms, expr.constant())
}

///
#[derive(Debug)]
struct ConstraintData {
    m_expression: Expression,
    m_strength: f64,
    m_op: RelationalOperator,
}

impl ConstraintData {
    ///
    fn new(expr: Expression, op: RelationalOperator, strength: f64) -> ConstraintData {
        let reduced_expr = reduce(expr);
        let clipped = clip(strength);
        ConstraintData {
            m_expression: reduced_expr,
            m_op: op,
            m_strength: clipped,
        }
    }

    ///
    fn from(constraint: &Constraint, strength: f64) -> ConstraintData {
        let data = constraint.m_data.borrow();
        let clipped = clip(strength);
        ConstraintData {
            m_expression: data.m_expression.clone(),
            m_op: data.m_op,
            m_strength: clipped,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Constraint {
    m_data: Rc<RefCell<ConstraintData>>,
}

impl Constraint {
    pub fn new(expr: Expression, op: RelationalOperator, strength: f64) -> Constraint {
        let data = ConstraintData::new(expr, op, strength);
        Constraint {
            m_data: Rc::new(RefCell::new(data)),
        }
    }

    ///
    pub fn from(constraint: &Constraint, strength: f64) -> Constraint {
        let data = ConstraintData::from(constraint, strength);
        Constraint {
            m_data: Rc::new(RefCell::new(data)),
        }
    }

    pub fn expression(&self) -> Ref<Expression> {
        Ref::map(self.m_data.borrow(), |borrow| &borrow.m_expression)
    }

    pub fn op(&self) -> Ref<RelationalOperator> {
        Ref::map(self.m_data.borrow(), |borrow| &borrow.m_op)
    }

    pub fn strength(&self) -> Ref<f64> {
        Ref::map(self.m_data.borrow(), |borrow| &borrow.m_strength)
    }
}

impl cmp::PartialEq for Constraint {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.m_data, &other.m_data)
    }
}

impl cmp::Eq for Constraint {}

impl cmp::Ord for Constraint {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.m_data.as_ptr().cmp(&other.m_data.as_ptr())
    }
}

impl cmp::PartialOrd for Constraint {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{} {} | strength = {}",
            self.expression(),
            match *self.op() {
                RelationalOperator::Equal => "== 0",
                RelationalOperator::GreaterEqual => ">= 0",
                RelationalOperator::LessEqual => "<= 0",
            },
            self.strength(),
        ))
    }
}

#[cfg(test)]
mod tests {}
