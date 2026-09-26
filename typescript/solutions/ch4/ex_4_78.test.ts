// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { microshaft } from "../../packages/ch4/src/04-logic.js";
import { depthFirstOr, ex_4_78, orQuery } from "./ex_4_78.js";

describe("exercise 4.78: backtracking versus streams", () => {
  it("answers or depth-first where the stream engine interleaves", () => {
    expect(depthFirstOr()).toStrictEqual([
      "(or (supervisor (Hacker Alyssa P) (Bitdiddle Ben)) (supervisor (Hacker Alyssa P) (Hacker Alyssa P)))",
      "(or (supervisor (Fect Cy D) (Bitdiddle Ben)) (supervisor (Fect Cy D) (Hacker Alyssa P)))",
      "(or (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (supervisor (Tweakit Lem E) (Hacker Alyssa P)))",
      "(or (supervisor (Reasoner Louis) (Bitdiddle Ben)) (supervisor (Reasoner Louis) (Hacker Alyssa P)))",
    ]);
    expect(microshaft().answers(orQuery)).toStrictEqual([
      "(or (supervisor (Hacker Alyssa P) (Bitdiddle Ben)) (supervisor (Hacker Alyssa P) (Hacker Alyssa P)))",
      "(or (supervisor (Reasoner Louis) (Bitdiddle Ben)) (supervisor (Reasoner Louis) (Hacker Alyssa P)))",
      "(or (supervisor (Fect Cy D) (Bitdiddle Ben)) (supervisor (Fect Cy D) (Hacker Alyssa P)))",
      "(or (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (supervisor (Tweakit Lem E) (Hacker Alyssa P)))",
    ]);
  });

  it("names try-again as the subsuming mechanism", () => {
    expect(ex_4_78()).toContain("try-again");
  });
});
