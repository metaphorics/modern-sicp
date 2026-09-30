// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 3.5

/**
 * Streams: delayed lists, spelled in the edition's idiom.
 *
 * The book's streams are pairs whose tail is a `delay`ed promise, so a
 * stream represents its rest lazily and infinite definitions terminate
 * on demand. This edition keeps exactly that shape over plain thunks:
 * a stream is a head plus a memoized zero-argument closure, and the
 * book's `delay` is the hand-rolled `memoProc` of the section itself
 * (the exercise-map note: only OCaml's `Lazy.t` memoizes out of the
 * box, so this edition hand-rolls the book's `memo-proc` the way the
 * text prescribes). The empty stream is `null`.
 *
 * Printing convention: the book's `;Value:` transcripts have no
 * counterpart here, so every answer the text prints is rendered as a
 * value instead. `displayLine` renders one line, `show` renders and
 * appends to a caller-supplied transcript array, and `displayStream`
 * collects the lines of a finite stream; finite prefixes of infinite
 * streams are inspected with `streamTake` or `streamRef`. Every
 * `// => ...` answer quoted in the book pages of this section is
 * produced by executing these helpers, never copied.
 */

import { gcd, randomInit, randUpdate } from "./01-assignment.js";

// ---------------------------------------------------------------------
// 3.5.1 Streams Are Delayed Lists
// ---------------------------------------------------------------------

/** The book's `delay`: a memoized zero-argument closure. Running it
 * computes the wrapped value once and returns the stored value after
 * that, the section's `memo-proc` discipline. */
export type Delayed<A> = () => A;

/** One stream cell: the book's `(cons a (delay b))` pair. The head is
 * eager, the tail is a promise. */
export interface StreamCell<A> {
  readonly head: A;
  readonly tail: Delayed<Stream<A>>;
}

/** A stream: a chain of cells ending in the empty stream, which this
 * edition spells `null`. */
export type Stream<A> = StreamCell<A> | null;

/** The book's `memo-proc`: wraps a zero-argument procedure so the
 * first run computes and stores the value, and later runs return the
 * stored value without repeating the computation. */
export const memoProc = <A>(proc: () => A): Delayed<A> => {
  let alreadyRun = false;
  let result: A | undefined;
  return () => {
    if (!alreadyRun) {
      result = proc();
      alreadyRun = true;
    }
    return result as A;
  };
};

/** The book's `delay`: `(delay exp)` is `(memo-proc (lambda () exp))`. */
export const delay = <A>(proc: () => A): Delayed<A> => memoProc(proc);

/** The book's `force`: the stored procedure of a delayed object. */
export const force = <A>(delayed: Delayed<A>): A => delayed();

/** The book's `the-empty-stream`. */
export const theEmptyStream: Stream<never> = null;

/** The book's `stream-null?`. */
export const streamIsNull = <A>(s: Stream<A>): boolean => s === null;

/** The book's `cons-stream`: the head is eager, the tail is the
 * memoized promise of the argument closure. Memoizing here is exactly
 * where the book's `delay` sits in `(cons-stream a (delay b))`. */
export const consStream = <A>(head: A, tail: () => Stream<A>): StreamCell<A> => ({
  head,
  tail: delay(tail),
});

/** The book's `stream-car`: the head of the pair. The empty stream has
 * no car, the book's error. */
export const streamCar = <A>(s: Stream<A>): A => {
  if (s === null) {
    throw new Error("stream-car: the empty stream has no car");
  }
  return s.head;
};

/** The book's `stream-cdr`: forcing the delayed tail of the pair. The
 * empty stream has no cdr, the book's error. */
export const streamCdr = <A>(s: Stream<A>): Stream<A> => {
  if (s === null) {
    throw new Error("stream-cdr: the empty stream has no cdr");
  }
  return force(s.tail);
};

/** The book's `stream-ref`: the n-th element (0-based), computed on
 * demand by walking tails. */
export const streamRef = <A>(s: Stream<A>, n: number): A => {
  if (s === null) {
    throw new Error(`stream-ref: no element at index ${n}`);
  }
  if (n === 0) {
    return s.head;
  }
  return streamRef(streamCdr(s), n - 1);
};

