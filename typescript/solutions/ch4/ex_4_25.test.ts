// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { format } from "../../packages/ch4/src/read.js";
import { bool, call, ident, num } from "../../packages/ch4/src/syntax/ast.js";
import { runLazyFactorial, strictEnv } from "./ex_4_25.js";

describe("exercise 4.25: unless breaks under applicative order", () => {
  it("the lazy unless-based factorial answers 120", () => {
    const outcome = runLazyFactorial().outcome;
    expect(outcome.tag).toBe("ok");
    if (outcome.tag === "ok") {
      expect(format(outcome.value)).toBe("120");
    }
  });

  it("under strict arguments the unused arm runs before unless is entered", () => {
    const { env, session } = strictEnv(500);
    const outcome = session.evaluate(
      call(ident("unlessStrict"), [bool(true), call(ident("mark"), [num(1)]), num(42)]),
      env,
    );
    expect(outcome.tag).toBe("ok");
    if (outcome.tag === "ok") {
      expect(format(outcome.value)).toBe("42");
    }
    const marks = session.lookupVariableValue("marks", env);
    expect(marks.tag).toBe("ok");
    if (marks.tag === "ok") {
      expect(format(marks.value)).toBe("1");
    }
  });

  it("under strict arguments the failing arm reaches the caller first", () => {
    const { env, session } = strictEnv(500);
    const outcome = session.evaluate(
      call(ident("unlessStrict"), [bool(true), call(ident("boom"), []), num(42)]),
      env,
    );
    expect(outcome).toMatchObject({ tag: "error", error: { tag: "bad-operand" } });
  });

  it("the strict factorial reaches the evaluation budget", () => {
    const { env, session, spent } = strictEnv(100);
    const outcome = session.evaluate(call(ident("factStrict"), [num(5)]), env);
    expect(outcome).toMatchObject({ tag: "error", error: { tag: "bad-operand" } });
    expect(spent()).toBe(101);
  });
});
