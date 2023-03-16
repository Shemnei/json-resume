#![allow(rustdoc::private_intra_doc_links)]
#![deny(
    // Documentation
	// TODO: rustdoc::broken_intra_doc_links,
	// TODO: rustdoc::missing_crate_level_docs,
	// TODO: missing_docs,
	// TODO: clippy::missing_docs_in_private_items,

    // Other
	deprecated_in_future,
	exported_private_dependencies,
	future_incompatible,
	missing_copy_implementations,
	missing_debug_implementations,
	private_in_public,
	rust_2018_compatibility,
	rust_2018_idioms,
	trivial_casts,
	trivial_numeric_casts,
	unsafe_code,
	unstable_features,
	unused_import_braces,
	unused_qualifications,

	// clippy attributes
	clippy::missing_const_for_fn,
	clippy::redundant_pub_crate,
	clippy::use_self
)]
#![cfg_attr(docsrs, feature(doc_cfg), feature(doc_alias))]

use serde::{Deserialize, Serialize};
#[cfg(feature = "validate")]
use serde_valid::Validate;

/// Resume Schema
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Resume {
	pub basics: Option<Basics>,
	pub work: Vec<Work>,
	pub volunteer: Vec<Volunteer>,
	pub education: Vec<Education>,
	/// Specify any awards you have received throughout your professional caree
	pub awards: Vec<Award>,
	/// Specify any certificates you have received throughout your professional career
	pub certificates: Vec<Certificate>,
	/// Specify your publications through your career
	pub publications: Vec<Publication>,
	/// List out your professional skill-set
	pub skills: Vec<Skill>,
	/// List any other languages you speak
	pub languages: Vec<Language>,
	pub interests: Vec<Interest>,
	/// List references you have received
	pub references: Vec<Reference>,
	/// Specify career projects
	pub projects: Vec<Project>,
	/// The schema version and any other tooling configuration lives here
	pub meta: Option<Meta>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Basics {
	pub name: Option<String>,
	/// e.g. Web Developer
	pub label: Option<String>,
	/// URL (as per RFC 3986) to a image in JPEG or PNG format
	pub image: Option<String>,
	/// e.g. thomas@gmail.com
	pub email: Option<String>,
	/// Phone numbers are stored as strings so use any format you like, e.g. 712-117-2923
	pub phone: Option<String>,
	/// URL (as per RFC 3986) to your website, e.g. personal homepage
	pub url: Option<String>,
	/// Write a short 2-3 sentence biography about yourself
	pub summary: Option<String>,
	pub location: Location,
	/// Specify any number of social networks that you participate in
	pub profiles: Vec<Profile>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Location {
	/// To add multiple address lines, use \n. For example, 1234 Glücklichkeit Straße\nHinterhaus 5. Etage li.
	pub address: Option<String>,
	#[serde(rename = "postalCode")]
	pub postal_code: Option<String>,
	pub city: Option<String>,
	/// code as per ISO-3166-1 ALPHA-2, e.g. US, AU, IN
	#[serde(rename = "countryCode")]
	pub country_code: Option<String>,
	/// The general region where you live. Can be a US state, or a province, for instance.
	pub region: Option<String>,
}

/// Specify any number of social networks that you participate in
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Profile {
	/// e.g. Facebook or Twitter
	pub network: Option<String>,
	/// e.g. neutralthoughts
	pub username: Option<String>,
	/// e.g. http://twitter.example.com/neutralthoughts
	pub url: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Work {
	/// e.g. Facebook
	pub name: Option<String>,
	/// e.g. Menlo Park, CA
	pub location: Option<String>,
	/// e.g. Social Media Company
	pub description: Option<String>,
	/// e.g. Software Engineer
	pub position: Option<String>,
	/// e.g. http://facebook.example.com
	pub url: Option<String>,
	#[serde(rename = "startDate")]
	#[cfg_attr(
		feature = "validate",
		validate(
			pattern = r"^([1-2][0-9]{3}-[0-1][0-9]-[0-3][0-9]|[1-2][0-9]{3}-[0-1][0-9]|[1-2][0-9]{3})$"
		)
	)]
	pub start_date: Option<String>,
	#[serde(rename = "endDate")]
	#[cfg_attr(
		feature = "validate",
		validate(
			pattern = r"^([1-2][0-9]{3}-[0-1][0-9]-[0-3][0-9]|[1-2][0-9]{3}-[0-1][0-9]|[1-2][0-9]{3})$"
		)
	)]
	pub end_date: Option<String>,
	/// Specify multiple accomplishments
	pub highlights: Vec<Highlight>,
}

