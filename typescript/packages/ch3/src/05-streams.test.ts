// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 3.5

import { describe, expect, it } from "vitest";

import {
  accelerateSequence,
  addStreams,
  consStream,
  delay,
  displayStream,
  eulerTransform,
  fibs,
  force,
  integers,
  integersStartingFrom,
  integral,
  isPrimeStream,
  makeZeroCrossings,
  memoProc,
  merge,
  noSevens,
  ones,
  pairs,
  partialSums,
  piEstimates,
  piStream,
  piSummands,
  primes,
  primesFiltered,
  randomNumbers,
  type Stream,
  type StreamCell,
  scaleStream,
  show,
  sieve,
  solve,
  sqrtImprove,
  sqrtStream,
  streamCar,
  streamCdr,
  streamEnumerateInterval,
  streamFilter,
  streamForEach,
  streamIsNull,
  streamMap,
  streamRef,
  streamTake,
  streamWithdraw,
  theEmptyStream,
} from "./05-streams.js";

/** Builds a finite stream from values, the tests' stand-in for the
 * quoted streams. */
const streamOf = <A>(values: ReadonlyArray<A>): Stream<A> => {
  const build = (index: number): Stream<A> =>
    index >= values.length
      ? theEmptyStream
      : consStream(values[index] as A, () => build(index + 1));
  return build(0);
};

describe("3.5.1: delay, force, and the stream pair", () => {
  it("memoProc computes the wrapped procedure once and stores the value", () => {
    let runs = 0;
    const slow = memoProc(() => {
      runs += 1;
      return runs * 10;
    });
    expect(slow()).toBe(10);
    expect(slow()).toBe(10);
    expect(slow()).toBe(10);
    expect(runs).toBe(1);
  });

  it("delay is memoProc over a thunk and force is the stored procedure", () => {
    let runs = 0;
    const promise = delay(() => {
      runs += 1;
      return "computed";
    });
    expect(force(promise)).toBe("computed");
    expect(force(promise)).toBe("computed");
    expect(runs).toBe(1);
  });

  it("cons-stream delays the tail: the closure runs on first cdr only", () => {
    let tailRuns = 0;
    const s: StreamCell<number> = consStream(1, () => {
      tailRuns += 1;
      return consStream(2, () => theEmptyStream);
    });
    expect(streamCar(s)).toBe(1);
    expect(tailRuns).toBe(0);
    const rest = streamCdr(s);
    expect(streamCar(rest)).toBe(2);
    expect(tailRuns).toBe(1);
    streamCdr(s);
    expect(tailRuns).toBe(1);
  });

  it("the filter/enumerate pipeline answers 10009, testing only as far as needed", () => {
    const secondPrime = streamCar(
      streamCdr(streamFilter(isPrimeStream, streamEnumerateInterval(10000, 1000000))),
    );
    expect(secondPrime).toBe(10009);
  });

  it("stream-ref primes 50 is 233", () => {
    expect(streamRef(primes, 50)).toBe(233);
  });

  it("the empty stream is null and streamIsNull asks for it", () => {
    expect(streamIsNull(theEmptyStream)).toBe(true);
    expect(streamIsNull(consStream(1, () => theEmptyStream))).toBe(false);
  });

  it("stream-for-each walks a finite stream in order", () => {
    const seen: number[] = [];
    streamForEach(
      (x) => {
        seen.push(x);
      },
      streamOf([1, 2, 3, 4, 5]),
    );
    expect(seen).toEqual([1, 2, 3, 4, 5]);
  });

  it("the printing convention renders lines and show appends to a transcript", () => {
    const transcript: string[] = [];
    const value = show(42, transcript);
    expect(value).toBe(42);
    expect(transcript).toEqual(["42"]);
    expect(displayStream(streamOf([1, 2, 3]))).toEqual(["1", "2", "3"]);
  });
});

