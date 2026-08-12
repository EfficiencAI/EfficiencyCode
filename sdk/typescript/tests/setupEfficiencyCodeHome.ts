import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { afterEach, beforeEach } from "@jest/globals";

const originalEfficiencyCodeHome = process.env.EfficiencyCode_HOME;
let currentEfficiencyCodeHome: string | undefined;

beforeEach(async () => {
  currentEfficiencyCodeHome = await fs.mkdtemp(path.join(os.tmpdir(), "EfficiencyCode-sdk-test-"));
  process.env.EfficiencyCode_HOME = currentEfficiencyCodeHome;
});

afterEach(async () => {
  const EfficiencyCodeHomeToDelete = currentEfficiencyCodeHome;
  currentEfficiencyCodeHome = undefined;

  if (originalEfficiencyCodeHome === undefined) {
    delete process.env.EfficiencyCode_HOME;
  } else {
    process.env.EfficiencyCode_HOME = originalEfficiencyCodeHome;
  }

  if (EfficiencyCodeHomeToDelete) {
    await fs.rm(EfficiencyCodeHomeToDelete, { recursive: true, force: true });
  }
});
