// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 3.1

import { Context, Effect, Layer, Ref, Schema } from "effect";

// ---------------------------------------------------------------------
// 3.1.1 Local State Variables
// ---------------------------------------------------------------------

// The book introduces assignment (`set!`) so a procedure can remember.
// This edition's declared stack is Effect (D33): a local state variable
// is an `Ref` captured by the procedure that owns it, `set!` becomes
// `Ref.set`/`Ref.update`, and the value of the book's `withdraw` call
// becomes the success value of an `Effect`. Where the book returns the
// message "Insufficient funds", the edition fails with a typed error in
// the effect's error channel.

/** The book's "Insufficient funds" answer, as a typed error. */
export class InsufficientFunds extends Schema.TaggedError<InsufficientFunds>()(
  "InsufficientFunds",
  Schema.Struct({}),
) {}

/** A withdrawal processor: the book's `(lambda (amount) ...)` over a
 * balance it can change. The effect answers the balance remaining, or
 * fails with `InsufficientFunds`. */
export type Withdrawal = Effect.Effect<number, InsufficientFunds>;

/** A withdrawal processor object: the book's `make-withdraw` value, a
 * procedure of one amount. */
export type WithdrawalProcessor = (amount: number) => Withdrawal;

/** The book's top-level `balance`, initialized to 100. It is a single
 * shared cell: exactly the global state the book warns about. */
export const balance = Ref.makeUnsafe(100);

/** The book's `withdraw` over the global `balance`. */
export const withdraw = (amount: number): Withdrawal =>
  Effect.gen(function* () {
    const current = yield* Ref.get(balance);
    if (current < amount) {
      return yield* new InsufficientFunds();
    }
    return yield* Ref.setAndGet(balance, current - amount);
  });

/** The book's `new-withdraw`: the balance is encapsulated, the `let` of
 * the book spelled as a factory whose captured `Ref` no other procedure
 * can reach. */
export const newWithdraw = (): WithdrawalProcessor => {
  const encapsulated = Ref.makeUnsafe(100);
  return (amount) =>
    Effect.gen(function* () {
      const current = yield* Ref.get(encapsulated);
      if (current < amount) {
        return yield* new InsufficientFunds();
      }
      return yield* Ref.setAndGet(encapsulated, current - amount);
    });
};

/** The book's `make-withdraw`: the formal parameter `balance` holds the
 * initial amount, and each call returns an independent withdrawal
 * processor over its own state. */
export const makeWithdraw = (initialBalance: number): WithdrawalProcessor => {
  const encapsulated = Ref.makeUnsafe(initialBalance);
  return (amount) =>
    Effect.gen(function* () {
      const current = yield* Ref.get(encapsulated);
      if (current < amount) {
        return yield* new InsufficientFunds();
      }
      return yield* Ref.setAndGet(encapsulated, current - amount);
    });
};

/** A request to a bank-account object: the book's message, as a member
 * of a closed union. The edition's dispatch is exhaustive, so the
 * book's "Unknown request: MAKE-ACCOUNT" arm is unrepresentable; the
 * compiler holds every caller to the two requests. */
export type AccountRequest =
  | { readonly _tag: "Withdraw"; readonly amount: number }
  | { readonly _tag: "Deposit"; readonly amount: number };

/** A bank-account object: the book's `dispatch`, a tagged dispatch over
 * one shared `Ref`. Withdraw and deposit answer the new balance. */
export type Account = (request: AccountRequest) => Effect.Effect<number, InsufficientFunds>;

/** The book's `make-account`: local `balance`, `withdraw` and `deposit`
 * over it, and the dispatch returned as the object. */
export const makeAccount = (initialBalance: number): Account => {
  const encapsulated = Ref.makeUnsafe(initialBalance);
  return (request) =>
    Effect.gen(function* () {
      if (request._tag === "Withdraw") {
        const current = yield* Ref.get(encapsulated);
        if (current < request.amount) {
          return yield* new InsufficientFunds();
        }
        return yield* Ref.setAndGet(encapsulated, current - request.amount);
      }
      return yield* Ref.updateAndGet(encapsulated, (b) => b + request.amount);
    });
};

// ---------------------------------------------------------------------
// 3.1.2 The Benefits of Introducing Assignment
// ---------------------------------------------------------------------

// The book's `rand-update` must only be a mathematical function whose
// successive values pass statistical tests (the footnote points at the
// (ax + b) mod m rule). This edition uses xorshift32 instead: any
// multiplier-odd LCG modulo 2^32 alternates the parity of its low bit,
// so *successive* states are never both even and the Cesaro fraction of
// 3.1.2 comes out visibly above 6/pi^2. Xorshift32 has no such bias,
// and the `>>>` steps keep the arithmetic on plain numbers.

/** Updates a pseudo-random state: xorshift32, `x` becomes the next
 * state of the 13/17/5 register shuffle. */
export const randUpdate = (x: number): number => {
  let v = x >>> 0;
  v ^= (v << 13) >>> 0;
  v ^= v >>> 17;
  v ^= (v << 5) >>> 0;
  return v >>> 0;
};

/** The book's `random-init`, the fixed seed the section starts from. */
export const randomInit = 42;

