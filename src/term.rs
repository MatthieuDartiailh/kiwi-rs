//!
//!

use super::variable::Variable;

#[derive(Debug, Clone)]
pub struct Term {
    m_variable: Variable,
    m_coefficient: f64,
}

impl Term {
    ///
    pub fn new(variable: Variable, coefficient: f64) -> Term {
        Term {
            m_variable: variable,
            m_coefficient: coefficient,
        }
    }

    ///
    pub fn variable(&self) -> &Variable {
        &self.m_variable
    }

    ///
    pub fn coefficient(&self) -> f64 {
        self.m_coefficient
    }

    ///
    pub fn value(&self) -> f64 {
        self.m_coefficient * *self.m_variable.value()
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
}
