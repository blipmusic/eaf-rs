pub mod eaf;
pub mod errors;
pub mod license;
pub mod header;
pub mod media_descriptor;
pub mod linked_file_descriptor;
pub mod property;
pub mod timeorder;
pub mod tier;
pub mod annotation;
pub mod linguistic_type;
pub mod constraint;
pub mod language;
pub mod lexicon_ref;
pub mod index;
pub mod locale;
pub mod controlled_vocabulary;
pub mod json;

pub use eaf::{Eaf, Scope};
pub use errors::EafError;
pub use license::License;
pub use header::Header;
pub use media_descriptor::MediaDescriptor;
pub use linked_file_descriptor::LinkedFileDescriptor;
pub use property::Property;
pub use timeorder::{TimeOrder, TimeSlot};
pub use tier::Tier;
pub use annotation::Annotation;
pub use linguistic_type::LinguisticType;
pub use constraint::{Constraint, StereoType};
pub use language::Language;
pub use lexicon_ref::LexiconRef;
pub use index::Index; // should perhaps not be public
pub use locale::Locale;
pub use controlled_vocabulary::ControlledVocabulary;
pub use json::{JsonAnnotation, JsonEaf, JsonTier};