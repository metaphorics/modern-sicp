// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import {
  exerciseMemory,
  memoryVectorDrawing,
  structureReadBack,
  yElementsShareX,
} from "./ex_5_20.ts";
import {
  car,
  cons,
  type MemoryError,
  MemoryFault,
  makeMemory,
  readTheCars,
} from "./exercise-memory.ts";

const expectMemoryFault = (run: () => unknown, fault: MemoryError): void => {
  try {
    run();
  } catch (error) {
    expect(error).toBeInstanceOf(MemoryFault);
    if (!(error instanceof MemoryFault)) throw error;
    expect(error.fault).toEqual(fault);
    return;
  }
  throw new Error("expected the memory operation to fail");
};

describe("exercise 5.20 memory vectors", () => {
  it("draws the three allocations cell for cell with x=p1, y=p3, and free=p4", () => {
    expect(memoryVectorDrawing()).toEqual([
      "index    0   1   2   3   4   5   6   7",
      "the-cars e0  n1  p1  p1  e0  e0  e0  e0",
      "the-cdrs e0  n2  e0  p2  e0  e0  e0  e0",
      "x = p1",
      "y = p3",
      "free = p4",
    ]);
    expect(structureReadBack()).toEqual(["(1 . 2)", "((1 . 2) (1 . 2))"]);
    expect(yElementsShareX()).toBe(true);
    expect(exerciseMemory().memory.free).toBe(4);
  });

  it("raises typed faults for exhausted allocation, non-pair selection, and bad indices", () => {
    const full = makeMemory(1);
    cons(full, 1, 2);
    expectMemoryFault(() => cons(full, 3, 4), {
      tag: "MemoryExhausted",
      free: 1,
      size: 1,
    });
    expectMemoryFault(() => car(makeMemory(2), 7), { tag: "NotAPair", op: "car", word: 7 });
    expectMemoryFault(() => readTheCars(makeMemory(2), 2), {
      tag: "IndexOutOfRange",
      op: "vector-ref",
      index: 2,
      size: 2,
    });
  });
});
