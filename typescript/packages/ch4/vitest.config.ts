// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    name: "ch4",
    include: [
      "src/**/*.test.ts",
      "../../examples/ch4/**/*.test.ts",
      "../../exercises/ch4/**/*.test.ts",
      "../../solutions/ch4/**/*.test.ts",
    ],
  },
});
