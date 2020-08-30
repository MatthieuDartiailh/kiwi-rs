//!
//!

use super::term::Term;

///
#[derive(Debug, Clone)]
pub struct Expression {
    m_terms: Vec<Term>,
    m_constant: f64,
}

impl Expression {
    ///
    pub fn new(terms: Vec<Term>, constant: f64) -> Expression {
        Expression {
            m_terms: terms,
            m_constant: constant,
        }
    }

    ///
    pub fn terms(&self) -> &Vec<Term> {
        &self.m_terms
    }

    ///
    pub fn terms_mut(&mut self) -> &mut Vec<Term> {
        &mut self.m_terms
    }

    ///
    pub fn constant(&self) -> f64 {
        self.m_constant
    }

    ///
    pub fn value(&self) -> f64 {
        let mut value = self.m_constant;
        for t in self.m_terms.iter() {
            value += t.value();
        }
        value
    }
}

#[cfg(test)]
mod tests {}
