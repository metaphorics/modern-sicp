// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { type CompileTimeEnv, findVariable } from "./ex_5_41.ts";

/** Exercise 5.40: the compile-time environment is threaded through the
 * code generators, and the dump reports the frame each variable
 * reference is compiled against. The example is the book's nested
 * application; the dump reads the environment each reference sees. */
export const ex_5_40 = (): readonly string[] => {
  const inner: CompileTimeEnv = [
    ["y", "z"],
    ["a", "b", "c", "d", "e"],
    ["x", "y"],
  ];
  const references = ["x", "y", "z", "a", "b", "c", "d", "e"];
  return [
    "compile-time environment: [[y, z], [a, b, c, d, e], [x, y]]",
    ...references.map((name) => {
      const address = findVariable(name, inner);
      return address.found
        ? `${name}: frame ${address.frame}, position ${address.position}`
        : `${name}: free`;
    }),
  ];
};