/** The book's `rand` as the `let`-bound lambda: one hidden state cell
 * behind a no-argument procedure that answers the next number. */
export const makeRand = (seed: number): Effect.Effect<number> => {
  const x = Ref.makeUnsafe(seed);
  return Ref.modify(x, (current) => {
    const next = randUpdate(current);
    return [next, next];
  });
};

/** The random-number generator as a service. The book defines `rand` at
 * top level and every procedure reads the same hidden cell; the Effect
 * rendering makes the dependency explicit instead, so the generator is
 * provided by a layer and everything that needs randomness demands it.
 * This is the section's `random-init` plus `rand`, relocated. */
export class Random extends Context.Service<
  Random,
  {
    readonly next: Effect.Effect<number>;
  }
>()("Random") {}

/** A layer providing `Random` over the section's `rand-update`, seeded
 * with `seed`, so the whole demonstration is reproducible. */
export const makeRandomLive = (seed: number): Layer.Layer<Random> =>
  Layer.effect(
    Random,
    Effect.gen(function* () {
      const x = yield* Ref.make(seed);
      return {
        next: Ref.modify(x, (current) => {
          const next = randUpdate(current);
          return [next, next];
        }),
      };
    }),
  );

/** The section's `gcd` (assumed from 1.2.1 in the book), by Euclid. */
export const gcd = (a: number, b: number): number => (b === 0 ? a : gcd(b, a % b));

/** The book's `cesaro-test`: two random numbers, true when their gcd is
 * 1. It needs the generator, and says so in its type. */
export const cesaroTest: Effect.Effect<boolean, never, Random> = Effect.gen(function* () {
  const random = yield* Random;
  const a = yield* random.next;
  const b = yield* random.next;
  return gcd(a, b) === 1;
});

/** The book's `monte-carlo`: run `experiment` for `trials` trials and
 * answer the fraction that passed. */
export const monteCarlo = <E, R>(
  trials: number,
  experiment: Effect.Effect<boolean, E, R>,
): Effect.Effect<number, E, R> => {
  const iter = (remaining: number, passed: number): Effect.Effect<number, E, R> =>
    remaining === 0
      ? Effect.succeed(passed / trials)
      : Effect.flatMap(experiment, (hit) => iter(remaining - 1, hit ? passed + 1 : passed));
  return iter(trials, 0);
};

/** The book's `estimate-pi`: the square root of 6 over the Cesaro
 * fraction. The generator requirement carries through. */
export const estimatePi = (trials: number): Effect.Effect<number, never, Random> =>
  Effect.map(monteCarlo(trials, cesaroTest), (fraction) => Math.sqrt(6 / fraction));

// The book's second version threads the random numbers explicitly, to
// show the modularity the assignment buys back. It stays pure here: the
// state is passed through the recursion instead of hidden in a cell.

/** The book's `random-gcd-test` without local state: each trial draws
 * two `rand-update` values and threads the newer one onward. The
 * book's named-let iteration spells as the edition's iterative `while`
 * loop over the same state (the ch1 convention for tail-recursive
 * processes). */
export const randomGcdTest = (trials: number, initialX: number): number => {
  let remaining = trials;
  let passed = 0;
  let x = initialX;
  while (remaining !== 0) {
    const x1 = randUpdate(x);
    const x2 = randUpdate(x1);
    if (gcd(x1, x2) === 1) {
      passed += 1;
    }
    remaining -= 1;
    x = x2;
  }
  return passed / trials;
};

/** The book's `estimate-pi` over `randomGcdTest`: the same estimate with
 * the generator's insides leaked into the caller. */
export const estimatePiWithoutState = (trials: number): number =>
  Math.sqrt(6 / randomGcdTest(trials, randomInit));

// ---------------------------------------------------------------------
// 3.1.3 The Costs of Introducing Assignment
// ---------------------------------------------------------------------

/** The book's `make-simplified-withdraw`: no insufficient-funds check,
 * the balance can go negative, and successive calls with the same
 * argument answer different values. */
export const makeSimplifiedWithdraw = (initialBalance: number) => {
  const encapsulated = Ref.makeUnsafe(initialBalance);
  return (amount: number): Effect.Effect<number> =>
    Ref.updateAndGet(encapsulated, (b) => b - amount);
};

/** The book's `make-decrementer`: no assignment, so the same call twice
 * answers the same value, every time. */
export const makeDecrementer = (balance: number) => {
  return (amount: number): Effect.Effect<number> => Effect.succeed(balance - amount);
};

/** The book's imperative `factorial`: the `product` and `counter` of the
 * iterative version become state cells. The `Effect.gen` body sequences
 * the two updates textually, so the book's order trap (counter before
 * product) is visible in the source, line by line. */
export const factorialImperative = (n: number): Effect.Effect<number> =>
  Effect.gen(function* () {
    const product = yield* Ref.make(1);
    const counter = yield* Ref.make(1);
    const iter = (): Effect.Effect<number> =>
      Effect.gen(function* () {
        const c = yield* Ref.get(counter);
        if (c > n) {
          return yield* Ref.get(product);
        }
        yield* Ref.update(product, (p) => c * p);
        yield* Ref.update(counter, (c) => c + 1);
        return yield* iter();
      });
    return yield* iter();
  });
