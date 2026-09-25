// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.45

package sicp.ch4.solutions

import sicp.ch4.AmbEvaluator

/** The five parses of "The professor lectures to the student in the
 * class with the cat.", in the search's order, then exhaustion. */
public fun ambiguousParses(): List<String> =
    answerLinesFaulted(
        ::AmbEvaluator,
        "$AMB_BASE_PRELUDE\n$PARSER_PROGRAM",
        "(parse '(the professor lectures to the student in the class with the cat))",
    )
