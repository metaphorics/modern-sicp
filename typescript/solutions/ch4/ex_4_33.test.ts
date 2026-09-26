// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import { lazyDriverWith } from "../../packages/ch4/src/02-lazy.js";
import { answers, ex_4_33, liftedEvaluator } from "./ex_4_33.js";

describe("exercise 4.33: quote produces lazy lists", () => {
  it("the plain evaluator fails Ben's expression, the lifted one answers", async () => {
    const observed = await Effect.runPromise(answers());
    expect(observed.plain).toBe("not a procedure: (a b c)");
    expect(observed.lifted).toStrictEqual(["a", "b", "d"]);
  });

  it("leaves quoted atoms ordinary data", async () => {
    const transcript = await Effect.runPromise(
      lazyDriverWith(liftedEvaluator, ["'a", "'()", "(car (cdr '(1 2 3)))"]),
    );
    const values = transcript.filter((_, i) => i % 4 === 3);
    expect(values[0]).toBe("a");
    expect(values[1]).toBe("()");
    expect(values[2]).toBe("2");
  });

  it("reports the lift", () => {
    expect(ex_4_33()).toContain("true lazy pairs");
  });
});
