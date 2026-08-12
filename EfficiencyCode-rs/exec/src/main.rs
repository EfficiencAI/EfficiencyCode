//! Entry-point for the `EfficiencyCode-exec` binary.
//!
//! When this CLI is invoked normally, it parses the standard `EfficiencyCode-exec` CLI
//! options and launches the non-interactive EfficiencyCode agent. However, if it is
//! invoked with arg0 as `EfficiencyCode-linux-sandbox`, we instead treat the invocation
//! as a request to run the logic for the standalone `EfficiencyCode-linux-sandbox`
//! executable (i.e., parse any -s args and then run a *sandboxed* command under
//! Landlock + seccomp.
//!
//! This allows us to ship a completely separate set of functionality as part
//! of the `EfficiencyCode-exec` binary.
use EfficiencyCode_arg0::Arg0DispatchPaths;
use EfficiencyCode_arg0::arg0_dispatch_or_else;
use EfficiencyCode_exec::Cli;
use EfficiencyCode_exec::run_main;
use EfficiencyCode_utils_cli::CliConfigOverrides;
use clap::Parser;

#[derive(Parser, Debug)]
struct TopCli {
    #[clap(flatten)]
    config_overrides: CliConfigOverrides,

    #[clap(flatten)]
    inner: Cli,
}

fn main() -> anyhow::Result<()> {
    arg0_dispatch_or_else(|arg0_paths: Arg0DispatchPaths| async move {
        let top_cli = TopCli::parse();
        // Merge root-level overrides into inner CLI struct so downstream logic remains unchanged.
        let mut inner = top_cli.inner;
        inner
            .config_overrides
            .prepend_root_overrides(top_cli.config_overrides);

        run_main(inner, arg0_paths).await?;
        Ok(())
    })
}

#[cfg(test)]
#[path = "main_tests.rs"]
mod tests;
