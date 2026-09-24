// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.22

package sicp.ch3.exercises

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VSym

public class E3_22Test :
    FunSpec({
        test("the book's session through the closure queue") {
            val q1 = makeQueue()
            q1.insert(VSym("a"))
            q1.insert(VSym("b"))
            q1.delete() shouldBe Either.Right(VSym("a"))
            q1.printQueue() shouldBe "(b)"
            q1.delete() shouldBe Either.Right(VSym("b"))
            q1.printQueue() shouldBe "()"
        }

        test("delete on an empty closure queue is Left(EmptyQueue)") {
            val q1 = makeQueue()
            q1.delete() shouldBe Either.Left(QueueError.EmptyQueue)
        }

        test("two makeQueue calls give two queues with independent state") {
            val q1 = makeQueue()
            val q2 = makeQueue()
            q1.insert(VSym("a"))
            q2.printQueue() shouldBe "()"
            q1.printQueue() shouldBe "(a)"
            (q1 === q2) shouldBe false
        }

        test("the closure queue is reusable after emptying") {
            val q1 = makeQueue()
            q1.insert(VSym("a"))
            q1.delete() shouldBe Either.Right(VSym("a"))
            q1.emptyQueue() shouldBe true
            q1.insert(VSym("b"))
            q1.delete() shouldBe Either.Right(VSym("b"))
            q1.emptyQueue() shouldBe true
        }
    })
