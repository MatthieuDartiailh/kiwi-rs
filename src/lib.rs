#[macro_use]
extern crate impl_ops;
mod assoc_vec;
mod constraint;
mod errors;
mod expression;
mod row;
mod solver;
mod strength;
mod symbol;
mod symbolics;
mod term;
mod util;
mod variable;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
