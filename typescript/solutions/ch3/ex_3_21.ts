// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { type Queue, queueItems, showMList } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.21: Ben types the queue interactions at the REPL and
 * reads `((a) a)`, `((a b) b)`, `((b) b)`, `(() b)` as evidence the
 * last item is inserted twice and never deleted. Eva Lu explains the
 * standard printer is just rendering the queue's representation pair:
 * its car is the front pointer (a list it prints fine) and its cdr is
 * the rear pointer (the last pair, printed as the one-item list of
 * its content). Nothing is stored twice and the deletion did happen;
 * the printer just does not know the two pointers mean front and
 * rear. The exercise-map row's addition idea, "render queue as list
 * view", is the print procedure she asks for.
 */

/** The queue's items rendered the way the book prints a list: the
 * print procedure Eva Lu asks for, the front pointer shown as the
 * sequence it names. */
export const printQueue = <A>(queue: Queue<A>): string => showMList(queueItems(queue));

/** The queue's representation pair rendered the way the standard
 * printer shows it to Ben: the front pointer as a list, then the
 * rear pointer labeled as what it is. The rear pointer is not part of
 * the queue's items, which is exactly the confusion: after the last
 * item leaves, the book's rear pointer still points at the deleted
 * pair, so the raw view keeps showing an item the front pointer no
 * longer names. */
export const benPrintQueue = <A>(queue: Queue<A>): string => {
  const rear = queue.rear;
  const rearText = rear === null ? "null" : showMList(rear);
  return `(front ${showMList(queueItems(queue))} rear ${rearText})`;
};
