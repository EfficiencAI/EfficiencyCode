#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
minimum_supported_release="$(
  sed -n 's/^pub const MINIMUM_SUPPORTED_EfficiencyCode_VERSION: &str = "\([^"]*\)";$/\1/p' \
    "${repo_root}/EfficiencyCode-rs/exec-server-protocol/src/lib.rs"
)"
: "${minimum_supported_release:?minimum supported EfficiencyCode release is missing}"
release_directory="$(mktemp -d "${TMPDIR:-/tmp}/EfficiencyCode-exec-server-skew.XXXXXX")"
trap 'rm -rf "${release_directory:?}"' EXIT

if [[ $# -eq 0 ]]; then
  releases=(latest "${minimum_supported_release}")
else
  releases=("$@")
fi

case "$(uname -s):$(uname -m)" in
  Darwin:arm64) target="aarch64-apple-darwin" ;;
  Darwin:x86_64) target="x86_64-apple-darwin" ;;
  Linux:aarch64 | Linux:arm64) target="aarch64-unknown-linux-musl" ;;
  Linux:x86_64) target="x86_64-unknown-linux-musl" ;;
  *)
    echo "Unsupported platform: $(uname -s) $(uname -m)" >&2
    exit 1
    ;;
esac

asset="EfficiencyCode-${target}.tar.gz"
cd "${repo_root}/EfficiencyCode-rs"
cargo build -p EfficiencyCode-cli --bin EfficiencyCode
export EfficiencyCode_TEST_CURRENT_EfficiencyCode="${CARGO_TARGET_DIR:-${repo_root}/EfficiencyCode-rs/target}/debug/EfficiencyCode"

echo "Testing current EfficiencyCode compatibility through authenticated Noise"
export EfficiencyCode_TEST_RELEASED_EfficiencyCode="${EfficiencyCode_TEST_CURRENT_EfficiencyCode}"
just test -p EfficiencyCode-exec-server --test relay version_skew --test-threads 1

tested_release_version=""
for release in "${releases[@]}"; do
  release="${release#rust-v}"
  if [[ "${release}" == "${tested_release_version}" ]]; then
    echo "Skipping EfficiencyCode ${release}; this release was already tested"
    continue
  fi

  if [[ "${release}" == "latest" ]]; then
    release_url="https://github.com/openai/EfficiencyCode/releases/latest/download/${asset}"
  else
    release_url="https://github.com/openai/EfficiencyCode/releases/download/rust-v${release}/${asset}"
  fi

  binary_directory="${release_directory}/${release}"
  mkdir -p "${binary_directory}"
  echo "Downloading released EfficiencyCode from ${release_url}"
  curl -fsSL "${release_url}" -o "${binary_directory}/${asset}"
  tar -xzf "${binary_directory}/${asset}" -C "${binary_directory}"

  export EfficiencyCode_TEST_RELEASED_EfficiencyCode="${binary_directory}/EfficiencyCode-${target}"
  release_output="$("${EfficiencyCode_TEST_RELEASED_EfficiencyCode}" --version)"
  echo "${release_output}"
  tested_release_version="${release_output##* }"

  just test -p EfficiencyCode-exec-server --test relay version_skew --test-threads 1
done
