// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.32

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_32Test :
    FunSpec({
        test("two actions added to one segment run in insertion order, first in first out") {
            sameSegmentRunOrder() shouldBe listOf("first", "second")
        }

        test("the queue, not the propagate loop, decides the order: deletion is at the front") {
            // Two segments, two actions each: the loop drains segment by
            // segment in time order, and within a segment the queue hands
            // back the oldest action first.
            val sim = Simulation()
            val ran = mutableListOf<String>()
            sim.agenda.addToAgenda(5) { ran.add("5:first") }
            sim.agenda.addToAgenda(5) { ran.add("5:second") }
            sim.agenda.addToAgenda(3) { ran.add("3:first") }
            sim.agenda.addToAgenda(3) { ran.add("3:second") }
            sim.propagate()
            ran shouldBe listOf("3:first", "3:second", "5:first", "5:second")
            sim.agenda.emptyAgenda() shouldBe true
        }

        test("fifo: the real and-gate settles at its actual final inputs (1, 0)") {
            // Both input changes land in one segment, so the gate
            // schedules and(1, 1) = 1 first and and(1, 0) = 0 second;
            // the queue applies them in schedule order.
            fifoResult() shouldBe 0L
        }

        test("lifo: the stale reading (1, 1) lands last and the gate keeps its superseded value") {
            // The identical circuit over a comparison agenda that pops
            // each segment as a stack runs the two updates backwards.
            lifoResult() shouldBe 1L
        }
    })
