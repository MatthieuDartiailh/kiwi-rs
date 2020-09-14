//! Symbols are used to represent in a compact and efficient manner the state of teh solver.

use std::cmp;

// We use an enum wrapped in a struct since we need to compare Symbol of different kind

/// Kind of symbol that can exist in the solver.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    /// Invalid symbol are used as place holder when a symbol cannot be found.
    Invalid,
    /// External symbol ("v"): correspond to a user created variable.
    External,
    /// Slack symbol ("s"): used to represent inequalities
    Slack,
    /// Error symbol ("e"): used to represent non-required constraints
    Error,
    /// Dummy symbol ("d"): always zero, used to keep track of the impact of an
    /// external variable in the tableau.
    Dummy,
}

/// Symbol used to represent the state of the solver
///
/// # Note
///
/// Since solving the system requires a large number of manipulation of the symbols
/// the operations have to compile down to an efficient representation. In Kiwi, symbols
/// compile down to u64 meaning that a vector of them fits in a CPU cache line.
///
#[derive(Debug, Clone)]
pub struct Symbol {
    m_id: u64,
    m_type: SymbolKind,
}

impl Symbol {
    ///
    pub fn new(t: SymbolKind, id: u64) -> Symbol {
        Symbol {
            m_id: id,
            m_type: t,
        }
    }

    ///
    pub fn id(&self) -> &u64 {
        &self.m_id
    }

    ///
    pub fn kind(&self) -> SymbolKind {
        self.m_type
    }
}

impl cmp::PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.m_id == other.m_id
    }
}

impl cmp::Eq for Symbol {}

impl cmp::Ord for Symbol {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.m_id.cmp(&other.m_id)
    }
}

impl cmp::PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {}
