// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { builtinMember } from "../../packages/ch4/src/runtime/builtins.ts";
import {
  ArrayValue,
  ErrorValue,
  MapValue,
  RecordValue,
  SetValue,
} from "../../packages/ch4/src/runtime/value.ts";
import { MachineErrorValue, makeEvaluator, type Word } from "../../packages/ch5/src/04-eceval.ts";
import type { Operation } from "./ex_5_07.ts";

/** Exercise 5.30: error signaling inside the evaluator. The checking
 * operations trap division by zero and the bad selectors of member and
 * index access, answering a typed error value the machine raises like
 * any guest throw: the run reports the error category and no plausible
 * value. Correct programs are left untouched. */
export const makeCheckingOperations = (): Readonly<Record<string, Operation<Word>>> => ({
  binaryValue: (args) => {
    const operator = args[0];
    const left = args[1];
    const right = args[2];
    if (operator === "/" && typeof right === "number" && right === 0) {
      return new MachineErrorValue({
        tag: "bad-operand",
        operator: "/",
        detail: "division by zero",
      });
    }
    if (operator === "%" && typeof right === "number" && right === 0) {
      return new MachineErrorValue({
        tag: "bad-operand",
        operator: "%",
        detail: "remainder by zero",
      });
    }
    return baseBinary(operator, left, right);
  },
  memberGet: (args) => {
    const object = args[0];
    const name = args[1];
    if (object instanceof RecordValue && typeof name === "string" && object.fields.has(name)) {
      return object.fields.get(name);
    }
    const value = baseMemberGet(object, name);
    if (value !== undefined) return value;
    return new MachineErrorValue({
      tag: "unknown-field",
      field: typeof name === "string" ? name : String(name),
    });
  },
  indexGet: (args) => {
    const object = args[0];
    const index = args[1];
    if (object instanceof ArrayValue && typeof index === "number") {
      if (!Number.isInteger(index) || index < 0 || index >= object.items.length) {
        return new MachineErrorValue({
          tag: "bad-operand",
          operator: "index",
          detail: String(index),
        });
      }
      return baseIndexGet(object, index);
    }
    return new MachineErrorValue({ tag: "bad-operand", operator: "index", detail: String(index) });
  },
});

/* The base shapes the checking operations defer to. The evaluator's own
   table supplies the unchecked versions; the checker only wraps the
   faulting cases, so a passing run sees the same values. */
const baseBinary = (operator: Word, left: Word, right: Word): Word => {
  const a = typeof left === "number" ? left : 0;
  const b = typeof right === "number" ? right : 0;
  switch (operator) {
    case "+":
      return typeof left === "string" && typeof right === "string" ? left + right : a + b;
    case "-":
      return a - b;
    case "*":
      return a * b;
    case "/":
      return a / b;
    case "%":
      return a % b;
    case "<":
      return a < b;
    case "<=":
      return a <= b;
    case ">":
      return a > b;
    case ">=":
      return a >= b;
    case "===":
      return left === right;
    case "!==":
      return left !== right;
    default:
      return undefined;
  }
};

const baseMemberGet = (object: Word, name: Word): Word => {
  if (typeof name !== "string") return undefined;
  if (object instanceof ArrayValue && name === "length") return object.items.length;
  if (object instanceof MapValue && name === "size") return object.entries.size;
  if (object instanceof SetValue && name === "size") return object.items.size;
  if (object instanceof ErrorValue && name === "message") return object.message;
  if (object instanceof RecordValue && object.fields.has(name)) return object.fields.get(name);
  if (typeof object === "string" && name === "length") return object.length;
  if (
    object instanceof ArrayValue ||
    object instanceof MapValue ||
    object instanceof SetValue ||
    typeof object === "string"
  ) {
    return builtinMember(object, name);
  }
  return undefined;
};

const baseIndexGet = (object: ArrayValue, index: number): Word => object.items[index] ?? undefined;

/** The three probes: a division by zero, a bad member access, and the
 * checks leaving a correct program untouched. */
export const ex_5_30 = (): {
  readonly divisionError: string | null;
  readonly memberError: string | null;
  readonly clean: readonly string[];
} => {
  const division = makeEvaluator("1 / 0;", makeCheckingOperations()).run();
  const member = makeEvaluator("const r = { x: 1 };\nr.y.z;", makeCheckingOperations()).run();
  const clean = makeEvaluator(
    [
      "function factorial(n: number): number { return n === 1 ? 1 : factorial(n - 1) * n; }",
      "console.log(factorial(5));",
    ].join("\n"),
    makeCheckingOperations(),
  ).run();
  return {
    divisionError: division.outcome.tag === "ok" ? null : JSON.stringify(division.outcome.error),
    memberError: member.outcome.tag === "ok" ? null : JSON.stringify(member.outcome.error),
    clean: clean.transcript,
  };
};
