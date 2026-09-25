// SPDX-License-Identifier: GPL-3.0-only
// Section 4.4 fixtures for the exercise solutions: the Microshaft data
// base and the section's prose rules, plus the answer helpers the demos
// share.

package sicp.ch4.solutions

import arrow.core.Either
import sicp.ch4.QueryOutcome
import sicp.ch4.QuerySystem
import sicp.ch4.printValue
import sicp.runtime.asSequence
import sicp.runtime.take

/** The Microshaft data base of 4.4.1, in the book's order. */
public val microshaftDatabase: String =
    """
    (assert! (address (Bitdiddle Ben) (Slumerville (Ridge Road) 10)))
    (assert! (job (Bitdiddle Ben) (computer wizard)))
    (assert! (salary (Bitdiddle Ben) 60000))
    (assert! (address (Hacker Alyssa P) (Cambridge (Mass Ave) 78)))
    (assert! (job (Hacker Alyssa P) (computer programmer)))
    (assert! (salary (Hacker Alyssa P) 40000))
    (assert! (supervisor (Hacker Alyssa P) (Bitdiddle Ben)))
    (assert! (address (Fect Cy D) (Cambridge (Ames Street) 3)))
    (assert! (job (Fect Cy D) (computer programmer)))
    (assert! (salary (Fect Cy D) 35000))
    (assert! (supervisor (Fect Cy D) (Bitdiddle Ben)))
    (assert! (address (Tweakit Lem E) (Boston (Bay State Road) 22)))
    (assert! (job (Tweakit Lem E) (computer technician)))
    (assert! (salary (Tweakit Lem E) 25000))
    (assert! (supervisor (Tweakit Lem E) (Bitdiddle Ben)))
    (assert! (address (Reasoner Louis) (Slumerville (Pine Tree Road) 80)))
    (assert! (job (Reasoner Louis) (computer programmer trainee)))
    (assert! (salary (Reasoner Louis) 30000))
    (assert! (supervisor (Reasoner Louis) (Hacker Alyssa P)))
    (assert! (supervisor (Bitdiddle Ben) (Warbucks Oliver)))
    (assert! (address (Warbucks Oliver) (Swellesley (Top Heap Road))))
    (assert! (job (Warbucks Oliver) (administration big wheel)))
    (assert! (salary (Warbucks Oliver) 150000))
    (assert! (address (Scrooge Eben) (Weston (Shady Lane) 10)))
    (assert! (job (Scrooge Eben) (accounting chief accountant)))
    (assert! (salary (Scrooge Eben) 75000))
    (assert! (supervisor (Scrooge Eben) (Warbucks Oliver)))
    (assert! (address (Cratchet Robert) (Allston (N Harvard Street) 16)))
    (assert! (job (Cratchet Robert) (accounting scrivener)))
    (assert! (salary (Cratchet Robert) 18000))
    (assert! (supervisor (Cratchet Robert) (Scrooge Eben)))
    (assert! (address (Aull DeWitt) (Slumerville (Onion Square) 5)))
    (assert! (job (Aull DeWitt) (administration secretary)))
    (assert! (salary (Aull DeWitt) 25000))
    (assert! (supervisor (Aull DeWitt) (Warbucks Oliver)))
    (assert! (can-do-job (computer wizard) (computer programmer)))
    (assert! (can-do-job (computer wizard) (computer technician)))
    (assert! (can-do-job (computer programmer) (computer programmer trainee)))
    (assert! (can-do-job (administration secretary) (administration big wheel)))
    """.trimIndent()

/** The section's prose rules, in the book's order. */
public val proseRules: String =
    """
    (assert! (rule (lives-near ?person-1 ?person-2)
                   (and (address ?person-1 (?town . ?rest-1))
                        (address ?person-2 (?town . ?rest-2))
                        (not (same ?person-1 ?person-2)))))
    (assert! (rule (same ?x ?x)))
    (assert! (rule (wheel ?person)
                   (and (supervisor ?middle-manager ?person)
                        (supervisor ?x ?middle-manager))))
    (assert! (rule (outranked-by ?staff-person ?boss)
                   (or (supervisor ?staff-person ?boss)
                       (and (supervisor ?staff-person ?middle-manager)
                            (outranked-by ?middle-manager ?boss)))))
    (assert! (rule (append-to-form () ?y ?y)))
    (assert! (rule (append-to-form (?u . ?v) ?y (?u . ?z))
                   (append-to-form ?v ?y ?z)))
    """.trimIndent()

/** A fresh query system with the data base and the prose rules loaded. */
public fun microshaftSystem(): QuerySystem {
    val system = QuerySystem()
    system.load(microshaftDatabase)
    system.load(proseRules)
    return system
}

/** The answers of one query as the driver prints them, forced. */
public fun answersOf(
    system: QuerySystem,
    query: String,
): List<String> =
    when (val outcome = system.run(query)) {
        is Either.Left -> {
            throw AssertionError("query fault: ${outcome.value}")
        }

        is Either.Right -> {
            when (val o = outcome.value) {
                is QueryOutcome.Answered -> {
                    o.answers
                        .asSequence()
                        .map { printValue(it) }
                        .toList()
                }

                is QueryOutcome.Asserted -> {
                    emptyList()
                }
            }
        }
    }

/** The first `n` printed answers of one query. */
public fun answersUpto(
    system: QuerySystem,
    query: String,
    n: Int,
): List<String> =
    when (val outcome = system.run(query)) {
        is Either.Left -> {
            throw AssertionError("query fault: ${outcome.value}")
        }

        is Either.Right -> {
            when (val o = outcome.value) {
                is QueryOutcome.Answered -> o.answers.take(n).map { printValue(it) }
                is QueryOutcome.Asserted -> emptyList()
            }
        }
    }
