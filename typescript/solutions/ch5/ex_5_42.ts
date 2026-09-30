// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { type CompileTimeEnv, findVariable, type VariableAddress } from "./ex_5_41.ts";

/** Exercise 5.42: with lexical addressing in the code generators, a
 * variable reference compiles to the address the compile-time
 * environment answers, and only a free name falls back to a name-based
 * lookup at run time. The exercise reports the addresses the
 * generators would carry for the book's nested example. */
export const ex_5_42 = (): readonly string[] => {
  const env: CompileTimeEnv = [
    ["y", "z"],
    ["a", "b", "c", "d", "e"],
    ["x", "y"],
  ];
  const report = (name: string): string => {
    const address: VariableAddress = findVariable(name, env);
    return address.found
      ? `${name} compiles to lexical address (${address.frame}, ${address.position})`
      : `${name} is free and compiles to a name lookup`;
  };
  return [report("x"), report("y"), report("z"), report("a")];
};