/** The book's one-argument `stream-map` of 3.5.1: applies `f` to each
 * element, lazily. The multi-argument generalization is exercise
 * 3.50; until the section needs it, two streams combine through
 * `streamMap2`. */
export const streamMap = <A, B>(f: (a: A) => B, s: Stream<A>): Stream<B> =>
  s === null ? null : consStream(f(s.head), () => streamMap(f, streamCdr(s)));

/** The two-stream map the text's `add-streams` needs: element-wise `f`
 * over `s1` and `s2`, ending where either ends. */
export const streamMap2 = <A, B, C>(
  f: (a: A, b: B) => C,
  s1: Stream<A>,
  s2: Stream<B>,
): Stream<C> => {
  if (s1 === null || s2 === null) {
    return null;
  }
  return consStream(f(s1.head, s2.head), () => streamMap2(f, streamCdr(s1), streamCdr(s2)));
};

/** The book's `stream-for-each`: applies `proc` to every element of a
 * finite stream. */
export const streamForEach = <A>(proc: (a: A) => void, s: Stream<A>): void => {
  if (s === null) {
    return;
  }
  proc(s.head);
  streamForEach(proc, streamCdr(s));
};

/** The book's `display-line`: renders one element as a line. The book
 * prints; this edition returns the line so callers and tests can pin
 * it. */
export const displayLine = <A>(x: A): string => String(x);

/** The book's `display-stream`: the lines of a finite stream, one per
 * element, in order. */
export const displayStream = <A>(s: Stream<A>): string[] => {
  const lines: string[] = [];
  streamForEach((x) => {
    lines.push(displayLine(x));
  }, s);
  return lines;
};

/** The book's `show` of exercise 3.51: renders its argument into the
 * supplied transcript and returns the argument unchanged. The
 * transcript array is the edition's stand-in for what the interpreter
 * prints. */
export const show = <A>(x: A, transcript: string[]): A => {
  transcript.push(displayLine(x));
  return x;
};

/** The edition's finite-prefix reader: the first `n` elements of `s`,
 * the visible slice of an infinite stream. */
export const streamTake = <A>(s: Stream<A>, n: number): A[] => {
  const items: A[] = [];
  let rest = s;
  for (let i = 0; i < n && rest !== null; i += 1) {
    items.push(rest.head);
    rest = streamCdr(rest);
  }
  return items;
};

/** The book's `stream-filter`: the elements satisfying `pred`, in
 * order, computed on demand. Skipped elements are walked in a loop:
 * a recursive skip would grow the control stack with the gap length,
 * and sparse filters (the primes over the integers, the Pythagorean
 * search of exercise 3.69) cross gaps of thousands of elements. Each
 * skipped element is still visited exactly once, since every output
 * cell is built at most once by the memoized tails. */
export const streamFilter = <A>(pred: (a: A) => boolean, s: Stream<A>): Stream<A> => {
  let rest = s;
  while (rest !== null && !pred(rest.head)) {
    rest = streamCdr(rest);
  }
  if (rest === null) {
    return null;
  }
  const head = rest.head;
  return consStream(head, () => streamFilter(pred, streamCdr(rest)));
};

/** The book's `stream-enumerate-interval`: `low` through `high`. */
export const streamEnumerateInterval = (low: number, high: number): Stream<number> => {
  if (low > high) {
    return theEmptyStream;
  }
  return consStream(low, () => streamEnumerateInterval(low + 1, high));
};

// ---------------------------------------------------------------------
// 3.5.2 Infinite Streams
// ---------------------------------------------------------------------

/** The book's `integers-starting-from`: the infinite stream `n, n+1,
 * n+2, ...`, defined in terms of itself. */
export const integersStartingFrom = (n: number): StreamCell<number> =>
  consStream(n, () => integersStartingFrom(n + 1));

/** The book's `divisible?`. */
export const divisible = (x: number, y: number): boolean => x % y === 0;

/** The book's `no-sevens`: the integers not divisible by 7, the first
 * infinite stream the section defines. */
