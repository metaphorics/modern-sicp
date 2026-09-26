import { describe, expect, it } from "vitest";
import { recursiveFactorialStack } from "./ex_5_27.js";

describe("exercise 5.27", () => {
  it("pins recursive factorial", () =>
    expect(recursiveFactorialStack(5)).toEqual({ n: 5, pushes: 144, maximumDepth: 29 }));
});
