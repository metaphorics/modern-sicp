// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { type MList, type Queue, queueItems } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.21: Ben types the queue interactions at the REPL and
 * reads `[[a], a]`, `[[a, b], b]`, `[[b], b]`, `[[], b]` as evidence the
 * last item is inserted twice and never deleted. Eva Lu explains the
 * standard printer is just rendering the queue's representation pair:
 * its car is the front pointer (a list it prints fine) and its cdr is
 * the rear pointer (the last pair, printed as the one-item list of
 * its content). Nothing is stored twice and the deletion did happen;
 * the printer just does not know the two pointers mean front and
 * rear. The exercise-map row's addition idea, "render queue as list
 * view", is the print procedure she asks for.
 */

/** The edition's bracket-comma list rendering: `[]`, `[a, b]`. */
const showBracketList = (l: MList<unknown>): string => {
  const items: string[] = [];
  for (let rest = l; rest._tag === "MCons"; rest = rest.tail) {
    items.push(String(rest.head));
  }
  return `[${items.join(", ")}]`;
};

/** The queue's items rendered as the edition's bracket-comma list:
 * the print procedure Eva Lu asks for, the front pointer shown as the
 * sequence it names. */
export const printQueue = <A>(queue: Queue<A>): string => showBracketList(queueItems(queue));

/** The queue's representation pair rendered as Ben's raw view: the
 * front pointer as a list, then the rear pointer labeled as what it
 * is. The rear pointer is not part of the queue's items, which is
 * exactly the confusion: after the last item leaves, the book's rear
 * pointer still points at the deleted pair, so the raw view keeps
 * showing an item the front pointer no longer names. */
export const benPrintQueue = <A>(queue: Queue<A>): string => {
  const rear = queue.rear;
  return `[front ${showBracketList(queueItems(queue))}, rear ${rear === null ? "null" : showBracketList(rear)}]`;
};
