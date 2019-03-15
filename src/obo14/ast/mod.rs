
mod date;
mod header;
mod id;
mod instance;
mod line;
mod misc;
mod strings;
mod synonym;
mod term;
mod typedef;

pub use self::date::*;
pub use self::header::*;
pub use self::id::*;
pub use self::instance::*;
pub use self::line::*;
pub use self::misc::*;
pub use self::strings::*;
pub use self::synonym::*;
pub use self::term::*;
pub use self::typedef::*;

/// A complete OBO document in format version 1.4.
pub struct Obo14Doc {
    header: HeaderFrame,
    entities: Vec<EntityFrame>,
}

/// An entity frame, either for a term, an instance, or a typedef.
pub enum EntityFrame {
    Term(TermFrame),
    Typedef(TypedefFrame),
    Instance(InstanceFrame),
}
