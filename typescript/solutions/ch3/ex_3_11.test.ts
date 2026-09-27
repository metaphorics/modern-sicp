// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect, Ref } from "effect";
import { describe, expect } from "vitest";

import {
  makeAccountTraced,
  openEnvLog,
  renderAccountStructure,
  runTraced,
  traceAccountSequence,
} from "./ex_3_11.js";

describe("exercise 3.11: where the account state lives", () => {
  it.effect("the book's sequence answers 90 and 30", () =>
    Effect.gen(function* () {
      const report = yield* traceAccountSequence(50, 100);
      expect(report.deposited).toBe(90);
      expect(report.withdrawn).toBe(30);
    }),
  );

  it.effect("each factory call made its own cell, so the states stay distinct", () =>
    Effect.gen(function* () {
      const report = yield* traceAccountSequence(50, 100);
      expect(Object.is(report.acc.balanceCell, report.acc2.balanceCell)).toBe(false);
      expect(yield* Ref.get(report.acc.balanceCell)).toBe(30);
      expect(yield* Ref.get(report.acc2.balanceCell)).toBe(100);
      const answer = yield* runTraced(report.acc2, { _tag: "Withdraw", amount: 60 }, openEnvLog());
      expect(answer).toBe(40);
      expect(yield* Ref.get(report.acc.balanceCell)).toBe(30);
    }),
  );

  it.effect("two dispatches, one code: the accounts are distinct objects over distinct cells", () =>
    Effect.gen(function* () {
      const report = yield* traceAccountSequence(50, 100);
      expect(Object.is(report.acc.account, report.acc2.account)).toBe(false);
      // The shared part is the make-account procedure object both calls
      // applied; each call built its own frame, arms, and cell.
      expect(report.events.filter((event) => event._tag === "FactoryEntered")).toHaveLength(2);
    }),
  );

  it.effect("the log shows every cell change belonging to its own frame", () =>
    Effect.gen(function* () {
      const report = yield* traceAccountSequence(50, 100);
      const changed = report.events.flatMap((event) =>
        event._tag === "CellChanged"
          ? [`${event.cell}: ${String(event.from)} -> ${String(event.to)}`]
          : [],
      );
      expect(changed).toEqual(["acc: 50 -> 90", "acc: 90 -> 30"]);
      expect(report.events.filter((event) => event._tag === "CellCreated")).toEqual([
        { _tag: "CellCreated", cell: "acc", frame: "E(acc)", initial: 50 },
        { _tag: "CellCreated", cell: "acc2", frame: "E(acc2)", initial: 100 },
      ]);
    }),
  );

  it.effect("the rendered structure matches the traced run", () =>
    Effect.gen(function* () {
      const log = openEnvLog();
      const acc = yield* makeAccountTraced(50, "acc", log);
      yield* runTraced(acc, { _tag: "Deposit", amount: 40 }, log);
      yield* runTraced(acc, { _tag: "Withdraw", amount: 60 }, log);
      const acc2 = yield* makeAccountTraced(100, "acc2", log);
      const accBalance = yield* Ref.get(acc.balanceCell);
      const acc2Balance = yield* Ref.get(acc2.balanceCell);
      expect(renderAccountStructure(acc, acc2, accBalance, acc2Balance)).toBe(
        [
          "global env",
          "  make-account: the one procedure object both calls applied",
          "  acc: --+                    acc2: --+",
          "         |                            |",
          "   E(acc): frame of make-account(50)     E(acc2): frame of make-account(100)",
          "     balance cell: 30                balance cell: 100",
          "     withdraw, deposit arms        withdraw, deposit arms",
          "     dispatch -> acc                dispatch -> acc2",
          "  acc's request frames died when  acc2 has made no requests,",
          "  they were answered; the cells   and its cell is untouched.",
          "  stay: the dispatches point at",
          "  the frames that hold them.",
        ].join("\n"),
      );
    }),
  );
});
