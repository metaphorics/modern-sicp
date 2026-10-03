// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.15: instruction counting. The book asks the model
// to keep track of executed instructions and to accept a message that
// prints the count and resets it to zero. The substrate's own counter
// counts every step, label definitions included; the book's count is the
// instructions a controller listing shows, so this counter steps the
// machine and counts the non-label instructions, and `printAndReset`
// answers the report and zeroes the count.

package sicp.ch5.solutions

import sicp.guest.GValue
import sicp.runtime.Label
import sicp.runtime.Machine

/** The counting machine: every executed instruction increments the
 *  counter, transfers included. */
public class InstructionCounter(
    private val machine: Machine,
) {
    /** Executed instructions since the counter was reset; labels excluded. */
    public var count: Long = 0
        private set

    /** The book's message: report the count and reset it to zero. */
    public fun printAndReset(): String {
        val line = "$count instructions"
        count = 0
        return line
    }

    /** Runs the machine to its halt under the counter. */
    public fun run(): Long {
        while (!machine.halted()) {
            val instruction = machine.controller[machine.pc]
            stepOrFail(machine, instruction)
            if (instruction !is Label) count++
        }
        return count
    }
}

/** The gcd and factorial machines with their instruction counts for one
 *  run each. */
public fun gcdInstructionCounts(): List<String> {
    val gcd =
        freshMachine(
            setOf("a", "b", "t"),
            machineArithmetic,
            gcdController,
            mapOf("a" to GValue.VLong(206), "b" to GValue.VLong(40)),
        )
    val gcdCount = InstructionCounter(gcd).run()
    val fact =
        freshMachine(
            setOf("n", "continue", "val"),
            machineArithmetic,
            recursiveFactorialController,
            mapOf("n" to GValue.VLong(5)),
        )
    val factCount = InstructionCounter(fact).run()
    return listOf(
        "gcd(206, 40): $gcdCount instructions",
        "factorial(5): $factCount instructions",
    )
}
