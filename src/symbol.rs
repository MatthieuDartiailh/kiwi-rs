//!
//!

use std::cmp;

// XXX swap type for kind to avoid having to escape type everywhere

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    Invalid,
    External,
    Slack,
    Error,
    Dummy,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    m_id: u64,
    m_type: SymbolType,
}

impl Symbol {
    ///
    pub fn new(t: SymbolType, id: u64) -> Symbol {
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
    pub fn r#type(&self) -> &SymbolType {
        &self.m_type
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
