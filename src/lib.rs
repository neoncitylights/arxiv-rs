//! # `arxiv`
//! A library for parsing and generating arXiv identifiers and stamps.
//!
//! ## Examples
//! ```rust
//! use std::str::FromStr;
//! use arxiv::{ArxivId, ArxivStamp};
//!
//! // Parse an arXiv identifier
//! let id = ArxivId::from_str("arXiv:9912.12345v2").unwrap();
//! assert_eq!(id.month, 12);
//! assert_eq!(id.year, 2099);
//! assert_eq!(id.number, "12345");
//! assert_eq!(id.version, Some(2));
//!
//! // Parse an arXiv stamp
//! let stamp = ArxivStamp::from_str("arXiv:0706.0001v1 [q-bio.CB] 1 Jun 2007").unwrap();
//! assert_eq!(stamp.category, "q-bio.CB");
//! assert_eq!(stamp.submitted.year(), 2007);
//! ```

mod identifier;
mod stamp;
pub use crate::identifier::*;
pub use crate::stamp::*;

/// Represents the versioned grammar that defines an arXiv identifier
#[derive(Debug, Clone, Copy)]
pub enum ArxivIdScheme {
	/// Identifier scheme up to March 2007
	/// <https://info.arxiv.org/help/arxiv_identifier.html#identifiers-up-to-march-2007-9107-0703>
	Old,

	/// Identifier scheme since 1 April 2007
	/// <https://info.arxiv.org/help/arxiv_identifier.html#identifier-scheme-since-1-april-2007-0704->
	New,
}
