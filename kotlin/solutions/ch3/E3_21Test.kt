// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.21

package sicp.ch3.exercises

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe
import sicp.runtime.VSym

public class E3_21Test :
    FunSpec({
        test("Ben's session: printQueue agrees with the front cell, the rear cell holds the last item") {
            val q1 = Queue()
            q1.insert(VSym("a"))
            q1.insert(VSym("b"))
            q1.insert(VSym("c"))
            q1.insert(VSym("d"))
            q1.printQueue() shouldBe "(a b c d)"
            q1.frontCell()?.toString() shouldBe "(a b c d)"
            q1.rearCell()?.car shouldBe VSym("d")
            benView(q1) shouldBe "((a b c d) d)"
        }

        test("the book's printed pairs: ((a) a), then ((a b) b), then ((b) b)") {
            val q1 = Queue()
            q1.insert(VSym("a"))
            benView(q1) shouldBe "((a) a)"
            q1.insert(VSym("b"))
            benView(q1) shouldBe "((a b) b)"
            q1.delete() shouldBe Either.Right(VSym("a"))
            benView(q1) shouldBe "((b) b)"
        }

        test("after the second delete this edition's cells are clean and the queue prints ()") {
            val q1 = Queue()
            q1.insert(VSym("a"))
            q1.insert(VSym("b"))
            q1.delete() shouldBe Either.Right(VSym("a"))
            q1.delete() shouldBe Either.Right(VSym("b"))
            q1.printQueue() shouldBe "()"
            q1.frontCell().shouldBeNull()
            q1.rearCell().shouldBeNull()
            benView(q1) shouldBe "(() ())"
        }

        test("the rear cell never holds a copy: it is the last cell of the front chain") {
            val q1 = Queue()
            q1.insert(VSym("a"))
            q1.insert(VSym("b"))
            (q1.rearCell() === q1.frontCell()?.cdr) shouldBe true
            q1.delete()
            q1.frontCell()?.toString() shouldBe "(b)"
            q1.rearCell()?.car shouldBe VSym("b")
        }
    })
