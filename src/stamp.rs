use crate::{ArxivId, ArxivIdError};
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;
use time::error::Parse as TimeParseError;
use time::macros::format_description;
use time::{Date, Month};

/// Convenient type alias for a [`Result`] holding either an [`ArxivStamp`] or [`ArxivStampError`]
pub type ArxivStampResult = Result<ArxivStamp, ArxivStampError>;
type CategoryResult = Result<String, ArxivStampError>;
type DateParseResult = Result<Date, TimeParseError>;

/// An error that can occur when parsing and validating arXiv stamp
///
/// # Examples
/// ```
/// use std::str::FromStr;
/// use arxiv::ArxivStamp;
///
/// let stamp = ArxivStamp::from_str("arXiv:2001.00001 [cs.LG] 1 Jan 2000");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArxivStampError {
	InvalidArxivId(ArxivIdError),
	InvalidDate(TimeParseError),
	InvalidCategory,
	NotEnoughComponents,
}

impl Error for ArxivStampError {}

impl Display for ArxivStampError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::InvalidArxivId(e) => write!(f, "Invalid arXiv ID: {}", e),
			Self::InvalidDate(e) => write!(f, "Invalid date: {}", e),
			Self::InvalidCategory => write!(f, "Invalid category"),
			Self::NotEnoughComponents => write!(f, "Not enough components"),
		}
	}
}

/// A stamp that is added onto the side of PDF version of arXiv articles
#[derive(Debug, PartialEq, Eq)]
pub struct ArxivStamp {
	pub id: ArxivId,
	pub category: String,
	pub submitted: Date,
}

impl ArxivStamp {
	pub(crate) const TOKEN_SPACE: char = ' ';

	/// Manually create a new [`ArxivStamp`] from the given components. This is useful
	/// if you want to dynamically create a stamp from user input; otherwise, it is
	/// recommended to use [`ArxivStamp::from_str()`] instead since it can be quite verbose.
	///
	/// # Examples
	/// ```
	/// use time::{Date, Month};
	/// use arxiv::{ArxivId, ArxivStamp};
	///
	/// let stamp = ArxivStamp::new(
	///    ArxivId::try_latest(2011, 1, String::from("00001")).unwrap(),
	///    String::from("cs.LG"),
	///    Date::from_calendar_date(2011, Month::January, 1).unwrap()
	/// );
	/// ```
	#[inline]
	pub fn new(id: ArxivId, category: String, submitted: Date) -> Self {
		Self {
			id,
			category,
			submitted,
		}
	}
}

impl Display for ArxivStamp {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(
			f,
			"{} [{}] {} {} {}",
			self.id,
			self.category,
			self.submitted.day(),
			month_as_abbr(self.submitted.month()),
			self.submitted.year()
		)
	}
}

impl FromStr for ArxivStamp {
	type Err = ArxivStampError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts = s.splitn(2, ArxivStamp::TOKEN_SPACE).collect::<Vec<&str>>();

		if parts.len() == 1 {
			return Err(ArxivStampError::NotEnoughComponents);
		}

		let arxiv_id = ArxivId::from_str(parts[0]);
		if let Err(e) = arxiv_id {
			return Err(ArxivStampError::InvalidArxivId(e));
		}

		// category is opitional, so we need to check if the second part is a category
		// and decide which index to use to parse each component
		let part2_is_category = parts[1].starts_with('[');
		let mut category: CategoryResult = Ok(String::new());
		let date: DateParseResult;

		if part2_is_category {
			let category_date = parts[1]
				.splitn(2, ArxivStamp::TOKEN_SPACE)
				.collect::<Vec<&str>>();

			category = parse_category(category_date[0]);
			date = parse_date(category_date[1]);
		} else {
			date = parse_date(parts[1]);
		}

		// validate the category and date components
		// converting this to category? will cause the value to move,
		#[allow(clippy::question_mark)]
		if let Err(e) = category {
			return Err(e);
		}

		if let Err(e) = date {
			return Err(ArxivStampError::InvalidDate(e));
		}

		// if we got this far, we can safely unwrap the results
		Ok(Self::new(arxiv_id.unwrap(), category.unwrap(), date.unwrap()))
	}
}

/// write a function that parses a value like "[<text>]" where <text> can be anything
/// and returns <text> as a string
fn parse_category(s: &str) -> Result<String, ArxivStampError> {
	if brackets_match(s) {
		Ok(s[1..s.len() - 1].to_string())
	} else {
		Err(ArxivStampError::InvalidCategory)
	}
}

