// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { isPair } from "../../packages/ch4/src/01-metacircular.js";
import { makeQueryEngine, Stream, streamFlatmap } from "../../packages/ch4/src/04-logic.js";
export function uniqueEngine() {
  const e = makeQueryEngine();
  e.load(
    "(job Ben (computer wizard)) (job Alyssa (computer programmer)) (job Louis (computer programmer))",
  );
  e.put("unique", (q, frames) =>
    streamFlatmap((f) => {
      if (!isPair(q)) return Stream.empty();
      const results = e.query(q.head, f).take(2);
      const single = results[0];
      return single !== undefined && results.length === 1
        ? Stream.cons(single, () => Stream.empty())
        : Stream.empty();
    }, frames),
  );
  return e;
}
export function ex_4_75() {
  return "unique retains an input frame exactly when its inner query has one answer, checking at most two.";
}
