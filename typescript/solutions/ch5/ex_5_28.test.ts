import { describe, expect, it } from "vitest";
import { tailRecursiveFactorialStack } from "./ex_5_28.js";

describe("exercise 5.28", () => {
  it("pins the open-coded tail row", () =>
    expect(tailRecursiveFactorialStack(5)).toEqual({ n: 5, pushes: 43, maximumDepth: 29 }));
});
