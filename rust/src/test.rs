use std::path::Path;

use crate::config::Config;
pub use crate::features::*;
pub use crate::flags::*;
pub use crate::runner::context::{SerializedTestContext, TestContext};

/// Function which indicates if the test should be skipped by returning an error.
pub type Guard = fn(&Config, &Path) -> Result<(), anyhow::Error>;

pub enum TestFn {
    Serialized(fn(&mut SerializedTestContext)),
    NonSerialized(fn(&mut TestContext)),
}

/// A single minimal test case.
pub struct TestCase {
    pub name: &'static str,
    pub description: &'static str,
    pub require_root: bool,
    pub fun: TestFn,
    pub required_features: &'static [FileSystemFeature],
    pub guards: &'static [Guard],
}

inventory::collect!(TestCase);
