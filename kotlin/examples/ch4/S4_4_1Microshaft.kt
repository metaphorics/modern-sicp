// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.4.1, deductive information retrieval: the book's
// Microshaft interactions over the typed query data base -- simple
// queries, compound queries, the prose rules (lives-near, same, wheel,
// outranked-by, append-to-form), and the answer order over the
// chronological data base.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.QAnd
import sicp.ch4.QFact
import sicp.ch4.QGuard
import sicp.ch4.QList
import sicp.ch4.QNot
import sicp.ch4.QOr
import sicp.ch4.QRule
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver

private fun town(
    name: String,
    street: List<String>,
    number: String? = null,
): QList {
    val streetTerm = person(*street.toTypedArray())
    return if (number == null) terms(sym(name), streetTerm) else terms(sym(name), streetTerm, sym(number))
}

private fun job(
    person: QList,
    role: String,
    title: String,
    secondTitle: String? = null,
): QFact =
    if (secondTitle == null) {
        fact(terms(sym("job"), person, terms(sym(role), sym(title))))
    } else {
        fact(terms(sym("job"), person, terms(sym(role), sym(title), sym(secondTitle))))
    }

/** The Microshaft data base of 4.4.1: 39 assertions in the book's order. */
public val microshaftFacts: List<QFact> =
    listOf(
        fact(terms(sym("address"), person("Bitdiddle", "Ben"), town("Slumerville", listOf("Ridge", "Road"), "10"))),
        job(person("Bitdiddle", "Ben"), "computer", "wizard"),
        fact(terms(sym("salary"), person("Bitdiddle", "Ben"), sym("60000"))),
        fact(terms(sym("address"), person("Hacker", "Alyssa", "P"), town("Cambridge", listOf("Mass", "Ave"), "78"))),
        job(person("Hacker", "Alyssa", "P"), "computer", "programmer"),
        fact(terms(sym("salary"), person("Hacker", "Alyssa", "P"), sym("40000"))),
        fact(terms(sym("supervisor"), person("Hacker", "Alyssa", "P"), person("Bitdiddle", "Ben"))),
        fact(terms(sym("address"), person("Fect", "Cy", "D"), town("Cambridge", listOf("Ames", "Street"), "3"))),
        job(person("Fect", "Cy", "D"), "computer", "programmer"),
        fact(terms(sym("salary"), person("Fect", "Cy", "D"), sym("35000"))),
        fact(terms(sym("supervisor"), person("Fect", "Cy", "D"), person("Bitdiddle", "Ben"))),
        fact(terms(sym("address"), person("Tweakit", "Lem", "E"), town("Boston", listOf("Bay", "State", "Road"), "22"))),
        job(person("Tweakit", "Lem", "E"), "computer", "technician"),
        fact(terms(sym("salary"), person("Tweakit", "Lem", "E"), sym("25000"))),
        fact(terms(sym("supervisor"), person("Tweakit", "Lem", "E"), person("Bitdiddle", "Ben"))),
        fact(terms(sym("address"), person("Reasoner", "Louis"), town("Slumerville", listOf("Pine", "Tree", "Road"), "80"))),
        job(person("Reasoner", "Louis"), "computer", "programmer", "trainee"),
        fact(terms(sym("salary"), person("Reasoner", "Louis"), sym("30000"))),
        fact(terms(sym("supervisor"), person("Reasoner", "Louis"), person("Hacker", "Alyssa", "P"))),
        fact(terms(sym("supervisor"), person("Bitdiddle", "Ben"), person("Warbucks", "Oliver"))),
        fact(terms(sym("address"), person("Warbucks", "Oliver"), town("Swellesley", listOf("Top", "Heap", "Road")))),
        job(person("Warbucks", "Oliver"), "administration", "big", "wheel"),
        fact(terms(sym("salary"), person("Warbucks", "Oliver"), sym("150000"))),
        fact(terms(sym("address"), person("Scrooge", "Eben"), town("Weston", listOf("Shady", "Lane"), "10"))),
        job(person("Scrooge", "Eben"), "accounting", "chief", "accountant"),
        fact(terms(sym("salary"), person("Scrooge", "Eben"), sym("75000"))),
        fact(terms(sym("supervisor"), person("Scrooge", "Eben"), person("Warbucks", "Oliver"))),
        fact(terms(sym("address"), person("Cratchet", "Robert"), town("Allston", listOf("N", "Harvard", "Street"), "16"))),
        job(person("Cratchet", "Robert"), "accounting", "scrivener"),
        fact(terms(sym("salary"), person("Cratchet", "Robert"), sym("18000"))),
        fact(terms(sym("supervisor"), person("Cratchet", "Robert"), person("Scrooge", "Eben"))),
        fact(terms(sym("address"), person("Aull", "DeWitt"), town("Slumerville", listOf("Onion", "Square"), "5"))),
        job(person("Aull", "DeWitt"), "administration", "secretary"),
        fact(terms(sym("salary"), person("Aull", "DeWitt"), sym("25000"))),
        fact(terms(sym("supervisor"), person("Aull", "DeWitt"), person("Warbucks", "Oliver"))),
        fact(terms(sym("can-do-job"), terms(sym("computer"), sym("wizard")), terms(sym("computer"), sym("programmer")))),
        fact(terms(sym("can-do-job"), terms(sym("computer"), sym("wizard")), terms(sym("computer"), sym("technician")))),
        fact(
            terms(
                sym("can-do-job"),
                terms(sym("computer"), sym("programmer")),
                terms(sym("computer"), sym("programmer"), sym("trainee")),
            ),
        ),
        fact(
            terms(
                sym("can-do-job"),
                terms(sym("administration"), sym("secretary")),
                terms(sym("administration"), sym("big"), sym("wheel")),
            ),
        ),
    )

