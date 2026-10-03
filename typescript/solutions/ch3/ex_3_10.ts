// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect, Ref } from "effect";

import type { Withdrawal, WithdrawalProcessor } from "../../packages/ch3/src/01-assignment.js";
import { InsufficientFunds } from "../../packages/ch3/src/01-assignment.js";

/**
 * Exercise 3.10: the `let` spelling of `make-withdraw`. The book's
 * alternate version binds `balance` with a `let`; since `let` is sugar
 * for applying a lambda (1.3.2), that spelling builds one frame more
 * than the parameter version. The edition's answer traces both
 * spellings: every frame a factory call opens is logged with the
 * binding it holds, and its fate after the factory returns is recorded,
 * so the lifetime difference between the versions is data produced by a
 * run, not a drawing.
 */

/** The statement's alternate `makeWithdraw`, spelled as its own
 * desugaring: an immediately-invoked arrow expression binds `balance`
 * to `initialAmount`, so the balance cell is born in the frame of an
 * immediately applied function, one call deeper than in the parameter
 * version. */
export const makeWithdrawLet = (initialAmount: number): WithdrawalProcessor =>
  (
    (balance: Ref.Ref<number>) =>
    (amount: number): Withdrawal =>
      Effect.gen(function* () {
        const current = yield* Ref.get(balance);
        if (current < amount) {
          return yield* new InsufficientFunds();
        }
        return yield* Ref.setAndGet(balance, current - amount);
      })
  )(Ref.makeUnsafe(initialAmount));

/** One frame event of a traced factory call: what the frame binds, or
 * whether it survives the factory's return. */
export type FrameEvent =
  | { readonly _tag: "FrameEntered"; readonly frame: string; readonly binds: string }
  | { readonly _tag: "FrameSurvives"; readonly frame: string; readonly why: string }
  | { readonly _tag: "FrameDies"; readonly frame: string; readonly why: string };

/** A traced factory run: the processor it built plus the frame log the
 * building produced. */
export interface WithdrawalTrace {
  readonly processor: WithdrawalProcessor;
  readonly events: ReadonlyArray<FrameEvent>;
}

/** Traces the parameter version (the section's own shape): the call
 * opens one frame; the cell is created and the processor is built in
 * that frame, so the frame's cell outlives the factory call through
 * the processor. */
export const makeWithdrawTraced = (initialBalance: number): WithdrawalTrace => {
  const events: FrameEvent[] = [
    {
      _tag: "FrameEntered",
      frame: "E1: call make-withdraw",
      binds: `balance: ${String(initialBalance)}`,
    },
  ];
  const encapsulated = Ref.makeUnsafe(initialBalance);
  const processor: WithdrawalProcessor = (amount) =>
    Effect.gen(function* () {
      const current = yield* Ref.get(encapsulated);
      if (current < amount) {
        return yield* new InsufficientFunds();
      }
      return yield* Ref.setAndGet(encapsulated, current - amount);
    });
  events.push({
    _tag: "FrameSurvives",
    frame: "E1",
    why: "the processor was created in E1 and carries E1's cell",
  });
  return { processor, events };
};

/** Traces the let spelling: the factory call opens E1 for
 * `initial-amount`, the desugared let is an immediately applied call
 * that opens E2 for the balance cell, the processor is created in E2,
 * and E1 dies at the factory's return because nothing below E2 reads
 * `initial-amount`. */
export const makeWithdrawLetTraced = (initialAmount: number): WithdrawalTrace => {
  const events: FrameEvent[] = [
    {
      _tag: "FrameEntered",
      frame: "E1: call make-withdraw-let",
      binds: `initial-amount: ${String(initialAmount)}`,
    },
  ];
  const processor = ((balance: Ref.Ref<number>) => {
    events.push({
      _tag: "FrameEntered",
      frame: "E2: the desugared let, applied",
      binds: "balance: the cell the let's argument created",
    });
    const inner: WithdrawalProcessor = (amount: number) =>
      Effect.gen(function* () {
        const current = yield* Ref.get(balance);
        if (current < amount) {
          return yield* new InsufficientFunds();
        }
        return yield* Ref.setAndGet(balance, current - amount);
      });
    events.push({
      _tag: "FrameSurvives",
      frame: "E2",
      why: "the processor was created in E2 and carries E2's cell",
    });
    return inner;
  })(Ref.makeUnsafe(initialAmount));
  events.push({
    _tag: "FrameDies",
    frame: "E1",
    why: "nothing below E2 reads initial-amount",
  });
  return { processor, events };
};

/** Renders the two structures the two spellings build for one factory
 * call, from their own traced runs: the frames, their bindings, and
 * their fates, exactly as the events logged them. */
export const renderWithdrawalStructures = (initial: number): string => {
  const parameter = makeWithdrawTraced(initial);
  const letSpelling = makeWithdrawLetTraced(initial);
  const lines: string[] = [];
  lines.push(`make-withdraw(${String(initial)}) parameter spelling`);
  lines.push("global env");
  const entered = parameter.events[0];
  if (entered !== undefined && entered._tag === "FrameEntered") {
    lines.push(`  ${entered.frame}`);
    lines.push(`    ${entered.binds}`);
  }
  const fate = parameter.events[1];
  if (fate !== undefined && fate._tag === "FrameSurvives") {
    lines.push(`  ${fate.frame} survives: ${fate.why}`);
  }
  lines.push("  w1 = the processor over E1's cell");
  lines.push("");
  lines.push(`make-withdraw-let(${String(initial)}) let spelling (the desugaring)`);
  lines.push("global env");
  for (const event of letSpelling.events) {
    if (event._tag === "FrameEntered") {
      lines.push(`  ${event.frame}`);
      lines.push(`    ${event.binds}`);
    } else if (event._tag === "FrameSurvives") {
      lines.push(`  ${event.frame} survives: ${event.why}`);
    } else {
      lines.push(`  ${event.frame} dies: ${event.why}`);
    }
  }
  lines.push("  w1 = the processor over E2's cell");
  lines.push("");
  lines.push("same behavior, one more frame: the let spelling makes one more");
  lines.push("call, and the factory's own frame is not the one that survives.");
  return lines.join("\n");
};
