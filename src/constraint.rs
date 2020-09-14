use crate::expression::Expression;
use crate::strength::clip;
use crate::term::Term;
use std::cell::Ref;
use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

/// The comparison operators that can be used in a constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationalOperator {
    LessEqual,
    Equal,
    GreaterEqual,
}

/// Helper function concatenating the terms of an expression involving the same constant.
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

/// Internal data associated with a constraint.
#[derive(Debug)]
struct ConstraintData {
    m_expression: Expression,
    m_strength: f64,
    m_op: RelationalOperator,
}

impl ConstraintData {
    /// Create a constraint data from an expression, a comparison and a strength.
    ///
    /// # Note
    /// - the Expression is first reduced to eliminate duplicate use of a variable
    /// - the strength is clipped to be below the strength::REQUIRED value.
    fn new(expr: Expression, op: RelationalOperator, strength: f64) -> ConstraintData {
        let reduced_expr = reduce(expr);
        let clipped = clip(strength);
        ConstraintData {
            m_expression: reduced_expr,
            m_op: op,
            m_strength: clipped,
        }
    }

    /// Create a new constraint data from an existing one and a strength.
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

/// Representation of a constraint in the solver.
#[derive(Debug, Clone)]
pub struct Constraint {
    m_data: Rc<RefCell<ConstraintData>>,
}

impl Constraint {
    /// Create a constraint data from an expression, a comparison and a strength.
    pub fn new(expr: Expression, op: RelationalOperator, strength: f64) -> Constraint {
        let data = ConstraintData::new(expr, op, strength);
        Constraint {
            m_data: Rc::new(RefCell::new(data)),
        }
    }

    /// Create a new constraint from an existing one and a strength.
    pub fn from(constraint: &Constraint, strength: f64) -> Constraint {
        let data = ConstraintData::from(constraint, strength);
        Constraint {
            m_data: Rc::new(RefCell::new(data)),
        }
    }

    /// Access the Expression used by the constraint.
    pub fn expression(&self) -> Ref<Expression> {
        Ref::map(self.m_data.borrow(), |borrow| &borrow.m_expression)
    }

    /// Access the comparison operator used in the constraint.
    pub fn op(&self) -> RelationalOperator {
        self.m_data.borrow().m_op
    }

    /// Access the strength of the conastraint.
    pub fn strength(&self) -> f64 {
        self.m_data.borrow().m_strength
    }
}

// Constraint are compared based on the data they point to.
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

// Constraint are ordered based on the pointer value of the data they refer to.
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
            match self.op() {
                RelationalOperator::Equal => String::from("== 0"),
                RelationalOperator::GreaterEqual => String::from(">= 0"),
                RelationalOperator::LessEqual => String::from("<= 0"),
            },
            self.strength(),
        ))
    }
}

#[cfg(test)]
mod tests {}