/** The section's prose rules, in the book's order. */
public val proseRules: List<QRule> =
    listOf(
        rule(
            terms(sym("lives-near"), variable("person-1"), variable("person-2")),
            QAnd(
                listOf(
                    pattern(terms(sym("address"), variable("person-1"), dotted(listOf(variable("town")), variable("rest-1")))),
                    pattern(terms(sym("address"), variable("person-2"), dotted(listOf(variable("town")), variable("rest-2")))),
                    QNot(pattern(terms(sym("same"), variable("person-1"), variable("person-2")))),
                ),
            ),
        ),
        rule(terms(sym("same"), variable("x"), variable("x")), QAnd(emptyList())),
        rule(
            terms(sym("wheel"), variable("person")),
            QAnd(
                listOf(
                    pattern(terms(sym("supervisor"), variable("middle-manager"), variable("person"))),
                    pattern(terms(sym("supervisor"), variable("x"), variable("middle-manager"))),
                ),
            ),
        ),
        rule(
            terms(sym("outranked-by"), variable("staff-person"), variable("boss")),
            QOr(
                listOf(
                    pattern(terms(sym("supervisor"), variable("staff-person"), variable("boss"))),
                    QAnd(
                        listOf(
                            pattern(terms(sym("supervisor"), variable("staff-person"), variable("middle-manager"))),
                            pattern(terms(sym("outranked-by"), variable("middle-manager"), variable("boss"))),
                        ),
                    ),
                ),
            ),
        ),
        rule(terms(sym("append-to-form"), terms(), variable("y"), variable("y")), QAnd(emptyList())),
        rule(
            terms(
                sym("append-to-form"),
                dotted(listOf(variable("u")), variable("v")),
                variable("y"),
                dotted(listOf(variable("u")), variable("z")),
            ),
            pattern(terms(sym("append-to-form"), variable("v"), variable("y"), variable("z"))),
        ),
    )

/** A fresh query system loaded with the data base and the prose rules. */
public fun microshaftDatabase(): QueryDatabase {
    val database = QueryDatabase()
    for (entry in microshaftFacts) {
        database.assertFact(entry)
    }
    for (entry in proseRules) {
        database.addRule(entry)
    }
    return database
}

