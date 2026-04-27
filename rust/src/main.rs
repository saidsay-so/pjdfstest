//! This is the main entry point for the test suite. It is responsible for parsing
//! command line arguments, reading the configuration file, and running the tests.
//! The test suite is composed of a set of test cases, each of which is a function
//! that takes a [`TestContext`] as an argument. The [`TestContext`] provides access to
//! the configuration, the dummy authentication entries, and a temporary directory
//! for the test to use.
//!
//! The test suite is built using the `inventory` crate, which
//! allows test cases to be registered at compile time. The test suite is run by
//! iterating over the registered test cases and running each one in turn.
//!
//! The [`TestContext`] is created for each test case, and the test case function is called
//! with the [`TestContext`] as an argument. The test case function can then use the
//! [`TestContext`] to access the configuration, the dummy authentication entries, and
//! the temporary directory. The test case function can perform whatever tests are
//! necessary, and panic if the test fails. The test suite catches the panic, prints
//! an error message, and continues running the remaining test cases. At the end of
//! the test suite, the number of failed, skipped, and passed tests is printed.

use std::{
    backtrace::{Backtrace, BacktraceStatus},
    collections::HashSet,
    env::current_dir,
    io::{stdout, Write},
    panic::{catch_unwind, set_hook},
    path::PathBuf,
    sync::Mutex,
};

use config::Config;
use figment::{
    providers::{Format, Serialized, Toml},
    Figment,
};
use gumdrop::Options;
use nix::{
    sys::stat::{umask, Mode},
    unistd::Uid,
};
use strum::{EnumMessage, IntoEnumIterator};

use tempfile::{tempdir_in, TempDir};

mod config;
mod context;
mod features;
mod flags;

mod macros;
pub(crate) use macros::*;

mod test;
mod tests;
mod utils;

use test::{FileSystemFeature, SerializedTestContext, TestCase, TestContext, TestFn};

use crate::utils::chmod;

static BACKTRACE: Mutex<Option<Backtrace>> = Mutex::new(None);

#[derive(Debug, Options)]
struct ArgOptions {
    #[options(help = "print help message")]
    help: bool,

    #[options(help = "Path of the configuration file")]
    configuration_file: Option<PathBuf>,

    #[options(help = "List opt-in features")]
    list_features: bool,

    #[options(help = "Match names exactly")]
    exact: bool,

    #[options(help = "Verbose mode")]
    verbose: bool,

    #[options(help = "Path where the test suite will be executed")]
    path: Option<PathBuf>,

    #[options(free, help = "Filter test names")]
    test_patterns: Vec<String>,

    #[options(help = "Path to a secondary file system")]
    secondary_fs: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, Default)]
struct OverallResult {
    pass: usize,
    fail: usize,
    skip: usize,
    expect_fail: usize,
    unexpect_pass: usize,
}

impl OverallResult {
    fn pass(&self) -> bool {
        self.fail + self.unexpect_pass == 0
    }

    fn total(&self) -> usize {
        self.pass + self.fail + self.skip + self.expect_fail + self.unexpect_pass
    }
}

fn main() -> anyhow::Result<()> {
    let args = ArgOptions::parse_args_default_or_exit();

    if args.list_features {
        for feature in FileSystemFeature::iter() {
            println!("{feature}: {}", feature.get_documentation().unwrap());
        }
        return Ok(());
    }

    let config: Config = {
        let mut figment = Figment::from(Serialized::defaults(Config::default()));
        if let Some(path) = args.configuration_file.as_deref() {
            figment = figment.merge(Toml::file(path))
        }

        let mut config: Config = figment.extract()?;
        config.features.secondary_fs = args.secondary_fs;
        config
    };

    let path = args
        .path
        .ok_or_else(|| anyhow::anyhow!("cannot get current dir"))
        .or_else(|_| current_dir())?;
    let base_dir = tempdir_in(path)?;

    set_hook(Box::new(|_| {
        *BACKTRACE.lock().unwrap() = Some(Backtrace::capture());
    }));

    let test_cases = inventory::iter::<TestCase>;
    let test_cases: Vec<_> = test_cases
        .into_iter()
        .filter(|case| {
            args.test_patterns.is_empty()
                || args.test_patterns.iter().any(|pat| {
                    if args.exact {
                        case.name == pat
                    } else {
                        case.name.contains(pat)
                    }
                })
        })
        .map(|tc: &TestCase| TestCase {
            // Ideally trim_start_matches could be done in test_case!, but only
            // const functions are allowed there.
            name: tc.name.trim_start_matches("pjdfstest::tests::"),
            description: tc.description,
            require_root: tc.require_root,
            fun: tc.fun,
            required_features: tc.required_features,
            guards: tc.guards,
        })
        .collect();

    umask(Mode::empty());

    let overall_result = run_test_cases(&test_cases, args.verbose, &config, base_dir)?;

    println!(
        "\nTests: {} failed, {} skipped, {} passed, {} expected failures {} total",
        overall_result.fail + overall_result.unexpect_pass,
        overall_result.skip,
        overall_result.pass,
        overall_result.expect_fail,
        overall_result.total()
    );

    if overall_result.pass() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Some tests have failed"))
    }
}

