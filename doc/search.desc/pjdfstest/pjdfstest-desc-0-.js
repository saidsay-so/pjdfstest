searchState.loadedDescShard("pjdfstest", 0, "This is the main entry point for the test suite. It is …\nConfiguration for the test suite.\nProvides a context for tests through the <code>TestContext</code> and …\nFile-system features which are not available on every file …\nFile flags (see …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nMacros for defining test cases.\nRun provided test cases and filter according to features …\nTest framework for testing the filesystem implementation.\nWhere the tests are defined.\nUtility functions for filesystem operations.\nConfiguration for dummy authentication.\nConfiguration for the test suite.\nConfiguration for file-system specific features. Please …\nAdjustable file-system specific settings. Please see the …\nAllow remounting the file system with different settings …\nDummy authentication configuration.\nFile-system features.\nFile flags available in the file system.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nFile-system specific features which are enabled and do not …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nTime to sleep within tests (in seconds) between …\nSecondary file system to use for cross-file-system tests.\nFile-system specific settings.\nStores configuration for authentication.\nA struct to deserialize user/group names into <code>User</code>/<code>Group</code>.\nAuth entries, which are composed of a <code>User</code> and its …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nAuth entries which are composed of a <code>User</code> and its …\nAllows to create a file using builder pattern.\nFile type, mainly used with TestContext::create and …\nAn iterator over the variants of Self\nSerialized test context which allows to execute functions …\nTest context which allows to create files, directories, …\nExecute the function as another user/group(s). If <code>groups</code> …\nAuth entries which are composed of a <code>User</code> and its …\nReturn the base path for this context.\nCreate a file with a random name.\nCreate the file according to the provided information.\nCreate a regular file and open it.\nCreate a file whose name length is _PC_NAME_MAX.\nCreate a file whose path length is _PC_PATH_MAX.\nFeatures configuration, used to determine which features …\n<code>Take</code> and return the path final form.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGenerate a random path.\nReturns a new entry.\nReturns a new entry.\nReturns a new group. Alias of <code>get_new_entry</code>.\nReturns a new user. Alias of <code>get_new_entry</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nChange file mode.\nJoin <code>name</code> to the base path. An absolute path can also be …\nA short sleep, long enough for file system timestamps to …\nDuration to sleep, used to wait for file system timestamps …\nCreate a new test context.\nCreate a file builder.\nReturn a file builder.\nCreate the file according to the provided information and …\nTemporary directory where the test will be executed and …\nExecute the function with another umask.\nThe <code>chflags</code> command is available\nThe <code>SF_SNAPSHOT</code> flag can be set with <code>chflags</code>\nFeatures which are not available for every file system.\nAn iterator over the variants of Self\nNFSv4 style Access Control Lists are available\nThe <code>posix_fallocate</code> syscall is available\n<code>rename</code> changes <code>st_ctime</code> on success (POSIX does not require …\n<code>struct stat</code> contains an <code>st_birthtime</code> field\nThe <code>UTIME_NOW</code> constant is available\nThe <code>utimensat</code> syscall is available\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nFile flags (see …\nAn iterator over the variants of Self\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nMacro for defining test cases, which are automatically …\nFunction which indicates if the test should be skipped by …\nA single minimal test case.\nFunction which runs the test. The function is passed a …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nMetadata which isn’t related to time.\nA handy extention to std::os::unix::fs::MetadataExt\nBuilder to create a time metadata assertion, which …\nArgument to set which fields should be compared for …\nAssert that a certain operation changes the ctime of a …\nAssert that a certain operation does not change the ctime …\nAssert that a certain operation changes the mtime of a …\nAssert that a certain operation does not change the ctime …\nAlias for <code>TimeAssertion::new(false)</code>.\nAlias for <code>TimeAssertion::new(true)</code>.\nReturn the file’s last accessed time as a <code>TimeSpec</code>, …\nReturn the file’s last changed time as a <code>TimeSpec</code>, …\nHelper functions for testing error handling.\nBuild the assertion and asserts that <code>before</code> metadata is …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nBuilder functions for <code>mk</code>-family syscalls tests.\nReturn the file’s last modified time as a <code>TimeSpec</code>, …\nReturn a new builder. Comparison will be an equality check …\nAdd a path that should compare with itself.\nAdd paths that should compare.\nCreate a test case which asserts that the sycall returns …\nCreate a test case which asserts that the sycall returns …\nCreate a test case which asserts that the sycall returns …\nCreate a loop between two symbolic links and return them.\nCreate a test case which asserts that the sycall returns …\nCreate a test case which asserts that the sycall returns …\nCreate a test case which asserts that the sycall returns …\nCreate a test case which asserts that the sycall returns …\nCreate a test case which asserts that the sycall returns …\nCreate a test case which asserts that the sycall returns …\nCreate a test case which asserts that the sycall returns …\nCreate a test case which asserts that the sycall returns …\nCreate a test case which asserts that the sycall returns …\nCreate a test case which asserts that the sycall returns …\nCreate a test case which asserts that the sycall returns …\nCreate a test case which asserts that the syscall returns …\nCreate a test case which asserts that the syscall returns …\nGuard to allow execution of this test only if it’s …\nCreate a test case which asserts that the syscall returns …\nCreate a test case which asserts that the syscall returns …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nRemount the file system mounted at <code>mountpoint</code> with the …\nExecute a function with a read-only file system and …\nCreate a test case which asserts that it returns ETXTBSY …\nGuard which checks if a file system is mounted with the …\nCreate a test-case for a syscall which returns <code>EXDEV</code> when …\nGuard which checks if a secondary file system has been …\nAssert that the created entry gets its permission bits …\nAssert that the entry’s user ID is set to the process’ …\nDummy mountpoint to check that rmdir returns EBUSY when …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nWrapper for …\nGet mountpoint.\nWrapper for …\nWrapper for …\nWrapper for <code>linkat(None, old_path, None, new_path)</code>.\nWrapper for open which returns [<code>Ownedfd</code>] instead of [<code>RawFd</code>]…\nWrapper for <code>renameat(None, old_path, None, new_path)</code>.\nWrapper for <code>rmdir</code>.\nWrapper for <code>symlinkat(path1, None, path2)</code>.")