#![allow(clippy::expect_used)]

use std::process::Command;

use divan::Bencher;

fn main() {
    divan::main();
}

/// Exercises the Bazel-backed end-to-end benchmark path with a cheap,
/// deterministic EfficiencyCode invocation. Richer scenarios can add separate
/// benchmark binaries without making the shared harness depend on them.
#[divan::bench(sample_count = 20, sample_size = 1)]
fn EfficiencyCode_help(bencher: Bencher) {
    let EfficiencyCode = EfficiencyCode_utils_cargo_bin::cargo_bin("EfficiencyCode")
        .expect("EfficiencyCode binary should be available through Bazel runfiles");

    bencher.bench_local(move || {
        let output = Command::new(&EfficiencyCode)
            .arg("--help")
            .output()
            .expect("EfficiencyCode --help should run");
        assert!(output.status.success(), "EfficiencyCode --help should succeed");
    });
}
