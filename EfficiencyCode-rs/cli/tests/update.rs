use anyhow::Result;
use predicates::str::contains;
use std::path::Path;
use tempfile::TempDir;

fn EfficiencyCode_command(EfficiencyCode_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd =
        assert_cmd::Command::new(EfficiencyCode_utils_cargo_bin::cargo_bin("EfficiencyCode")?);
    cmd.env("EfficiencyCode_HOME", EfficiencyCode_home);
    Ok(cmd)
}

#[cfg(debug_assertions)]
#[tokio::test]
async fn update_does_not_start_interactive_prompt() -> Result<()> {
    let EfficiencyCode_home = TempDir::new()?;

    EfficiencyCode_command(EfficiencyCode_home.path())?
        .arg("update")
        .assert()
        .failure()
        .stderr(contains(
            "`EfficiencyCode update` is not available in debug builds",
        ));

    Ok(())
}
