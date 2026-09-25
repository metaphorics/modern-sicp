// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect, Ref } from "effect";

import type { Account, AccountRequest } from "../../packages/ch3/src/01-assignment.js";
import { InsufficientFunds } from "../../packages/ch3/src/01-assignment.js";

/**
 * Exercise 3.11: where the account state lives. The book asks for the
 * environment structure a sequence of `make-account` interactions
 * generates, where each account's local state is kept, and which parts
 * of the structure two accounts share. The edition's answer is a
 * traced run: each factory call is logged with the frame it opens and
 * the balance cell it creates, each request is logged against the
 * frame it runs in and the cell it moves, and the structure at the end
 * of the sequence is rendered from the log. The state of an account is
 * the cell its own factory call created; two factories share no cell;
 * what they share is the one `makeAccount` procedure object the global
 * frame holds and the arm code its body specifies.
 */

/** One logged event of the traced account run. */
export type EnvEvent =
  | { readonly _tag: "FactoryEntered"; readonly frame: string; readonly binds: string }
  | {
      readonly _tag: "CellCreated";
      readonly cell: string;
      readonly frame: string;
      readonly initial: number;
    }
  | { readonly _tag: "FactoryReturned"; readonly frame: string; readonly carried: string }
  | { readonly _tag: "RequestRun"; readonly inFrame: string; readonly reaches: string }
  | {
      readonly _tag: "CellChanged";
      readonly cell: string;
      readonly from: number;
      readonly to: number;
    };

/** The running log of a traced account sequence. */
export type EnvLog = Ref.Ref<ReadonlyArray<EnvEvent>>;

/** Opens the log every traced helper appends to. */
export const openEnvLog = (): EnvLog => Ref.makeUnsafe<ReadonlyArray<EnvEvent>>([]);

const record = (log: EnvLog, event: EnvEvent): Effect.Effect<void> =>
  Ref.update(log, (events) => [...events, event]);

/** One traced account: the dispatch, the balance cell its factory call
 * created, and the initial that call was handed. */
export interface AccountInspection {
  readonly account: Account;
  /** The name this factory call's frame and cell carry in the log. */
  readonly label: string;
  /** The initial balance the factory call bound. */
  readonly initial: number;
  /** The cell the factory call created: the account's local state. */
  readonly balanceCell: Ref.Ref<number>;
}

/** The book's `make-account`, traced: the call opens a frame for
 * `balance`, creates the balance cell inside it, builds the withdraw
 * and deposit arms and the dispatch there, and returns the dispatch as
 * the account object. */
export const makeAccountTraced = (
  initialBalance: number,
  label: string,
  log: EnvLog,
): Effect.Effect<AccountInspection> =>
  Effect.gen(function* () {
    const frame = `E(${label}): the call make-account(${String(initialBalance)})`;
    yield* record(log, {
      _tag: "FactoryEntered",
      frame,
      binds: `balance: ${String(initialBalance)}`,
    });
    const balanceCell = Ref.makeUnsafe(initialBalance);
    yield* record(log, {
      _tag: "CellCreated",
      cell: label,
      frame: `E(${label})`,
      initial: initialBalance,
    });
    const account: Account = (request: AccountRequest) =>
      Effect.gen(function* () {
        if (request._tag === "Withdraw") {
          const current = yield* Ref.get(balanceCell);
          if (current < request.amount) {
            return yield* new InsufficientFunds();
          }
          return yield* Ref.setAndGet(balanceCell, current - request.amount);
        }
        return yield* Ref.updateAndGet(balanceCell, (b) => b + request.amount);
      });
    yield* record(log, {
      _tag: "FactoryReturned",
      frame: `E(${label})`,
      carried: "the dispatch, born here, keeps this frame and its cell alive",
    });
    return { account, label, initial: initialBalance, balanceCell };
  });

/** Runs one request against a traced account, logging the request
 * frame it opens and the cell change it makes. */
export const runTraced = (
  inspected: AccountInspection,
  request: AccountRequest,
  log: EnvLog,
): Effect.Effect<number, InsufficientFunds> =>
  Effect.gen(function* () {
    yield* record(log, {
      _tag: "RequestRun",
      inFrame: `a request frame under E(${inspected.label}), dead once answered`,
      reaches: `the cell of E(${inspected.label}) through the dispatch`,
    });
    const before = yield* Ref.get(inspected.balanceCell);
    const answer = yield* inspected.account(request);
    const after = yield* Ref.get(inspected.balanceCell);
    if (before !== after) {
      yield* record(log, { _tag: "CellChanged", cell: inspected.label, from: before, to: after });
    }
    return answer;
  });

/** The rendered structure at the end of a traced sequence: the global
 * frame, the two factory frames with their cells, and the two
 * dispatches that keep them alive. */
export const renderAccountStructure = (
  acc: AccountInspection,
  acc2: AccountInspection,
  accBalance: number,
  acc2Balance: number,
): string =>
  [
    "global env",
    "  make-account: the one procedure object both calls applied",
    "  acc: --+                    acc2: --+",
    "         |                            |",
    `   E(acc): frame of make-account(${String(acc.initial)})` +
      `     E(acc2): frame of make-account(${String(acc2.initial)})`,
    `     balance cell: ${String(accBalance)}                balance cell: ${String(acc2Balance)}`,
    "     withdraw, deposit arms        withdraw, deposit arms",
    "     dispatch -> acc                dispatch -> acc2",
    "  acc's request frames died when  acc2 has made no requests,",
    "  they were answered; the cells   and its cell is untouched.",
    "  stay: the dispatches point at",
    "  the frames that hold them.",
  ].join("\n");

/** The book's interaction sequence, traced end to end: `acc` is
 * created with 50, deposits 40, withdraws 60; `acc2` is created with
 * 100 and left alone. */
export interface AccountTraceReport {
  readonly acc: AccountInspection;
  readonly acc2: AccountInspection;
  /** The answers the two requests on `acc` gave (90 and 30). */
  readonly deposited: number;
  readonly withdrawn: number;
  readonly events: ReadonlyArray<EnvEvent>;
  /** The structure at the end of the sequence, rendered from the log. */
  readonly structure: string;
}

/** Runs the book's sequence over traced accounts and reports the log,
 * the answers, and the rendered structure. */
export const traceAccountSequence = (
  accInitial: number,
  acc2Initial: number,
): Effect.Effect<AccountTraceReport, InsufficientFunds> =>
  Effect.gen(function* () {
    const log = openEnvLog();
    const acc = yield* makeAccountTraced(accInitial, "acc", log);
    const deposited = yield* runTraced(acc, { _tag: "Deposit", amount: 40 }, log);
    const withdrawn = yield* runTraced(acc, { _tag: "Withdraw", amount: 60 }, log);
    const acc2 = yield* makeAccountTraced(acc2Initial, "acc2", log);
    const events = yield* Ref.get(log);
    const accBalance = yield* Ref.get(acc.balanceCell);
    const acc2Balance = yield* Ref.get(acc2.balanceCell);
    return {
      acc,
      acc2,
      deposited,
      withdrawn,
      events,
      structure: renderAccountStructure(acc, acc2, accBalance, acc2Balance),
    };
  });
