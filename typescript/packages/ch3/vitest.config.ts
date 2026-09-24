// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    name: "ch3",
    include: [
      "src/**/*.test.ts",
      "../../examples/ch3/**/*.test.ts",
      "../../exercises/ch3/**/*.test.ts",
      "../../solutions/ch3/**/*.test.ts",
    ],
  },
});
