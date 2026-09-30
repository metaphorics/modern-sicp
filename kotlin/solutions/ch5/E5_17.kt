// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.17: traced lines with their labels. The tracing
// machine keeps the label currently in effect -- the most recently passed
// label definition -- and prints it ahead of each traced instruction, so
// the trace reads like the controller listing. Label definitions
// themselves take no trace line and never disturb the instruction count.

package sicp.ch5.solutions

import sicp.guest.GValue
import sicp.runtime.Label
import sicp.runtime.MachineOp
import sicp.runtime.Stmt

/** The label-aware tracing machine: a label at the current address is
 *  printed before the instruction text. */
public class LabelTracingMachine(
    registerNames: Set<String>,
    ops: Map<String, MachineOp>,
    controller: List<Stmt>,
    initial: Map<String, GValue> = emptyMap(),
) {
    /** The trace switch. */
    public var traceOn: Boolean = false

    private val machine = freshMachine(registerNames, ops, controller, initial)

    /** Runs the machine, answering the labeled trace lines. */
    public fun run(): List<String> {
        var currentLabel: String? = null
        return traceDrive(machine, { traceOn }) { instruction ->
            if (instruction is Label) {
                currentLabel = instruction.name
                null
            } else {
                val label = currentLabel
                if (label == null) instructionText(instruction) else "$label: ${instructionText(instruction)}"
            }
        }
    }
}

/** The traced gcd run with labels: every executed line is named by the
 *  label in effect, `test-b` for the loop; `gcd-done` never takes a line,
 *  because the machine halts on reaching it. */
public fun labelTracedGcdTrace(): List<String> =
    LabelTracingMachine(
        setOf("a", "b", "t"),
        machineArithmetic,
        gcdController,
        mapOf("a" to GValue.VLong(206), "b" to GValue.VLong(40)),
    ).apply { traceOn = true }.run()
