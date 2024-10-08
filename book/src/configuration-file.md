# Configuration file

The test runner can read a configuration file. For now, only the TOML format is supported.
Its path can be specified by using the `-c PATH` flag.

## Sections

### [features]

Some features are not available for every file system.
For tests requiring such features,
the execution becomes opt-in.
The user can enable their execution,
by adding the corresponding feature as a key in this section.
A list of these opt-in features is provided
when executing the runner with `-l` argument.

For example, with `posix_fallocate`:

```toml
[features]
posix_fallocate = {}

# Can also be specified by using key notation
# [features.posix_fallocate]
```

#### Feature list

The following features can be enabled but do not require any additional configuration:

<!-- cmdrun python3 ../list_features.py -->

Following features require additional configuration.

#### file_flags

Some tests are related to file flags.
However, not all file systems and operating systems support all flags.
To give a sufficient level of granularity, each supported flag can be
specified in the configuration with the `file_flags` array.

```toml
[features]
posix_fallocate = {}
file_flags = ["UF_IMMUTABLE"]
```

#### secondary_fs

Some tests require a secondary file system.
This can be specified in the configuration with the `secondary_fs` key,
but also with the `secondary_fs` argument.
The argument takes precedence over the configuration.

```toml
[features]
secondary_fs = "/mnt/ISO"
```

### [dummy_auth]

This section allows to modify the mechanism for switching users, which is required by some tests.

```toml
[dummy_auth]
entries = [
  ["nobody", "nobody"],
  # nogroup instead for some Linux distros
  # ["nobody", "nogroup"],
  ["tests", "tests"],
  ["pjdfstest", "pjdfstest"],
]
```

- `entries` - An entry is composed of a username and its associated group.
  Exactly 3 entries need to be specified if the default ones cannot be used.

### [settings]

```toml
[settings]
naptime = 0.001
allow_remount = false
```

- `naptime` - The duration for a "short" sleep. It should be greater than the
  timestamp granularity of the file system under test. The default value is 1
  second.
- `allow_remount` - If set to `true`, the runner will run the EROFS tests,
  which require to remount the file system on which
  pjdsfstest is run as read-only.
