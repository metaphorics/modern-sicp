// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { type Frame, unifyMatch } from "../../packages/ch4/src/04-logic.js";
export function mergeFrames(a: Frame, b: Frame): Frame | undefined {
  let out = a;
  for (const [v, x] of b.bindings) {
    const next = unifyMatch(v, x, out);
    if (!next) return undefined;
    out = next;
  }
  return out;
}
export function joinFrames(a: ReadonlyArray<Frame>, b: ReadonlyArray<Frame>): ReadonlyArray<Frame> {
  const out: Frame[] = [];
  for (const x of a)
    for (const y of b) {
      const m = mergeFrames(x, y);
      if (m) out.push(m);
    }
  return out;
}
export function ex_4_76() {
  return "Evaluate conjunction clauses independently, then unify each pair of frames and discard conflicts.";
}
