// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    name: "ch1",
    include: [
      "src/**/*.test.ts",
      "../../examples/ch1/**/*.test.ts",
      "../../exercises/ch1/**/*.test.ts",
      "../../solutions/ch1/**/*.test.ts",
    ],
  },
});
