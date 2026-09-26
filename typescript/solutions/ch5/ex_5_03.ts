// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  runMachine,
  sqrtExpanded,
  sqrtPrimitive,
} from "../../packages/ch5/src/01-register-machines.js";

/** Compares primitive and expanded Newton machine answers. */
export const ex_5_03 = (x: number) => {
  const primitives = {
    "good-enough?": ([g, n]: readonly (number | boolean | { readonly label: string })[]) =>
      typeof g === "number" && typeof n === "number" && Math.abs(g * g - n) < 0.001,
    improve: ([g, n]: readonly (number | boolean | { readonly label: string })[]) =>
      typeof g === "number" && typeof n === "number" ? (g + n / g) / 2 : 0,
  };
  return {
    primitive: runMachine(sqrtPrimitive, { x }, 100_000, primitives),
    expanded: runMachine(sqrtExpanded, { x }),
  };
};
