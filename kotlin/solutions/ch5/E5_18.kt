// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.18: traced registers. The traced register
// overrides the one write path, `store`, reporting the register's name,
// its old content, and the incoming value; the machine builds all of its
// named registers from the traced class by overriding `newRegister`. The
// flag is an ordinary register, so only the controller's registers report.

package sicp.ch5.solutions

import sicp.ch5.Machine
import sicp.ch5.Op
import sicp.ch5.Register
import sicp.ch5.arithOperations
import sicp.ch5.getRegisterContents
import sicp.ch5.setRegisterContents
import sicp.runtime.Reg
import sicp.runtime.VInt
import sicp.runtime.Value

/** A register that reports every store: name, old content, new content. */
public class TracedRegister(
    name: Reg,
    private val sink: StringBuilder,
) : Register(name) {
    public override fun store(value: Value) {
        sink.appendLine("$name: $content -> $value")
        super.store(value)
    }
}

/** The machine whose named registers are traced registers. The base
 *  constructor allocates nothing (empty list), so every register is
 *  built by the override after `sink` is initialized. */
public class TracedRegisterMachine(
    registerNames: List<Reg>,
    userOperations: Map<String, Op>,
    sink: StringBuilder,
) : Machine(emptyList(), userOperations) {
    private val tracedSink: StringBuilder = sink

    init {
        registerNames.forEach { allocateRegister(it) }
    }

    override fun newRegister(name: Reg): Register = TracedRegister(name, tracedSink)
}

/** The gcd machine with traced registers: every store the controller
 *  makes, in order, ending with the run's answer read back. */
public fun tracedRegisterGcdLog(): List<String> =
    machineRun {
        val sink = StringBuilder()
        val machine = TracedRegisterMachine(listOf("a", "b", "t"), arithOperations, sink)
        machine.install(gcdController)
        machine.setRegisterContents("a", VInt(12))
        machine.setRegisterContents("b", VInt(8))
        machine.start()
        sink.toString().trimEnd().lines() + "gcd(12, 8) = ${machine.getRegisterContents("a")}"
    }
