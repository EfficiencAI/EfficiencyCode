use std::path::Path;

use EfficiencyCode_arg0::Arg0DispatchPaths;
use EfficiencyCode_arg0::Arg0PathEntryGuard;
use EfficiencyCode_arg0::arg0_dispatch;
use tempfile::TempDir;

pub struct TestBinaryDispatchGuard {
    _EfficiencyCode_home: TempDir,
    arg0: Arg0PathEntryGuard,
    _previous_EfficiencyCode_home: Option<std::ffi::OsString>,
}

impl TestBinaryDispatchGuard {
    pub fn paths(&self) -> &Arg0DispatchPaths {
        self.arg0.paths()
    }
}

pub enum TestBinaryDispatchMode {
    DispatchArg0Only,
    Skip,
    InstallAliases,
}

pub fn configure_test_binary_dispatch<F>(
    EfficiencyCode_home_prefix: &str,
    classify: F,
) -> Option<TestBinaryDispatchGuard>
where
    F: FnOnce(&str, Option<&str>) -> TestBinaryDispatchMode,
{
    let mut args = std::env::args_os();
    let argv0 = args.next().unwrap_or_default();
    let exe_name = Path::new(&argv0)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    let argv1 = args.next();
    match classify(exe_name, argv1.as_deref().and_then(|arg| arg.to_str())) {
        TestBinaryDispatchMode::DispatchArg0Only => {
            let _ = arg0_dispatch();
            None
        }
        TestBinaryDispatchMode::Skip => None,
        TestBinaryDispatchMode::InstallAliases => {
            let EfficiencyCode_home = match tempfile::Builder::new()
                .prefix(EfficiencyCode_home_prefix)
                .tempdir()
            {
                Ok(EfficiencyCode_home) => EfficiencyCode_home,
                Err(error) => panic!("failed to create test EfficiencyCode_HOME: {error}"),
            };
            let previous_EfficiencyCode_home = std::env::var_os("EfficiencyCode_HOME");
            // Safety: this runs from a test ctor before test threads begin.
            unsafe {
                std::env::set_var("EfficiencyCode_HOME", EfficiencyCode_home.path());
            }

            let arg0 = match arg0_dispatch() {
                Some(arg0) => arg0,
                None => panic!("failed to configure arg0 dispatch aliases for test binary"),
            };
            match previous_EfficiencyCode_home.as_ref() {
                Some(value) => unsafe {
                    std::env::set_var("EfficiencyCode_HOME", value);
                },
                None => unsafe {
                    std::env::remove_var("EfficiencyCode_HOME");
                },
            }

            Some(TestBinaryDispatchGuard {
                _EfficiencyCode_home: EfficiencyCode_home,
                arg0,
                _previous_EfficiencyCode_home: previous_EfficiencyCode_home,
            })
        }
    }
}
