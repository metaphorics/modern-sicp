// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.25

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.25: `unless` under delayed arguments. The `unless`-based
 * `factorial` is an ordinary procedure whose arms are thunks in the lazy
 * evaluator, so the recursion bottoms out and answers; in an
 * applicative-order language the arms evaluate before `unless` is called,
 * which the armed call shows as an immediate fault and the recursion shows
 * as an unbounded descent (budgeted honestly rather than hung).
 *
 * Expected answers: the lazy `(factorial 5)` answers 120; the strict armed
 * the armed call `unless(true, 1 / 0, 42)` raises `DivisionByZero`; the
 * strict factorial under a 200-step budget fails with `BudgetExhausted
 * after 200 steps`.
 */
public fun lazyFactorialTranscript(): String = throw PendingSolution()

public fun strictArmedUnlessTranscript(): String = throw PendingSolution()

public fun strictFactorialTranscript(): String = throw PendingSolution()
