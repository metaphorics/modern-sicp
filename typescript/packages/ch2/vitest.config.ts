// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    name: "ch2",
    include: [
      "src/**/*.test.ts",
      "../../examples/ch2/**/*.test.ts",
      "../../exercises/ch2/**/*.test.ts",
      "../../solutions/ch2/**/*.test.ts",
    ],
  },
});
