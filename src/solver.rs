//!
//!
//!
//!
use crate::assoc_vec::AssocVec;
use crate::constraint::{Constraint, RelationalOperator};
use crate::errors::{ErrorType, KiwiError};
use crate::expression::Expression;
use crate::row::Row;
use crate::strength;
use crate::symbol::{Symbol, SymbolType};
use crate::term::Term;
use crate::util::near_zero;
use crate::variable::Variable;

///
#[derive(Clone)]
struct Tag {
    marker: Symbol,
    other: Option<Symbol>,
}

///
struct EditInfo {
    tag: Tag,
    constraint: Constraint,
    constant: f64,
}

// XXX Probably not the best way to approach this use case
// Using a function taking a closure as illustrated in
// https://users.rust-lang.org/t/how-to-do-raii-pattern-with-rust/36881/10
// may be cleaner (it is also less likely to cause borrow checker issue I think
// at least)
// struct DualOptimizeGuard {
//     solver: &mut Solver,
// }

struct Solver {
    m_cns: AssocVec<Constraint, Tag>,
    m_rows: AssocVec<Symbol, Row>,
    m_vars: AssocVec<Variable, Symbol>,
    m_edits: AssocVec<Variable, EditInfo>,
    m_infeasible_rows: Vec<Symbol>,
    m_objective: Row,
    m_artificial: Option<Row>,
    m_id_tick: u64,
}

type SolverResult = Result<(), KiwiError>;

impl Solver {
    // XXX Need to also implement Default
    fn new() -> Solver {
        Solver {
            m_cns: AssocVec::new(),
            m_rows: AssocVec::new(),
            m_vars: AssocVec::new(),
            m_edits: AssocVec::new(),
            m_infeasible_rows: Vec::new(),
            m_objective: Row::new(1.0),
            m_artificial: None,
            m_id_tick: 1,
        }
    }

    /// Add a constraint to the solver.
    pub fn add_constraint(&mut self, constraint: Constraint) -> Result<&Tag, KiwiError> {
        if self.m_cns.contains_key(&constraint) {
            return Err(KiwiError {
                err_type: ErrorType::DuplicateConstraint { constraint },
            });
        }

        // Creating a row causes symbols to be reserved for the variables
        // in the constraint. If this method exits with an exception,
        // then its possible those variables will linger in the var map.
        // Since its likely that those variables will be used in other
        // constraints and since exceptional conditions are uncommon,
        // i'm not too worried about aggressive cleanup of the var map.
        let (row, tag) = self.create_row(&constraint);
        let subject = self.choose_subject(&row, &tag);

        // If chooseSubject could not find a valid entering symbol, one
        // last option is available if the entire row is composed of
        // dummy variables. If the constant of the row is zero, then
        // this represents redundant constraints and the new dummy
        // marker can enter the basis. If the constant is non-zero,
        // then it represents an unsatisfiable constraint.
        if *subject.r#type() == SymbolType::Invalid && self.all_dummies(&row) {
            if !near_zero(*row.constant()) {
                return Err(KiwiError {
                    err_type: ErrorType::UnsatisfiableConstraint { constraint },
                });
            } else {
                let subject = &tag.marker;
            }
        }

