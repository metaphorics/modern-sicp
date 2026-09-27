// SPDX-License-Identifier: GPL-3.0-only
// Original tests

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import {
  contractQuestionMark,
  Frame,
  isVar,
  makeQueryEngine,
  microshaft,
  patternMatch,
  queryDriverLoop,
  readQuery,
  unifyMatch,
} from "./04-logic.js";
import { format, read } from "./read.js";

describe("section 4.4 query evaluator", () => {
  it("matches a pattern and returns an immutable binding frame", () => {
    const query = readQuery("(parent ?child ?parent)");
    const fact = read("(parent Ada Grace)");
    const frame = patternMatch(query, fact, new Frame());
    expect(frame).toBeDefined();
    expect(
      format(
        frame === undefined
          ? read("#f")
          : (frame.bindingInFrame(readQuery("?child")) ?? read("#f")),
      ),
    ).toBe("Ada");
    expect(frame?.isEmpty).toBe(false);
  });

  it("unifies both directions and rejects a cyclic binding with the occurs check", () => {
    const x = readQuery("?x");
    const y = readQuery("?y");
    const frame = unifyMatch(x, y, new Frame());
    expect(frame).toBeDefined();
    expect(unifyMatch(x, readQuery("(f ?x)"), new Frame())).toBeUndefined();
    expect(contractQuestionMark(x)).toEqual(read("?x"));
    expect(isVar(x)).toBe(true);
  });

  it("answers simple and compound Microshaft queries in database order", () => {
    const engine = microshaft();
    expect(engine.answers("(supervisor ?name (Bitdiddle Ben))")).toEqual([
      "(supervisor (Hacker Alyssa P) (Bitdiddle Ben))",
      "(supervisor (Fect Cy D) (Bitdiddle Ben))",
      "(supervisor (Tweakit Lem E) (Bitdiddle Ben))",
    ]);
    expect(
      engine.answers("(and (job ?name (computer programmer)) (supervisor ?name (Bitdiddle Ben)))"),
    ).toEqual([
      "(and (job (Hacker Alyssa P) (computer programmer)) (supervisor (Hacker Alyssa P) (Bitdiddle Ben)))",
      "(and (job (Fect Cy D) (computer programmer)) (supervisor (Fect Cy D) (Bitdiddle Ben)))",
    ]);
  });

  it("matches dotted-tail patterns and prints their contracted form", () => {
    expect(microshaft().answers("(job ?name (accounting . ?type))")).toEqual([
      "(job (Scrooge Eben) (accounting chief accountant))",
      "(job (Cratchet Robert) (accounting scrivener))",
    ]);
  });

  it("applies recursively renamed rules", () => {
    const engine = makeQueryEngine();
    engine.load(
      "(parent Ada Grace) (parent Grace Lin) (rule (grandparent ?x ?z) (and (parent ?x ?y) (parent ?y ?z)))",
    );
    expect(engine.answers("(grandparent ?who ?descendant)")).toEqual(["(grandparent Ada Lin)"]);
  });

  it("implements delayed not, always-true, and assertion driver input", async () => {
    const engine = microshaft();
    expect(
      engine.answers(
        "(and (job ?name (computer programmer)) (not (supervisor ?name (Bitdiddle Ben))))",
      ),
    ).toEqual([]);
    expect(engine.answers("(and (always-true) (job ?name (computer wizard)))")).toEqual([
      "(and (always-true) (job (Bitdiddle Ben) (computer wizard)))",
    ]);
    const output = await Effect.runPromise(
      queryDriverLoop(engine, [
        "(assert! (job (New Person) (computer programmer)))",
        "(job ?name (computer programmer))",
      ]),
    );
    expect(output).toContain("Assertion added to data base.");
    expect(output).toContain("  (job (New Person) (computer programmer))");
  });

  it("applies a registered lisp-value predicate to instantiated arguments", () => {
    const engine = microshaft();
    engine.setPredicates({
      "salary-above-50000": (args) => {
        const salary = args[0];
        return salary !== undefined && salary._tag === "Number" && salary.n > 50000;
      },
    });
    expect(
      engine.answers("(and (salary ?person ?amount) (lisp-value salary-above-50000 ?amount))"),
    ).toEqual([
      "(and (salary (Bitdiddle Ben) 60000) (lisp-value salary-above-50000 60000))",
      "(and (salary (Warbucks Oliver) 150000) (lisp-value salary-above-50000 150000))",
      "(and (salary (Scrooge Eben) 75000) (lisp-value salary-above-50000 75000))",
    ]);
  });
});