/// Run provided test cases and filter according to features and flags availability.
//TODO: Refactor this function
fn run_test_cases(
    test_cases: &[TestCase],
    verbose: bool,
    config: &Config,
    base_dir: TempDir,
) -> Result<OverallResult, anyhow::Error> {
    let mut failed_tests_count: usize = 0;
    let mut succeeded_tests_count: usize = 0;
    let mut skipped_tests_count: usize = 0;
    let mut unexpected_success_count: usize = 0;
    let mut expected_fail_count: usize = 0;

    let is_root = Uid::current().is_root();

    let enabled_features: HashSet<_> = config.features.fs_features.keys().collect();

    let entries = &config.dummy_auth.entries;

    for test_case in test_cases {
        //TODO: There's probably a better way to do this...
        let mut should_skip = test_case.require_root && !is_root;
        let mut skip_reasons = Vec::<String>::new();

        let expect_fail = config.settings.expected_failures.contains(test_case.name);
        if should_skip {
            skip_reasons.push(String::from("requires root privileges"));
        }

        let features: HashSet<_> = test_case.required_features.iter().collect();
        let missing_features: Vec<_> = features.difference(&enabled_features).collect();
        if !missing_features.is_empty() {
            should_skip = true;

            let features = &missing_features
                .iter()
                .map(|feature| format!("{}", feature))
                .collect::<Vec<_>>()
                .join(", ");

            skip_reasons.push(format!("requires features: {}", features));
        }

        let temp_dir = tempdir_in(base_dir.path()).unwrap();
        // FIX: some tests need a 0o755 base dir
        chmod(temp_dir.path(), Mode::from_bits_truncate(0o755)).unwrap();

        if test_case
            .guards
            .iter()
            .any(|guard| guard(config, temp_dir.path()).is_err())
        {
            should_skip = true;
            skip_reasons.extend(
                test_case
                    .guards
                    .iter()
                    .filter_map(|guard| guard(config, base_dir.path()).err())
                    .map(|err| err.to_string()),
            );
        }

        stdout().lock().flush()?;

        if should_skip {
            println!("{:72} skipped", test_case.name);
            if verbose && !test_case.description.is_empty() {
                println!("\t{}", test_case.description);
            }
            for reason in &skip_reasons {
                println!("\t{}", reason);
            }
            skipped_tests_count += 1;
            continue;
        }

        let result = catch_unwind(|| match test_case.fun {
            TestFn::NonSerialized(fun) => {
                let mut context = TestContext::new(config, entries, temp_dir.path());

                (fun)(&mut context)
            }
            TestFn::Serialized(fun) => {
                let mut context = SerializedTestContext::new(config, entries, temp_dir.path());

                (fun)(&mut context)
            }
        });

        match result {
            Ok(_) if !expect_fail => {
                println!("{:77} ok", test_case.name);
                succeeded_tests_count += 1;
            }
            Ok(_) => {
                println!("{:60} PASSED UNEXPECTEDLY", test_case.name);
                unexpected_success_count += 1;
            }
            Err(_) if expect_fail => {
                println!("{:61} failed as expected", test_case.name);
                expected_fail_count += 1;
            }
            Err(e) => {
                let backtrace = BACKTRACE
                    .lock()
                    .unwrap()
                    .take()
                    .filter(|bt| bt.status() == BacktraceStatus::Captured);
                let panic_information = match e.downcast::<String>() {
                    Ok(v) => *v,
                    Err(e) => match e.downcast::<&str>() {
                        Ok(v) => v.to_string(),
                        _ => "Unknown Source of Error".to_owned(),
                    },
                };
                println!("{:73} FAILED\n\t{}", test_case.name, panic_information);
                if let Some(backtrace) = backtrace {
                    println!("Backtrace:\n{}", backtrace);
                }
                failed_tests_count += 1;
            }
        }

        if verbose && !test_case.description.is_empty() {
            println!("\t{}", test_case.description);
        }
    }

    Ok(OverallResult {
        fail: failed_tests_count,
        skip: skipped_tests_count,
        pass: succeeded_tests_count,
        unexpect_pass: unexpected_success_count,
        expect_fail: expected_fail_count,
    })
}
