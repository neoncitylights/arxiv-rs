#![doc = include_str!("../README.md")]

mod identifier;
mod stamp;
pub use crate::identifier::*;
pub use crate::stamp::*;

/// Represents the versioned grammar that defines an arXiv identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArxivIdScheme {
	/// Identifier scheme up to [March 2007][arxiv-march-2007]
	///
	/// [arxiv-march-2007]: https://info.arxiv.org/help/arxiv_identifier.html#identifiers-up-to-march-2007-9107-0703
	Old,

	/// Identifier scheme since [1 April 2007][arxiv-april-2007]
	///
	/// [arxiv-april-2007]: https://info.arxiv.org/help/arxiv_identifier.html#identifier-scheme-since-1-april-2007-0704-
	New,
}
