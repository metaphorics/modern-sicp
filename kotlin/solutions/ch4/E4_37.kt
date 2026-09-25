// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.37

package sicp.ch4.solutions

import sicp.ch4.AmbEvaluator
import sicp.runtime.Env
import sicp.runtime.Random

private val plain: (Env, Random?) -> AmbEvaluator = { env, _ -> AmbEvaluator(env) }

/** Ben's generator answers (3 4 5) first, like the book-order program.
 * => "(3 4 5)" */
public fun benFirstTriple(): String =
    firstAnswerLine(
        ::AmbEvaluator,
        "$AMB_BASE_PRELUDE\n$BEN_TRIPLE_PROGRAM",
        "(a-pythagorean-triple-ben 1 20)",
    )

/** The 4.35 program's backtracks to the first triple: 461. */
public fun bookOrderBacktracksToFirst(): Long =
    backtracksToFirst(plain, "$AMB_BASE_PRELUDE\n$TRIPLES_PROGRAM", "(a-pythagorean-triple-between 1 20)")

/** Ben's backtracks to the first triple: 42, about a ninth of the
 * book-order cost -- Ben is right about the search size. */
public fun benBacktracksToFirst(): Long =
    backtracksToFirst(plain, "$AMB_BASE_PRELUDE\n$BEN_TRIPLE_PROGRAM", "(a-pythagorean-triple-ben 1 20)")
