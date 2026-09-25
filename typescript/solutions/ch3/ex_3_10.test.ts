// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";
import type { WithdrawalProcessor } from "../../packages/ch3/src/01-assignment.js";
import { makeWithdraw } from "../../packages/ch3/src/01-assignment.js";

import {
  makeWithdrawLet,
  makeWithdrawLetTraced,
  makeWithdrawTraced,
  renderWithdrawalStructures,
} from "./ex_3_10.js";

describe("exercise 3.10: the let spelling's binding lifetimes", () => {
  it.effect("the two spellings answer the same withdrawal script identically", () =>
    Effect.gen(function* () {
      const script = (make: (initial: number) => WithdrawalProcessor) =>
        Effect.gen(function* () {
          const w1 = make(100);
          const first = yield* w1(50);
          const w2 = make(100);
          const second = yield* w2(10);
          return [first, second];
        });
      expect(yield* script(makeWithdraw)).toEqual([50, 90]);
      expect(yield* script(makeWithdrawLet)).toEqual([50, 90]);
    }),
  );

  it("the parameter spelling opens one frame; the let spelling opens two", () => {
    const parameter = makeWithdrawTraced(100);
    const letSpelling = makeWithdrawLetTraced(100);
    const entered = (events: typeof parameter.events) =>
      events.filter((event) => event._tag === "FrameEntered");
    expect(entered(parameter.events).map((event) => event.frame)).toEqual([
      "E1: call make-withdraw",
    ]);
    expect(entered(letSpelling.events).map((event) => event.frame)).toEqual([
      "E1: call make-withdraw-let",
      "E2: the desugared let, applied",
    ]);
  });

  it("the let spelling's factory frame dies; the let's applied frame survives", () => {
    const letSpelling = makeWithdrawLetTraced(100);
    expect(letSpelling.events[3]).toEqual({
      _tag: "FrameDies",
      frame: "E1",
      why: "nothing below E2 reads initial-amount",
    });
    expect(letSpelling.events[2]).toEqual({
      _tag: "FrameSurvives",
      frame: "E2",
      why: "the processor was created in E2 and carries E2's cell",
    });
    const parameter = makeWithdrawTraced(100);
    expect(parameter.events[1]).toEqual({
      _tag: "FrameSurvives",
      frame: "E1",
      why: "the processor was created in E1 and carries E1's cell",
    });
  });

  it("the rendered structures match the traced runs", () => {
    expect(renderWithdrawalStructures(100)).toBe(
      [
        "make-withdraw(100) parameter spelling",
        "global env",
        "  E1: call make-withdraw",
        "    balance: 100",
        "  E1 survives: the processor was created in E1 and carries E1's cell",
        "  w1 = the processor over E1's cell",
        "",
        "make-withdraw-let(100) let spelling (the desugaring)",
        "global env",
        "  E1: call make-withdraw-let",
        "    initial-amount: 100",
        "  E2: the desugared let, applied",
        "    balance: the cell the let's argument created",
        "  E2 survives: the processor was created in E2 and carries E2's cell",
        "  E1 dies: nothing below E2 reads initial-amount",
        "  w1 = the processor over E2's cell",
        "",
        "same behavior, one more frame: the let spelling makes one more",
        "call, and the factory's own frame is not the one that survives.",
      ].join("\n"),
    );
  });
});
