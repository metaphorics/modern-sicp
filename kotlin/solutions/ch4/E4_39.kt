// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.39

package sicp.ch4.solutions

import sicp.ch4.AmbEvaluator
import sicp.runtime.Env
import sicp.runtime.Random

private val plain: (Env, Random?) -> AmbEvaluator = { env, _ -> AmbEvaluator(env) }

/** Both orders answer the same assignment. */
public fun dwellingAnswer(): String = firstAnswerLine(plain, "$AMB_BASE_PRELUDE\n$DWELLING_PROGRAM", "(multiple-dwelling)")

/** The book order's backtracks to the first answer: 1470. */
public fun bookOrderBacktracks(): Long = backtracksToFirst(plain, "$AMB_BASE_PRELUDE\n$DWELLING_PROGRAM", "(multiple-dwelling)")

/** The Fletcher-first reorder's backtracks to the first answer: also
 * 1470 -- every restriction runs after all five choices, so the reorder
 * changes which requirement rejects an assignment, not where the search
 * backtracks to. */
public fun reorderedBacktracks(): Long =
    backtracksToFirst(plain, "$AMB_BASE_PRELUDE\n$DWELLING_REORDERED_PROGRAM", "(multiple-dwelling-reordered)")
