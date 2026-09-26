// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import { lazyDriverWith, lazyEvaluator } from "../../packages/ch4/src/02-lazy.js";
import { answers, ex_4_28 } from "./ex_4_28.js";

describe("exercise 4.28: forcing the operator", () => {
  it("the forced operator dispatches and answers 5", async () => {
    const transcript = await Effect.runPromise(
      lazyDriverWith(lazyEvaluator, ["(define (id x) x)", "((id +) 2 3)"]),
    );
    expect(transcript[transcript.length - 1]).toBe("5");
  });

  it("the unforced variant hands the thunk to apply", async () => {
    const observed = await Effect.runPromise(answers());
    expect(observed.forced).toBe("5");
    expect(observed.unforced).toBe("not a procedure: #[thunk]");
  });

  it("explains the need for the forcing", () => {
    expect(ex_4_28()).toContain("actual-value");
  });
});
