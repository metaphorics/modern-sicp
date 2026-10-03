// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.21

package sicp.ch3.exercises

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe
import sicp.runtime.Datum
import sicp.runtime.PairCell
import sicp.runtime.Symbol

public class E3_21Test :
    FunSpec({
        test("the host-data view reports front items and the rear datum") {
            val queue = Queue()
            queue.insert(Symbol("a"))
            queue.insert(Symbol("b"))
            queue.insert(Symbol("c"))
            queue.insert(Symbol("d"))

            queue.items() shouldBe listOf(Symbol("a"), Symbol("b"), Symbol("c"), Symbol("d"))
            benView(queue) shouldBe (listOf(Symbol("a"), Symbol("b"), Symbol("c"), Symbol("d")) to Symbol("d"))
            queue.rearCell()?.first shouldBe Symbol("d")
            val first = queue.frontCell()
            val second = first?.second as? PairCell
            val third = second?.second as? PairCell
            (third?.second === queue.rearCell()) shouldBe true
        }

        test("the rear datum follows mutations to the queue state") {
            val queue = Queue()
            queue.insert(Symbol("a"))
            benView(queue) shouldBe (listOf(Symbol("a")) to Symbol("a"))
            queue.insert(Symbol("b"))
            benView(queue) shouldBe (listOf(Symbol("a"), Symbol("b")) to Symbol("b"))
            queue.delete() shouldBe Either.Right(Symbol("a"))
            benView(queue) shouldBe (listOf(Symbol("b")) to Symbol("b"))
        }

        test("after deleting the last item both pointer cells are absent") {
            val queue = Queue()
            queue.insert(Symbol("a"))
            queue.insert(Symbol("b"))
            queue.delete() shouldBe Either.Right(Symbol("a"))
            queue.delete() shouldBe Either.Right(Symbol("b"))

            queue.items() shouldBe emptyList<Datum>()
            queue.frontCell().shouldBeNull()
            queue.rearCell().shouldBeNull()
            benView(queue) shouldBe (emptyList<Datum>() to null)
        }

        test("items visit each reachable cell once through a mutable cycle") {
            val queue = Queue()
            queue.insert(Symbol("a"))
            val head = queue.frontCell() ?: error("single-item queue lost its front cell")
            head.second = head

            queue.items() shouldBe listOf(Symbol("a"))
        }
    })