describe("3.5.2: infinite streams", () => {
  it("no-sevens answers 117 at index 100", () => {
    expect(streamRef(noSevens, 100)).toBe(117);
  });

  it("fibs begins 0 1 1 2 3 5 8 13 21 34", () => {
    expect(streamTake(fibs, 10)).toEqual([0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
  });

  it("ones repeats through memoized self-reference", () => {
    expect(streamTake(ones, 4)).toEqual([1, 1, 1, 1]);
  });

  it("integers is ones plus itself shifted, beginning 1 2 3 4 5", () => {
    expect(streamTake(integers, 5)).toEqual([1, 2, 3, 4, 5]);
  });

  it("add-streams adds two streams element-wise", () => {
    expect(streamTake(addStreams(integers, integers), 5)).toEqual([2, 4, 6, 8, 10]);
  });

  it("scale-stream maps multiplication: 2x the integers", () => {
    expect(streamTake(scaleStream(integersStartingFrom(1), 2), 6)).toEqual([2, 4, 6, 8, 10, 12]);
  });

  it("the sieve primes begin with the primes and both definitions agree", () => {
    expect(streamTake(primes, 12)).toEqual([2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]);
    expect(streamTake(primesFiltered, 12)).toEqual(streamTake(primes, 12));
  });

  it("sieve of the integers from 2 is the primes the text constructs", () => {
    expect(streamTake(sieve(integersStartingFrom(2)), 8)).toEqual([2, 3, 5, 7, 11, 13, 17, 19]);
  });
});

describe("3.5.3: iterations, accelerations, and pairs", () => {
  it("sqrt-stream converges on sqrt(2) through Newton steps", () => {
    const guesses = streamTake(sqrtStream(2), 6);
    expect(guesses[0]).toBe(1);
    expect(guesses[1]).toBe(sqrtImprove(1, 2));
    expect(guesses[5]).toBeCloseTo(Math.sqrt(2), 12);
  });

  it("pi-stream's first eight approximations bracket pi", () => {
    const approximations = streamTake(piStream, 8);
    expect(approximations[0]).toBe(4);
    expect(approximations[1]).toBeCloseTo(2.6666667, 6);
    expect(approximations[7]).toBeCloseTo(3.0170718, 6);
  });

  it("pi-summands alternates 1, -1/3, 1/5", () => {
    expect(streamTake(piSummands(1), 3)).toEqual([1, -1 / 3, 1 / 5]);
  });

  it("euler-transform accelerates the pi approximations", () => {
    const accelerated = streamTake(eulerTransform(piStream), 5);
    expect(accelerated[0]).toBeCloseTo(3.1666667, 6);
    expect(accelerated[1]).toBeCloseTo(3.1333333, 6);
    expect(accelerated[2]).toBeCloseTo(3.1452381, 6);
  });

  it("the accelerated sequence reaches pi to the book's ten decimal places by the sixth term", () => {
    const accelerated = streamTake(accelerateSequence(eulerTransform, piStream), 6);
    expect(accelerated[3]).toBeCloseTo(3.1415994, 6);
    expect(Math.abs((accelerated[5] as number) - Math.PI)).toBeLessThan(1e-9);
  });

  it("partial-sums runs a running total", () => {
    expect(streamTake(partialSums(integers), 6)).toEqual([1, 3, 6, 10, 15, 21]);
  });

  it("pairs of the integers begin in the text's diagonal order", () => {
    const first = streamTake(pairs(integers, integers), 8);
    expect(first).toEqual([
      [1, 1],
      [1, 2],
      [2, 2],
      [1, 3],
      [2, 3],
      [1, 4],
      [3, 3],
      [1, 5],
    ]);
  });

  it("merge orders two sorted streams and eliminates repeats", () => {
    const squares = streamMap((x: number) => x * x, integersStartingFrom(1));
    expect(streamTake(merge(integersStartingFrom(1), squares), 10)).toEqual([
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    ]);
  });
});

describe("3.5.4: streams and delayed evaluation", () => {
  it("integral accumulates dt times the integrand from the initial value", () => {
    expect(streamTake(integral(integers, 0, 1), 6)).toEqual([0, 1, 3, 6, 10, 15]);
  });

  it("make-zero-crossings reads the text's sense data as the text prints it", () => {
    const senseData = streamOf([1, 2, 1.5, 1, 0.5, -0.1, -2, -3, -2, -0.5, 0.2, 3, 4]);
    const crossings = streamTake(makeZeroCrossings(senseData, 0), 12);
    expect(crossings).toEqual([0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1, 0]);
  });

  it("solve integrates dy/dt = y toward e by step 1000", () => {
    const solution = solve((y) => y, 1, 0.001);
    expect(streamRef(solution, 1000)).toBeCloseTo(2.7169239, 6);
  });
});

describe("3.5.5: modularity of functional programs", () => {
  it("random-numbers seeds at 42 and maps rand-update over itself", () => {
    expect(streamTake(randomNumbers, 5)).toEqual([42, 11355432, 2836018348, 476557059, 3648046016]);
  });

  it("monte-carlo's Cesaro pi estimates drift toward pi as trials accumulate", () => {
    expect(streamRef(piEstimates, 100)).toBeGreaterThan(2.9);
    expect(streamRef(piEstimates, 1000)).toBeCloseTo(3.14, 1);
  });

  it("stream-withdraw is a function from balance and amounts to balances", () => {
    const amounts = streamOf([10, 25, 5]);
    expect(streamTake(streamWithdraw(100, amounts), 4)).toEqual([100, 90, 65]);
  });
});
