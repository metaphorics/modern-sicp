// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.46: the same analysis for tree-recursive
// Fibonacci. The report lists compiled, evaluator, and special-purpose
// machine counters in that order without claiming a cross-engine ranking.
package sicp.ch5.solutions

import sicp.guest.GValue

private fun specialFibMeasurements(n: Long): MachineMeasurements {
    val machine =
        freshMachine(
            setOf("n", "continue", "val"),
            machineArithmetic,
            fibController,
            mapOf("n" to GValue.VLong(n), "val" to GValue.VLong(0)),
        )
    runToHalt(machine)
    return MachineMeasurements(machine.instructions, machine.stack.pushes, machine.stack.maxDepth)
}

/** Chapter 5.5 Exercise 5.46 records machine-specific measurements in
 *  compiled, evaluator, special-purpose order; it makes no cross-machine
 *  ordering claim. */
public fun fibonacciStackReport(): List<String> {
    val program = measuredCall(fibSource, "fib(5L)")
    val compiled = compiledMeasurements(program)
    val evaluator = evaluatorMeasurements(program)
    val special = specialFibMeasurements(5)
    return listOf(
        "compiled fib(5): instructions = ${compiled.instructions}, total-pushes = ${compiled.pushes}, maximum-depth = ${compiled.maximumDepth}",
        "eceval fib(5): instructions = ${evaluator.instructions}, total-pushes = ${evaluator.pushes}, maximum-depth = ${evaluator.maximumDepth}",
        "special-purpose fib(5): instructions = ${special.instructions}, total-pushes = ${special.pushes}, maximum-depth = ${special.maximumDepth}",
    )
}
