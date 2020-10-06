//!
//!
//!
use crate::assoc_vec::AssocVec;
use crate::symbol::Symbol;
use crate::util::near_zero;

// FIXME add comments and tests
#[derive(Clone)]
pub struct Row {
    m_constant: f64,
    m_cells: AssocVec<Symbol, f64>,
}

impl Row {
    pub fn new(constant: f64) -> Row {
        Row {
            m_constant: constant,
            m_cells: AssocVec::new(),
        }
    }

    pub fn from(other: &Row) -> Row {
        Row {
            m_constant: other.m_constant,
            m_cells: other.m_cells.clone(),
        }
    }

    pub fn cells(&self) -> &AssocVec<Symbol, f64> {
        &self.m_cells
    }

    pub fn constant(&self) -> &f64 {
        &self.m_constant
    }

    pub fn add(&mut self, value: f64) -> &f64 {
        self.m_constant += value;
        &self.m_constant
    }

    pub fn insert_symbol(&mut self, symbol: &Symbol, coefficient: f64) {
        // FIXME can be made more efficient by implementing Entry on AssocVec
        if let Some(c) = self.m_cells.get_mut(symbol) {
            *c += coefficient;
            if near_zero(*c) {
                self.m_cells.remove(symbol);
            }
        } else {
            self.m_cells.insert(symbol.clone(), coefficient);
        }
    }

    pub fn insert_row(&mut self, row: &Row, coefficient: f64) {
        self.m_constant += row.m_constant * coefficient;
        for (key, value) in row.m_cells.iter() {
            // FIXME can be made more efficient by implementing Entry on AssocVec
            let coeff = value * coefficient;
            if let Some(c) = self.m_cells.get_mut(key) {
                *c += coeff;
                if near_zero(*c) {
                    self.m_cells.remove(key);
                }
            } else {
                self.m_cells.insert(key.clone(), coeff);
            }
        }
    }

    pub fn remove(&mut self, symbol: &Symbol) {
        self.m_cells.remove(symbol);
    }

    pub fn reverse_sign(&mut self) {
        self.m_constant *= -1.0;
        for (symbol, coeff) in self.m_cells.iter_mut() {
            *coeff *= -1.0;
        }
    }

    pub fn solve_for(&mut self, symbol: &Symbol) {
        if let Some(target_coeff) = self.m_cells.get(symbol) {
            let coeff = -1.0 / (*target_coeff);
            self.m_cells.remove(symbol);
            self.m_constant *= coeff;
            for (key, value) in self.m_cells.iter_mut() {
                *value *= coeff;
            }
        }
        // FIXME add a nice else clause
    }

    /* Solve the row for the given symbols.

    This method assumes the row is of the form x = b * y + c and will
    solve the row such that y = x / b - c / b. The rhs symbol will be
    removed from the row, the lhs added, and the result divided by the
    negative inverse of the rhs coefficient.

    The lhs symbol *must not* exist in the row, and the rhs symbol
    *must* exist in the row.

    */
    pub fn solve_for_symbols(&mut self, lhs: &Symbol, rhs: &Symbol) {
        self.insert_symbol(lhs, -1.0);
        self.solve_for(rhs);
    }

    /* Get the coefficient for the given symbol.

    If the symbol does not exist in the row, zero will be returned.

    */
    pub fn coefficient_for(&self, symbol: &Symbol) -> f64 {
        match self.m_cells.get(symbol) {
            None => 0.0,
            Some(c) => *c,
        }
    }

    /* Substitute a symbol with the data from another row.

    Given a row of the form a * x + b and a substitution of the
    form x = 3 * y + c the row will be updated to reflect the
    expression 3 * a * y + a * c + b.

    If the symbol does not exist in the row, this is a no-op.

    */
    pub fn substitute(&mut self, symbol: &Symbol, row: &Row) {
        if let Some(c) = self.m_cells.remove(symbol) {
            self.insert_row(row, c)
        }
    }
}
