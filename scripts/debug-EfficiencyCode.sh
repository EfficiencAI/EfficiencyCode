#!/bin/bash

# Set "chatgpt.cliExecutable": "/Users/<USERNAME>/code/EfficiencyCode/scripts/debug-EfficiencyCode.sh" in VSCode settings to always get the 
# latest EfficiencyCode-rs binary when debugging EfficiencyCode Extension.


set -euo pipefail

EfficiencyCode_RS_DIR=$(realpath "$(dirname "$0")/../EfficiencyCode-rs")
(cd "$EfficiencyCode_RS_DIR" && cargo run --quiet --bin EfficiencyCode -- "$@")