/// e.g. Increased profits by 20% from 2011-2012 through viral advertising
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
pub struct Highlight(String);

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Volunteer {
	/// e.g. Facebook
	pub organization: Option<String>,
	/// e.g. Software Engineer
	pub position: Option<String>,
	/// e.g. http://facebook.example.com
	pub url: Option<String>,
	#[serde(rename = "startDate")]
	#[cfg_attr(
		feature = "validate",
		validate(
			pattern = r"^([1-2][0-9]{3}-[0-1][0-9]-[0-3][0-9]|[1-2][0-9]{3}-[0-1][0-9]|[1-2][0-9]{3})$"
		)
	)]
	pub start_date: Option<String>,
	#[serde(rename = "endDate")]
	#[cfg_attr(
		feature = "validate",
		validate(
			pattern = r"^([1-2][0-9]{3}-[0-1][0-9]-[0-3][0-9]|[1-2][0-9]{3}-[0-1][0-9]|[1-2][0-9]{3})$"
		)
	)]
	pub end_date: Option<String>,
	/// Give an overview of your responsibilities at the company
	pub summary: Option<String>,
	/// Specify multiple accomplishments
	pub highlights: Vec<Highlight>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Education {
	/// e.g. Massachusetts Institute of Technology
	pub institution: Option<String>,
	/// e.g. http://facebook.example.com
	pub url: Option<String>,
	/// e.g. Arts
	pub arae: Option<String>,
	/// e.g. Bachelor
	pub study_type: Option<String>,
	#[serde(rename = "startDate")]
	#[cfg_attr(
		feature = "validate",
		validate(
			pattern = r"^([1-2][0-9]{3}-[0-1][0-9]-[0-3][0-9]|[1-2][0-9]{3}-[0-1][0-9]|[1-2][0-9]{3})$"
		)
	)]
	pub start_date: Option<String>,
	#[serde(rename = "endDate")]
	#[cfg_attr(
		feature = "validate",
		validate(
			pattern = r"^([1-2][0-9]{3}-[0-1][0-9]-[0-3][0-9]|[1-2][0-9]{3}-[0-1][0-9]|[1-2][0-9]{3})$"
		)
	)]
	pub end_date: Option<String>,
	/// grade point average, e.g. 3.67/4.0
	pub score: Option<String>,
	/// List notable courses/subjects
	#[serde(default)]
	pub courses: Vec<Course>,
}

/// e.g. H1302 - Introduction to American history
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
pub struct Course(String);

/// Specify any awards you have received throughout your professional caree
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Award {
	/// e.g. One of the 100 greatest minds of the century
	pub title: Option<String>,
	#[cfg_attr(
		feature = "validate",
		validate(
			pattern = r"^([1-2][0-9]{3}-[0-1][0-9]-[0-3][0-9]|[1-2][0-9]{3}-[0-1][0-9]|[1-2][0-9]{3})$"
		)
	)]
	pub date: Option<String>,
	/// e.g. Time Magazine
	pub awarder: Option<String>,
	/// e.g. Received for my work with Quantum Physics
	pub summary: Option<String>,
}

/// Specify any certificates you have received throughout your professional career
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Certificate {
	/// e.g. Certified Kubernetes Administrator
	pub name: Option<String>,
	/// e.g. 1989-06-12
	pub date: Option<String>,
	/// e.g. http://example.com
	pub url: Option<String>,
	/// e.g. CNCF
	pub issuer: Option<String>,
}

