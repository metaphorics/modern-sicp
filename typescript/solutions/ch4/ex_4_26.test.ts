// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import { taggedList } from "../../packages/ch4/src/01-metacircular.js";
import { read } from "../../packages/ch4/src/read.js";
import { alyssaSession, benSession, ex_4_26, unlessToIf } from "./ex_4_26.js";

const valuesOf = (transcript: ReadonlyArray<string>): ReadonlyArray<string> =>
  transcript.filter((_, i) => i % 4 === 3);

describe("exercise 4.26: unless as special form debate", () => {
  it("the derived unless picks the arm before anything evaluates", () => {
    const rewritten = unlessToIf(read("(unless (= 1 1) (/ 1 0) 42)"));
    expect(taggedList("if", rewritten)).toBe(true);
    expect(unlessToIf(read("(car '(a b))"))).toStrictEqual(read("(car '(a b))"));
  });

  it("Ben's special form answers armed calls and mappings, not names", async () => {
    const ben = await Effect.runPromise(benSession());
    expect(ben.armed).toBe("42");
    expect(ben.mapped).toBe("(0 7)");
    expect(ben.nameError).toBe("UnboundVariable: unless");
  });

  it("Alyssa's lazy procedure answers the same and stays first-class", async () => {
    const alyssa = valuesOf(await Effect.runPromise(alyssaSession()));
    expect(alyssa).toStrictEqual(["ok", "ok", "42", "(0 7)", "ok", "7"]);
  });

  it("reports both sides", () => {
    const report = ex_4_26();
    expect(report).toContain("UnboundVariable: unless");
    expect(report).toContain("first-class");
  });
});
