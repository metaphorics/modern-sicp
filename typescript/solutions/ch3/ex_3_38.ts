// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  allInterleavings,
  type Process,
  runInterleaving,
} from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.38: Peter deposits 10, Paul withdraws 20, and Mary
 * withdraws half the balance, all on a joint account that starts at
 * 100. Each command is the statement's three-step update: access the
 * balance, compute the new value, set it. The possible outcomes are
 * produced by running the schedules, not by hand: sequential orders
 * through the module's step scheduler, interleaved orders likewise,
 * one schedule at a time.
 */

/** The joint account: one balance cell the three commands mutate. */
export interface JointAccount {
  balance: number;
}

/** Makes the account with the exercise's initial 100. */
export const makeJointAccount = (): JointAccount => ({ balance: 100 });

/** Peter's `balance = balance + 10` as three steps. */
export const peterProcess = (account: JointAccount): Process =>
  (function* () {
    const accessed = account.balance;
    yield;
    const next = accessed + 10;
    yield;
    account.balance = next;
  })();

/** Paul's `balance = balance - 20` as three steps. */
export const paulProcess = (account: JointAccount): Process =>
  (function* () {
    const accessed = account.balance;
    yield;
    const next = accessed - 20;
    yield;
    account.balance = next;
  })();

/** Mary's `balance = balance - balance / 2` as three steps:
 * the two accesses of the book's expression are two separate steps, so
 * an interleaving can change the balance between them. */
export const maryProcess = (account: JointAccount): Process =>
  (function* () {
    const minuend = account.balance;
    yield;
    const subtrahend = account.balance;
    yield;
    account.balance = minuend - subtrahend / 2;
  })();

const expand = (permutation: ReadonlyArray<number>): ReadonlyArray<number> =>
  permutation.flatMap((which) => [which, which, which]);

const runWith = (account: JointAccount, order: ReadonlyArray<number>): number => {
  const peter = peterProcess(account);
  const paul = paulProcess(account);
  const mary = maryProcess(account);
  runInterleaving([peter, paul, mary], order);
  return account.balance;
};

/** Part 1: the values after the three transactions complete in every
 * sequential order (the six permutations of the three processes), in
 * the order the permutations run. */
export const sequentialValues = (): ReadonlyArray<number> => {
  const permutations = [
    [0, 1, 2],
    [0, 2, 1],
    [1, 0, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
  ];
  return permutations.map((permutation) => runWith(makeJointAccount(), expand(permutation)));
};

/** Part 2: every distinct value over all interleavings of the three
 * three-step commands, ascending. Three processes of three steps give
 * 9!/(3!3!3!) = 1680 schedules, each really executed. */
export const interleavedValues = (): ReadonlyArray<number> => {
  const values = new Set<number>();
  for (const order of allInterleavings([3, 3, 3])) {
    values.add(runWith(makeJointAccount(), order));
  }
  return [...values].sort((a, b) => a - b);
};

/** Part 2's timing-diagram answer: the values only interleaving can
 * produce, those absent from every sequential order, ascending. */
export const interleavingOnlyValues = (): ReadonlyArray<number> => {
  const sequential = new Set<number>(sequentialValues());
  return interleavedValues().filter((value) => !sequential.has(value));
};
