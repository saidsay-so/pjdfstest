use std::{
    collections::HashSet,
    io::{stdout, Write},
    panic::{catch_unwind, set_hook, AssertUnwindSafe},
    path::{Path, PathBuf},
};

use config::Config;
use figment::{
    providers::{Format, Toml},
    Figment,
};
use gumdrop::Options;
use once_cell::sync::OnceCell;
use strum::IntoEnumIterator;

use pjdfs_tests::test::{FileSystemFeature, TestContext, TEST_CASES};

mod config;

struct PanicLocation(u32, u32, String);

static PANIC_LOCATION: OnceCell<PanicLocation> = OnceCell::new();

#[derive(Debug, Options)]
struct ArgOptions {
    #[options(help = "print help message")]
    help: bool,

    #[options(help = "Path of the configuration file")]
    configuration_file: Option<PathBuf>,

    #[options(help = "List opt-in features")]
    list_features: bool,
}

fn main() -> anyhow::Result<()> {
    let args = ArgOptions::parse_args_default_or_exit();

    if args.list_features {
        for feature in FileSystemFeature::iter() {
            println!("{}", feature);
        }
        return Ok(());
    }

    let config: Config = Figment::new()
        .merge(Toml::file(
            args.configuration_file
                .as_deref()
                .unwrap_or(Path::new("pjdfstest.toml")),
        ))
        .extract()?;

    let enabled_features: HashSet<_> = config
        .features
        .fs_features
        .keys()
        .into_iter()
        .cloned()
        .collect();

    set_hook(Box::new(|ctx| {
        if let Some(location) = ctx.location() {
            let _ = PANIC_LOCATION.set(PanicLocation(
                location.line(),
                location.column(),
                location.file().into(),
            ));
        } else {
            unimplemented!()
        }
    }));

    #[cfg(any(
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "dragonfly",
        target_os = "macos",
        target_os = "ios",
        target_os = "watchos",
    ))]
    let enabled_flags: HashSet<_> = config
        .features
        .file_flags
        .unwrap_or_default()
        .iter()
        .cloned()
        .collect();

    for test_case in TEST_CASES {
        //TODO: There's probably a better way to do this...
        let mut should_skip = false;

        let mut message = None;

        let features = test_case
            .required_features
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
        let missing_features = features.difference(&enabled_features);
        if missing_features.clone().count() > 0 {
            should_skip = true;

            let message = message.get_or_insert(String::new());
            *message += "please add the following features to your configuration:\n";
            *message += &missing_features
                .map(|feature| format!("\t[features.{}]\n", feature))
                .collect::<String>();
        }

        #[cfg(any(
            target_os = "openbsd",
            target_os = "netbsd",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "macos",
            target_os = "ios",
            target_os = "watchos",
        ))]
        {
            let required_flags: HashSet<_> =
                test_case.required_file_flags.iter().cloned().collect();
            let missing_flags = required_flags.difference(&enabled_flags);

            if missing_flags.clone().count() > 0 {
                should_skip = true;

                let flags: String = missing_flags
                    .map(|f| {
                        let f = f.to_string();

                        ["\"", f.as_str(), "\""].join("")
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                let message = message.get_or_insert(String::new());
                *message += "please add the following flags to your configuration:\n";
                *message += &format!("\tfile_flags = [{}]\n", flags);
            }
        }

        if should_skip {
            println!(
                "skipped '{}'\n{}",
                test_case.name,
                message.unwrap_or_default()
            );
            continue;
        }

        print!("{}\t", test_case.name);
        stdout().lock().flush()?;
        let mut context = TestContext::new();
        //TODO: AssertUnwindSafe should be used with caution
        let mut ctx_wrapper = AssertUnwindSafe(&mut context);
        match catch_unwind(move || {
            (test_case.fun)(&mut ctx_wrapper);
        }) {
            Ok(_) => println!("success"),
            Err(e) => {
                let location = PANIC_LOCATION.get().unwrap();
                anyhow::bail!(
                    "{}
                    Located in file {} at {}:{}
                    ",
                    e.downcast_ref::<String>()
                        .cloned()
                        .or_else(|| e.downcast_ref::<&str>().map(|&s| s.to_string()))
                        .unwrap_or_default(),
                    location.2,
                    location.0,
                    location.1
                )
            }
        }
    }

    Ok(())
}
