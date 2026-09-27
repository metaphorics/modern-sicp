// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.47

package sicp.ch4.solutions

import arrow.core.raise.either
import sicp.ch4.AmbEvaluator
import sicp.ch4.printValue

private val louisPrelude: String = "$AMB_BASE_PRELUDE\n$PARSER_PROGRAM\n$LOUIS_VERB_PHRASE"

/** Louis's version delivers the text's first parse for "The cat eats".
 * The generous budget never fires for the first parse (two choices); it
 * exists so the divergent variants of this exercise stay bounded. */
public fun louisFirstParse(): String =
    try {
        either {
            val driver = newDriver({ env, _ -> BudgetAmb(env, 5000) }, louisPrelude)
            val answer = driver.solve("(parse '(the cat eats))")
            printValue(answer ?: throw AssertionError("no parse"))
        }.fold(
            { e -> "scheme error: $e" },
            { it },
        )
    } catch (budget: BudgetExhausted) {
        "BUDGET FIRED: ${budget.message}"
    }

/** Its try-again diverges once the input is spent: the second
 * alternative recurses before anything is consumed. => "choice budget
 * exhausted after 500 choices" */
public fun louisTryAgainFault(): String =
    try {
        either {
            val driver = newDriver({ env, _ -> BudgetAmb(env, 500) }, louisPrelude)
            val answers = mutableListOf<String>()
            var next = driver.solve("(parse '(the cat eats))")
            while (next != null && answers.size < 8) {
                answers.add(printValue(next))
                next = driver.tryAgain()
            }
            "no fault in ${answers.size} answers"
        }.fold(
            { e -> e.toString() },
            { it },
        )
    } catch (budget: BudgetExhausted) {
        budget.message ?: "budget exhausted"
    }

/** The interchanged order diverges on the first parse: the recursion
 * runs before any word is consumed. => "choice budget exhausted after
 * 300 choices" */
public fun interchangedFault(): String =
    budgetedFault(
        "$AMB_BASE_PRELUDE\n$PARSER_PROGRAM\n$LOUIS_INTERCHANGED",
        "(parse '(the cat eats))",
        cap = 300,
    )
