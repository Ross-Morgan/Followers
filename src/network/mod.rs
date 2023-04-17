mod version;

use version::Version;

pub struct SocialNetwork {
    version: Version,
}

impl SocialNetwork {
    pub fn version(&self) -> String { self.raw_version().to_string() }
    pub(self) fn raw_version(&self) -> Version { self.version }
}