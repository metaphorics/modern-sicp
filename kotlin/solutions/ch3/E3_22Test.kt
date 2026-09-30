// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.22

package sicp.ch3.exercises

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Datum
import sicp.runtime.Symbol

public class E3_22Test :
    FunSpec({
        test("the closure queue session updates its host-data view") {
            val queue = makeQueue()
            queue.insert(Symbol("a"))
            queue.insert(Symbol("b"))
            queue.delete() shouldBe Either.Right(Symbol("a"))
            queue.items() shouldBe listOf(Symbol("b"))
            queue.delete() shouldBe Either.Right(Symbol("b"))
            queue.items() shouldBe emptyList<Datum>()
        }

        test("delete on an empty closure queue is Left(EmptyQueue)") {
            val queue = makeQueue()
            queue.delete() shouldBe Either.Left(QueueError.EmptyQueue)
        }

        test("two makeQueue calls have independent captured state") {
            val first = makeQueue()
            val second = makeQueue()
            first.insert(Symbol("a"))
            second.items() shouldBe emptyList<Datum>()
            first.items() shouldBe listOf(Symbol("a"))
            (first === second) shouldBe false
        }

        test("the closure queue is reusable after emptying") {
            val queue = makeQueue()
            queue.insert(Symbol("a"))
            queue.delete() shouldBe Either.Right(Symbol("a"))
            queue.emptyQueue() shouldBe true
            queue.insert(Symbol("b"))
            queue.delete() shouldBe Either.Right(Symbol("b"))
            queue.emptyQueue() shouldBe true
        }
    })
