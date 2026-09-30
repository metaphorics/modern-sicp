// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { list, none, some } from "../../packages/ch2/src/02-picture-language.js";
import { listDatum, symDatum } from "../../packages/ch2/src/03-symbolic-data.js";
import {
  predictCadr,
  predictCdr,
  predictHeadIsList,
  predictList,
  predictMemqFlat,
  predictMemqSublists,
  predictNested,
} from "./ex_2_53.js";

describe("exercise 2.53", () => {
  it("predicts the seven printed values", () => {
    expect(predictList()).toBe("[a, b, c]");
    expect(predictNested()).toBe("[[george]]");
    expect(predictCdr()).toBe("[[y1, y2]]");
    expect(predictCadr()).toBe("[y1, y2]");
    expect(predictHeadIsList(symDatum("a"))).toBe(false);
    expect(predictHeadIsList(listDatum(symDatum("a")))).toBe(true);
    expect(predictMemqSublists()).toStrictEqual(none);
    expect(predictMemqFlat()).toStrictEqual(
      some(list(symDatum("red"), symDatum("shoes"), symDatum("blue"), symDatum("socks"))),
    );
  });
});
