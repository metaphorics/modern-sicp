// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.34: compile the iterative factorial and
// annotate its maximum stack depth. The annotation reads the compiled
// code's own machine run -- the section's `Compiler.machine` seam -- so
// every number is the compiled machine's stack output, and the depth's
// behavior in n is the exercise's relation.

package sicp.ch5.solutions

import arrow.core.raise.either
import sicp.guest.GuestError

/** The compiled iterative factorial's stack annotation for the measured
 *  n, with the relation the exercise concludes with. */
public fun compiledFactorialAnnotation(): List<String> {
    val ns = listOf(1, 2, 3, 4, 5, 6)
    val rows =
        ns.map { n ->
            val machine = compiledMachine(measuredCall(iterativeFactorialSource, "factorial(${n}L)"))
            val outcome = either<GuestError, Unit> { machine.run() }
            outcome.fold({ error -> error("the compiled run faulted: ${error.category}") }, { })
            "compiled iterative factorial n=$n: total-pushes = ${machine.stack.pushes} maximum-depth = ${machine.stack.maxDepth}"
        }
    val depths =
        ns.map { n ->
            val machine = compiledMachine(measuredCall(iterativeFactorialSource, "factorial(${n}L)"))
            val outcome = either<GuestError, Unit> { machine.run() }
            outcome.fold({ error -> error("the compiled run faulted: ${error.category}") }, { })
            machine.stack.maxDepth
        }
    return rows + "compiled maximum depth independent of n: ${depths.distinct().size == 1}"
}
