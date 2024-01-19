use chrono::Datelike;

#[derive(Debug)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
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
