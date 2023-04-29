use crate::subject_tables::*;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// An identifier for arXiv categories, which are composed of an archive and category
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArxivCategoryId {
	group: ArxivGroup,
	archive: ArxivArchive,
	subject: String,
}

impl ArxivCategoryId {
	pub(crate) const TOKEN_DELIM: char = '.';

	pub(super) const fn new(group: ArxivGroup, archive: ArxivArchive, subject: String) -> Self {
		Self {
			group,
			archive,
			subject,
		}
	}

	/// Checks if the string is a valid group identifier, based on the archive and category.
	///
	/// Valid archive identifiers are listed under the official website's page for [category taxonomy][arxiv-cat].
	///
	/// [arxiv-cat]: <https://arxiv.org/category_taxonomy>
	#[rustfmt::skip]
	pub fn try_new(archive: ArxivArchive, subject: &str) -> Option<Self> {
		let is_valid = match archive {
			ArxivArchive::AstroPh => matches!(subject, "CO" | "EP" | "GA" | "HE" | "IM" | "SR"),
			ArxivArchive::CondMat => matches!(subject,
					| "dis-nn" | "mes-hall" | "mtrl-sci"
					| "other" | "quant-gas" | "soft"
					| "stat-mech" | "str-el" | "supr-con"
			),
			ArxivArchive::Cs => COMPSCI_TABLE.binary_search(&subject).is_ok(),
			ArxivArchive::Econ => matches!(subject, "EM" | "GN" | "TH"),
			ArxivArchive::Eess => matches!(subject, "AS" | "IV" | "SP" | "SY"),
			ArxivArchive::GrQc => subject.is_empty(),
			ArxivArchive::HepEx => subject.is_empty(),
			ArxivArchive::HepLat => subject.is_empty(),
			ArxivArchive::HepPh => subject.is_empty(),
			ArxivArchive::HepTh => subject.is_empty(),
			ArxivArchive::MathPh => subject.is_empty(),
			ArxivArchive::Math => MATH_TABLE.binary_search(&subject).is_ok(),
			ArxivArchive::Nlin => matches!(subject, "AO" | "CD" | "CG" | "PS" | "SI"),
			ArxivArchive::NuclEx => subject.is_empty(),
			ArxivArchive::NuclTh => subject.is_empty(),
			ArxivArchive::Physics => PHYSICS_TABLE.binary_search(&subject).is_ok(),
			ArxivArchive::QBio    => matches!(subject, "BM" | "CB" | "GN" | "MN" | "NC" | "OT" | "PE" | "QM" | "SC" | "TO"),
			ArxivArchive::QFin => {
				matches!(subject, "CP" | "EC" | "GN" | "MF" | "PM" | "PR" | "RM" | "ST" | "SR")
			}
			ArxivArchive::QuantPh => subject.is_empty(),
			ArxivArchive::Stat => matches!(subject, "AP" | "CO" | "ME" | "ML" | "OT" | "TH"),
		};

		match is_valid {
			true => Some(Self::new(ArxivGroup::from(archive), archive, String::from(subject))),
			false => None,
		}
	}

	/// The group, which contains one or more archives
	#[must_use]
	#[inline]
	pub const fn group(&self) -> ArxivGroup {
		self.group
	}

	/// The archive, representing a collection of publications
	/// that relate to each other by a specific field of study
	#[must_use]
	#[inline]
	pub const fn archive(&self) -> ArxivArchive {
		self.archive
	}

	/// The subject class of the arXiv category
	#[must_use]
	#[inline]
	pub fn subject(&self) -> String {
		self.subject.to_owned()
	}
}

impl Display for ArxivCategoryId {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}.{}", self.archive, self.subject)
	}
}

impl FromStr for ArxivCategoryId {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: Vec<&str> = s.split(Self::TOKEN_DELIM).collect();
		if parts.len() != 2 {
			return Err(());
		}

		let archive = ArxivArchive::from_str(parts[0])?;
		let subject = parts[1];

		Self::try_new(archive, subject).ok_or(())
	}
}

/// A type of classification for arXiv publications
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArxivGroup {
	/// Computer Science
	Cs,
	/// Economics
	Econ,
	/// Electrical Engineering and Systems Science
	Eess,
	/// Mathematics
	Math,
	/// Physics
	Physics,
	/// Quantitative Biology
	QBio,
	/// Quantitative Finance
	QFin,
	/// Statistics
	Stat,
}

