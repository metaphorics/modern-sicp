// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.29

package sicp.ch4.solutions

import kotlinx.collections.immutable.PersistentList
import sicp.ch4.LazyEvaluator
import sicp.ch4.lazyTranscriptOn
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.VThunkNoMemo
import sicp.runtime.Value

// Exercise 4.29: what memoization buys. The slow-without-memoization
// program is the counting session itself: `(* x x)` and `(* x x x)` are
// strict primitive applications, so each use of the parameter is a demand,
// and an unmemoized thunk re-runs `(id 10)` -- counter, side effect and
// all -- at every demand. Memoized, `(square (id 10))` forces once
// (count 1) and the second use reads the stored value; `(cube (id 10))`
// adds exactly one more computation (count 2). Unmemoized, every demand
// recomputes: counts 2 and then 5. Any program whose delayed arguments are
// used more than they are forced -- a nested numerical fold is the
// textbook shape -- pays multiplicatively without the memo.

/** The counting session's definitions. */
private val COUNTING_PROGRAM: String =
    """
    (define count 0)
    (define (id x) (set! count (+ count 1)) x)
    (define (square x) (* x x))
    (define (cube x) (* x x x))
    (square (id 10))
    count
    (cube (id 10))
    count
    """.trimIndent()

/** Memoized thunks: `(square (id 10))` then `(cube (id 10))` count 1 and
 * 2. => "100\n1\n1000\n2\n" */
public fun memoizedCountsTranscript(): String = lazyTranscriptOn(::LazyEvaluator, COUNTING_PROGRAM)

/** The unmemoized probe evaluator: every delayed operand is a
 * [VThunkNoMemo], which re-runs its expression at each demand. */
public class NoMemoLazy(
    global: Env,
) : LazyEvaluator(global) {
    override fun delayOperands(
        operands: PersistentList<Expr>,
        env: Env,
    ): List<Value> = operands.map { VThunkNoMemo(it, env) }
}

/** The same session under unmemoized delay: the counts read 2 and 5.
 * => "100\n2\n1000\n5\n" */
public fun unmemoizedCountsTranscript(): String = lazyTranscriptOn(::NoMemoLazy, COUNTING_PROGRAM)
