// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.48

package sicp.ch4.solutions

import sicp.ch4.AmbEvaluator

/** "The quick brown cat sleeps" under the extended grammar. */
public fun adjectiveParse(): String =
    firstAnswerLine(
        ::AmbEvaluator,
        "$AMB_BASE_PRELUDE\n$PARSER_PROGRAM\n$ADJECTIVE_EXTENSION",
        "(parse-extended '(the quick brown cat sleeps))",
    )

/** "The cat sleeps" parses with the empty modifier choice. */
public fun noAdjectiveParse(): String =
    firstAnswerLine(
        ::AmbEvaluator,
        "$AMB_BASE_PRELUDE\n$PARSER_PROGRAM\n$ADJECTIVE_EXTENSION",
        "(parse-extended '(the cat sleeps))",
    )
