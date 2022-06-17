use std::fmt::{Display, Formatter, Result};
use std::ops::Add;

#[derive(Debug, PartialEq, Eq)]
pub struct SemVer {
    major: u32,
    minor: u32,
    patch: u32,
    pre_release: Option<String>,
    build: Option<String>,
}

impl SemVer {
    pub const ZERO: Self = Self {
        major: 0,
        minor: 0,
        patch: 0,
        pre_release: None,
        build: None,
    };

    pub const ONE_MAJOR: Self = Self {
        major: 1,
        minor: 0,
        patch: 0,
        pre_release: None,
        build: None,
    };

    pub const ONE_MINOR: Self = Self {
        major: 0,
        minor: 1,
        patch: 0,
        pre_release: None,
        build: None,
    };

    pub const ONE_PATCH: Self = Self {
        major: 0,
        minor: 0,
        patch: 1,
        pre_release: None,
        build: None,
    };
}

impl SemVer {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            pre_release: None,
            build: None,
        }
    }

    pub fn with_pre_release(self, pre_release: &str) -> Self {
        // TODO: need validation
        Self {
            pre_release: Some(String::from(pre_release)),
            ..self
        }
    }

    pub fn with_build(self, build: &str) -> Self {
        // TODO: need validation
        Self {
            build: Some(String::from(build)),
            ..self
        }
    }

    pub fn inc_major(&mut self) {
        self.major += 1;
    }

    pub fn inc_minor(&mut self) {
        self.minor += 1;
    }

    pub fn inc_patch(&mut self) {
        self.patch += 1;
    }
}

impl Default for SemVer {
    fn default() -> Self {
        Self::ONE_MINOR
    }
}

impl Display for SemVer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let pre_release = match &self.pre_release {
            Some(pre_release) => format!("-{}", pre_release),
            None => String::new(),
        };

        let build = match &self.build {
            Some(build) => format!("+{}", build),
            None => String::new(),
        };

        write!(
            f,
            "{}.{}.{}{}{}",
            self.major, self.minor, self.patch, pre_release, build,
        )
    }
}

impl Add for SemVer {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            major: self.major + rhs.major,
            minor: self.minor + rhs.minor,
            patch: self.patch + rhs.patch,
            pre_release: None,
            build: None,
        }
    }
}

pub struct RangeSemVer {
    min: SemVer,
    max: SemVer,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_construction() {
        let semver = SemVer::default();
        assert_eq!(semver, SemVer::ONE_MINOR);
    }

    #[test]
    fn construction_with_new() {
        assert_eq!(SemVer::new(0, 1, 0), SemVer::ONE_MINOR);
    }

    #[test]
    fn construction_with_pre_release() {
        let semver = SemVer {
            major: 0,
            minor: 1,
            patch: 0,
            pre_release: Some(String::from("alpha.1")),
            build: None,
        };
        assert_eq!(SemVer::new(0, 1, 0).with_pre_release("alpha.1"), semver);
    }

    #[test]
    fn construction_with_build() {
        let semver = SemVer {
            major: 0,
            minor: 1,
            patch: 0,
            pre_release: None,
            build: Some(String::from("123456")),
        };
        assert_eq!(SemVer::new(0, 1, 0).with_build("123456"), semver);
    }

    #[test]
    fn construction_with_pre_release_and_build() {
        let semver = SemVer {
            major: 0,
            minor: 1,
            patch: 0,
            pre_release: Some(String::from("alpha.1")),
            build: Some(String::from("123456")),
        };
        assert_eq!(
            SemVer::new(0, 1, 0)
                .with_pre_release("alpha.1")
                .with_build("123456"),
            semver
        );
    }

    #[test]
    fn correct_display() {
        assert!(false);
    }

    #[test]
    fn major_incrementation() {
        assert!(false);
    }

    #[test]
    fn minor_incrementation() {
        assert!(false);
    }

    #[test]
    fn patch_incrementation() {
        assert!(false);
    }

    #[test]
    fn semver_addition() {
        assert!(false);
    }
}
