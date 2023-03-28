use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Convenient type alias for a [`Result`] holding either an [`ArxivId`] or [`ArxivIdError`]
pub type ArxivIdResult = Result<ArxivId, ArxivIdError>;

/// An error that can occur when parsing and validating arXiv identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArxivIdError {
	/// A generic parsing syntax error
	Syntax,
	/// An ivalid month outside of the inclusive [1, 12] interval
	InvalidMonth,
	/// An invalid year outside of the inclusive [2007, 2099] interval
	InvalidYear,
	/// An invalid year outside of the inclusive [1, 99999] interval
	InvalidId,
}

impl Error for ArxivIdError {}

impl Display for ArxivIdError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Syntax => write!(f, "There was a syntax error; an ArXiv identifier must conform to the schema of arXiv:YYMM.number{{vV}}."),
			Self::InvalidMonth => write!(f, "A valid month must be between 1 and 12."),
			Self::InvalidYear => write!(f, "A valid year must be be between 2007 and 2099."),
			Self::InvalidId => write!(f, "A valid identifier must be between 1 and 99999."),
		}
	}
}

/// A unique identifier for articles published on arXiv.org
///
/// See also: [Official arXiv.org documentation][arxiv-docs]
///
/// # Examples
/// ```
/// use std::str::FromStr;
/// use arxiv::ArxivId;
///
/// let id = ArxivId::from_str("arXiv:2001.00001");
/// ```
///
/// [arxiv-docs]: https://info.arxiv.org/help/arxiv_identifier.html
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArxivId {
	pub year: u16,
	pub month: u8,
	pub number: String,
	pub version: Option<u8>,
}

impl ArxivId {
	pub const MIN_YEAR: u16 = 2007u16;
	pub const MAX_YEAR: u16 = 2099u16;
	pub const MIN_MONTH: u8 = 1u8;
	pub const MAX_MONTH: u8 = 12u8;
	pub(crate) const TOKEN_COLON: char = ':';
	pub(crate) const TOKEN_DOT: char = '.';
	pub(crate) const TOKEN_VERSION: char = 'v';

	/// This allows manually creating an [`ArxivId`] from the given components without any
	/// validation. Only do this if you have already verified that the components are valid.
	///
	/// # Examples
	/// ```
	/// use arxiv::ArxivId;
	///
	/// let id = ArxivId::new_raw(2011, 1, String::from("00001"), Some(1));
	/// ```
	#[inline]
	pub fn new_raw(year: u16, month: u8, id: String, version: Option<u8>) -> Self {
		Self {
			year,
			month,
			number: id,
			version,
		}
	}

	/// This allows manually creating an [`ArxivId`] from the given components without any version
	/// (assuming it is the latest version). Only do this if you have already verified that the
	/// components are valid.
	///
	/// # Examples
	/// ```
	/// use arxiv::ArxivId;
	///
	/// let id = ArxivId::new_latest(2011, 1, String::from("00001"));
	/// ```
	#[inline]
	pub fn new_latest(year: u16, month: u8, id: String) -> Self {
		Self::new_raw(year, month, id, None)
	}

	/// This allows manually creating an [`ArxivId`] from the given components with a version, and
	/// will also validate each component for correctness. If any component is invalid, it will return
	/// an [`ArxivIdError`].
	///
	/// # Examples
	/// ```
	/// use arxiv::{ArxivId, ArxivIdError};
	///
	/// let id = ArxivId::try_new(2011, 1, String::from("00001"), Some(1));
	/// ```
	pub fn try_new(year: u16, month: u8, id: String, version: Option<u8>) -> ArxivIdResult {
		if !(Self::MIN_YEAR..=Self::MAX_YEAR).contains(&year) {
			return Err(ArxivIdError::InvalidYear);
		}

		if !(1..=12).contains(&month) {
			return Err(ArxivIdError::InvalidMonth);
		}

		if id.len() < 4 || id.len() > 5 {
			return Err(ArxivIdError::InvalidId);
		}

		Ok(Self::new_raw(year, month, id, version))
	}

	/// This allows manually creating an [`ArxivId`] from the given components without a version
	/// (assuming it is the latest version), and will also validate each component for correctness.
	/// If any component is invalid, it will return an [`ArxivIdError`].
	///
	/// # Examples
	/// ```
	/// use arxiv::{ArxivId, ArxivIdError};
	///
	/// let id = ArxivId::try_latest(2011, 1, String::from("00001"));
	/// ```
	#[inline]
	pub fn try_latest(year: u16, month: u8, id: String) -> ArxivIdResult {
		Self::try_new(year, month, id, None)
	}

	/// Whether or not the identifier refers to the most recent version of the arXiv article
	#[inline]
	pub fn is_latest(&self) -> bool {
		self.version.is_none()
	}

