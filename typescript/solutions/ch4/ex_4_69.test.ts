// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { greatAnswers, openRelationship } from "./ex_4_69.js";

const fiveGreats = '["great", "great", "great", "great", "great", "grandson"]';

describe("exercise 4.69: greats through grandson-ended relations", () => {
  it("finds Irad as Adam's great-grandson", () => {
    const [irad] = greatAnswers();
    expect(irad).toStrictEqual(['related(["great", "grandson"], "Adam", "Irad")']);
  });

  it("finds both five-greats grandsons of Adam", () => {
    const [, distant] = greatAnswers();
    expect(distant).toStrictEqual([
      `related(${fiveGreats}, "Adam", "Jabal")`,
      `related(${fiveGreats}, "Adam", "Jubal")`,
    ]);
  });

  it("exhibits the open-relationship query without running it", () => {
    expect(openRelationship.tag).toBe("atom");
  });
});
