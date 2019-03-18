use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use crate::obo14::ast::Line;
use crate::obo14::ast::ClassId;
use super::TermClause;

/// A term frame, describing a class.
pub struct TermFrame {
    id: Line<ClassId>,
    clauses: Vec<Line<TermClause>>,
}

impl Display for TermFrame {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("[Term]\nid: ").and(self.id.fmt(f))?;
        self.clauses.iter().try_for_each(|clause| clause.fmt(f))
    }

}
