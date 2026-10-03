// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.2: the iterative factorial machine of exercise 5.1
// described in the register-machine language. Where 5.1 draws, 5.2 writes:
// the controller of `factorialController` is that description, read back
// through the assembler's own summary (instruction count, labels, and
// label positions) and run on sample inputs.

package sicp.ch5.solutions

import sicp.runtime.Label
import sicp.runtime.Stmt
import sicp.runtime.assemble

/** The assembly report of a transcribed controller: how many instructions
 *  and labels it carries and where each label sits. A label names the
 *  position of the instruction that follows it; a trailing label names the
 *  stop address one past the last instruction. */
public fun assemblyReport(controller: List<Stmt>): String {
    val summary =
        assemble(controller).fold(
            { error -> error("the controller does not assemble: ${error.category}") },
            { it },
        )
    val instructions = controller.count { it !is Label }
    val positions =
        summary.labels.entries.joinToString(" ") { (name, index) ->
            "$name=${controller.take(index).count { it !is Label }}"
        }
    return "$instructions instructions, ${summary.labels.size} labels; $positions"
}

/** The controller of exercise 5.2 is the machine of exercise 5.1, written
 *  in the register-machine language: five instructions around two labels. */
public fun factorialAssemblyReport(): List<String> =
    listOf(
        assemblyReport(factorialController),
        render(runFactorialMachine(5)),
        render(runFactorialMachine(6)),
    )
