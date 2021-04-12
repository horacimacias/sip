//! SIP version
//!
//! This module contains a definition of the `Version` type. The `Version`
//! type is intended to be accessed through the root of the crate
//! (`sip::Version`) rather than this module.
//!
//! So far SIP only uses one version, "2.0", but this struct has been kept as it's a copy of http crate
//! ```

use std::fmt;

/// Represents a version of the SIP spec.
#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub struct Version(Sip);

impl Version {
    /// `SIP/2.0`
    pub const SIP_20: Version = Version(Sip::Sip20);
}

#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
enum Sip {
    Sip20,
    __NonExhaustive,
}

impl Default for Version {
    #[inline]
    fn default() -> Version {
        Version::SIP_20
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Sip::*;

        f.write_str(match self.0 {
            Sip20 => "SIP/2.0",
            __NonExhaustive => unreachable!(),
        })
    }
}
