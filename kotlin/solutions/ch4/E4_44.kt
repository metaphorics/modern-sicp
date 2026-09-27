// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.44

package sicp.ch4.solutions

import sicp.ch4.AmbEvaluator
import sicp.runtime.Env
import sicp.runtime.Random

private val plain: (Env, Random?) -> AmbEvaluator = { env, _ -> AmbEvaluator(env) }

private const val QUEENS_QUERY_FORMAT = "(queens %d)"

/** The first solution for [boardSize], newest column first. */
public fun queensFirst(boardSize: Int): String =
    firstAnswerLine(
        plain,
        "$AMB_BASE_PRELUDE\n$QUEENS_PROGRAM",
        QUEENS_QUERY_FORMAT.format(boardSize),
    )

/** Backtracks to the first solution of [boardSize]. */
public fun queensBacktracks(boardSize: Int): Long =
    backtracksToFirst(
        plain,
        "$AMB_BASE_PRELUDE\n$QUEENS_PROGRAM",
        QUEENS_QUERY_FORMAT.format(boardSize),
    )
