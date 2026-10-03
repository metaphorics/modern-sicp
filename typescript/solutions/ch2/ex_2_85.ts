// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { err, ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import { attachTag } from "../../packages/ch2/src/04-data-directed.js";
import {
  type ArithContents,
  type ArithDatum,
  applyGeneric,
  equQ,
  type GenError,
  makeRational,
  op,
  pred,
  put,
  typeTagOf,
} from "../../packages/ch2/src/05-generic-operations.js";
import { raiseToward } from "./ex_2_84.js";

/**
 * Exercise 2.85: `drop`, the lowering operation. Each level but the
 * bottom installs a `project`; a datum can be dropped when projecting
 * it and raising the result back answers something `equ?`-al to what
 * it started as, and the drop repeats at the lower level. The section
dispatch then simplifies its answers with drop.
 */

/** Installs the three project entries and the real-level equ? the
 * comparison needs. Idempotent. */
export const installProject = (): void => {
  put(
    "project",
    ["complex"],
    op((args) => {
      const c = args[0];
      const re = c !== undefined ? repPartOf(c) : undefined;
      return re !== undefined ? ok(attachTag("real", re)) : miss85("project", "complex");
    }),
  );
  put(
    "project",
    ["real"],
    op((args) => {
      const c = args[0];
      if (typeof c !== "number" || !Number.isInteger(c)) {
        return miss85("project", "real");
      }
      return ok(makeRational(BigInt(c), 1n));
    }),
  );
  put(
    "project",
    ["rational"],
    op((args) => {
      const c = args[0];
      if (!Array.isArray(c) || typeof c[0] !== "bigint" || typeof c[1] !== "bigint") {
        return miss85("project", "rational");
      }
      return c[0] % c[1] === 0n ? ok(c[0] / c[1]) : miss85("project", "rational");
    }),
  );
  put(
    "equ?",
    ["real", "real"],
    pred((args) => {
      const x = args[0];
      const y = args[1];
      return typeof x === "number" && typeof y === "number"
        ? ok(x === y)
        : err({ _tag: "NoMethod", op: "equ?", tags: ["real", "real"] });
    }),
  );
};

/** Reads the real coordinate of a complex contents. */
const repPartOf = (c: ArithContents): number | undefined => {
  if (typeof c !== "object" || c === null || Array.isArray(c) || !("_tag" in c)) {
    return undefined;
  }
  if (c._tag === "rectangular") {
    return c.contents[0];
  }
  if (c._tag === "polar") {
    return c.contents[0] * Math.cos(c.contents[1]);
  }
  return undefined;
};

const miss85 = (opName: string, tag: string): Result<ArithDatum, GenError> => ({
  _tag: "Error",
  error: { _tag: "NoMethod", op: opName, tags: [tag] },
});

/** Drops a datum as far as the tower allows: project, raise back,
 * compare, and only then keep the lower form. A datum with no project
 * entry, or whose round trip loses information, stays as it is. */
export const drop = (x: ArithDatum): Result<ArithDatum, GenError> => {
  const p = applyGeneric("project", x);
  if (p._tag === "Error") {
    return ok(x);
  }
  const back = raiseToward(p.value, typeTagOf(x));
  if (back._tag === "Error") {
    return ok(x);
  }
  const eq = equQ(back.value, x);
  if (eq._tag !== "Ok" || !eq.value) {
    return ok(x);
  }
  return drop(p.value);
};

/** The generic dispatch of this exercise: the answer of the section's
 * dispatch, simplified by drop. */
export const applyGenericDropping = (
  opName: string,
  ...args: ReadonlyArray<ArithDatum>
): Result<ArithDatum, GenError> => {
  const r = applyGeneric(opName, ...args);
  return r._tag === "Ok" ? drop(r.value) : r;
};
