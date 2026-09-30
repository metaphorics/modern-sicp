// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.18: traced registers. A traced register reports
// every store: the register's name, its old content, and the incoming
// value. The substrate's registers are closed to subclassing, so the
// trace rides the stepping seam: the input stores and every `assign` the
// controller executes report through one log. The flag is an ordinary
// register the controller never names, so only the controller's registers
// report.

package sicp.ch5.solutions

import sicp.guest.GValue
import sicp.runtime.Assign

/** One reported store: `name: old -> new`. */
private fun storeLine(
    name: String,
    old: GValue,
    new: GValue,
): String = "$name: ${render(old)} -> ${render(new)}"

/** The gcd machine with traced registers: every store the controller
 *  makes, in order, ending with the run's answer read back. */
public fun tracedRegisterGcdLog(): List<String> {
    val log = mutableListOf<String>()
    val machine =
        freshMachine(
            setOf("a", "b", "t"),
            machineArithmetic,
            gcdController,
        )
    for ((name, value) in listOf("a" to GValue.VLong(12), "b" to GValue.VLong(8))) {
        val old = machine.registers.getValue(name).content
        machine.registers.getValue(name).content = value
        log.add(storeLine(name, old, value))
    }
    while (!machine.halted()) {
        val instruction = machine.controller[machine.pc]
        val target = instruction as? Assign
        val old = target?.let { machine.registers.getValue(it.reg).content }
        stepOrFail(machine, instruction)
        if (target != null && old != null) {
            log.add(storeLine(target.reg, old, machine.registers.getValue(target.reg).content))
        }
    }
    return log + "gcd(12, 8) = ${render(machine.registers.getValue("a").content)}"
}