impl From<ArxivArchive> for ArxivGroup {
	fn from(archive: ArxivArchive) -> Self {
		match archive {
			ArxivArchive::Cs => Self::Cs,
			ArxivArchive::Econ => Self::Econ,
			ArxivArchive::Eess => Self::Eess,
			ArxivArchive::Math => Self::Math,
			ArxivArchive::AstroPh
			| ArxivArchive::CondMat
			| ArxivArchive::GrQc
			| ArxivArchive::HepEx
			| ArxivArchive::HepLat
			| ArxivArchive::HepPh
			| ArxivArchive::HepTh
			| ArxivArchive::MathPh
			| ArxivArchive::Nlin
			| ArxivArchive::NuclEx
			| ArxivArchive::NuclTh
			| ArxivArchive::Physics
			| ArxivArchive::QuantPh => Self::Physics,
			ArxivArchive::QBio => Self::QBio,
			ArxivArchive::QFin => Self::QFin,
			ArxivArchive::Stat => Self::Stat,
		}
	}
}

/// A collection of publications that relate under the same field of study
///
/// Valid archive identifiers are listed under the official website's page for [category taxonomy][arxiv-cat].
///
/// [arxiv-cat]: <https://arxiv.org/category_taxonomy>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArxivArchive {
	/// Astro physics
	AstroPh,
	/// Condensed matter
	CondMat,
	/// Computer science
	Cs,
	/// Economics
	Econ,
	/// Electrical Engineering and Systems Science
	Eess,
	/// General Relativity and Quantum Cosmology
	GrQc,
	/// High energy physics - Experiment
	HepEx,
	/// High energy physics - Lattice
	HepLat,
	/// High energy physics - Phenomenology
	HepPh,
	/// High energy physics - Theory
	HepTh,
	/// Mathematical Physics
	MathPh,
	/// Mathematics
	Math,
	/// Nonlinear Sciences
	Nlin,
	/// Nuclear Experiment
	NuclEx,
	/// Nuclear Theory
	NuclTh,
	/// Physics
	Physics,
	/// Quantitative Biology
	QBio,
	/// Quantitative Finance
	QFin,
	/// Quantum Physics
	QuantPh,
	/// Statistics
	Stat,
}

impl Display for ArxivArchive {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(
			f,
			"{}",
			match self {
				ArxivArchive::AstroPh => "astro-ph",
				ArxivArchive::CondMat => "cond-mat",
				ArxivArchive::Cs => "cs",
				ArxivArchive::Econ => "econ",
				ArxivArchive::Eess => "eess",
				ArxivArchive::GrQc => "gr-qc",
				ArxivArchive::HepEx => "hep-ex",
				ArxivArchive::HepLat => "hep-lat",
				ArxivArchive::HepPh => "hep-ph",
				ArxivArchive::HepTh => "hep-th",
				ArxivArchive::MathPh => "math-ph",
				ArxivArchive::Math => "math",
				ArxivArchive::Nlin => "nlin",
				ArxivArchive::NuclEx => "nucl-ex",
				ArxivArchive::NuclTh => "nucl-th",
				ArxivArchive::Physics => "physics",
				ArxivArchive::QBio => "q-bio",
				ArxivArchive::QFin => "q-fin",
				ArxivArchive::QuantPh => "quant-ph",
				ArxivArchive::Stat => "stat",
			}
		)
	}
}

impl FromStr for ArxivArchive {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"astro-ph" => Ok(Self::AstroPh),
			"cond-mat" => Ok(Self::CondMat),
			"cs" => Ok(Self::Cs),
			"econ" => Ok(Self::Econ),
			"eess" => Ok(Self::Eess),
			"gr-qc" => Ok(Self::GrQc),
			"hep-ex" => Ok(Self::HepEx),
			"hep-lat" => Ok(Self::HepLat),
			"hep-ph" => Ok(Self::HepPh),
			"hep-th" => Ok(Self::HepTh),
			"math-ph" => Ok(Self::MathPh),
			"math" => Ok(Self::Math),
			"nlin" => Ok(Self::Nlin),
			"nucl-ex" => Ok(Self::NuclEx),
			"nucl-th" => Ok(Self::NuclTh),
			"physics" => Ok(Self::Physics),
			"q-bio" => Ok(Self::QBio),
			"q-fin" => Ok(Self::QFin),
			"quant-ph" => Ok(Self::QuantPh),
			"stat" => Ok(Self::Stat),
			_ => Err(()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_category_id() {
		let cat_id = ArxivCategoryId::from_str("cs.LG");
		assert_eq!(
			cat_id,
			Ok(ArxivCategoryId::new(ArxivGroup::Cs, ArxivArchive::Cs, String::from("LG")))
		);
	}

	#[test]
	fn display_category() {
		assert_eq!(
			ArxivCategoryId::try_new(ArxivArchive::AstroPh, "HE")
				.unwrap()
				.to_string(),
			"astro-ph.HE"
		);
	}

	#[test]
	fn group_from_archive() {
		assert_eq!(ArxivGroup::from(ArxivArchive::AstroPh), ArxivGroup::Physics);
	}

	#[test]
	fn parse_archive() {
		let archive = ArxivArchive::from_str("astro-ph");
		assert_eq!(archive, Ok(ArxivArchive::AstroPh));
	}
}
