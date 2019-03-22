mod date;
mod header;
mod id;
mod instance;
mod line;
mod misc;
mod qualifier;
mod strings;
mod synonym;
mod term;
mod typedef;
mod xref;

use pest::iterators::Pair;

pub use self::date::*;
pub use self::header::*;
pub use self::id::*;
pub use self::instance::*;
pub use self::line::*;
pub use self::misc::*;
pub use self::qualifier::*;
pub use self::strings::*;
pub use self::synonym::*;
pub use self::term::*;
pub use self::typedef::*;
pub use self::xref::*;

use crate::error::Result;
use crate::obo14::parser::FromPair;
use crate::obo14::parser::Parser;
use crate::obo14::parser::Rule;

/// A complete OBO document in format version 1.4.
pub struct OboDoc {
    header: HeaderFrame,
    entities: Vec<EntityFrame>,
}

impl FromPair for OboDoc {
    const RULE: Rule = Rule::OboDoc;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();

        let mut entities = Vec::new();
        let header = HeaderFrame::from_pair_unchecked(inner.next().unwrap())?;

        let mut pair = inner.next().unwrap();
        while pair.as_rule() != Rule::EOI {
            entities.push(EntityFrame::from_pair_unchecked(pair)?);
            pair = inner.next().unwrap();
        }
        Ok(OboDoc { header, entities })
    }
}
impl_fromstr!(OboDoc);

/// An entity frame, either for a term, an instance, or a typedef.
pub enum EntityFrame {
    Term(TermFrame),
    Typedef(TypedefFrame),
    Instance(InstanceFrame),
}

impl From<TermFrame> for EntityFrame {
    fn from(frame: TermFrame) -> Self {
        EntityFrame::Term(frame)
    }
}

impl From<TypedefFrame> for EntityFrame {
    fn from(frame: TypedefFrame) -> Self {
        EntityFrame::Typedef(frame)
    }
}

impl From<InstanceFrame> for EntityFrame {
    fn from(frame: InstanceFrame) -> Self {
        EntityFrame::Instance(frame)
    }
}

impl FromPair for EntityFrame {
    const RULE: Rule = Rule::EntityFrame;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::TermFrame => TermFrame::from_pair_unchecked(inner).map(From::from),
            Rule::TypedefFrame => TypedefFrame::from_pair_unchecked(inner).map(From::from),
            Rule::InstanceFrame => unimplemented!(),
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(EntityFrame);
