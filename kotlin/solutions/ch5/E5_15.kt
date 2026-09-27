// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.15: instruction counting. The counting machine
// overrides the one execution loop: before each execution procedure runs,
// the counter advances. The counts are read after the run, the way
// Alyssa's monitor would report them.

package sicp.ch5.solutions

import arrow.core.raise.Raise
import sicp.ch5.Machine
import sicp.ch5.MachineError
import sicp.ch5.Op
import sicp.ch5.arithOperations
import sicp.ch5.setRegisterContents
import sicp.runtime.Reg
import sicp.runtime.VInt

/** The counting machine: every executed instruction increments the
 *  counter, transfers included. */
public class CountingMachine(
    registerNames: List<Reg>,
    userOperations: Map<String, Op>,
) : Machine(registerNames, userOperations) {
    /** Executed instructions since the machine was built or the counter
     *  was reset. */
    public var instructionCount: Long = 0

    context(r: Raise<MachineError>)
    override fun execute() {
        while (pc < insts.size) {
            instructionCount += 1
            insts[pc].exec(r)
        }
    }
}

/** The gcd and factorial machines with their instruction counts for one
 *  run each. */
public fun gcdInstructionCounts(): List<String> =
    machineRun {
        val gcd =
            CountingMachine(listOf("a", "b", "t"), arithOperations).apply {
                install(gcdController)
                setRegisterContents("a", VInt(206))
                setRegisterContents("b", VInt(40))
                start()
            }
        val fact =
            CountingMachine(listOf("n", "continue", "val"), arithOperations).apply {
                install(recursiveFactorialSimController)
                setRegisterContents("n", VInt(5))
                start()
            }
        listOf(
            "gcd(206, 40): ${gcd.instructionCount} instructions",
            "factorial(5): ${fact.instructionCount} instructions",
        )
    }