	/// Sets the version of the arXiv article.
	///
	/// # Examples
	/// ```
	/// use std::str::FromStr;
	/// use arxiv::ArxivId;
	///
	/// let mut id = ArxivId::from_str("arXiv:2001.00001").unwrap();
	/// id.set_version(1);
	/// ```
	#[inline]
	pub fn set_version(&mut self, version: u8) {
		self.version = Some(version);
	}

	/// Sets the version of the arXiv article to the latest version.
	///
	/// # Examples
	/// ```
	/// use std::str::FromStr;
	/// use arxiv::ArxivId;
	///
	/// let mut id = ArxivId::from_str("arXiv:2001.00001v1").unwrap();
	/// id.set_latest();
	/// ```
	#[inline]
	pub fn set_latest(&mut self) {
		self.version = None;
	}
}

impl Display for ArxivId {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let mut binding = self.year.to_string();
		let (_, half_year) = binding.as_mut_str().split_at(2);

		if self.number.len() == 4usize {
			write!(f, "arXiv:{:02}{:02}.{:04}", half_year, self.month, self.number)
		} else {
			write!(f, "arXiv:{:02}{:02}.{:05}", half_year, self.month, self.number)
		}
	}
}

impl FromStr for ArxivId {
	type Err = ArxivIdError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		// break down the arxiv string into its components
		let parts: Vec<&str> = value.split(ArxivId::TOKEN_COLON).collect();
		if parts.len() != 2 || parts[0] != "arXiv" {
			return Err(ArxivIdError::Syntax);
		}

		let inner_parts: Vec<&str> = parts[1].split(ArxivId::TOKEN_DOT).collect();
		if inner_parts.len() != 2 {
			return Err(ArxivIdError::Syntax);
		}

		// validate and compose the final Arxiv struct
		let year = inner_parts[0][0..2].parse::<u16>();
		let month = inner_parts[0][2..4].parse::<u8>();
		if year.is_err() || month.is_err() {
			return Err(ArxivIdError::Syntax);
		}

		let (id, version) = parse_numbervv(inner_parts[1]);

		ArxivId::try_new(year.unwrap() + 2000, month.unwrap(), id, version)
	}
}

/// Parses a string in the format of "number{vV}",
/// where:
/// - `number` is a unique integer up 4 to 5 digits
/// - `{vV}` (optional): a `v` literal followed by 1 or more digits
fn parse_numbervv(s: &str) -> (String, Option<u8>) {
	let parts = s.split(ArxivId::TOKEN_VERSION).collect::<Vec<&str>>();

	let number = String::from(parts[0]);
	let mut version: Option<u8> = None;

	if parts.len() == 1 {
		return (number, version);
	}

	let parsed_version = parts[1].parse::<u8>();
	if let Ok(t) = parsed_version {
		version = Some(t);
	}

	(number, version)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_arxiv_empty() {
		assert_eq!(ArxivId::from_str(""), Err(ArxivIdError::Syntax));
	}

	#[test]
	fn parse_arxiv_without_version() {
		assert_eq!(
			ArxivId::from_str("arXiv:1501.00001"),
			Ok(ArxivId::new_latest(2015, 1, String::from("00001")))
		);
	}

	#[test]
	fn parse_arxiv_with_version() {
		assert_eq!(
			ArxivId::from_str("arXiv:9912.12345v2"),
			Ok(ArxivId::new_raw(2099, 12, String::from("12345"), Some(2)))
		)
	}

	#[test]
	fn arxiv_as_string_number4digits() {
		assert_eq!(
			ArxivId::new_latest(2014, 1, String::from("7878")).to_string(),
			String::from("arXiv:1401.7878")
		);
		assert_eq!(
			ArxivId::new_latest(2014, 12, String::from("7878")).to_string(),
			String::from("arXiv:1412.7878")
		);
	}

	#[test]
	fn arxiv_as_string_number5digits() {
		assert_eq!(
			ArxivId::new_latest(2014, 1, String::from("00008")).to_string(),
			String::from("arXiv:1401.00008")
		);
		assert_eq!(
			ArxivId::new_latest(2014, 12, String::from("00008")).to_string(),
			String::from("arXiv:1412.00008")
		);
	}

	#[test]
	fn parse_arxiv_invalid_year() {
		assert_eq!(
			ArxivId::try_latest(2006, 1, String::from("00001")),
			Err(ArxivIdError::InvalidYear)
		);
	}

	#[test]
	fn parse_arxiv_invalid_month() {
		assert_eq!(
			ArxivId::try_latest(2007, u8::MAX, String::from("00001")),
			Err(ArxivIdError::InvalidMonth)
		)
	}

	#[test]
	fn parse_arxiv_invalid_id() {
		assert_eq!(
			ArxivId::try_latest(2007, 11, String::new()),
			Err(ArxivIdError::InvalidId)
		)
	}
}
