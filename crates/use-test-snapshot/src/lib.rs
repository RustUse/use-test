#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

macro_rules! snapshot_string_type {
    ($type_name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        pub struct $type_name(pub String);

        impl $type_name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }

            pub fn into_string(self) -> String {
                self.0
            }
        }
    };
}

snapshot_string_type!(TestSnapshotId);
snapshot_string_type!(TestSnapshotName);
snapshot_string_type!(TestSnapshotVersion);

/// Snapshot comparison or review status.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TestSnapshotStatus {
    #[default]
    Missing,
    Created,
    Matched,
    Changed,
    Accepted,
    Rejected,
}

#[cfg(test)]
mod tests {
    use super::{TestSnapshotId, TestSnapshotName, TestSnapshotStatus, TestSnapshotVersion};

    #[test]
    fn stores_snapshot_identity_wrappers() {
        let id = TestSnapshotId::new("home-v1");
        let name = TestSnapshotName::new("home");
        let version = TestSnapshotVersion::new("v1");

        assert_eq!(id.as_str(), "home-v1");
        assert_eq!(name.as_str(), "home");
        assert_eq!(version.as_str(), "v1");
        assert_eq!(version.into_string(), "v1".to_string());
    }

    #[test]
    fn missing_is_default_status() {
        assert_eq!(TestSnapshotStatus::default(), TestSnapshotStatus::Missing);
    }
}
