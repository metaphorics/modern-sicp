// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { EmptyQueueError } from "../../packages/ch3/src/03-mutable-data.js";

import { makeQueueClosure } from "./ex_3_22.js";

describe("exercise 3.22", () => {
  it("the figure 3.18 sequence runs through the dispatch", () => {
    const q = makeQueueClosure<string>();
    expect(q("empty?")).toBe(true);
    q("insert!", "a");
    q("insert!", "b");
    expect(q("print")).toBe("(a b)");
    expect(q("front")).toBe("a");
    q("delete!");
    expect(q("print")).toBe("(b)");
    q("insert!", "c");
    q("insert!", "d");
    q("delete!");
    expect(q("print")).toBe("(c d)");
    expect(q("empty?")).toBe(false);
  });

  it("insert returns the queue so calls chain", () => {
    const q = makeQueueClosure<string>();
    const returned = q("insert!", "a")("insert!", "b");
    expect(returned("print")).toBe("(a b)");
  });

  it("two queues are independent objects", () => {
    const q1 = makeQueueClosure<string>();
    const q2 = makeQueueClosure<string>();
    q1("insert!", "a");
    expect(q2("empty?")).toBe(true);
    expect(q1("empty?")).toBe(false);
    expect(() => q2("front")).toThrow(EmptyQueueError);
    expect(q1("front")).toBe("a");
  });

  it("front and delete! on an empty queue fail with the book's errors", () => {
    const q = makeQueueClosure<string>();
    expect(() => q("front")).toThrow(EmptyQueueError);
    expect(() => q("front")).toThrow("FRONT called with an empty queue");
    expect(() => q("delete!")).toThrow(EmptyQueueError);
    expect(() => q("delete!")).toThrow("DELETE! called with an empty queue");
  });

  it("deleting the last item empties the closure queue", () => {
    const q = makeQueueClosure<string>();
    q("insert!", "only");
    q("delete!");
    expect(q("empty?")).toBe(true);
    expect(q("print")).toBe("()");
    expect(() => q("front")).toThrow(EmptyQueueError);
  });
});
