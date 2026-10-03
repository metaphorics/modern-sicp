// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.77: filters must wait for their bindings.

package sicp.ch4.solutions

import sicp.ch4.QAnd
import sicp.ch4.QGuard
import sicp.ch4.QNot
import sicp.ch4.QPattern
import sicp.ch4.QQuery
import sicp.ch4.QRule
import sicp.ch4.QueryDriver

// Exercise 4.77: filters wait for their bindings. A filter over unbound
// variables decides nothing: the negation's subquery succeeds on the open
// pattern and drops every frame, and the guard has no amount to compare and
// drops every frame. `QueryDriver.postponing` carries the filter on the
// frame as a promise instead: a `not` or guard that meets an unbound
// variable waits, and runs the moment the last of its variables is bound --
// as soon as possible, never earlier. The promise leaves a rule body with
// the frame, so a filter the caller's later conjunct binds still works.

// Exercise 4.77: the postponing driver keeps the frames the naive order drops.

private val notProgrammer: QQuery = QNot(QPattern(list(sym("job"), v("x"), list(sym("computer"), sym("programmer")))))
private val supervised: QQuery = QPattern(list(sym("supervisor"), v("x"), v("y")))
private val salaryOf: QQuery = QPattern(list(sym("salary"), v("who"), v("amount")))
private val over30000: QQuery = QGuard({ terms -> termLong(terms[0]) > termLong(terms[1]) }, listOf(v("amount"), sym("30000")))

/** The person is not a programmer, phrased as a rule whose whole body is the filter. */
private val nonProgrammer: QRule = QRule(list(sym("non-programmer"), v("x")), notProgrammer)

/** Naive orders that the stock driver fails and the postponing driver
 * answers, the orders that never needed the wait, a promise that leaves a
 * rule body, and a filter whose variable nothing binds. */
public fun delayedFilterDemos(): List<String> {
    val db = microshaftSystem()
    db.addRule(nonProgrammer)
    val stock = QueryDriver.streaming(db)
    val postponing = QueryDriver.postponing(db)
    val pair = listOf(v("x"), v("y"))
    val salary = listOf(v("who"), v("amount"))
    val notFirst = QAnd(listOf(notProgrammer, supervised))
    val guardFirst = QAnd(listOf(over30000, salaryOf))
    val viaRule = QAnd(listOf(QPattern(list(sym("non-programmer"), v("x"))), supervised))
    val noSuchJob = QNot(QPattern(list(sym("job"), v("ghost"), list(sym("no"), sym("such"), sym("job")))))
    val unbound = QAnd(listOf(supervised, noSuchJob))
    val unboundAgrees = answerLines(postponing, unbound, pair) == answerLines(stock, unbound, pair)
    val boundNot = answerLines(stock, QAnd(listOf(supervised, notProgrammer)), pair)
    val boundGuard = answerLines(stock, QAnd(listOf(salaryOf, over30000)), salary)
    val postponedNot = answerLines(postponing, notFirst, pair)
    val postponedGuard = answerLines(postponing, guardFirst, salary)
    return listOf(
        "stock not-first: ${answerLines(stock, notFirst, pair).size} line(s), the unbound filter drops everything",
        "stock lisp-value-first: ${answerLines(stock, guardFirst, salary).size} line(s), the unbound guard drops everything",
        "postponed not-first: ${postponedNot.size / pair.size} answer(s), the bound order's: ${postponedNot == boundNot}",
        "postponed lisp-value-first: ${postponedGuard.size / salary.size} answer(s), the bound order's: ${postponedGuard == boundGuard}",
        "stock rule filter, bound later by the caller: ${answerLines(stock, viaRule, pair).size} line(s)",
        "postponed rule filter, bound later by the caller: ${answerLines(postponing, viaRule, pair) == boundNot}",
        "postponed bound order: ${answerLines(postponing, QAnd(listOf(supervised, notProgrammer)), pair) == boundNot}",
        "a filter no conjunct binds runs at the end, as stock: $unboundAgrees",
    ) + postponedNot + postponedGuard
}
