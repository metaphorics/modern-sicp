// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Stream, streamMap } from "../../packages/ch4/src/04-logic.js";
export function simpleStreamFlatmap<A, B>(f: (x: A) => Stream<B>, s: Stream<A>): Stream<B> {
  const mapped = streamMap(f, s);
  const walk = (m: Stream<Stream<B>>): Stream<B> => {
    if (m.empty || m.head === undefined) return Stream.empty();
    const x = m.head;
    if (x.empty || x.head === undefined) return walk(m.tail());
    return Stream.cons(x.head, () => walk(m.tail()));
  };
  return walk(mapped);
}
export function ex_4_74() {
  return "Simple flatten filters empty streams and maps each singleton head; it is equivalent under the empty-or-singleton contract.";
}
