// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  type Frame,
  listValue,
  microshaft,
  Stream,
  streamAppendDelayed,
  streamFlatmap,
} from "../../packages/ch4/src/04-logic.js";
import { toArray } from "../../packages/ch4/src/list.js";

/** The book's or example: Ben's supervisees against Alyssa's. */
export const orQuery = "(or (supervisor ?x (Bitdiddle Ben)) (supervisor ?x (Hacker Alyssa P)))";

/**
 * The amb engine's depth-first order over the same query. Each query
 * produces one answer and try-again resumes the choice points, so the
 * first disjunct's whole subtree comes out before the second's: an
 * appending disjoin over the branch streams reproduces it.
 */
export const depthFirstOr = (): ReadonlyArray<string> => {
  const engine = microshaft();
  engine.put("or", (operands, frames) =>
    streamFlatmap((frame: Frame) => {
      const branches = toArray(listValue(operands));
      const walk = (index: number): Stream<Frame> => {
        const branch = branches[index];
        if (branch === undefined) return Stream.empty();
        return streamAppendDelayed(engine.query(branch, frame), () => walk(index + 1));
      };
      return walk(0);
    }, frames),
  );
  return engine.answers(orQuery);
};

export function ex_4_78(): string {
  return "Much of the stream machinery is subsumed by backtracking choice points with try-again, but depth-first search answers the or branches in turn while the stream engine interleaves them.";
}
