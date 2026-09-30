// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { EmptyQueueError } from "../../packages/ch3/src/03-mutable-data.js";

import {
  deleteFrontDeque,
  deleteRearDeque,
  frontDeque,
  insertFrontDeque,
  insertRearDeque,
  isEmptyDeque,
  makeDeque,
  rearDeque,
  showDeque,
} from "./ex_3_23.js";

describe("exercise 3.23: the deque with constant-time ends", () => {
  it("inserts and deletes from both ends in the book's order", () => {
    const deque = makeDeque<string>();
    insertRearDeque(deque, "a");
    insertRearDeque(deque, "b");
    expect(showDeque(deque)).toBe("[a, b]");
    expect(deleteFrontDeque(deque)).toBe("a");
    insertFrontDeque(deque, "c");
    insertRearDeque(deque, "d");
    expect(showDeque(deque)).toBe("[c, b, d]");
    expect(deleteRearDeque(deque)).toBe("d");
    expect(showDeque(deque)).toBe("[c, b]");
  });

  it("keeps both links consistent after every operation", () => {
    const deque = makeDeque<string>();
    insertRearDeque(deque, "a");
    insertFrontDeque(deque, "b");
    insertRearDeque(deque, "c");
    // [b, a, c]: forward walk and backward walk agree.
    const front = deque.front;
    const rear = deque.rear;
    expect(front?.item).toBe("b");
    expect(rear?.item).toBe("c");
    expect(front?.next?.item).toBe("a");
    expect(front?.next?.next === rear).toBe(true);
    expect(rear?.prev?.prev === front).toBe(true);
    expect(front?.prev).toBeNull();
    expect(rear?.next).toBeNull();
    deleteFrontDeque(deque);
    // [a, c]: the new front's prev link is reset.
    expect(deque.front?.item).toBe("a");
    expect(deque.front?.prev).toBeNull();
    deleteRearDeque(deque);
    // [a]: the new rear's next link is reset.
    expect(deque.rear?.item).toBe("a");
    expect(deque.rear?.next).toBeNull();
  });

  it("deleting the last item through either end empties both pointers", () => {
    const frontOnly = makeDeque<string>();
    insertFrontDeque(frontOnly, "only");
    expect(deleteFrontDeque(frontOnly)).toBe("only");
    expect(isEmptyDeque(frontOnly)).toBe(true);
    expect(frontOnly.rear).toBeNull();
    insertRearDeque(frontOnly, "next");
    expect(showDeque(frontOnly)).toBe("[next]");
    const rearOnly = makeDeque<string>();
    insertRearDeque(rearOnly, "only");
    expect(deleteRearDeque(rearOnly)).toBe("only");
    expect(isEmptyDeque(rearOnly)).toBe(true);
    expect(rearOnly.front).toBeNull();
  });

  it("fails on reading or deleting an empty deque", () => {
    const deque = makeDeque<string>();
    expect(() => frontDeque(deque)).toThrow(EmptyQueueError);
    expect(() => rearDeque(deque)).toThrow(EmptyQueueError);
    expect(() => deleteFrontDeque(deque)).toThrow(EmptyQueueError);
    expect(() => deleteRearDeque(deque)).toThrow(EmptyQueueError);
  });
});
