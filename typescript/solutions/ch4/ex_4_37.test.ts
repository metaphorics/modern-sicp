// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import { library, tripleDefinition } from "./ex_4_35.js";
import {
  benDefinition,
  ex_4_37,
  firstWithFailures,
  gteDefinition,
  isqrtDefinition,
} from "./ex_4_37.js";

describe("exercise 4.37: Ben's triple generator", () => {
  it("both generators answer (3 4 5); Ben's prune cuts the failures", async () => {
    const plain = await Effect.runPromise(
      firstWithFailures(
        [library, tripleDefinition, "(a-pythagorean-triple-between 1 20)"].join("\n"),
      ),
    );
    const ben = await Effect.runPromise(
      firstWithFailures(
        [
          library,
          gteDefinition,
          isqrtDefinition,
          benDefinition,
          "(a-pythagorean-triple-between 1 20)",
        ].join("\n"),
      ),
    );
    expect(plain.answer).toBe("(3 4 5)");
    expect(ben.answer).toBe("(3 4 5)");
    expect(plain.failuresToFirst).toBe(918);
    expect(ben.failuresToFirst).toBe(81);
  });

  it("reports the counts", () => {
    expect(ex_4_37()).toContain("918");
  });
});
