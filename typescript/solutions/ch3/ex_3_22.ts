// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  EmptyQueueError,
  type MCons,
  type MList,
  mcons,
  mnil,
  setCdr,
  showMList,
} from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.22: instead of representing a queue as a pair of
 * pointers, build it as a procedure with local state: the front and
 * rear pointers become locals of the make-queue call and the queue
 * operations become internal procedures the dispatch shares them
 * with. The edition spells the book's cond dispatch as an overloaded
 * call, so each message keeps its own argument and answer type while
 * the closure stays the single entry point the book draws.
 */

/** The book's dispatch messages for the closure queue. */
export type QueueClosureMessage = "insert!" | "delete!" | "front" | "empty?" | "print";

/** The book's message-passing queue: the dispatch the make-queue call
 * returns, answering insert!, delete!, front, empty?, and print over
 * the front and rear pointers captured in its frame. */
export interface QueueClosure<A> {
  (message: "insert!", item: A): QueueClosure<A>;
  (message: "delete!"): QueueClosure<A>;
  (message: "front"): A;
  (message: "empty?"): boolean;
  (message: "print"): string;
}

/** Builds the closure queue of exercise 3.22: one call, one frame,
 * front and rear as locals, the five operations defined inside and
 * reached through the returned dispatch. Deleting or reading the
 * front of an empty queue fails with the module's `EmptyQueueError`,
 * the book's error calls. */
export function makeQueueClosure<A>(): QueueClosure<A> {
  let front: MList<A> = mnil;
  let rear: MCons<A> | null = null;

  function dispatch(message: "insert!", item: A): QueueClosure<A>;
  function dispatch(message: "delete!"): QueueClosure<A>;
  function dispatch(message: "front"): A;
  function dispatch(message: "empty?"): boolean;
  function dispatch(message: "print"): string;
  function dispatch(
    message: QueueClosureMessage,
    item?: A,
  ): QueueClosure<A> | A | boolean | string {
    switch (message) {
      case "insert!": {
        if (item === undefined) {
          throw new Error("insert! called without an item");
        }
        const newPair = mcons<A>(item, mnil);
        if (front._tag === "MNil") {
          front = newPair;
        } else {
          const last = rear;
          if (last === null) {
            throw new Error("corrupt queue: nonempty front with null rear");
          }
          setCdr(last, newPair);
        }
        rear = newPair;
        return dispatch;
      }
      case "delete!": {
        if (front._tag === "MNil") {
          throw new EmptyQueueError("DELETE!");
        }
        front = front.tail;
        if (front._tag === "MNil") {
          rear = null;
        }
        return dispatch;
      }
      case "front": {
        if (front._tag === "MNil") {
          throw new EmptyQueueError("FRONT");
        }
        return front.head;
      }
      case "empty?":
        return front._tag === "MNil";
      case "print":
        return showMList(front);
    }
  }

  return dispatch;
}
