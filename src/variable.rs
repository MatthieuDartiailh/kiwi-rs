use std::cell::{Ref, RefCell};
use std::cmp;
use std::hash;
use std::rc::Rc;
use std::string::String;

/// Internal data of a variable.
#[derive(Debug)]
struct VariableData {
    name: String,
    value: f64,
}

/// Variable used to define constraints in the solver.
#[derive(Clone, Debug)]
pub struct Variable {
    m_variable: Rc<RefCell<VariableData>>,
}

impl Variable {
    /// Create a new anonymous variable
    pub fn new() -> Variable {
        Variable {
            m_variable: Rc::new(RefCell::new(VariableData {
                name: String::from(""),
                value: 0.0,
            })),
        }
    }

    /// Create a new named variable
    pub fn new_with_name(name: &str) -> Variable {
        Variable {
            m_variable: Rc::new(RefCell::new(VariableData {
                name: String::from(name),
                value: 0.0,
            })),
        }
    }

    /// Access the name of the variable.
    pub fn name(&self) -> Ref<String> {
        Ref::map(self.m_variable.borrow(), |borrow| &borrow.name)
    }

    /// Set the name of the variable.
    pub fn set_name(&self, name: &str) -> String {
        let mut borrow = self.m_variable.borrow_mut();
        let old = String::from(&borrow.name);
        borrow.name = String::from(name);
        old
    }

    /// Access the current value of the variable.
    pub fn value(&self) -> Ref<f64> {
        Ref::map(self.m_variable.borrow(), |borrow| &borrow.value)
    }

    /// Set the value stored in teh variable.
    pub fn set_value(&self, value: f64) -> f64 {
        let mut borrow = self.m_variable.borrow_mut();
        let old = borrow.value;
        borrow.value = value;
        old
    }
}

impl cmp::PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.m_variable, &other.m_variable)
    }
}

impl cmp::Eq for Variable {}

impl cmp::Ord for Variable {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.m_variable.as_ptr().cmp(&other.m_variable.as_ptr())
    }
}

impl cmp::PartialOrd for Variable {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl hash::Hash for Variable {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.m_variable.as_ptr() as usize)
    }
}

#[cfg(test)]
mod tests {

    use super::Variable;

    #[test]
    fn test_variable_access() {
        let var = Variable::new();
        assert_eq!(*var.name(), "");
        assert_eq!(var.value().floor(), 0.0);
    }

    #[test]
    fn test_variable_setter() {
        let var = Variable::new();
        assert_eq!(var.set_name("test"), "");
        assert_eq!(*var.name(), "test");
        assert_eq!(var.set_value(1.0).floor(), 0.0);
        assert_eq!(var.value().floor(), 1.0);
    }

    #[test]
    fn test_constructor() {
        let var = Variable::new_with_name("test");
        assert_eq!(*var.name(), "test");
        let var2 = var.clone();
        var.set_name("test2");
        assert_eq!(*var.name(), "test2");
        assert_eq!(*var2.name(), "test2");
    }
}
