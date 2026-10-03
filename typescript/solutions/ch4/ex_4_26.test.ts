// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { evaluate, Session } from "../../packages/ch4/src/01-metacircular.js";
import { bool, call, ident, num } from "../../packages/ch4/src/syntax/ast.js";
import { alyssaSession, benDerived, unlessNode, unlessToIf } from "./ex_4_26.js";

describe("exercise 4.26: unless as a special form, the debate", () => {
  it("the derived form picks the arm before anything evaluates", () => {
    const { env } = { env: new Session("core").globalEnv() };
    const armed = unlessNode(bool(true), call(ident("missing"), []), num(42));
    expect(unlessToIf(armed).tag).toBe("conditional");
    expect(unlessToIf(num(1)).tag).toBe("number");
    const outcome = benDerived(armed, env);
    expect(outcome.tag).toBe("ok");
    if (outcome.tag === "ok") {
      expect(outcome.value).toBe(42);
    }
  });

  it("the derived mapping answers [0, 7]", () => {
    const env = new Session("core").globalEnv();
    const onFalse = benDerived(unlessNode(bool(false), num(0), num(7)), env);
    const onTrue = benDerived(unlessNode(bool(true), num(0), num(7)), env);
    expect([
      onFalse.tag === "ok" ? onFalse.value : null,
      onTrue.tag === "ok" ? onTrue.value : null,
    ]).toEqual([0, 7]);
  });

  it("Ben's name stays syntax: reading unless fails with unbound-name", () => {
    const outcome = evaluate(ident("unless"), new Session("core").globalEnv());
    expect(outcome.tag).toBe("error");
    if (outcome.tag === "error" && outcome.error.tag === "unbound-name") {
      expect(outcome.error.name).toBe("unless");
    }
  });

  it("Alyssa's lazy procedure answers the same calls and stays first-class", () => {
    const result = alyssaSession();
    expect(result.transcript).toEqual(["42", "[0, 7]", "7"]);
  });
});
