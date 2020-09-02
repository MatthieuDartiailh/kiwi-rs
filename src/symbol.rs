//!
//!

use std::cmp;

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Invalid,
    External,
    Slack,
    Error,
    Dummy,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    m_id: u64,
    m_type: Type,
}

impl Symbol {
    ///
    pub fn new(id: u64, t: Type) -> Symbol {
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
    pub fn r#type(&self) -> &Type {
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
