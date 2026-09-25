// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.49

package sicp.ch4.solutions

import arrow.core.raise.either
import sicp.ch4.AmbEvaluator

/** The first six generated sentences: the footnote's boring descent
 * through the grammar's first alternatives, left to right. */
public fun generatedSentences(): List<String> =
    either {
        val driver = newDriver({ env, _ -> BudgetAmb(env, 500) }, "$AMB_BASE_PRELUDE\n$PARSER_PROGRAM\n$GENERATOR_PROGRAM")
        answerLines(driver, "(parse '(any input at all))", limit = 6)
    }.fold(
        { e -> throw AssertionError(e.toString()) },
        { it },
    )
