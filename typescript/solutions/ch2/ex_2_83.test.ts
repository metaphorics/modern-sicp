// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  makeComplexFromMagAng,
  show,
  typeTagOf,
} from "../../packages/ch2/src/05-generic-operations.js";
import { installRaise, raise, rat83, real83, sn83 } from "./ex_2_83.js";

describe("exercise 2.83: generic raise", () => {
  installRaise();
  it("raises an ordinary number to a rational", () => {
    expect(show(raise(sn83(5n)))).toBe("[rational, 5, 1]");
  });

  it("raises a rational to a real", () => {
    expect(show(raise(rat83(3n, 2n)))).toBe("[real, 1.5]");
  });

  it("raises a real to a complex number", () => {
    expect(show(raise(real83(2)))).toBe("[complex, rectangular, 2, 0]");
  });

  it("walks the whole tower in three raises", () => {
    const first = raise(sn83(7n));
    const second = first._tag === "Ok" ? raise(first.value) : first;
    const third = second._tag === "Ok" ? raise(second.value) : second;
    expect(third._tag === "Ok" && typeTagOf(third.value) === "complex").toBe(true);
    expect(show(third)).toBe("[complex, rectangular, 7, 0]");
  });

  it("has no entry above the top of the tower", () => {
    const top = makeComplexFromMagAng(1, 0);
    const up = raise(top);
    expect(up._tag === "Error" && up.error._tag === "NoMethod").toBe(true);
  });
});