export const noSevens: Stream<number> = streamFilter(
  (x) => !divisible(x, 7),
  integersStartingFrom(1),
);

/** The book's `fibgen`: the Fibonacci stream from consecutive seeds,
 * `a` now and the promise of `fibgen(b, a + b)` next. */
export const fibgen = (a: number, b: number): StreamCell<number> =>
  consStream(a, () => fibgen(b, a + b));

/** The book's `fibs`: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ... */
export const fibs: StreamCell<number> = fibgen(0, 1);

/** The book's `ones`: the infinite stream of ones, defined in terms of
 * itself through the closure: the definition evaluates to a cell whose
 * tail promise references the finished binding. */
export const ones: StreamCell<number> = consStream(1, () => ones);

/** The book's `add-streams`: element-wise addition of two streams. */
export const addStreams = (s1: Stream<number>, s2: Stream<number>): Stream<number> =>
  streamMap2((a, b) => a + b, s1, s2);

/** The book's `integers`: 1, 2, 3, ..., the integers plus the ones,
 * defined in terms of itself. */
export const integers: StreamCell<number> = consStream(1, () => addStreams(ones, integers));

/** The book's `scale-stream`: each element times the factor. */
export const scaleStream = (s: Stream<number>, factor: number): Stream<number> =>
  streamMap((x) => x * factor, s);

/** The book's `sieve`: the first element is kept, the rest is the
 * sieve of everything not divisible by it. */
export const sieve = (s: Stream<number>): Stream<number> => {
  if (s === null) {
    return null;
  }
  const first = s.head;
  return consStream(first, () => sieve(streamFilter((x) => !divisible(x, first), streamCdr(s))));
};

/** The book's `primes`: the sieve started at 2. */
export const primes: Stream<number> = sieve(integersStartingFrom(2));

/** The section's primality predicate over the primes stream: `n` is prime
 * when no prime up to its square root divides it. It tests candidates
 * against the filtered primes stream itself — the self-reference the
 * section teaches. */
export const isPrimeStream = (n: number): boolean => {
  const iter = (ps: Stream<number>): boolean => {
    if (ps === null) {
      return true;
    }
    if (ps.head * ps.head > n) {
      return true;
    }
    if (divisible(n, ps.head)) {
      return false;
    }
    return iter(streamCdr(ps));
  };
  return iter(primesFiltered);
};

/** The book's second `primes`: the filter version, testing candidates
 * against the stream itself. Both definitions generate the same
 * stream; the sieve is cheaper. */
export const primesFiltered: StreamCell<number> = consStream(2, () =>
  streamFilter(isPrimeStream, integersStartingFrom(3)),
);

// ---------------------------------------------------------------------
// 3.5.3 Exploiting the Stream Paradigm
// ---------------------------------------------------------------------

/** The book's `partial-sums` (exercise 3.55, which the section's
 * approximating streams use): running total of the elements. The tail
 * closes over the `sums` binding itself, the section's self-reference
 * idiom: each element is then one memoized lookup plus one addition.
 * Calling the function again inside the tail would build a fresh
 * stream per element and make the walk quadratic. */
export const partialSums = (s: Stream<number>): Stream<number> => {
  if (s === null) {
    return null;
  }
  const sums: StreamCell<number> = consStream(s.head, () => addStreams(sums, streamCdr(s)));
  return sums;
};

/** The book's `sqrt-improve`: one Newton step for sqrt(x). */
export const sqrtImprove = (guess: number, x: number): number => (guess + x / guess) / 2;

/** The book's `sqrt-stream`: the Newton approximations to sqrt(x),
 * each computed from the one before, defined in terms of itself. The
 * local `guesses` binding is the section's answer to exercise 3.63's
 * memoization question: one shared stream, memoized by `consStream`. */
export const sqrtStream = (x: number): StreamCell<number> => {
  const guesses: StreamCell<number> = consStream(1, () =>
    streamMap((g) => sqrtImprove(g, x), guesses),
  );
  return guesses;
};

