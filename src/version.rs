use anyhow::Result;
use chrono::Datelike;

pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn parse(version: &str) -> Result<Self> {
        let mut parts = version.split('.');
        let major = parts.next().unwrap().parse()?;
        let minor = parts.next().unwrap().parse()?;
        let patch = parts.next().unwrap().parse()?;
        Ok(Self {
            major,
            minor,
            patch,
        })
    }

    /// Bump version to the next monoversion
    pub fn bump(self) -> Self {
        let now = chrono::Utc::now();
        let year: u32 = now.year() as u32 - 2000;
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
}
