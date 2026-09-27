// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { EmptyQueueError } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.23: the deque, a sequence with both-end access whose
 * insertions and deletions are all constant time. The book points out
 * the pointer-pair queue cannot do this and asks for a doubly linked
 * representation: each pair has forward and backward links, and the
 * deque holds a front and a rear pointer. The edition spells the
 * doubly linked pair as a record with mutable `prev` and `next`
 * fields; every operation rewires a constant number of links, so no
 * operation loops.
 */

/** One doubly linked cell: the item plus the two links the book's
 * deque pairs carry. */
export interface DequeNode<A> {
  item: A;
  prev: DequeNode<A> | null;
  next: DequeNode<A> | null;
}

/** The deque: front and rear pointers over the doubly linked cells,
 * both null when the deque is empty. */
export interface Deque<A> {
  front: DequeNode<A> | null;
  rear: DequeNode<A> | null;
}

/** Builds an empty deque: both pointers empty. */
export const makeDeque = <A>(): Deque<A> => ({ front: null, rear: null });

/** Whether the deque has no items. */
export const isEmptyDeque = <A>(deque: Deque<A>): boolean => deque.front === null;

/** The item at the front, without modifying the deque; fails with
 * `EmptyQueueError` when the deque is empty. */
export const frontDeque = <A>(deque: Deque<A>): A => {
  const front = deque.front;
  if (front === null) {
    throw new EmptyQueueError("FRONT-DEQUE");
  }
  return front.item;
};

/** The item at the rear, without modifying the deque; fails with
 * `EmptyQueueError` when the deque is empty. */
export const rearDeque = <A>(deque: Deque<A>): A => {
  const rear = deque.rear;
  if (rear === null) {
    throw new EmptyQueueError("REAR-DEQUE");
  }
  return rear.item;
};

/** Inserts `item` at the front and returns the deque: one cell, two
 * link writes, one pointer write. */
export const insertFrontDeque = <A>(deque: Deque<A>, item: A): Deque<A> => {
  const node: DequeNode<A> = { item, prev: null, next: deque.front };
  if (deque.front !== null) {
    deque.front.prev = node;
  } else {
    deque.rear = node;
  }
  deque.front = node;
  return deque;
};

/** Inserts `item` at the rear and returns the deque: one cell, two
 * link writes, one pointer write. */
export const insertRearDeque = <A>(deque: Deque<A>, item: A): Deque<A> => {
  const node: DequeNode<A> = { item, prev: deque.rear, next: null };
  if (deque.rear !== null) {
    deque.rear.next = node;
  } else {
    deque.front = node;
  }
  deque.rear = node;
  return deque;
};

/** Removes the front item and returns it; fails with
 * `EmptyQueueError` when the deque is empty. Deleting the last item
 * empties both pointers. */
export const deleteFrontDeque = <A>(deque: Deque<A>): A => {
  const front = deque.front;
  if (front === null) {
    throw new EmptyQueueError("DELETE-FRONT-DEQUE");
  }
  const item = front.item;
  deque.front = front.next;
  if (deque.front !== null) {
    deque.front.prev = null;
  } else {
    deque.rear = null;
  }
  return item;
};

/** Removes the rear item and returns it; fails with
 * `EmptyQueueError` when the deque is empty. Deleting the last item
 * empties both pointers. */
export const deleteRearDeque = <A>(deque: Deque<A>): A => {
  const rear = deque.rear;
  if (rear === null) {
    throw new EmptyQueueError("DELETE-REAR-DEQUE");
  }
  const item = rear.item;
  deque.rear = rear.prev;
  if (deque.rear !== null) {
    deque.rear.next = null;
  } else {
    deque.front = null;
  }
  return item;
};

/** Renders the deque's items front to rear, for the pins. */
export const showDeque = <A>(deque: Deque<A>): string => {
  const items: A[] = [];
  for (let node = deque.front; node !== null; node = node.next) {
    items.push(node.item);
  }
  const render = (value: A): string => (typeof value === "string" ? value : String(value));
  return `(${items.map(render).join(" ")})`;
};
