// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { length } from "../../packages/ch2/src/02-picture-language.js";
import { decode, symbolsOf, weightOf } from "../../packages/ch2/src/03-symbolic-data.js";
import { encodedSong, fixedBits, rockTree, song, songBits } from "./ex_2_70.js";

describe("exercise 2.70", () => {
  it("encodes the song in 84 bits against 108 fixed", () => {
    expect(songBits).toBe(84);
    expect(fixedBits).toBe(108);
    expect(songBits).toBeLessThan(fixedBits);
  });

  it("round-trips the song through the generated tree", () => {
    expect(decode(encodedSong, rockTree)).toStrictEqual({ _tag: "Ok", value: song });
  });

  it("the generated tree covers all eight symbols and totals 36", () => {
    expect(length(symbolsOf(rockTree))).toBe(8);
    expect(weightOf(rockTree)).toBe(36);
  });
});
