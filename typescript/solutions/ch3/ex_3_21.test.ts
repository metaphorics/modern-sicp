// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  deleteQueue,
  insertQueue,
  makeQueue,
  type Queue,
} from "../../packages/ch3/src/03-mutable-data.js";

import { benPrintQueue, printQueue } from "./ex_3_21.js";

/** Runs Ben's REPL session: insert a, insert b, delete, insert c,
 * insert d, delete, the figure 3.18 flow, returning the queue. */
const benSession = (): Queue<string> => {
  const q = makeQueue<string>();
  insertQueue(q, "a");
  insertQueue(q, "b");
  deleteQueue(q);
  insertQueue(q, "c");
  insertQueue(q, "d");
  deleteQueue(q);
  return q;
};

describe("exercise 3.21", () => {
  it("printQueue renders the item sequence after the figure 3.18 session", () => {
    expect(printQueue(benSession())).toBe("(c d)");
  });

  it("benPrintQueue exposes the rear pointer the printer trips over", () => {
    const q = benSession();
    const ben = benPrintQueue(q);
    expect(ben).toContain("(c d)");
    expect(ben).toContain("rear");
    expect(ben).not.toBe(printQueue(q));
    expect(ben).toBe("(front (c d) rear (d))");
  });

  it("Ben's transcript states come out of the raw view", () => {
    const q = makeQueue<string>();
    insertQueue(q, "a");
    expect(printQueue(q)).toBe("(a)");
    expect(benPrintQueue(q)).toBe("(front (a) rear (a))");
    insertQueue(q, "b");
    expect(benPrintQueue(q)).toBe("(front (a b) rear (b))");
    deleteQueue(q);
    expect(benPrintQueue(q)).toBe("(front (b) rear (b))");
  });

  it("after the queue empties, the rear pointer is cleared here", () => {
    const q = makeQueue<string>();
    insertQueue(q, "a");
    insertQueue(q, "b");
    deleteQueue(q);
    deleteQueue(q);
    expect(printQueue(q)).toBe("()");
    expect(benPrintQueue(q)).toBe("(front () rear null)");
  });
});
