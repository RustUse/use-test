#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Fixture lifetime or reuse scope.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TestFixtureScope {
    Case,
    Suite,
    Module,
    Session,
}

/// Fixture metadata without setup or teardown behavior.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TestFixture {
    pub name: String,
    pub scope: TestFixtureScope,
}

impl TestFixture {
    pub fn new(name: impl Into<String>, scope: TestFixtureScope) -> Self {
        Self {
            name: name.into(),
            scope,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn scope(&self) -> TestFixtureScope {
        self.scope
    }
}

#[cfg(test)]
mod tests {
    use super::{TestFixture, TestFixtureScope};

    #[test]
    fn creates_fixture_metadata() {
        let fixture = TestFixture::new("temp-dir", TestFixtureScope::Suite);

        assert_eq!(fixture.name(), "temp-dir");
        assert_eq!(fixture.scope(), TestFixtureScope::Suite);
    }
}
