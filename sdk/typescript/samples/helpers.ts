import path from "node:path";

export function EfficiencyCodePathOverride() {
  return (
    process.env.EfficiencyCode_EXECUTABLE ??
    path.join(process.cwd(), "..", "..", "EfficiencyCode-rs", "target", "debug", "EfficiencyCode")
  );
}
