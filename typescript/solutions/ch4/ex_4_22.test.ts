// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { evaluate, type Outcome, Session } from "../../packages/ch4/src/01-metacircular.js";
import { format } from "../../packages/ch4/src/read.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import { bin, call, exprStmt, ident, lam, num, param } from "../../packages/ch4/src/syntax/ast.js";
import { noSpan } from "../../packages/ch4/src/syntax/diagnostics.js";
import { letNode, letToCall } from "./ex_4_06.js";
import { analyzeLet, evalAnalyzedLet } from "./ex_4_22.js";

const session = (): { session: Session; env: Env } => {
  const engine = new Session("core");
  return { session: engine, env: engine.globalEnv() };
};

const shown = (outcome: Outcome): string =>
  outcome.tag === "ok" ? format(outcome.value) : `error:${outcome.error.tag}`;

describe("exercise 4.22: let in the analyzed evaluator", () => {
  it("a grouped binding through analysis answers 7", () => {
    const { env } = session();
    const node = letNode(
      [{ name: "x", init: num(3) }],
      [exprStmt(bin("+", ident("x"), num(4)))],
      noSpan,
    );
    expect(shown(evalAnalyzedLet(node, env))).toBe("7");
  });

  it("a lowered let call in a procedure body answers 20 for 4", () => {
    const { env } = session();
    const inner = letNode(
      [{ name: "x", init: bin("*", ident("y"), ident("y")) }],
      [exprStmt(bin("+", ident("x"), ident("y")))],
      noSpan,
    );
    const f = lam([param("y")], [exprStmt(letToCall(inner))]);
    expect(shown(evalAnalyzedLet(call(f, [num(4)]), env))).toBe("20");
  });

  it("nested grouped bindings answer 12, agreeing with the derived form", () => {
    const nested = letNode(
      [{ name: "x", init: num(3) }],
      [
        exprStmt(
          letToCall(
            letNode(
              [{ name: "y", init: bin("+", ident("x"), num(1)) }],
              [exprStmt(bin("*", ident("x"), ident("y")))],
              noSpan,
            ),
          ),
        ),
      ],
      noSpan,
    );
    const { session: engine, env } = session();
    expect(shown(evalAnalyzedLet(nested, env))).toBe("12");
    const manual = call(
      lam(
        [param("x")],
        [
          exprStmt(
            call(lam([param("y")], [exprStmt(bin("*", ident("x"), ident("y")))]), [
              bin("+", ident("x"), num(1)),
            ]),
          ),
        ],
      ),
      [num(3)],
    );
    expect(shown(evaluate(manual, engine.globalEnv()))).toBe("12");
  });

  it("the execution procedure from analyzeLet answers 7 on a fresh environment", () => {
    const node = letNode(
      [{ name: "x", init: num(3) }],
      [exprStmt(bin("+", ident("x"), num(4)))],
      noSpan,
    );
    const procedure = analyzeLet(node);
    expect(shown(procedure(new Session("core").globalEnv()))).toBe("7");
    expect(letToCall(node).tag).toBe("call");
  });
});
