use std::path::PathBuf;

use EfficiencyCode_utils_absolute_path::AbsolutePathBuf;

/// Runtime paths needed by exec-server child processes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecServerRuntimePaths {
    /// Stable path to the EfficiencyCode executable used to launch hidden helper modes.
    pub EfficiencyCode_self_exe: AbsolutePathBuf,
    /// Path to the Linux sandbox helper alias used when the platform sandbox
    /// needs to re-enter EfficiencyCode by argv0.
    pub EfficiencyCode_linux_sandbox_exe: Option<AbsolutePathBuf>,
}

impl ExecServerRuntimePaths {
    pub fn from_optional_paths(
        EfficiencyCode_self_exe: Option<PathBuf>,
        EfficiencyCode_linux_sandbox_exe: Option<PathBuf>,
    ) -> std::io::Result<Self> {
        let EfficiencyCode_self_exe = EfficiencyCode_self_exe.ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "EfficiencyCode executable path is not configured",
            )
        })?;
        Self::new(EfficiencyCode_self_exe, EfficiencyCode_linux_sandbox_exe)
    }

    pub fn new(
        EfficiencyCode_self_exe: PathBuf,
        EfficiencyCode_linux_sandbox_exe: Option<PathBuf>,
    ) -> std::io::Result<Self> {
        Ok(Self {
            EfficiencyCode_self_exe: absolute_path(EfficiencyCode_self_exe)?,
            EfficiencyCode_linux_sandbox_exe: EfficiencyCode_linux_sandbox_exe
                .map(absolute_path)
                .transpose()?,
        })
    }
}

fn absolute_path(path: PathBuf) -> std::io::Result<AbsolutePathBuf> {
    AbsolutePathBuf::from_absolute_path(path.as_path())
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput, err))
}