/** The book's `pi-summands`: 1, -1/3, 1/5, ..., the alternating series
 * whose partial sums times 4 approach pi. */
export const piSummands = (n: number): StreamCell<number> =>
  consStream(1 / n, () => streamMap((x) => -x, piSummands(n + 2)));

/** The book's `pi-stream`: four times the partial sums of
 * `piSummands(1)`. */
export const piStream: Stream<number> = scaleStream(partialSums(piSummands(1)), 4);

/** The book's `euler-transform`: one acceleration step over the first
 * three elements, with the rest delayed. */
export const eulerTransform = (s: Stream<number>): Stream<number> => {
  if (s === null) {
    return null;
  }
  const s0 = streamRef(s, 0);
  const s1 = streamRef(s, 1);
  const s2 = streamRef(s, 2);
  return consStream(s2 - ((s2 - s1) * (s2 - s1)) / (s0 - 2 * s1 + s2), () =>
    eulerTransform(streamCdr(s)),
  );
};

/** The book's `make-tableau`: each row is the transform of the row
 * before, a stream of streams. */
export const makeTableau = (
  transform: (s: Stream<number>) => Stream<number>,
  s: Stream<number>,
): Stream<Stream<number>> => consStream(s, () => makeTableau(transform, transform(s)));

/** The book's `accelerated-sequence`: the first element of every row
 * of the tableau. */
export const accelerateSequence = (
  transform: (s: Stream<number>) => Stream<number>,
  s: Stream<number>,
): Stream<number> =>
  streamMap((row) => (row === null ? Number.NaN : row.head), makeTableau(transform, s));

/** The book's `interleave`: alternates elements from `s1` and `s2`,
 * so even an infinite `s1` cannot starve `s2`. */
export const interleave = <A>(s1: Stream<A>, s2: Stream<A>): Stream<A> => {
  if (s1 === null) {
    return s2;
  }
  return consStream(s1.head, () => interleave(s2, streamCdr(s1)));
};

/** The book's `pairs` (after exercise 3.67's generalization the text
 * lands on): the pairs `(x, y)` with `x` from `s` and `y` from `t`,
 * ordered by the interleave of the first row with the diagonal. */
export const pairs = <A>(s: Stream<A>, t: Stream<A>): Stream<[A, A]> => {
  if (s === null || t === null) {
    return null;
  }
  return consStream([s.head, t.head], () =>
    interleave(
      streamMap((x) => [s.head, x] as [A, A], streamCdr(t)),
      pairs(streamCdr(s), streamCdr(t)),
    ),
  );
};

/** The book's `merge` (exercise 3.56): combines two ordered streams
 * into one ordered result, eliminating repeats. */
export const merge = (s1: Stream<number>, s2: Stream<number>): Stream<number> => {
  if (s1 === null) {
    return s2;
  }
  if (s2 === null) {
    return s1;
  }
  if (s1.head < s2.head) {
    return consStream(s1.head, () => merge(streamCdr(s1), s2));
  }
  if (s1.head > s2.head) {
    return consStream(s2.head, () => merge(s1, streamCdr(s2)));
  }
  return consStream(s1.head, () => merge(streamCdr(s1), streamCdr(s2)));
};

// ---------------------------------------------------------------------
// 3.5.4 Streams and Delayed Evaluation
// ---------------------------------------------------------------------

/** The book's `integral` of 3.5.3: initial value plus dt times the
 * integrand, accumulated. The self-reference through the closure is
 * the feedback loop of figure 3.32; it works when the integrand
 * does not need the integral (the implicit-style definition). */
export const integral = (
  integrand: Stream<number>,
  initialValue: number,
  dt: number,
): Stream<number> => {
  const int: StreamCell<number> = consStream(initialValue, () =>
    addStreams(scaleStream(integrand, dt), int),
  );
  return int;
};

/** The book's `sign-change-detector`: 1 when the signal crossed from
 * negative `previous` to positive `current`, -1 for the crossing
 * down, 0 otherwise. */
export const signChangeDetector = (current: number, previous: number): number => {
  if (previous < 0 && current > 0) {
    return 1;
  }
  if (previous > 0 && current < 0) {
    return -1;
  }
  return 0;
};

