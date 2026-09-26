import { describe, expect, it } from "vitest";
import { iterativeFactorialStack } from "./ex_5_26.js";

describe("exercise 5.26", () => {
  it("pins the stack rows", () =>
    expect(iterativeFactorialStack(5)).toEqual({ n: 5, pushes: 204, maximumDepth: 28 }));
});
