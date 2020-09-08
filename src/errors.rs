//!
//!

use crate::constraint::Constraint;
use crate::variable::Variable;
use std::error::Error;
use std::fmt;

///
///
#[derive(Debug)]
pub enum ErrorType {
    UnsatisfiableConstraint { constraint: Constraint },
    UnknownConstraint { constraint: Constraint },
    DuplicateConstraint { constraint: Constraint },
    UnknownEditVariable { variable: Variable },
    DuplicateEditVariable { variable: Variable },
    BadRequiredStrength,
    InternalSolverError { msg: String },
}

///
///
#[derive(Debug)]
pub struct KiwiError {
    err_type: ErrorType,
}

impl fmt::Display for KiwiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.err_type {
            ErrorType::BadRequiredStrength => {
                write!(f, "A required strength cannot be used in this context.")
            }
            ErrorType::InternalSolverError { msg } => f.write_fmt(format_args!("{}", msg)),
            ErrorType::DuplicateEditVariable { variable } => f.write_fmt(format_args!(
                "The edit variable {} has already been added to the solver..",
                variable.name()
            )),
            ErrorType::UnknownEditVariable { variable } => f.write_fmt(format_args!(
                "The edit variable {} has not been added to the solver.",
                variable.name()
            )),
            ErrorType::DuplicateConstraint { constraint } => f.write_fmt(format_args!(
                "The constraint {} has already been added to the solver.",
                constraint
            )),
            ErrorType::UnknownConstraint { constraint } => f.write_fmt(format_args!(
                "The constraint {} has not been added to the solver.",
                constraint
            )),
            ErrorType::UnsatisfiableConstraint { constraint } => f.write_fmt(format_args!(
                "The constraint {} cannot be satisfied.",
                constraint
            )),
        }
    }
}

impl Error for KiwiError {}