/** The book's `make-zero-crossings`: the sense-data stream run through
 * `signChangeDetector`, each output paired with the value before it. */
export const makeZeroCrossings = (
  inputStream: Stream<number>,
  lastValue: number,
): Stream<number> => {
  if (inputStream === null) {
    return null;
  }
  return consStream(signChangeDetector(inputStream.head, lastValue), () =>
    makeZeroCrossings(streamCdr(inputStream), inputStream.head),
  );
};

/** The book's `integral` of 3.5.4: the integrand arrives delayed, so
 * feedback systems such as `solve` can be built even though the
 * integrand's first element needs the answer's first element. */
export const integralDelayed = (
  delayedIntegrand: () => Stream<number>,
  initialValue: number,
  dt: number,
): Stream<number> => {
  const int: StreamCell<number> = consStream(initialValue, () =>
    addStreams(scaleStream(delayedIntegrand(), dt), int),
  );
  return int;
};

/** The book's `solve`: the solution of dy/dt = f(y) with y(0) = y0,
 * integrated at step dt, built by delaying `dy` until `integral`
 * asks for it. */
export const solve = (f: (y: number) => number, y0: number, dt: number): Stream<number> => {
  const y: Stream<number> = integralDelayed(() => streamMap(f, y), y0, dt);
  return y;
};

// ---------------------------------------------------------------------
// 3.5.5 Modularity of Functional Programs and Modularity of Objects
// ---------------------------------------------------------------------

/** The book's `random-numbers`: the seeded generator's successive
 * words, defined by mapping `rand-update` over the stream itself, with
 * no assignment anywhere. The seed and generator come from the 3.1
 * module, so this section's pins match that section's. */
export const randomNumbers: StreamCell<number> = consStream(randomInit, () =>
  streamMap((x) => randUpdate(x), randomNumbers),
);

/** The book's `map-successive-pairs`: `f` over each consecutive pair
 * of `s`, consuming two elements per output. */
export const mapSuccessivePairs = <A, B>(f: (a1: A, a2: A) => B, s: Stream<A>): Stream<B> => {
  if (s === null) {
    return null;
  }
  const second = streamCdr(s);
  if (second === null) {
    return null;
  }
  const rest = streamCdr(second);
  if (rest === null) {
    return null;
  }
  return consStream(f(s.head, second.head), () => mapSuccessivePairs(f, streamCdr(rest)));
};

/** The book's `cesaro-stream`: the Cesaro experiment on consecutive
 * pairs of `randomNumbers`, true when the pair is coprime. */
export const cesaroStream: Stream<boolean> = mapSuccessivePairs(
  (r1, r2) => gcd(r1, r2) === 1,
  randomNumbers,
);

/** The book's `monte-carlo` of 3.5.5: a stream of running estimates of
 * the experiment's probability, one per trial, with no state but the
 * two counters carried forward. */
export const monteCarloStream = (
  experiments: Stream<boolean>,
  passed: number,
  failed: number,
): Stream<number> => {
  if (experiments === null) {
    return null;
  }
  const pass = experiments.head ? passed + 1 : passed;
  const fail = experiments.head ? failed : failed + 1;
  return consStream(pass / (pass + fail), () =>
    monteCarloStream(streamCdr(experiments), pass, fail),
  );
};

/** The book's `pi` of 3.5.5: the square-root-of-6-over-probability map
 * over the Cesaro Monte Carlo run, a stream of ever-better estimates. */
export const piEstimates: Stream<number> = streamMap(
  (p) => Math.sqrt(6 / p),
  monteCarloStream(cesaroStream, 0, 0),
);

/** The book's `stream-withdraw`: the balance history of a withdrawal
 * processor fed a stream of amounts, a mathematical function with the
 * behavior of the object of 3.1.3 and none of its state. */
export const streamWithdraw = (balance: number, amounts: Stream<number>): Stream<number> => {
  if (amounts === null) {
    return null;
  }
  return consStream(balance, () => streamWithdraw(balance - amounts.head, streamCdr(amounts)));
};