        // If an entering symbol still isn't found, then the row must
        // be added using an artificial variable. If that fails, then
        // the row represents an unsatisfiable constraint.
        if *subject.r#type() == SymbolType::Invalid {
            if !self.add_with_artificial_variable(&row) {
                return Err(KiwiError {
                    err_type: ErrorType::UnsatisfiableConstraint { constraint },
                });
            }
        } else {
            row.solve_for(subject);
            self.substitute(subject, &row);
            self.m_rows.insert(subject.clone(), row);
        }

        self.m_cns.insert(constraint, tag);

        // Optimizing after each constraint is added performs less
        // aggregate work due to a smaller average system size. It
        // also ensures the solver remains in a consistent state.
        self.optimize(&self.m_objective)?;
        Ok(&tag)
    }

    /// Remove a constraint from the solver.
    ///
    pub fn remove_constraint(&mut self, constraint: &Constraint) -> SolverResult {
        let tag: Tag;
        match self.m_cns.remove(constraint) {
            Some(t) => tag = t,
            None => {
                return Err(KiwiError {
                    err_type: ErrorType::UnknownConstraint {
                        constraint: constraint.clone(),
                    },
                });
            }
        }

        // Remove the error effects from the objective function
        // *before* pivoting, or substitutions into the objective
        // will lead to incorrect solver results.
        self.remove_constraint_effects(constraint, &tag);

        // If the marker is basic, simply drop the row. Otherwise,
        // pivot the marker into the basis and then drop the row.
        if let Some(row) = self.m_rows.remove(&tag.marker) {
        } else {
            match self.get_marker_leaving_row(&tag.marker) {
                Some((leaving_symbol, leaving_row)) => {
                    leaving_row.solve_for_symbols(&leaving_symbol, &tag.marker);
                    self.substitute(&tag.marker, &leaving_row);
                }
                None => {
                    return Err(KiwiError {
                        err_type: ErrorType::InternalSolverError {
                            msg: String::from("failed to find leaving row"),
                        },
                    })
                }
            }
        }

        // Optimizing after each constraint is removed ensures that the
        // solver remains consistent. It makes the solver api easier to
        // use at a small tradeoff for speed.
        self.optimize(&self.m_objective)
    }

    /// Test whether a constraint has been added to the solver.
    ///
    pub fn has_constraint(&self, constraint: &Constraint) -> bool {
        self.m_cns.contains_key(constraint)
    }

    /* Add an edit variable to the solver.

    This method should be called before the `suggestValue` method is
    used to supply a suggested value for the given edit variable.

    Throws
    ------
    DuplicateEditVariable
        The given edit variable has already been added to the solver.

    BadRequiredStrength
        The given strength is >= required.

    */
    pub fn add_edit_variable(&mut self, variable: &Variable, strength: f64) -> SolverResult {
        if self.m_edits.contains_key(variable) {
            return Err(KiwiError {
                err_type: ErrorType::DuplicateEditVariable {
                    variable: variable.clone(),
                },
            });
        }
        let c_strength = strength::clip(strength);
        if c_strength == strength::REQUIRED {
            return Err(KiwiError {
                err_type: ErrorType::BadRequiredStrength {},
            });
        }
        let cn = Constraint::new(
            Expression::new(vec![Term::new(variable.clone(), 1.0)], 0.0),
            RelationalOperator::Equal,
            c_strength,
        );

        // Add the constraint and get the associated tag
        let tag = self.add_constraint(cn.clone())?;
        self.m_edits.insert(
            variable.clone(),
            EditInfo {
                tag: tag.clone(),
                constraint: cn,
                constant: 0.0,
            },
        );
        Ok(())
    }

    // /* Remove an edit variable from the solver.

    // Throws
    // ------
    // UnknownEditVariable
    // 	The given edit variable has not been added to the solver.

    // */
    pub fn remove_edit_variable(&mut self, variable: &Variable) -> SolverResult {
        if let Some(edit_info) = self.m_edits.get(variable) {
            self.remove_constraint(&edit_info.constraint)?;
            self.m_edits.remove(variable);
            Ok(())
        } else {
            Err(KiwiError {
                err_type: ErrorType::UnknownEditVariable {
                    variable: variable.clone(),
                },
            })
        }
    }

    // /* Test whether an edit variable has been added to the solver.

    // */
    pub fn has_edit_variable(&self, variable: &Variable) -> bool {
        self.m_edits.contains_key(variable)
    }

    /* Suggest a value for the given edit variable.

    This method should be used after an edit variable as been added to
    the solver in order to suggest the value for that variable.

    Throws
    ------
    UnknownEditVariable
        The given edit variable has not been added to the solver.

    */
    // void suggestValue( const Variable& variable, double value )
    // {
    // 	EditMap::iterator it = m_edits.find( variable );
    // 	if( it == m_edits.end() )
    // 		throw UnknownEditVariable( variable );

    // 	DualOptimizeGuard guard( *this );
    // 	EditInfo& info = it->second;
    // 	double delta = value - info.constant;
    // 	info.constant = value;

    // 	// Check first if the positive error variable is basic.
    // 	RowMap::iterator row_it = m_rows.find( info.tag.marker );
    // 	if( row_it != m_rows.end() )
    // 	{
    // 		if( row_it->second->add( -delta ) < 0.0 )
    // 			m_infeasible_rows.push_back( row_it->first );
    // 		return;
    // 	}

    // 	// Check next if the negative error variable is basic.
    // 	row_it = m_rows.find( info.tag.other );
    // 	if( row_it != m_rows.end() )
    // 	{
    // 		if( row_it->second->add( delta ) < 0.0 )
    // 			m_infeasible_rows.push_back( row_it->first );
    // 		return;
    // 	}

    // 	// Otherwise update each row where the error variables exist.
    // 	RowMap::iterator end = m_rows.end();
    // 	for( row_it = m_rows.begin(); row_it != end; ++row_it )
    // 	{
    // 		double coeff = row_it->second->coefficientFor( info.tag.marker );
    // 		if( coeff != 0.0 &&
    // 			row_it->second->add( delta * coeff ) < 0.0 &&
    // 			row_it->first.type() != Symbol::External )
    // 			m_infeasible_rows.push_back( row_it->first );
    // 	}
    // }

    // /* Update the values of the external solver variables.

    // */
    // void updateVariables()
    // {
    // 	typedef RowMap::iterator row_iter_t;
    // 	typedef VarMap::iterator var_iter_t;
    // 	row_iter_t row_end = m_rows.end();
    // 	var_iter_t var_end = m_vars.end();
    // 	for( var_iter_t var_it = m_vars.begin(); var_it != var_end; ++var_it )
    // 	{
    // 		Variable& var( const_cast<Variable&>( var_it->first ) );
    // 		row_iter_t row_it = m_rows.find( var_it->second );
    // 		if( row_it == row_end )
    // 			var.setValue( 0.0 );
    // 		else
    // 			var.setValue( row_it->second->constant() );
    // 	}
    // }

    // /* Reset the solver to the empty starting condition.

    // This method resets the internal solver state to the empty starting
    // condition, as if no constraints or edit variables have been added.
    // This can be faster than deleting the solver and creating a new one
    // when the entire system must change, since it can avoid unecessary
    // heap (de)allocations.

    // */
    // void reset()
    // {
    // 	clearRows();
    // 	m_cns.clear();
    // 	m_vars.clear();
    // 	m_edits.clear();
    // 	m_infeasible_rows.clear();
    // 	m_objective.reset( new Row() );
    // 	m_artificial.reset();
    // 	m_id_tick = 1;
    // }

    // Private functions

    /* Create a new Row object for the given constraint.

    The terms in the constraint will be converted to cells in the row.
    Any term in the constraint with a coefficient of zero is ignored.
    This method uses the `getVarSymbol` method to get the symbol for
    the variables added to the row. If the symbol for a given cell
    variable is basic, the cell variable will be substituted with the
    basic row.

    The necessary slack and error variables will be added to the row.
    If the constant for the row is negative, the sign for the row
    will be inverted so the constant becomes positive.

    The tag will be updated with the marker and error symbols to use
    for tracking the movement of the constraint in the tableau.

    */
    fn create_row(&mut self, constraint: &Constraint) -> (Row, Tag) {
        let expr = constraint.expression();
        let mut row = Row::new(expr.constant());
        let mut tag: Tag;
        tag.other = None;

        // Substitute the current basic variables into the row.
        for term in expr.terms().iter() {
            if !near_zero(term.coefficient()) {
                let symbol = self.get_var_symbol(term.variable());
                match self.m_rows.get(symbol) {
                    Some(existing_row) => row.insert_row(existing_row, term.coefficient()),
                    None => row.insert_symbol(symbol, term.coefficient()),
                }
            }
        }

        type RO = RelationalOperator;

        // Add the necessary slack, error, and dummy variables.
        let op = *constraint.op();
        let c_strength = *constraint.strength();
        match op {
            RO::GreaterEqual | RO::LessEqual => {
                let coeff = if op == RO::LessEqual { 1.0 } else { -1.0 };
                let slack = Symbol::new(SymbolType::Slack, self.next_symbol_id());
                row.insert_symbol(&slack, coeff);
                tag.marker = slack;
                if c_strength < strength::REQUIRED {
                    let error = Symbol::new(SymbolType::Error, self.next_symbol_id());
                    row.insert_symbol(&error, -coeff);
                    self.m_objective.insert_symbol(&error, c_strength);
                    tag.other = Some(error);
                }
            }
            RO::Equal => {
                if c_strength < strength::REQUIRED {
                    let errplus = Symbol::new(SymbolType::Error, self.next_symbol_id());
                    let errminus = Symbol::new(SymbolType::Error, self.next_symbol_id());
                    row.insert_symbol(&errplus, -1.0); // v = eplus - eminus
                    row.insert_symbol(&errminus.clone(), 1.0); // v - eplus + eminus = 0
                    self.m_objective.insert_symbol(&errplus, c_strength);
                    self.m_objective.insert_symbol(&errminus, c_strength);
                    tag.marker = errplus;
                    tag.other = Some(errminus);
                } else {
                    let dummy = Symbol::new(SymbolType::Dummy, self.next_symbol_id());
                    row.insert_symbol(&dummy, 1.0);
                    tag.marker = dummy;
                }
            }
        }

        // Ensure the row as a positive constant.
        if *row.constant() < 0.0 {
            row.reverse_sign();
        }

        (row, tag)
    }

    /* Choose the subject for solving for the row.

    This method will choose the best subject for using as the solve
    target for the row. An invalid symbol will be returned if there
    is no valid target.

    The symbols are chosen according to the following precedence:

    1) The first symbol representing an external variable.
    2) A negative slack or error tag variable.

    If a subject cannot be found, an invalid symbol will be returned.

    */
    fn choose_subject(&self, row: &Row, tag: &Tag) -> &Symbol {
        for (symbol, coeff) in row.cells().iter() {
            if *symbol.r#type() == SymbolType::External {
                return &symbol.clone();
            }
        }

        if *tag.marker.r#type() == SymbolType::Slack
            || *tag.marker.r#type() == SymbolType::Error && row.coefficient_for(&tag.marker) < 0.0
        {
            return &tag.marker.clone();
        }

        match tag.other {
            Some(symbol) => {
                if *symbol.r#type() == SymbolType::Slack
                    || *symbol.r#type() == SymbolType::Error && row.coefficient_for(&symbol) < 0.0
                {
                    return &symbol;
                }
            }
            None => (),
        }
        &Symbol::new(SymbolType::Invalid, 0)
    }

    /// Get the symbol for the given variable.
    ///
    /// If a symbol does not exist for the variable, one will be created.
    ///
    fn get_var_symbol(&mut self, variable: &Variable) -> &Symbol {
        match self.m_vars.get(variable) {
            Some(symbol) => symbol,
            None => {
                let symbol = Symbol::new(SymbolType::External, self.next_symbol_id());
                self.m_vars.insert(variable.clone(), symbol);
                &symbol
            }
        }
    }

    ///
    fn all_dummies(&self, row: &Row) -> bool {
        for (symbol, coeff) in row.cells().iter() {
            if *symbol.r#type() != SymbolType::Dummy {
                return false;
            }
        }
        true
    }

    ///
    fn add_with_artificial_variable(&mut self, row: &Row) -> bool {
        // Create and add the artificial variable to the tableau
        let art = Symbol::new(SymbolType::Slack, self.next_symbol_id());
        self.m_rows.insert(art.clone(), *row.clone());
        self.m_artificial = Some(*row.clone());

        // Optimize the artificial objective. This is successful
        // only if the artificial objective is optimized to zero.
        // Using unwrap here is safe since we just set the artificial row
        self.optimize(&self.m_artificial.unwrap());
        let success = near_zero(*self.m_artificial.unwrap().constant());
        self.m_artificial = None;

        // If the artificial variable is not basic, pivot the row so that
        // it becomes basic. If the row is constant, exit early.
        if let Some(art_row) = self.m_rows.get_mut(&art) {
            self.m_rows.remove(&art);
            if art_row.cells().is_empty() {
                return success;
            }
            let entering = self.any_pivotable_symbol(&art_row);
            if *entering.r#type() == SymbolType::Invalid {
                return false;
            } // unsatisfiable (will this ever happen?)
            art_row.solve_for_symbols(&art, &entering);
            self.substitute(&entering, art_row);
            self.m_rows.insert(entering, *art_row);
        }

        // Remove the artificial variable from the tableau.
        for (s, r) in self.m_rows.iter_mut() {
            r.remove(&art);
        }
        self.m_objective.remove(&art);
        success
    }

    ///
    fn remove_constraint_effects(&mut self, constraint: &Constraint, tag: &Tag) {
        if *tag.marker.r#type() == SymbolType::Error {
            self.remove_marker_effects(&tag.marker, *constraint.strength());
        }
        if let Some(symbol) = tag.other {
            if *symbol.r#type() == SymbolType::Error {
                self.remove_marker_effects(&symbol, *constraint.strength());
            }
        }
    }

    /* Remove the effects of an error marker on the objective function.

    */
    fn remove_marker_effects(&mut self, marker: &Symbol, strength: f64) {
        match self.m_rows.get(marker) {
            Some(row) => {
                self.m_objective.insert_row(row, -strength);
            }
            None => {
                self.m_objective.insert_symbol(marker, -strength);
            }
        }
    }

    /// Substitute the parametric symbol with the given row.
    ///
    /// This method will substitute all instances of the parametric symbol
    /// in the tableau and the objective function with the given row.
    ///
    fn substitute(&mut self, symbol: &Symbol, row: &Row) {
        for (s, r) in self.m_rows.iter_mut() {
            r.substitute(symbol, row);
            if *s.r#type() != SymbolType::External && *r.constant() < 0.0 {
                self.m_infeasible_rows.push(s.clone());
            }
        }
        self.m_objective.substitute(symbol, row);
        if let Some(art_row) = &mut self.m_artificial {
            art_row.substitute(symbol, row);
        }
    }

    fn optimize(&mut self, objective: &Row) -> SolverResult {
        loop {
            let entering = self.get_entering_symbol(objective);
            if *entering.r#type() == SymbolType::Invalid {
                return Ok(());
            }
            if let Some((leaving_symbol, leaving_row)) = self.get_leaving_row(&entering) {
                // pivot the entering symbol into the basis
                leaving_row.solve_for_symbols(&leaving_symbol, &entering);
                self.substitute(&entering, &leaving_row);
                self.m_rows.insert(entering.clone(), leaving_row);
            } else {
                return Err(KiwiError {
                    err_type: ErrorType::InternalSolverError {
                        msg: String::from("The objective is unbounded."),
                    },
                });
            }
        }
    }

    /// Compute the entering variable for a pivot operation.

    /// This method will return first symbol in the objective function which
    /// is non-dummy and has a coefficient less than zero. If no symbol meets
    /// the criteria, it means the objective function is at a minimum, and an
    /// invalid symbol is returned.
    ///
    fn get_entering_symbol(&self, objective: &Row) -> Symbol {
        for (s, c) in objective.cells().iter() {
            if *s.r#type() != SymbolType::Dummy && *c < 0.0 {
                return s.clone();
            }
        }
        Symbol::new(SymbolType::Invalid, 0)
    }

    /// Compute the row which holds the exit symbol for a pivot.
    ///
    /// This method will the exit symbol and the row containing it. If no
    /// appropriate exit symbol is found, None will be returned. This indicates
    /// that the objective function is unbounded.
    ///
    /// The leaving row is removed from the row map.
    ///
    fn get_leaving_row(&mut self, entering: &Symbol) -> Option<(Symbol, Row)> {
        let mut ratio = f64::MAX;
        let found: Option<Symbol> = None;
        for (s, r) in self.m_rows.iter() {
            if *s.r#type() != SymbolType::External {
                let temp = r.coefficient_for(entering);
                if temp < 0.0 {
                    let temp_ratio = -(*r.constant()) / temp;
                    if temp_ratio < ratio {
                        ratio = temp_ratio;
                        found = Some(s.clone());
                    }
                }
            }
        }
        if let Some(symbol) = found {
            // Unwrapping is safe since we know the symbol exist in the map.
            Some((symbol, self.m_rows.remove(&symbol).unwrap()))
        } else {
            None
        }
    }

    ///
    /* Compute the leaving row for a marker variable.

    This method will return an iterator to the row in the row map
    which holds the given marker variable. The row will be chosen
    according to the following precedence:

    1) The row with a restricted basic varible and a negative coefficient
       for the marker with the smallest ratio of -constant / coefficient.

    2) The row with a restricted basic variable and the smallest ratio
       of constant / coefficient.

    3) The last unrestricted row which contains the marker.

    If the marker does not exist in any row, the row map end() iterator
    will be returned. This indicates an internal solver error since
    the marker *should* exist somewhere in the tableau.

    */
    fn get_marker_leaving_row(&mut self, marker: &Symbol) -> Option<(Symbol, Row)> {
        let r1 = f64::MAX;
        let r2 = f64::MAX;
        let first: Option<&Symbol> = None;
        let second: Option<&Symbol> = None;
        let third: Option<&Symbol> = None;
        for (symbol, row) in self.m_rows.iter() {
            let c = row.coefficient_for(marker);
            if c == 0.0 {
                continue;
            }
            if *symbol.r#type() == SymbolType::External {
                third = Some(symbol);
            } else if c < 0.0 {
                let r = -(*row.constant()) / c;
                if r < r1 {
                    r1 = r;
                    first = Some(symbol);
                }
            } else {
                let r = (*row.constant()) / c;
                if r < r2 {
                    r2 = r;
                    second = Some(symbol);
                }
            }
        }

        // If we have a symbol it exists in the mapping so unwraping is safe
        // The following cannot currently use || to reduce the redundancy
        if let Some(leaving) = first {
            return Some((leaving.clone(), self.m_rows.remove(leaving).unwrap()));
        }
        if let Some(leaving) = second {
            return Some((leaving.clone(), self.m_rows.remove(leaving).unwrap()));
        }
        if let Some(leaving) = third {
            return Some((leaving.clone(), self.m_rows.remove(leaving).unwrap()));
        }
        None
    }

    /// Get the first Slack or Error symbol in the row.
    ///
    /// If no such symbol is present, and Invalid symbol will be returned.
    ///
    fn any_pivotable_symbol(&self, row: &Row) -> Symbol {
        for (s, coeff) in row.cells().iter() {
            if *s.r#type() == SymbolType::Slack || *s.r#type() == SymbolType::Error {
                return s.clone();
            }
        }
        Symbol::new(SymbolType::Invalid, 0)
    }

    ///
    #[inline]
    fn next_symbol_id(&mut self) -> u64 {
        self.m_id_tick += 1;
        self.m_id_tick
    }
}

impl Drop for Solver {
    fn drop(&mut self) {
        self.clear_rows()
    }
}