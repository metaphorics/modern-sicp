// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.76: merging frames for `and`. The alternative strategy
 * evaluates each clause separately and keeps compatible frame
 * pairs: two frames merge when every shared variable unifies to one
 * value, like unification over bindings. `mergeFrames` implements
 * the check over exported frame operations, and the clause-at-a-time
 * evaluation agrees with the delivered conjunction over real
 * database clauses.
 */
import {
  bindingInFrame,
  conjoin,
  extend,
  type Frame,
  formatQuery,
  instantiate,
  makeBinding,
  mapOverSymbols,
  type Query,
  qeval,
  qlist,
  qtext,
  queryAtom,
  qvar,
  type Stream,
  streamToList,
  type Term,
  unifyMatch,
} from "../../packages/ch4/src/04-logic.js";
import { microshaftDatabase } from "./ex_4_55.js";

/**
 * Merges two frames when compatible: every variable bound in both
 * must unify, otherwise the pair is discarded. Unbound sides merge
 * freely.
 */
export const mergeFrames = (left: Frame, right: Frame): Frame | undefined => {
  let merged: Frame = [...left];
  for (const binding of right) {
    const existing = bindingInFrame(binding.name, merged);
    if (existing === undefined) {
      merged = extend(binding.name, binding.value, merged);
      continue;
    }
    const unified = unifyMatch(existing, binding.value, merged);
    if (unified === undefined) {
      return undefined;
    }
    merged = unified;
  }
  return merged;
};

/** Two compatible frames sharing ?x on one person. */
export const compatiblePair: readonly [Frame, Frame] = [
  [makeBinding("x", { tag: "text", value: "Ben" } satisfies Term)],
  [
    makeBinding("x", { tag: "text", value: "Ben" } satisfies Term),
    makeBinding("y", { tag: "text", value: "60000" } satisfies Term),
  ],
];

/** Two frames disagreeing on ?x. */
export const conflictingPair: readonly [Frame, Frame] = [
  [makeBinding("x", { tag: "text", value: "Ben" } satisfies Term)],
  [makeBinding("x", { tag: "text", value: "Alyssa" } satisfies Term)],
];

/**
 * Clause-at-a-time agreement: each clause evaluated separately
 * over literal seeds, pairwise merged, against the delivered
 * conjunction over the same clauses.
 */
export const mergedAndAgreement = (): readonly [ReadonlyArray<string>, ReadonlyArray<string>] => {
  const db = microshaftDatabase();
  const first: Query = queryAtom(
    "supervisor",
    qvar("person"),
    qlist(qtext("Bitdiddle"), qtext("Ben")),
  );
  const second: Query = queryAtom("address", qvar("person"), qvar("where"));
  const clauses: ReadonlyArray<Query> = [first, second];
  const pattern: Query = { tag: "and", clauses };
  const seed: Stream<Frame> = { kind: "singleton", value: [] };
  const leftFrames = streamToList(qeval(first, seed, db));
  const rightFrames = streamToList(qeval(second, seed, db));
  const merged: Frame[] = [];
  for (const left of leftFrames) {
    for (const right of rightFrames) {
      const both = mergeFrames(left, right);
      if (both !== undefined) {
        merged.push(both);
      }
    }
  }
  const render = (frames: ReadonlyArray<Frame>): string[] =>
    frames.map((frame) =>
      formatQuery(mapOverSymbols(pattern, (term) => instantiate(term, frame, qvar))),
    );
  const direct = streamToList(conjoin(clauses, seed, db));
  return [render(merged), render(direct)];
};

export function ex_4_76(): string {
  const merged = mergeFrames(compatiblePair[0], compatiblePair[1]);
  const rejected = mergeFrames(conflictingPair[0], conflictingPair[1]);
  const [pairwise, direct] = mergedAndAgreement();
  const agree = JSON.stringify([...pairwise].sort()) === JSON.stringify([...direct].sort());
  return (
    `Compatible frames merge to ${merged === undefined ? "nothing" : `${merged.length} bindings`}; ` +
    `conflicting frames merge to ${rejected === undefined ? "nothing" : "something"}. ` +
    `Clause-at-a-time merging ${agree ? "agrees" : "disagrees"} with the delivered conjunction. ` +
    `Merging trades per-frame database scans for pairwise compatibility checks.`
  );
}
