// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { list } from "../../packages/ch2/src/02-picture-language.js";
import { decode, sym } from "../../packages/ch2/src/03-symbolic-data.js";
import { decodeNarrowed, runOffTreeMessage, sampleMessage, sampleTree } from "./ex_2_67a.js";

describe("exercise 2.67a", () => {
  it("pins the sample message's decoding", () => {
    const decoded = decodeNarrowed(sampleMessage, sampleTree);
    expect(decoded).toStrictEqual({
      _tag: "Ok",
      value: list(sym("A"), sym("D"), sym("A"), sym("B"), sym("B"), sym("C"), sym("A")),
    });
  });

  it("pins the run-off-the-tree error the book's decode silently accepts", () => {
    // Bits 1,1 descend to the {D C} branch and then run out mid-symbol.
    expect(decodeNarrowed(list(1, 1), sampleTree)).toStrictEqual({
      _tag: "Error",
      error: runOffTreeMessage,
    });
    // The book's decode answers () for the same input.
    expect(decode(list(1, 1), sampleTree)).toStrictEqual({ _tag: "Ok", value: list() });
  });

  it("still refuses a bad bit", () => {
    expect(decodeNarrowed(list(2), sampleTree)).toStrictEqual({
      _tag: "Error",
      error: "chooseBranch: bad bit 2",
    });
  });
});
