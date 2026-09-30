// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { runSource } from "../../packages/ch4/src/01-metacircular.js";
import { format } from "../../packages/ch4/src/read.js";
import { child } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { fail, ok } from "../../packages/ch4/src/runtime/errors.js";
import {
  isArrayValue,
  makeArray,
  makeClosure,
  makePrimitive,
  type Value,
} from "../../packages/ch4/src/runtime/value.js";
import { block, returnStmt } from "../../packages/ch4/src/syntax/ast.js";
import { evaMapSource, makeHostMap } from "./ex_4_14.js";

/** The observable result: the rendered value, or the fault category and detail. */
const shown = (outcome: Outcome): string => {
  if (outcome.tag === "ok") {
    return format(outcome.value);
  }
  return outcome.error.tag === "bad-operand"
    ? `error:bad-operand:${outcome.error.detail}`
    : `error:${outcome.error.tag}`;
};

const headPrimitive = makePrimitive("head", (args: ReadonlyArray<Value>): Outcome => {
  const first = args[0];
  return isArrayValue(first)
    ? ok(first.items[0])
    : fail({ tag: "bad-operand", operator: "head", detail: "not an array" });
});

const pairs = makeArray([makeArray([1, 2]), makeArray([3, 4])]);

describe("exercise 4.14: two maps, two fates", () => {
  it("Louis's host map calls a host primitive per element: [1, 3]", () => {
    expect(shown(makeHostMap().fn([headPrimitive, pairs]))).toBe("[1, 3]");
  });

  it("Louis's host map cannot call an evaluator closure", () => {
    const identity = makeClosure(
      ["x"],
      null,
      block([
        returnStmt({ tag: "variable", name: "x", span: { start: 0, end: 0, line: 1, column: 1 } }),
      ]),
      child(null),
    );
    expect(shown(makeHostMap().fn([identity, makeArray([makeArray([9])])]))).toBe(
      "error:bad-operand:map: the host could not call this procedure",
    );
  });

  it("Eva's map answers [1, 3] with head and [1, 4, 9] with square", () => {
    const heads = runSource(`${evaMapSource}\nmap(head, [[1, 2], [3, 4]]);`);
    expect(shown(heads.outcome)).toBe("[1, 3]");
    const squares = runSource(`${evaMapSource}\nmap(square, [1, 2, 3]);`);
    expect(shown(squares.outcome)).toBe("[1, 4, 9]");
  });

  it("Eva's map handles the call that killed Louis's map: [[9]]", () => {
    const identities = runSource(`${evaMapSource}\nmap((x: number[]) => x, [[9]]);`);
    expect(shown(identities.outcome)).toBe("[[9]]");
  });
});
