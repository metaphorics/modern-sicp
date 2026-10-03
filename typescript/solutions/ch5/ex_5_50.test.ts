// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { runSelfInterpretation } from "../../packages/ch5/src/06-selfinterp.ts";

const SELF_INTERPRETER_SOURCE = readFileSync(
  new URL(
    "../../../spec/host-subsets/typescript/witnesses/metacircular-evaluator.ts",
    import.meta.url,
  ),
  "utf8",
);

import { ex_5_50 } from "./ex_5_50.ts";

describe("exercise 5.50 compiled self-interpretation", () => {
  it("all three engines answer the guest program with the same value and transcript", () => {
    const direct = runSelfInterpretation("direct", SELF_INTERPRETER_SOURCE);
    expect(direct.outcome.tag).toBe("ok");
    expect(runSelfInterpretation("eceval", SELF_INTERPRETER_SOURCE).transcript).toEqual(
      direct.transcript,
    );
    expect(runSelfInterpretation("compiled", SELF_INTERPRETER_SOURCE).transcript).toEqual(
      direct.transcript,
    );
  });
  it("the solution returns the shared guest session transcript", () => {
    expect(ex_5_50()).toEqual(runSelfInterpretation("direct", SELF_INTERPRETER_SOURCE).transcript);
    expect(ex_5_50().length).toBeGreaterThan(0);
  });
});
