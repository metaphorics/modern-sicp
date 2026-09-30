// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.74: Alyssa's simple stream-flatmap. Where the mapped
 * procedure yields only empty or singleton streams, combining needs
 * no interleaving: keep the non-empty streams and take their single
 * elements in order. (a) The engine already ships both halves of
 * her program as `simpleStreamFlatmap` and `simpleFlatten`; this
 * solution drives them over supervisor frames filtered to
 * singletons and empties. (b) Behavior does not change on finite
 * streams — the simple and interleaving flatmaps agree exactly —
 * because fairness only matters behind unbounded streams.
 */
import {
  bindingInFrame,
  type Frame,
  findAssertions,
  formatTerm,
  qlist,
  qtext,
  queryAtom,
  qvar,
  type Stream,
  simpleStreamFlatmap,
  streamFlatmap,
  streamToList,
} from "../../packages/ch4/src/04-logic.js";
import { microshaftDatabase } from "./ex_4_55.js";

/** Supervisor frames with Fect filtered out, as singleton/empty streams. */
export const singletonOrEmpty = (frame: Frame): Stream<Frame> => {
  const bound = bindingInFrame("x", frame);
  const key = bound === undefined ? "" : formatTerm(bound);
  return key.includes("Fect") ? { kind: "empty" } : { kind: "singleton", value: frame };
};

/** Simple versus interleaving flatmap over the filtered frames. */
export const flatmapAgreement = (): readonly [ReadonlyArray<string>, ReadonlyArray<string>] => {
  const db = microshaftDatabase();
  const ben = qlist(qtext("Bitdiddle"), qtext("Ben"));
  const seed: Stream<Frame> = findAssertions(queryAtom("supervisor", qvar("x"), ben), [], db);
  const render = (frames: ReadonlyArray<Frame>): string[] =>
    frames.map((frame) => {
      const bound = bindingInFrame("x", frame);
      return bound === undefined ? "?x" : formatTerm(bound);
    });
  const simple = streamToList(simpleStreamFlatmap(seed, singletonOrEmpty));
  const seeds: Stream<Frame> = findAssertions(queryAtom("supervisor", qvar("x"), ben), [], db);
  const interleaved = streamToList(streamFlatmap(seeds, singletonOrEmpty));
  return [render(simple), render(interleaved)];
};

export function ex_4_74(): string {
  const [simple, interleaved] = flatmapAgreement();
  return (
    `Simple and interleaving flatmaps agree on all ${simple.length} kept ` +
    `frames (${interleaved.length} by the other road): with only empty or ` +
    `singleton streams to combine, fairness has nothing to schedule.`
  );
}
