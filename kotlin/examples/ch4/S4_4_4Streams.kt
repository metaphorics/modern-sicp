// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.4.4.6, stream operations: the answer stream is a
// lazy sequence of frames -- a partial take reads only the frames it
// needs, duplicates are stream elements until a view collapses them, and
// `or` interleaves its disjunct streams so neither can starve the other.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.QOr
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver

public class S4_4_4StreamsTest :
    FunSpec({
        val x = variable("x")

        test("or interleaves the disjunct streams; appending would starve") {
            val database = QueryDatabase()
            database.assertFact(fact(terms(sym("p"), sym("a1"))))
            database.assertFact(fact(terms(sym("p"), sym("a2"))))
            database.assertFact(fact(terms(sym("q"), sym("b1"))))
            val driver = QueryDriver.streaming(database)
            val query = QOr(listOf(pattern(terms(sym("p"), x)), pattern(terms(sym("q"), x))))
            answerLines(driver, query, listOf(x)) shouldBe
                listOf(
                    "?x = a1",
                    "?x = b1",
                    "?x = a2",
                )
        }

        test("the answer stream delivers one frame per step, duplicates included") {
            val driver = QueryDriver.streaming(microshaftDatabase())
            val wheel = pattern(terms(sym("wheel"), x))
            val firstTwo =
                driver
                    .run(wheel, listOf(x))
                    .take(2)
                    .flatMap { sicp.ch4.renderAnswer(it, listOf(x)) }
                    .toList()
            firstTwo shouldBe
                listOf(
                    "?x = [Bitdiddle, Ben]",
                    "?x = [Warbucks, Oliver]",
                )
            val every = answerLines(driver, wheel, listOf(x))
            every.size shouldBe 5
        }

        test("repeated disjuncts repeat their frames until a view collapses them") {
            val database = QueryDatabase()
            database.assertFact(fact(terms(sym("p"), sym("a1"))))
            val streaming = QueryDriver.streaming(database)
            val deduplicating = QueryDriver.deduplicating(database)
            val query = QOr(listOf(pattern(terms(sym("p"), x)), pattern(terms(sym("p"), x))))
            answerLines(streaming, query, listOf(x)) shouldBe
                listOf(
                    "?x = a1",
                    "?x = a1",
                )
            answerLines(deduplicating, query, listOf(x)) shouldBe listOf("?x = a1")
        }
    })