/// Specify your publications through your career
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Publication {
	/// e.g. The World Wide Web
	pub name: Option<String>,
	/// e.g. IEEE, Computer Magazine
	pub publisher: Option<String>,
	#[serde(rename = "releaseDate")]
	#[cfg_attr(
		feature = "validate",
		validate(
			pattern = r"^([1-2][0-9]{3}-[0-1][0-9]-[0-3][0-9]|[1-2][0-9]{3}-[0-1][0-9]|[1-2][0-9]{3})$"
		)
	)]
	pub release_date: Option<String>,
	/// e.g. http://www.computer.org.example.com/csdl/mags/co/1996/10/rx069-abs.html
	pub url: Option<String>,
	/// Short summary of publication. e.g. Discussion of the World Wide Web, HTTP, HTML.
	pub summary: Option<String>,
}

/// List out your professional skill-set
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Skill {
	/// e.g. Web Development
	pub name: Option<String>,
	/// e.g. Master
	pub level: Option<String>,
	/// List some keywords pertaining to this skill
	pub keywords: Vec<Keyword>,
}

/// e.g. HTML
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
pub struct Keyword(String);

/// List any other languages you speak
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Language {
	/// e.g. English, Spanish
	pub language: Option<String>,
	/// e.g. Fluent, Beginner
	pub fluency: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Interest {
	/// e.g. Philosophy
	pub name: Option<String>,
	pub keywords: Vec<Keyword>,
}

/// List references you have received
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Reference {
	/// e.g. Timothy Cook
	pub name: Option<String>,
	/// e.g. Joe blogs was a great employee, who turned up to work at least once a week. He exceeded my expectations when it came to doing nothing.
	pub reference: Option<String>,
}

/// Specify career projects
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Project {
	/// e.g. The World Wide Web
	pub name: Option<String>,
	/// Short summary of project. e.g. Collated works of 2017.
	pub description: Option<String>,
	/// Specify multiple features
	pub highlights: Vec<Highlight>,
	/// Specify special elements involved
	pub keywords: Vec<Keyword>,
	#[serde(rename = "startDate")]
	#[cfg_attr(
		feature = "validate",
		validate(
			pattern = r"^([1-2][0-9]{3}-[0-1][0-9]-[0-3][0-9]|[1-2][0-9]{3}-[0-1][0-9]|[1-2][0-9]{3})$"
		)
	)]
	pub start_date: Option<String>,
	#[serde(rename = "endDate")]
	#[cfg_attr(
		feature = "validate",
		validate(
			pattern = r"^([1-2][0-9]{3}-[0-1][0-9]-[0-3][0-9]|[1-2][0-9]{3}-[0-1][0-9]|[1-2][0-9]{3})$"
		)
	)]
	pub end_date: Option<String>,
	/// e.g. http://www.computer.org/csdl/mags/co/1996/10/rx069-abs.html
	pub url: Option<String>,
	/// Specify your role on this project or in company
	pub roles: Vec<Role>,
	/// Specify the relevant company/entity affiliations e.g. 'greenpeace', 'corporationXYZ'
	pub entity: Option<String>,
	/// e.g. 'volunteering', 'presentation', 'talk', 'application', 'conference'
	pub r#type: Option<String>,
}

/// e.g. Team Lead, Speaker, Writer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
pub struct Role(String);

/// The schema version and any other tooling configuration lives here
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Meta {
	/// URL (as per RFC 3986) to latest version of this document
	pub canonical: Option<String>,
	/// A version field which follows semver - e.g. v1.0.0
	pub version: Option<String>,
	/// Using ISO 8601 with YYYY-MM-DDThh:mm:ss
	#[serde(rename = "lastModified")]
	pub last_modified: Option<String>,
}

#[cfg(feature = "validate")]
#[cfg(test)]
mod validate {
	use super::*;

	#[test]
	fn sample() -> Result<(), Box<dyn std::error::Error>> {
		const SAMPLE: &str = include_str!("../sample.resume.json");

		let resume: Resume = serde_json::from_str(SAMPLE)?;
		resume.validate()?;

		Ok(())
	}

	#[test]
	#[ignore = "Run explicitly"]
	fn stdin() -> Result<(), Box<dyn std::error::Error>> {
		let resume_file = std::env::var_os("RESUME_FILE").unwrap();
		let resume = std::fs::read_to_string(resume_file)?;

		let resume: Resume = serde_json::from_str(&resume)?;
		resume.validate()?;

		println!("{resume:#?}");

		Ok(())
	}
}
