// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    name: "ch0",
    include: [
      "src/**/*.test.ts",
      "../../examples/ch0/**/*.test.ts",
      "../../exercises/ch0/**/*.test.ts",
      "../../solutions/ch0/**/*.test.ts",
    ],
  },
});
