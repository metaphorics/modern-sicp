// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import { puzzleLibrary } from "./ex_4_38.js";
import { bookOrder, ex_4_39, firstWithFailures, reorderedOrder } from "./ex_4_39.js";

const run = (program: string) =>
  Effect.runPromise(firstWithFailures([puzzleLibrary, program].join("\n")));

describe("exercise 4.39: the order of the restrictions", () => {
  it("both orders answer identically with identical failure counts", async () => {
    const book = await run(bookOrder);
    const reordered = await run(reorderedOrder);
    expect(book.answer).toBe("((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))");
    expect(reordered.answer).toBe(book.answer);
    expect(book.failuresToFirst).toBe(1835);
    expect(reordered.failuresToFirst).toBe(1835);
  });

  it("reports the argument", () => {
    expect(ex_4_39()).toContain("identical counts");
  });
});
