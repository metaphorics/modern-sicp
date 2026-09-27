// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 0.4: an exhaustive `area` over the three-variant `Shape` union.
 *
 * The switch covers every variant, so `area` never falls off the end. The
 * compile experiment in the statement (delete the `Square` case, rebuild,
 * record the compiler's message, restore) is recorded in the rationale;
 * tests pin the three areas.
 */
export type Shape =
  | { readonly _tag: "Circle"; readonly r: number }
  | { readonly _tag: "Rect"; readonly w: number; readonly h: number }
  | { readonly _tag: "Square"; readonly s: number };

export function area(s: Shape): number {
  switch (s._tag) {
    case "Circle":
      return Math.PI * s.r * s.r;
    case "Rect":
      return s.w * s.h;
    case "Square":
      return s.s * s.s;
  }
}
