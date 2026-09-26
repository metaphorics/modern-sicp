// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { addAssertionBinding } from "./ex_4_70.js";

describe("4.70", () =>
  it("captures one extension result", () => {
    let calls = 0;
    const tail = addAssertionBinding("x", (x) => {
      calls++;
      return `${x}!`;
    });
    expect(calls).toBe(1);
    expect(tail()).toBe("x!");
    expect(tail()).toBe("x!");
    expect(calls).toBe(1);
  }));
