//! Errors that can be returned by the solver.

use crate::constraint::Constraint;
use crate::variable::Variable;
use std::error::Error;
use std::fmt;

///
///
#[derive(Debug)]
pub enum KiwiError {
    UnsatisfiableConstraint { constraint: Constraint },
    UnknownConstraint { constraint: Constraint },
    DuplicateConstraint { constraint: Constraint },
    UnknownEditVariable { variable: Variable },
    DuplicateEditVariable { variable: Variable },
    BadRequiredStrength,
    InternalSolverError { msg: String },
}
// Since the errors are well defined, small and do not need to be propagated a lot we use a simple
// enum

impl fmt::Display for KiwiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KiwiError::BadRequiredStrength => {
                write!(f, "A required strength cannot be used in this context.")
            }
            KiwiError::InternalSolverError { msg } => f.write_fmt(format_args!("{}", msg)),
            KiwiError::DuplicateEditVariable { variable } => f.write_fmt(format_args!(
                "The edit variable {} has already been added to the solver..",
                variable.name()
            )),
            KiwiError::UnknownEditVariable { variable } => f.write_fmt(format_args!(
                "The edit variable {} has not been added to the solver.",
                variable.name()
            )),
            KiwiError::DuplicateConstraint { constraint } => f.write_fmt(format_args!(
                "The constraint {} has already been added to the solver.",
                constraint
            )),
            KiwiError::UnknownConstraint { constraint } => f.write_fmt(format_args!(
                "The constraint {} has not been added to the solver.",
                constraint
            )),
            KiwiError::UnsatisfiableConstraint { constraint } => f.write_fmt(format_args!(
                "The constraint {} cannot be satisfied.",
                constraint
            )),
        }
    }
}

impl Error for KiwiError {}
