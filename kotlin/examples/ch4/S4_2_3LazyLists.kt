// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.2
// Chapter 4, section 4.2.3, streams as lazy lists: the lazy pair with its
// memoized tail, the list operations over it, and the two self-referential
// sessions the section runs -- `ones` feeding itself through the captured
// var cell, `integers` defined as its own tail shifted by `ones` and
// demanded at element 17, and the `solve` integral demanded at element
// 1000, which answers e to the pinned decimal.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.LazyModule
import sicp.ch4.LazyRun

/** The driver's printed form of element 1000 of the solve integral: the
 * host's shortest round-trip decimal for the computed double. */
private const val SOLVE_ELEMENT_1000: String = "2.716923932235896"

private val LAZY_LIST_SESSION: String =
    """
    fun main() {
        val p = lazyPair(1L, thunk {
            println("tail")
            listOf(2L, 3L)
        })
        println(p.get(0))
        println(p.get(1))
        println(p.get(2))
    }
    """.trimIndent()

private val STREAMS_SESSION: String =
    """
    data class Cell(val head: Long, val tail: Thunk<Cell>)

    fun zeroCells(): Cell = Cell(0L, thunk { zeroCells() })

    fun demand(cell: Cell, n: Long): Long =
        if (n == 0L) cell.head else demand(force(cell.tail), n - 1L)

    fun addCells(a: Cell, b: Cell): Cell =
        Cell(a.head + b.head, thunk { addCells(force(a.tail), force(b.tail)) })

    data class DCell(val head: Double, val tail: Thunk<DCell>)

    fun demandDouble(cell: DCell, n: Long): Double =
        if (n == 0L) cell.head else demandDouble(force(cell.tail), n - 1L)

    fun solveStep(prev: Double, dt: Double): DCell =
        DCell(prev, thunk { solveStep(prev + dt * prev, dt) })

    fun main() {
        // ones feeds itself: the tail thunk reads the same var cell the
        // assignment filled, so every demand observes the one structure
        var ones: Cell = Cell(1L, thunk { zeroCells() })
        ones = Cell(1L, thunk { ones })
        println(demand(ones, 17L))
        // integers is its own tail shifted by ones
        var integers: Cell = Cell(1L, thunk { zeroCells() })
        integers = Cell(1L, thunk { addCells(ones, integers) })
        println(demand(integers, 17L))
        // the solve integral for y' = y from 1 in steps of 0.001
        println(demandDouble(solveStep(1.0, 0.001), 1000L))
    }
    """.trimIndent()

private fun lazyRun(source: String): LazyRun =
    LazyModule.run(source).fold(
        { e -> throw AssertionError("admission rejected the unit: ${e.category}: ${e.message}") },
        { it },
    )

/** Runs [block] on a thread with stack room for the deep non-tail force
 * chain of a thousand demands, and returns its answer. */
private inline fun <T> forcingThread(crossinline block: () -> T): T {
    var answer: Result<T>? = null
    val worker =
        java.lang.Thread(
            null,
            { answer = runCatching(block) },
            "lazy-force",
            256L * 1024L * 1024L,
        )
    worker.start()
    worker.join()
    return checkNotNull(answer).getOrThrow()
}

public class S4_2_3LazyListsTest :
    FunSpec({
        test("the lazy pair's head is eager and its tail computes on demand, once") {
            val run = lazyRun(LAZY_LIST_SESSION)
            run.result.output shouldBe "1\ntail\n2\n3\n"
            run.result.error shouldBe null
        }

        test("the self-referential streams deliver their values at depth") {
            // the deep demand runs where the force chain has stack room
            val output = forcingThread { lazyRun(STREAMS_SESSION).result.output }
            output.lines().get(0) shouldBe "1"
        }

        test("the integer stream demanded at element 17 answers 18") {
            val output = forcingThread { lazyRun(STREAMS_SESSION).result.output }
            output.lines().get(1) shouldBe "18"
        }

        test("the solve integral demanded at element 1000 answers e") {
            val output = forcingThread { lazyRun(STREAMS_SESSION).result.output }
            output.lines().get(2) shouldBe SOLVE_ELEMENT_1000
        }
    })
