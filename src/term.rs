use crate::variable::Variable;
use std::fmt;

/// Term represent a variable with a multiplicative coefficient.
#[derive(Debug, Clone)]
pub struct Term {
    m_variable: Variable,
    m_coefficient: f64,
}

impl Term {
    /// Create a term from a Variable and a floating point coefficient.
    pub fn new(variable: Variable, coefficient: f64) -> Term {
        Term {
            m_variable: variable,
            m_coefficient: coefficient,
        }
    }

    /// Access the term variable.
    pub fn variable(&self) -> &Variable {
        &self.m_variable
    }

    /// Access the term coefficient.
    pub fn coefficient(&self) -> f64 {
        self.m_coefficient
    }

    /// Compute the product of the coefficient and variable value.
    pub fn value(&self) -> f64 {
        self.m_coefficient * *self.m_variable.value()
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{} * {}",
            self.m_coefficient,
            self.m_variable.name()
        ))
    }
}

#[cfg(test)]
mod tests {

    use super::Term;
    use crate::variable::Variable;

    #[test]
    fn test_term() {
        let v = Variable::new_with_name("test");
        let t = Term::new(v.clone(), 2.0);
        assert_eq!(*t.variable(), v);
        assert_eq!(t.coefficient().floor(), 2.0);
        assert_eq!(t.value().floor(), 0.0);
    }

    #[test]
    fn test_term_from_pair() {
        let v = Variable::new_with_name("test");
        let t = Term::new(v.clone(), 2.0);
        assert_eq!(*t.variable(), v);
        assert_eq!(t.coefficient().floor(), 2.0);
        assert_eq!(t.value().floor(), 0.0);
    }

    #[test]
    fn test_display() {
        let v = Variable::new_with_name("test");
        let t = Term::new(v, 2.0);
        assert_eq!(format!("{}", t), "2.0 * test")
    }
}
