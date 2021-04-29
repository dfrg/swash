/*!
Character properties and textual analysis.
*/

mod analyze;
mod compose;
mod lang;
mod lang_data;
mod unicode;
mod unicode_data;

pub mod cluster;

pub use lang::{Cjk, Language};
pub use unicode::*;
pub use analyze::{analyze, Analyze};
