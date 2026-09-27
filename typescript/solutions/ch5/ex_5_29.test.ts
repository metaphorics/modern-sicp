import { describe, expect, it } from "vitest";
import { fibRecurrenceConstant, fibStackPushes } from "./ex_5_29.js";

describe("exercise 5.29", () => {
  it("pins k and the closed form", () => {
    expect(fibRecurrenceConstant).toBe(40);
    expect(fibStackPushes(5)).toBe(408);
  });
});
