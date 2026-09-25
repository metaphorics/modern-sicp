// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  allInterleavings,
  type Process,
  runInterleaving,
} from "../../packages/ch3/src/04-concurrency.js";

/**
 * Exercise 3.43: three accounts start at 10, 20, and 30, and exchanges
 * run concurrently. Peter exchanges a1 with a2 while Paul exchanges a1
 * with a3. Three versions are enumerated on the step scheduler: the
 * text's first exchange over individually serialized accounts (the
 * multiset can break, the sum cannot), the serialized exchange (only
 * permutations), and the same exchange over raw unserialized withdraw
 * and deposit steps (even the sum can break). Every final is produced
 * by an executed schedule.
 */

/** The three account balances, a1 first. */
export type Balances = readonly [number, number, number];

const START: readonly [number, number, number] = [10, 20, 30];

/** Reads a balance by index; the working tuples always have three
 * entries, so the range guard only answers the checker. */
const at = (a: readonly number[], index: number): number => {
  const value = a[index];
  if (value === undefined) {
    throw new RangeError("balance index out of range");
  }
  return value;
};

const isPermutationOfStart = (balances: Balances): boolean => {
  const sorted = [...balances].sort((a, b) => a - b);
  return sorted[0] === 10 && sorted[1] === 20 && sorted[2] === 30;
};

/** The first version of `exchange` over individually serialized
 * accounts: read both balances, then withdraw the difference from one
 * account and deposit it into the other, each update an atomic chunk. */
export const firstVersionFinals = (): ReadonlyArray<Balances> => {
  const finals: Array<Balances> = [];
  for (const order of allInterleavings([4, 4])) {
    const a: [number, number, number] = [...START];
    const makeExchange = (other: number): Process =>
      (function* () {
        const a1 = a[0];
        yield;
        const otherBalance = at(a, other);
        yield;
        const difference = a1 - otherBalance;
        a[0] = a[0] - difference;
        yield;
        a[other] = at(a, other) + difference;
      })();
    runInterleaving([makeExchange(1), makeExchange(2)], order);
    finals.push([a[0], a[1], a[2]]);
  }
  return finals;
};

/** The first distinct first-version final whose multiset is not {10,
 * 20, 30}, or undefined when every schedule preserved it. */
export const firstMultisetViolatingFinal = (): Balances | undefined =>
  firstVersionFinals().find((balances) => !isPermutationOfStart(balances));

/** The distinct sums the first-version schedules produce: only 60,
 * since each exchange moves a matched amount. */
export const firstVersionSums = (): ReadonlyArray<number> => {
  const sums = new Set<number>(firstVersionFinals().map(([a1, a2, a3]) => a1 + a2 + a3));
  return [...sums].sort((a, b) => a - b);
};

/** The serialized exchange: the whole swap is one chunk per process,
 * so the two exchanges run in one of two orders and the balances stay
 * a permutation of the start. */
export const serializedFinals = (): ReadonlyArray<Balances> => {
  const finals: Array<Balances> = [];
  for (const order of allInterleavings([1, 1])) {
    const a: [number, number, number] = [...START];
    const swapWith = (other: number): void => {
      const difference = a[0] - at(a, other);
      a[0] -= difference;
      a[other] = at(a, other) + difference;
    };
    const peter: Process =
      // biome-ignore lint/correctness/useYield: a serialized chunk is one scheduler step
      (function* () {
        swapWith(1);
      })();
    const paul: Process =
      // biome-ignore lint/correctness/useYield: a serialized chunk is one scheduler step
      (function* () {
        swapWith(2);
      })();
    runInterleaving([peter, paul], order);
    finals.push([a[0], a[1], a[2]]);
  }
  return finals;
};

/** The first version again, with the per-account serialization
 * removed: each withdraw and deposit is two raw steps (access, then
 * set), so two processes can interleave inside a single update and
 * lose money outright. */
export const noAccountSerializationFinals = (): ReadonlyArray<Balances> => {
  const finals: Array<Balances> = [];
  for (const order of allInterleavings([7, 7])) {
    const a: [number, number, number] = [...START];
    const makeExchange = (other: number): Process =>
      (function* () {
        const a1 = a[0];
        yield;
        const otherBalance = at(a, other);
        yield;
        const difference = a1 - otherBalance;
        yield;
        const withdrawAccess = a[0];
        yield;
        a[0] = withdrawAccess - difference;
        yield;
        const depositAccess = at(a, other);
        yield;
        a[other] = depositAccess + difference;
      })();
    runInterleaving([makeExchange(1), makeExchange(2)], order);
    finals.push([a[0], a[1], a[2]]);
  }
  return finals;
};

/** The distinct sums the unserialized schedules produce, ascending:
 * every sum below 60 is money the races destroyed. */
export const noAccountSerializationSums = (): ReadonlyArray<number> => {
  const sums = new Set<number>(noAccountSerializationFinals().map(([a1, a2, a3]) => a1 + a2 + a3));
  return [...sums].sort((a, b) => a - b);
};
