// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.13: derive the machine's register set from the
// controller text instead of passing it to the machine constructor. The
// exercise's suggestion -- allocate each register when assembly first sees
// it -- is realized by reading the names out of the typed instruction data
// and handing exactly that set to the machine.

package sicp.ch5.solutions

import sicp.guest.GValue
import sicp.runtime.Stmt

/** The registers of [controller], derived the way 5.13's `make-machine`
 *  allocates them: every name the instruction data mentions. */
public fun derivedRegisters(controller: List<Stmt>): List<String> = controllerRegisters(controller).sorted()

/** The gcd machine built from its controller alone: the derived register
 *  list, the run, and the register set the machine ended up allocating. */
public fun derivedRegisterRuns(): List<String> {
    val names = derivedRegisters(gcdController).toSet()
    val machine =
        freshMachine(
            names,
            machineArithmetic,
            gcdController,
            mapOf("a" to GValue.VLong(206), "b" to GValue.VLong(40)),
        )
    runToHalt(machine)
    return listOf(
        "derived registers: ${names.sorted().joinToString(" ")}",
        "gcd(206, 40) = ${render(machine.registers.getValue("a").content)}",
        "allocated registers: ${machine.registers.keys.sorted().joinToString(" ")}",
    )
}
