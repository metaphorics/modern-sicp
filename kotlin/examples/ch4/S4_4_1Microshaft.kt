// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.4.1, deductive information retrieval: the book's
// Microshaft interactions pinned through the driver -- simple queries,
// compound queries, the prose rules (lives-near, same, wheel,
// outranked-by, append-to-form), and the query language's answer order
// over the chronological data base.

package sicp.ch4.examples

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.QueryOutcome
import sicp.ch4.QuerySystem
import sicp.ch4.printValue
import sicp.runtime.asSequence

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

/** A fresh query system loaded with the data base and the prose rules. */
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

public class S4_4_1MicroshaftTest :
    FunSpec({
        val system = microshaftSystem()

        test("the book's first query") {
            system.repl("(job ?x (computer programmer))") shouldBe
                """
                ;;; Query input:
                (job ?x (computer programmer))
                ;;; Query results:
                (job (Hacker Alyssa P) (computer programmer))
                (job (Fect Cy D) (computer programmer))
                """.trimIndent() + "\n"
        }

        test("every employee's address, in data-base order") {
            answersOf(system, "(address ?x ?y)") shouldBe
                listOf(
                    "(address (Bitdiddle Ben) (Slumerville (Ridge Road) 10))",
                    "(address (Hacker Alyssa P) (Cambridge (Mass Ave) 78))",
                    "(address (Fect Cy D) (Cambridge (Ames Street) 3))",
                    "(address (Tweakit Lem E) (Boston (Bay State Road) 22))",
                    "(address (Reasoner Louis) (Slumerville (Pine Tree Road) 80))",
                    "(address (Warbucks Oliver) (Swellesley (Top Heap Road)))",
                    "(address (Scrooge Eben) (Weston (Shady Lane) 10))",
                    "(address (Cratchet Robert) (Allston (N Harvard Street) 16))",
                    "(address (Aull DeWitt) (Slumerville (Onion Square) 5))",
                )
        }

        test("nobody supervises themselves") {
            answersOf(system, "(supervisor ?x ?x)") shouldBe emptyList()
        }

        test("two-element computer jobs") {
            answersOf(system, "(job ?x (computer ?type))") shouldBe
                listOf(
                    "(job (Bitdiddle Ben) (computer wizard))",
                    "(job (Hacker Alyssa P) (computer programmer))",
                    "(job (Fect Cy D) (computer programmer))",
                    "(job (Tweakit Lem E) (computer technician))",
                )
        }

        test("the dotted tail reaches the trainee") {
            answersOf(system, "(job ?x (computer . ?type))") shouldBe
                listOf(
                    "(job (Bitdiddle Ben) (computer wizard))",
                    "(job (Hacker Alyssa P) (computer programmer))",
                    "(job (Fect Cy D) (computer programmer))",
                    "(job (Tweakit Lem E) (computer technician))",
                    "(job (Reasoner Louis) (computer programmer trainee))",
                )
        }

        test("and processes in series") {
            answersOf(system, "(and (job ?person (computer programmer)) (address ?person ?where))") shouldBe
                listOf(
                    "(and (job (Hacker Alyssa P) (computer programmer)) (address (Hacker Alyssa P) (Cambridge (Mass Ave) 78)))",
                    "(and (job (Fect Cy D) (computer programmer)) (address (Fect Cy D) (Cambridge (Ames Street) 3)))",
                )
        }

        test("or interleaves the disjunct streams") {
            answersOf(system, "(or (supervisor ?x (Bitdiddle Ben)) (supervisor ?x (Hacker Alyssa P)))") shouldBe
                listOf(
                    "(or (supervisor (Hacker Alyssa P) (Bitdiddle Ben)) (supervisor (Hacker Alyssa P) (Hacker Alyssa P)))",
                    "(or (supervisor (Reasoner Louis) (Bitdiddle Ben)) (supervisor (Reasoner Louis) (Hacker Alyssa P)))",
                    "(or (supervisor (Fect Cy D) (Bitdiddle Ben)) (supervisor (Fect Cy D) (Hacker Alyssa P)))",
                    "(or (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (supervisor (Tweakit Lem E) (Hacker Alyssa P)))",
                )
        }

        test("not filters the frames the subquery satisfies") {
            answersOf(system, "(and (supervisor ?x ?y) (not (job ?x (computer programmer))))") shouldBe
                listOf(
                    "(and (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (not (job (Tweakit Lem E) (computer programmer))))",
                    "(and (supervisor (Reasoner Louis) (Hacker Alyssa P)) (not (job (Reasoner Louis) (computer programmer))))",
                    "(and (supervisor (Bitdiddle Ben) (Warbucks Oliver)) (not (job (Bitdiddle Ben) (computer programmer))))",
                    "(and (supervisor (Scrooge Eben) (Warbucks Oliver)) (not (job (Scrooge Eben) (computer programmer))))",
                    "(and (supervisor (Cratchet Robert) (Scrooge Eben)) (not (job (Cratchet Robert) (computer programmer))))",
                    "(and (supervisor (Aull DeWitt) (Warbucks Oliver)) (not (job (Aull DeWitt) (computer programmer))))",
                )
        }

        test("lisp-value filters over the underlying primitives") {
            answersOf(system, "(and (salary ?person ?amount) (lisp-value > ?amount 30000))") shouldBe
                listOf(
                    "(and (salary (Bitdiddle Ben) 60000) (lisp-value > 60000 30000))",
                    "(and (salary (Hacker Alyssa P) 40000) (lisp-value > 40000 30000))",
                    "(and (salary (Fect Cy D) 35000) (lisp-value > 35000 30000))",
                    "(and (salary (Warbucks Oliver) 150000) (lisp-value > 150000 30000))",
                    "(and (salary (Scrooge Eben) 75000) (lisp-value > 75000 30000))",
                )
        }

        test("the lives-near rule answers in data-base order") {
            answersOf(system, "(lives-near ?x (Bitdiddle Ben))") shouldBe
                listOf(
                    "(lives-near (Reasoner Louis) (Bitdiddle Ben))",
                    "(lives-near (Aull DeWitt) (Bitdiddle Ben))",
                )
        }

        test("a compound query over a rule") {
            answersOf(system, "(and (job ?x (computer programmer)) (lives-near ?x (Bitdiddle Ben)))") shouldBe
                emptyList()
        }

        test("append-to-form runs in both directions") {
            answersOf(system, "(append-to-form (a b) (c d) ?z)") shouldBe
                listOf("(append-to-form (a b) (c d) (a b c d))")
            answersOf(system, "(append-to-form (a b) ?y (a b c d))") shouldBe
                listOf("(append-to-form (a b) (c d) (a b c d))")
            answersOf(system, "(append-to-form ?x ?y (a b c d))") shouldBe
                listOf(
                    "(append-to-form () (a b c d) (a b c d))",
                    "(append-to-form (a) (b c d) (a b c d))",
                    "(append-to-form (a b) (c d) (a b c d))",
                    "(append-to-form (a b c) (d) (a b c d))",
                    "(append-to-form (a b c d) () (a b c d))",
                )
        }

        test("the wheel rule's fourfold listing") {
            answersOf(system, "(wheel ?who)") shouldBe
                listOf(
                    "(wheel (Bitdiddle Ben))",
                    "(wheel (Warbucks Oliver))",
                    "(wheel (Warbucks Oliver))",
                    "(wheel (Warbucks Oliver))",
                    "(wheel (Warbucks Oliver))",
                )
        }

        test("an assert! adds to the data base instead of answering") {
            microshaftSystem().run("(assert! (meeting whole-company (Wednesday 4pm)))") shouldBe
                Either.Right(QueryOutcome.Asserted as QueryOutcome)
        }
    })