public class S4_4_1MicroshaftTest :
    FunSpec({
        val x = variable("x")
        val y = variable("y")
        val z = variable("z")

        test("the book's first query") {
            val driver = QueryDriver.streaming(microshaftDatabase())
            answerLines(driver, pattern(terms(sym("job"), x, terms(sym("computer"), sym("programmer")))), listOf(x)) shouldBe
                listOf(
                    "?x = [Hacker, Alyssa, P]",
                    "?x = [Fect, Cy, D]",
                )
        }

        test("every employee's address, in data-base order") {
            val driver = QueryDriver.streaming(microshaftDatabase())
            answerLines(driver, pattern(terms(sym("address"), x, y)), listOf(y)) shouldBe
                listOf(
                    "?y = [Slumerville, [Ridge, Road], 10]",
                    "?y = [Cambridge, [Mass, Ave], 78]",
                    "?y = [Cambridge, [Ames, Street], 3]",
                    "?y = [Boston, [Bay, State, Road], 22]",
                    "?y = [Slumerville, [Pine, Tree, Road], 80]",
                    "?y = [Swellesley, [Top, Heap, Road]]",
                    "?y = [Weston, [Shady, Lane], 10]",
                    "?y = [Allston, [N, Harvard, Street], 16]",
                    "?y = [Slumerville, [Onion, Square], 5]",
                )
        }

        test("nobody supervises themselves") {
            val driver = QueryDriver.streaming(microshaftDatabase())
            answerLines(driver, pattern(terms(sym("supervisor"), x, x)), listOf(x)) shouldBe emptyList()
        }

        test("two-element computer jobs") {
            val driver = QueryDriver.streaming(microshaftDatabase())
            val type = variable("type")
            answerLines(driver, pattern(terms(sym("job"), x, terms(sym("computer"), type))), listOf(type)) shouldBe
                listOf(
                    "?type = wizard",
                    "?type = programmer",
                    "?type = programmer",
                    "?type = technician",
                )
        }

        test("the dotted tail reaches the trainee") {
            val driver = QueryDriver.streaming(microshaftDatabase())
            val type = variable("type")
            answerLines(driver, pattern(terms(sym("job"), x, dotted(listOf(sym("computer")), type))), listOf(x)) shouldBe
                listOf(
                    "?x = [Bitdiddle, Ben]",
                    "?x = [Hacker, Alyssa, P]",
                    "?x = [Fect, Cy, D]",
                    "?x = [Tweakit, Lem, E]",
                    "?x = [Reasoner, Louis]",
                )
        }

        test("and processes in series") {
            val driver = QueryDriver.streaming(microshaftDatabase())
            val person = variable("person")
            val where = variable("where")
            val query =
                QAnd(
                    listOf(
                        pattern(terms(sym("job"), person, terms(sym("computer"), sym("programmer")))),
                        pattern(terms(sym("address"), person, where)),
                    ),
                )
            answerLines(driver, query, listOf(person)) shouldBe
                listOf(
                    "?person = [Hacker, Alyssa, P]",
                    "?person = [Fect, Cy, D]",
                )
        }

        test("or interleaves the disjunct streams") {
            val driver = QueryDriver.streaming(microshaftDatabase())
            val query =
                QOr(
                    listOf(
                        pattern(terms(sym("supervisor"), x, person("Bitdiddle", "Ben"))),
                        pattern(terms(sym("supervisor"), x, person("Hacker", "Alyssa", "P"))),
                    ),
                )
            answerLines(driver, query, listOf(x)) shouldBe
                listOf(
                    "?x = [Hacker, Alyssa, P]",
                    "?x = [Reasoner, Louis]",
                    "?x = [Fect, Cy, D]",
                    "?x = [Tweakit, Lem, E]",
                )
        }

        test("not filters the frames the subquery satisfies") {
            val driver = QueryDriver.streaming(microshaftDatabase())
            val supervisor = variable("y")
            val query =
                QAnd(
                    listOf(
                        pattern(terms(sym("supervisor"), x, supervisor)),
                        QNot(pattern(terms(sym("job"), x, terms(sym("computer"), sym("programmer"))))),
                    ),
                )
            answerLines(driver, query, listOf(x)) shouldBe
                listOf(
                    "?x = [Tweakit, Lem, E]",
                    "?x = [Reasoner, Louis]",
                    "?x = [Bitdiddle, Ben]",
                    "?x = [Scrooge, Eben]",
                    "?x = [Cratchet, Robert]",
                    "?x = [Aull, DeWitt]",
                )
        }

        test("a guard filters over the underlying comparison") {
            val driver = QueryDriver.streaming(microshaftDatabase())
            val person = variable("person")
            val amount = variable("amount")
            val query =
                QAnd(
                    listOf(
                        pattern(terms(sym("salary"), person, amount)),
                        QGuard(
                            { bound -> (bound.get(0) as sicp.ch4.QSym).name.toLong() > 30000L },
                            listOf(amount),
                        ),
                    ),
                )
            answerLines(driver, query, listOf(person)) shouldBe
                listOf(
                    "?person = [Bitdiddle, Ben]",
                    "?person = [Hacker, Alyssa, P]",
                    "?person = [Fect, Cy, D]",
                    "?person = [Warbucks, Oliver]",
                    "?person = [Scrooge, Eben]",
                )
        }

        test("the lives-near rule answers in data-base order") {
            val driver = QueryDriver.streaming(microshaftDatabase())
            answerLines(driver, pattern(terms(sym("lives-near"), x, person("Bitdiddle", "Ben"))), listOf(x)) shouldBe
                listOf(
                    "?x = [Reasoner, Louis]",
                    "?x = [Aull, DeWitt]",
                )
        }

        test("a compound query over a rule") {
            val driver = QueryDriver.streaming(microshaftDatabase())
            val query =
                QAnd(
                    listOf(
                        pattern(terms(sym("job"), x, terms(sym("computer"), sym("programmer")))),
                        pattern(terms(sym("lives-near"), x, person("Bitdiddle", "Ben"))),
                    ),
                )
            answerLines(driver, query, listOf(x)) shouldBe emptyList()
        }

        test("append-to-form runs in both directions") {
            val driver = QueryDriver.streaming(microshaftDatabase())
            val abc = terms(sym("a"), sym("b"))
            val cd = terms(sym("c"), sym("d"))
            val abcd = terms(sym("a"), sym("b"), sym("c"), sym("d"))
            answerLines(driver, pattern(terms(sym("append-to-form"), abc, cd, z)), listOf(z)) shouldBe
                listOf("?z = [a, b, c, d]")
            answerLines(driver, pattern(terms(sym("append-to-form"), abc, y, abcd)), listOf(y)) shouldBe
                listOf("?y = [c, d]")
            answerLines(driver, pattern(terms(sym("append-to-form"), x, y, abcd)), listOf(x)) shouldBe
                listOf(
                    "?x = []",
                    "?x = [a]",
                    "?x = [a, b]",
                    "?x = [a, b, c]",
                    "?x = [a, b, c, d]",
                )
        }

        test("the wheel rule's fourfold listing") {
            val driver = QueryDriver.streaming(microshaftDatabase())
            answerLines(driver, pattern(terms(sym("wheel"), x)), listOf(x)) shouldBe
                listOf(
                    "?x = [Bitdiddle, Ben]",
                    "?x = [Warbucks, Oliver]",
                    "?x = [Warbucks, Oliver]",
                    "?x = [Warbucks, Oliver]",
                    "?x = [Warbucks, Oliver]",
                )
        }

        test("the deduplicating answer view collapses the fourfold listing") {
            val driver = QueryDriver.deduplicating(microshaftDatabase())
            answerLines(driver, pattern(terms(sym("wheel"), x)), listOf(x)) shouldBe
                listOf(
                    "?x = [Bitdiddle, Ben]",
                    "?x = [Warbucks, Oliver]",
                )
        }

        test("an assertion is filed instead of answered") {
            val database = microshaftDatabase()
            database.assertFact(fact(terms(sym("meeting"), sym("whole-company"), terms(sym("Wednesday"), sym("4pm")))))
            val driver = QueryDriver.streaming(database)
            val who = variable("who")
            answerLines(driver, pattern(terms(sym("meeting"), who, terms(sym("Wednesday"), sym("4pm")))), listOf(who)) shouldBe
                listOf("?who = whole-company")
        }
    })
