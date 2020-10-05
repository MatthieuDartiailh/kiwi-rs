use crate::term::Term;
use std::fmt;

/// An expression represent a sum of term plus a constant.
#[derive(Debug, Clone)]
pub struct Expression {
    m_terms: Vec<Term>,
    m_constant: f64,
}

impl Expression {
    /// Create an expression from a vector of terms and a floating point constant.
    pub fn new(terms: Vec<Term>, constant: f64) -> Expression {
        Expression {
            m_terms: terms,
            m_constant: constant,
        }
    }

    /// Access the terms in the expression.
    pub fn terms(&self) -> &Vec<Term> {
        &self.m_terms
    }

    /// Mutable access the terms in the expression.
    pub fn terms_mut(&mut self) -> &mut Vec<Term> {
        &mut self.m_terms
    }

    /// Access the expression constant.
    pub fn constant(&self) -> f64 {
        self.m_constant
    }

    /// Compute the expression value.
    pub fn value(&self) -> f64 {
        let mut value = self.m_constant;
        for t in self.m_terms.iter() {
            value += t.value();
        }
        value
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{}{}",
            self.m_terms.iter().fold(String::new(), |mut acc, term| {
                acc.push_str(&format!("{} + ", term));
                acc
            }),
            self.m_constant
        ))
    }
}

#[cfg(test)]
mod tests {

    use super::Expression;
    use crate::term::Term;
    use crate::variable::Variable;

    #[test]
    fn test_expression() {
        let v = Variable::new_with_name("test");
        v.set_value(1.0);
        let ts = vec![Term::new(v, 2.0)];
        let e = Expression::new(ts.clone(), 1.0);
        for (t1, t2) in e.terms().iter().zip(ts) {
            assert_eq!(*t1.variable(), *t2.variable());
            assert_eq!(t1.coefficient(), t2.coefficient());
        }
        assert_eq!(e.constant().floor(), 1.0);
        assert_eq!(e.value().floor(), 3.0);
    }

    #[test]
    fn test_mutate_expression() {
        let v1 = Variable::new_with_name("test");
        v1.set_value(1.0);
        let v2 = Variable::new_with_name("test2");
        v2.set_value(1.0);
        let t1 = Term::new(v1, 2.0);
        let t2 = Term::new(v2, 4.0);
        let mut e = Expression::new(vec![t1], 1.0);
        assert_eq!(e.value().floor(), 3.0);
        {
            let ts = e.terms_mut();
            ts.insert(0, t2);
        }
        assert_eq!(e.value().floor(), 7.0);
    }

    #[test]
    fn test_display() {
        let t1 = Term::new(Variable::new_with_name("test"), 2.0);
        let t2 = Term::new(Variable::new_with_name("test2"), 4.0);
        let e = Expression::new(vec![t1, t2], 1.0);
        assert_eq!(format!("{}", e), "2 * test + 4 * test2 + 1")
    }
}
