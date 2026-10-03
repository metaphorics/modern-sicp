// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.73: why flattenStream delays. The engine flattens by
 * interleaving each inner stream with the flattening of the rest,
 * deferring the tail; an eager flatten would force the outer tail
 * before yielding the inner head. Over two supervisor streams the
 * delayed flatten interleaves Louis among Ben's supervisees, while
 * appending lists Hacker's branch last. The starvation moral is the
 * same as 4.71's: eagerness lets one unbounded stream bury the rest.
 */
import {
  bindingInFrame,
  type Frame,
  findAssertions,
  flattenStream,
  formatTerm,
  qlist,
  qtext,
  queryAtom,
  qvar,
  type Stream,
  streamAppendDelayed,
  streamToList,
} from "../../packages/ch4/src/04-logic.js";
import { microshaftDatabase } from "./ex_4_55.js";

/** The ?x names of realized frames, in order. */
export const xNames = (frames: ReadonlyArray<Frame>): string[] =>
  frames.map((frame) => {
    const bound = bindingInFrame("x", frame);
    return bound === undefined ? "?x" : formatTerm(bound);
  });

/** Delayed flatten versus eager append over two supervisor streams. */
export const flattenOrders = (): readonly [ReadonlyArray<string>, ReadonlyArray<string>] => {
  const db = microshaftDatabase();
  const ben = qlist(qtext("Bitdiddle"), qtext("Ben"));
  const hacker = qlist(qtext("Hacker"), qtext("Alyssa"), qtext("P"));
  const benStream = (): Stream<Frame> =>
    findAssertions(queryAtom("supervisor", qvar("x"), ben), [], db);
  const hackerStream = (): Stream<Frame> =>
    findAssertions(queryAtom("supervisor", qvar("x"), hacker), [], db);
  const outer: Stream<Stream<Frame>> = {
    kind: "cons",
    head: benStream(),
    rest: () => ({ kind: "singleton", value: hackerStream() }),
  };
  const flattened = streamToList(flattenStream(outer));
  const appended = streamToList(streamAppendDelayed(benStream(), () => hackerStream()));
  return [xNames(flattened), xNames(appended)];
};

export function ex_4_73(): string {
  const [flattened, appended] = flattenOrders();
  return (
    `Delayed flatten answers ${flattened.length} frames with Hacker's branch ` +
    `interleaved; eager append answers the same ${appended.length} with it ` +
    `last. Forcing the outer tail first would bury inner answers behind ` +
    `any unbounded stream.`
  );
}
