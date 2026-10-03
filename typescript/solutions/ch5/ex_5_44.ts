// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { type CompileTimeEnv, findVariable } from "./ex_5_41.ts";

/** The primitives the open-coded dispatch recognizes. */
export const openCodedPrimitives: readonly string[] = ["+", "-", "*", "<", "="];

/** Exercise 5.44: open coding respects shadowing. A name is open-coded
 * only when it is one of the open-coded primitives AND the compile-time
 * environment does not bind it: a parameter named `+` shadows the
 * primitive, and the call must go through the ordinary application
 * path. */
export const isOpenCoded = (name: string, env: CompileTimeEnv): boolean =>
  openCodedPrimitives.includes(name) && !findVariable(name, env).found;

/** The three probes: the primitive in the global scope is open-coded,
 * the same name bound by a frame is not, and an unknown name is not. */
export const ex_5_44 = (): readonly string[] => {
  const shadowed: CompileTimeEnv = [["+"], ["x"]];
  const global: CompileTimeEnv = [["x"]];
  return [
    `+ in the global scope: ${isOpenCoded("+", global)}`,
    `+ shadowed by a parameter: ${isOpenCoded("+", shadowed)}`,
    `unknown name: ${isOpenCoded("f", global)}`,
  ];
};
