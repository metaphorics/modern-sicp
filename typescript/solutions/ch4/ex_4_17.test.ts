// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { Session } from "../../packages/ch4/src/01-metacircular.js";
import { format } from "../../packages/ch4/src/read.js";
import { child } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import {
  type Closure,
  isClosure,
  makeClosure,
  type Value,
} from "../../packages/ch4/src/runtime/value.js";
import {
  bin,
  block,
  type Decl,
  ident,
  lam,
  num,
  returnStmt,
  type Stmt,
  varDecl,
} from "../../packages/ch4/src/syntax/ast.js";
import { noSpan } from "../../packages/ch4/src/syntax/diagnostics.js";
import {
  applySameFrame,
  applyScannedExtraFrame,
  applySequential,
  environmentDepth,
} from "./ex_4_17.js";

const session = (): { session: Session; env: ReturnType<typeof child> } => {
  const engine = new Session("core");
  return { session: engine, env: engine.globalEnv() };
};

const shown = (outcome: Outcome): string =>
  outcome.tag === "ok" ? format(outcome.value) : `error:${outcome.error.tag}`;

const procedureOf = (items: ReadonlyArray<Decl | Stmt>): Closure =>
  makeClosure([], null, block(items), child(null));

describe("exercise 4.17: the extra frame of scanned-out definitions", () => {
  it("the same body answers 3 under all three application strategies", () => {
    const items: Array<Decl | Stmt> = [
      varDecl("const", "a", num(1), null, noSpan),
      varDecl("const", "b", bin("+", ident("a"), num(1)), null, noSpan),
      returnStmt(bin("+", ident("a"), ident("b")), noSpan),
    ];
    const first = session();
    expect(shown(applySequential(procedureOf(items), [], first.session))).toBe("3");
    const second = session();
    expect(shown(applyScannedExtraFrame(procedureOf(items), [], second.session))).toBe("3");
    const third = session();
    expect(shown(applySameFrame(procedureOf(items), [], third.session))).toBe("3");
  });

  it("the scanned variants differ from sequential by exactly one frame", () => {
    const items: Array<Decl | Stmt> = [
      varDecl("const", "a", num(1), null, noSpan),
      returnStmt(lam([], [returnStmt(ident("a"))]), noSpan),
    ];
    const depths: number[] = [];
    for (const apply of [applySequential, applyScannedExtraFrame, applySameFrame]) {
      const { session: engine, env } = session();
      const outcome = apply(procedureOf(items), [], engine);
      expect(outcome.tag).toBe("ok");
      if (outcome.tag === "ok" && isClosure(outcome.value)) {
        depths.push(environmentDepth(outcome.value.env));
      }
    }
    expect(depths).toEqual([2, 3, 2]);
  });

  it("the same-frame closure still answers 1", () => {
    const items: Array<Decl | Stmt> = [
      varDecl("const", "a", num(1), null, noSpan),
      returnStmt(lam([], [returnStmt(ident("a"))]), noSpan),
    ];
    const { session: engine } = session();
    const outcome = applySameFrame(procedureOf(items), [], engine);
    expect(outcome.tag).toBe("ok");
    if (outcome.tag === "ok" && isClosure(outcome.value)) {
      expect(shown(applySequential(outcome.value, [], engine))).toBe("1");
    }
  });
});