fn brackets_match(s: &str) -> bool {
	s.starts_with('[') && s.ends_with(']')
		|| s.starts_with('(') && s.ends_with(')')
		|| s.starts_with('{') && s.ends_with('}')
}

fn month_as_abbr<'a>(month: Month) -> &'a str {
	match month {
		Month::January => "Jan",
		Month::February => "Feb",
		Month::March => "Mar",
		Month::April => "Apr",
		Month::May => "May",
		Month::June => "June",
		Month::July => "July",
		Month::August => "Aug",
		Month::September => "Sept",
		Month::October => "Oct",
		Month::November => "Nov",
		Month::December => "Dec",
	}
}

/// Parses a date in the form of "1 Jan 2000", where:
///  - the day is a number without zero padding
///  - the month is the first three letters of the full month name
///  - the year is a 4-digit number
///
/// See also: [`time` documentation for format descriptions][time-format-desc]
///
/// [time-format-desc]: https://time-rs.github.io/book/api/format-description.html
fn parse_date(date_str: &str) -> DateParseResult {
	Date::parse(date_str, &format_description!("[day padding:none] [month repr:short] [year]"))
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::str::FromStr;
	use time::error::ParseFromDescription;
	use time::Date;

	#[test]
	fn display_stamp() {
		let stamp = ArxivStamp::new(
			ArxivId::from_str("arXiv:2011.00001").unwrap(),
			String::from("cs.LG"),
			Date::from_calendar_date(2011, Month::January, 1).unwrap(),
		);
		assert_eq!(stamp.to_string(), "arXiv:2011.00001 [cs.LG] 1 Jan 2011");
	}

	#[test]
	fn parse_stamp() {
		let stamp = "arXiv:2001.00001 [cs.LG] 1 Jan 2000";
		let parsed = ArxivStamp::from_str(stamp);
		assert_eq!(
			parsed,
			Ok(ArxivStamp::new(
				ArxivId::from_str("arXiv:2001.00001").unwrap(),
				String::from("cs.LG"),
				Date::from_calendar_date(2000, Month::January, 1).unwrap()
			))
		);
	}

	#[test]
	fn parse_stamp_without_category() {
		let stamp = "arXiv:2001.00001 1 Jan 2000";
		let parsed = ArxivStamp::from_str(stamp);
		assert_eq!(
			parsed,
			Ok(ArxivStamp::new(
				ArxivId::from_str("arXiv:2001.00001").unwrap(),
				String::new(),
				Date::from_calendar_date(2000, Month::January, 1).unwrap()
			))
		);
	}

	#[test]
	fn parse_stamp_empty() {
		let stamp = "";
		let parsed = ArxivStamp::from_str(stamp);
		assert_eq!(parsed, Err(ArxivStampError::NotEnoughComponents));
	}

	#[test]
	fn parse_stamp_not_enough_components() {
		let stamp = "arXiv:2001.00001";
		let parsed = ArxivStamp::from_str(stamp);
		assert_eq!(parsed, Err(ArxivStampError::NotEnoughComponents));
	}

	#[test]
	fn parse_stamp_invalid_category() {
		let stamp = "arXiv:2001.00001 [cs.LG 1 Jan 2000";
		let parsed = ArxivStamp::from_str(stamp);
		assert_eq!(parsed, Err(ArxivStampError::InvalidCategory));
	}

	#[test]
	fn parse_stamp_invalid_date_day() {
		let stamp = "arXiv:2001.00001 [cs.LG] 32 Jan 2000";
		let parsed = ArxivStamp::from_str(stamp);

		// hack to get a ComponentRange error
		let date = parse_date("32 Jan 2000").unwrap_err();

		assert_eq!(parsed, Err(ArxivStampError::InvalidDate(date)));
	}

	#[test]
	fn parse_stamp_invalid_date_month() {
		let stamp = "arXiv:2001.00001 [cs.LG] 1 Zan 2000";
		let parsed = ArxivStamp::from_str(stamp);
		assert_eq!(parsed, Err(invalid_date_component("month")));
	}

	#[test]
	fn parse_stamp_invalid_date_year() {
		let stamp = "arXiv:2001.00001 [cs.LG] 1 Jan 200";
		let parsed = ArxivStamp::from_str(stamp);
		assert_eq!(parsed, Err(invalid_date_component("year")));
	}

	fn invalid_date_component(component: &'static str) -> ArxivStampError {
		ArxivStampError::InvalidDate(TimeParseError::ParseFromDescription(
			ParseFromDescription::InvalidComponent(component),
		))
	}
}
