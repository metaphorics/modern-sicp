// SPDX-License-Identifier: GPL-3.0-only
package sicp.ch5

import arrow.core.Either
import arrow.core.getOrElse
import arrow.core.raise.either
import sicp.ch4.Direct
import sicp.ch4.QAnd
import sicp.ch4.QFact
import sicp.ch4.QGuard
import sicp.ch4.QList
import sicp.ch4.QNot
import sicp.ch4.QOr
import sicp.ch4.QPattern
import sicp.ch4.QQuery
import sicp.ch4.QRule
import sicp.ch4.QSym
import sicp.ch4.QTerm
import sicp.ch4.QUnique
import sicp.ch4.QVar
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver
import sicp.ch4.renderAnswer
import sicp.guest.Admission
import sicp.guest.AdmissionError
import sicp.guest.GValue
import sicp.guest.GuestError
import sicp.guest.Mode
import sicp.guest.NO_POSITION
import sicp.guest.Primitives

/**
 * The query engine of the conformance driver. A query case is domain data
 * (grammar 4.3): the checked fixture declares its facts, rules, query, and
 * variables through the [QUERY_ENTRIES] functions, and this engine loads that
 * data into a [QueryDatabase] and answers it with the teaching
 * [QueryDriver] -- the section 4.4 evaluator itself, not a guest program
 * that re-implements one.
 */
internal object QueryCase {
    /** Raised inside a `QGuard` predicate when the guest function faults. */
    private class GuardFault : RuntimeException(null, null, false, false)

    /** Raised when case data does not match the declared query entry shapes. */
    private class CaseFault : RuntimeException(null, null, false, false)

    fun observe(text: String): Either<AdmissionError, ReferenceModels.Observation> =
        either {
            val checked = Admission.admitOrRaise(text, Mode.QUERY)
            Direct.values(checked, QUERY_ENTRIES).fold({ ReferenceModels.Observation("error", "") }, ::answer)
        }

    private fun answer(values: List<GValue>): ReferenceModels.Observation {
        try {
            val (facts, rules, query, variables) = values
            val database = QueryDatabase()
            for (fact in items(facts)) database.assertFact(QFact(hostTerm(field(fact, "term"))))
            for (rule in items(rules)) {
                database.addRule(QRule(hostTerm(field(rule, "conclusion")), hostQuery(field(rule, "body"))))
            }
            val reported = items(variables).map { hostTerm(it) as? QVar ?: throw CaseFault() }
            val lines =
                QueryDriver
                    .streaming(database)
                    .run(hostQuery(query), reported)
                    .flatMap { renderAnswer(it, reported) }
                    .toList()
            return ReferenceModels.Observation("value", if (lines.isEmpty()) "" else lines.joinToString("\n") + "\n")
        } catch (_: GuardFault) {
            return ReferenceModels.Observation("error", "")
        } catch (_: CaseFault) {
            return ReferenceModels.Observation("error", "")
        } catch (_: IndexOutOfBoundsException) {
            return ReferenceModels.Observation("error", "")
        }
    }

    private fun className(value: GValue): String = (value as? GValue.VObject)?.className ?: throw CaseFault()

    private fun field(
        value: GValue,
        name: String,
    ): GValue = (value as? GValue.VObject)?.fields?.get(name) ?: throw CaseFault()

    private fun items(value: GValue): List<GValue> = (value as? GValue.VList)?.items ?: throw CaseFault()

    private fun text(value: GValue): String = (value as? GValue.VString)?.value ?: throw CaseFault()

    private fun hostTerm(value: GValue): QTerm =
        when (className(value)) {
            "QSym" -> {
                QSym(text(field(value, "name")))
            }

            "QVar" -> {
                QVar(text(field(value, "name")))
            }

            else -> {
                val tail = field(value, "tail").takeUnless { it is GValue.VNull }
                QList(items(field(value, "items")).map(::hostTerm), tail?.let(::hostTerm))
            }
        }

    private fun hostQuery(value: GValue): QQuery =
        when (className(value)) {
            "QPattern" -> QPattern(hostTerm(field(value, "term")))
            "QAnd" -> QAnd(items(field(value, "parts")).map(::hostQuery))
            "QOr" -> QOr(items(field(value, "parts")).map(::hostQuery))
            "QNot" -> QNot(hostQuery(field(value, "part")))
            "QUnique" -> QUnique(hostQuery(field(value, "part")))
            else -> QGuard(guard(field(value, "predicate")), items(field(value, "args")).map(::hostTerm))
        }

    /** The guest predicate as a host one: the terms travel back into guest
     * values, and a guest fault stops the whole query. */
    private fun guard(predicate: GValue): (List<QTerm>) -> Boolean =
        { terms ->
            val argument = GValue.VList(terms.map(::guestTerm).toMutableList(), false)
            val verdict =
                either<GuestError, GValue> { Primitives.invoke(predicate, listOf(argument), NO_POSITION) }
                    .getOrElse { throw GuardFault() }
            (verdict as? GValue.VBool)?.value ?: throw GuardFault()
        }

    private fun guestTerm(term: QTerm): GValue =
        when (term) {
            is QSym -> {
                GValue.VObject("QSym", true, linkedMapOf("name" to GValue.VString(term.name)))
            }

            is QVar -> {
                GValue.VObject("QVar", true, linkedMapOf("name" to GValue.VString(term.name)))
            }

            is QList -> {
                GValue.VObject(
                    "QList",
                    true,
                    linkedMapOf(
                        "items" to GValue.VList(term.items.map(::guestTerm).toMutableList(), false),
                        "tail" to (term.tail?.let(::guestTerm) ?: GValue.VNull),
                    ),
                )
            }
        }
}
