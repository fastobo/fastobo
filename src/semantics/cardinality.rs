use crate::error::CardinalityError;

/// The cardinality constraint for a given clause type.
pub enum Cardinality {
    ZeroOrOne,
    One,
    NotOne,
    Any,
}

///
pub trait CardinalityBound {
    fn cardinality(&self) -> Cardinality;
}

// impl CardinalityBound for crate::ast::TermClause {
//     fn cardinality(&self) -> ClauseCardinality {
//         use self::ClauseCardinality::*;
//         match self {
//             IsAnonymous(_) => ZeroOrOne,
//             Name(_) => ZeroOrOne,
//             Namespace(_) => One,
//             AltId(_) => Any,
//             Def(_, _) => ZeroOrOne,
//             Comment(_) => ZeroOrOne,
//             Subset(_) => Any,
//             Synonym(_) => Any,
//             Xref(_) => Any,
//             Builtin(_) => ZeroOrOne,
//             PropertyValue(_) => ZeroOrOne,
//             IsA(_) => Any,
//             IntersectionOf(_, _) => Any,
//             UnionOf(_) => Any,
//             EquivalentTo(_) => Any,
//             DisjointFrom(_) => Any,
//             Relationship(_, _) => Any,
//             CreatedBy(_) => ZeroOrOne,
//             CreationDate(_) => ZeroOrOne,
//             IsObsolete(_) => ZeroOrOne,
//             ReplacedBy(_) => Any,
//             Consider(_) => Any,
//         }
//     }
// }

// impl CardinalityBound for crate::typedef::TypedefClause {
//
// }
//
// pub trait CardinalityCheck {
//     fn check_cardinality(&self) -> Result<(), CardinalityError>;
// }
