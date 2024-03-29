use core::fmt;

use chrono::Datelike;

use crate::edit::LineContext;

/// A version in the format of YY.MM.PATCH
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[derive(Debug, Clone)]
pub struct VersionContext {
    pub version: Version,
    pub next_version: Version,
    pub line_number: usize,
}

impl Version {
    pub fn parse(version: &str) -> Self {
        let mut parts = version.split('.');
        let major = parts.next().unwrap_or("").parse().unwrap_or_else(|_| {
            log::warn!("Failed to parse major version, defaulting to 0");
            0
        });
        let minor = parts.next().unwrap_or("").parse().unwrap_or_else(|_| {
            log::warn!("Failed to parse minor version, defaulting to 0");
            0
        });
        let patch = parts.next().unwrap_or("").parse().unwrap_or_else(|_| {
            log::warn!("Failed to parse patch version, defaulting to 0");
            0
        });
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Bump version to the next monoversion
    pub fn bump(&self) -> Self {
        let now = chrono::Utc::now();
        let year = now.year() as u32 - 2000;
        let month = now.month();

        if year != self.major || month != self.minor {
            Self {
                major: year,
                minor: month,
                patch: 0,
            }
        } else {
            Self {
                major: self.major,
                minor: self.minor,
                patch: self.patch + 1,
            }
        }
    }

    /// Bump patch version
    pub fn bump_patch(&self) -> Self {
        Self {
            major: self.major,
            minor: self.minor,
            patch: self.patch + 1,
        }
    }
}

impl VersionContext {
    pub fn new(version: Version, line_number: usize) -> Self {
        let next_version = version.bump();
        Self {
            version,
            next_version,
            line_number,
        }
    }

    pub fn from_line_context(line_context: LineContext) -> Self {
        let version = line_context.value.to_version();
        let next_version = version.bump();
        Self {
            version,
            next_version,
            line_number: line_context.line_number,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

pub trait ToVersion {
    fn to_version(&self) -> Version;
}

impl ToVersion for &str {
    fn to_version(&self) -> Version {
        Version::parse(self)
    }
}

impl ToVersion for String {
    fn to_version(&self) -> Version {
        Version::parse(self)
    }
}
