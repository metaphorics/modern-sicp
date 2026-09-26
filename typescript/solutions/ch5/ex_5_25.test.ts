import { describe, expect, it } from "vitest";
import { normalOrderBaseline } from "./ex_5_25.js";

describe("exercise 5.25", () => {
  it("retains the controller path", () => expect(normalOrderBaseline().includes("3")).toBe(true));
});
