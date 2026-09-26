import { describe, expect, it } from "vitest";
import { basicConditionals } from "./ex_5_24.js";

describe("exercise 5.24", () => {
  it("evaluates both branches", () =>
    expect(basicConditionals().filter((x) => x === "7" || x === "8")).toEqual(["7", "8"]));
});
