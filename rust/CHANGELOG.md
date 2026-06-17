## [Unreleased] - ReleaseDate

### Added

- Added support for toolchains as old as Rust 1.74.0.  Users will need to
  downgrade several dependencies in order to make that work.  They can copy
  Cargo.lock.msrv to downgrade them all.
  ([#186](https://github.com/saidsay-so/pjdfstest/pull/186))

## [0.2.2] - 2026-06-16

### Fixed

- Fixed the build on 32-bit platforms.
  ([#183](https://github.com/saidsay-so/pjdfstest/pull/183))

## [0.2.1] - 2026-06-09

### Fixed

- Fixed the --secondary-fs argument
  ([#181](https://github.com/saidsay-so/pjdfstest/pull/181))

### Changed

- Removed the need for a third user from the config file.
  ([#180](https://github.com/saidsay-so/pjdfstest/pull/180))
