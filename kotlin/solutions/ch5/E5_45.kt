// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.45: compare the stack operations of compiled
// code, the evaluator, and a special-purpose machine for the same
// computation. The report lists each machine's own counters in that
// order; it does not assert a cross-engine ordering because each machine
// has its own calling and continuation conventions.
//
// The compiled code runs on the compiler's machine, the evaluator on the
// explicit-control machine, and the special-purpose computation on the
// substrate machine of section 5.1.

package sicp.ch5.solutions

import arrow.core.raise.either
import sicp.guest.GValue
import sicp.guest.GuestError

internal data class MachineMeasurements(
    val instructions: Long,
    val pushes: Long,
    val maximumDepth: Long,
)

internal fun compiledMeasurements(source: String): MachineMeasurements {
    val machine = compiledMachine(source)
    val outcome = either<GuestError, Unit> { machine.run() }
    outcome.fold({ error -> error("the compiled run faulted: ${error.category}") }, { })
    return MachineMeasurements(machine.instructions, machine.stack.pushes, machine.stack.maxDepth)
}

internal fun compiledStats(source: String): Pair<Long, Long> {
    val measured = compiledMeasurements(source)
    return measured.pushes to measured.maximumDepth
}

internal fun evaluatorMeasurements(source: String): MachineMeasurements {
    val machine = monitoredMachine(source)
    return MachineMeasurements(machine.instructions, machine.stack.pushes, machine.stack.maxDepth)
}

private fun specialPurposeMeasurements(n: Long): MachineMeasurements {
    val machine =
        freshMachine(
            setOf("n", "continue", "val"),
            machineArithmetic,
            recursiveFactorialController,
            mapOf("n" to GValue.VLong(n)),
        )
    runToHalt(machine)
    return MachineMeasurements(machine.instructions, machine.stack.pushes, machine.stack.maxDepth)
}

/** Chapter 5.5 Exercise 5.45 records machine-specific measurements in
 *  compiled, evaluator, special-purpose order; it makes no cross-machine
 *  ordering claim. */
public fun factorialStackReport(): List<String> {
    val program = measuredCall(recursiveFactorialSource, "factorial(5L)")
    val compiled = compiledMeasurements(program)
    val evaluator = evaluatorMeasurements(program)
    val special = specialPurposeMeasurements(5)
    return listOf(
        "compiled factorial(5): instructions = ${compiled.instructions}, total-pushes = ${compiled.pushes}, maximum-depth = ${compiled.maximumDepth}",
        "eceval factorial(5): instructions = ${evaluator.instructions}, total-pushes = ${evaluator.pushes}, maximum-depth = ${evaluator.maximumDepth}",
        "special-purpose factorial(5): instructions = ${special.instructions}, total-pushes = ${special.pushes}, maximum-depth = ${special.maximumDepth}",
    )
}
