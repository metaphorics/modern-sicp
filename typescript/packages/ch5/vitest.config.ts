// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    name: "ch5",
    include: [
      "src/**/*.test.ts",
      "../../examples/ch5/**/*.test.ts",
      "../../exercises/ch5/**/*.test.ts",
      "../../solutions/ch5/**/*.test.ts",
    ],
  },
});
