// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.22: register machines for append and append!
// over the list-structure memory. The append machine copies x cell by
// cell and shares y: the recursion of exercise 3.12's procedure with
// the car parked on the stack, three fresh cells for x = (1 2 3), and
// the answer lands in z. The append! machine walks to the last pair of
// x and splices y in with a set-cdr!: no cell is allocated, there is no
// z, and the value of x is the answer, which is why the free pointer
// does not move.

package sicp.ch5.solutions

import arrow.core.raise.Raise
import sicp.guest.GValue
import sicp.guest.GuestError
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.OpAct
import sicp.runtime.Perform
import sicp.runtime.Restore
import sicp.runtime.Save
import sicp.runtime.Stmt
import sicp.runtime.Test

/** Plants a proper list of numbers through the allocation path. */
context(r: Raise<GuestError>)
private fun plantList(
    memory: Memory,
    ns: List<Long>,
): GValue {
    var acc: GValue = GValue.VNull
    for (n in ns.asReversed()) {
        acc = memory.cons(numberWord(n), acc)
    }
    return acc
}

/** The append machine: it copies x and shares y; the answer lands in
 *  z. */
public val appendController: List<Stmt> =
    listOf(
        Assign("continue", labelSrc("append-done")),
        Label("append-loop"),
        Test(opCond("null?", reg("x"))),
        Branch("base"),
        Assign("temp", opSrc("car", reg("x"))),
        Save("temp"),
        Save("continue"),
        Assign("continue", labelSrc("after-car")),
        Assign("x", opSrc("cdr", reg("x"))),
        Goto(GotoTarget.Lbl("append-loop")),
        Label("base"),
        Assign("z", reg("y")),
        Goto(GotoTarget.ByReg("continue")),
        Label("after-car"),
        Restore("continue"),
        Restore("temp"),
        Assign("z", opSrc("cons", reg("temp"), reg("z"))),
        Goto(GotoTarget.ByReg("continue")),
        Label("append-done"),
    )

/** The append! machine: walk to the last pair of x and splice y in
 *  with a set-cdr!; no cell is allocated and there is no z. */
public val appendBangController: List<Stmt> =
    listOf(
        Assign("temp", reg("x")),
        Label("last-pair"),
        Assign("cand", opSrc("cdr", reg("temp"))),
        Test(opCond("null?", reg("cand"))),
        Branch("splice"),
        Assign("temp", opSrc("cdr", reg("temp"))),
        Goto(GotoTarget.Lbl("last-pair")),
        Label("splice"),
        Perform(OpAct("set-cdr!", listOf(reg("temp"), reg("y")))),
    )

/** The exercise's runs: append over planted x = (1 2 3) and y = (4 5),
 *  then append! on a fresh copy, with the memory drawn before and
 *  after the splice. */
public fun appendRuns(): List<String> =
    machineScope {
        val memory = Memory(size = 16)
        val x = plantList(memory, listOf(1, 2, 3))
        val y = plantList(memory, listOf(4, 5))
        val machine =
            freshMachine(
                setOf("x", "y", "z", "temp", "continue"),
                listOperations(memory),
                appendController,
                mapOf("x" to x, "y" to y),
            )
        runToHalt(machine)
        val z = machine.registers.getValue("z").content
        val appendLines =
            listOf(
                "append: z = ${wordToString(z)} = ${memory.write(z)}",
                "append: x is still ${memory.write(x)} (${wordToString(x)}), free moved to " +
                    "${wordToString(pairPointer(memory.free))}, three fresh cells",
            )

        val memory2 = Memory(size = 8)
        val x2 = plantList(memory2, listOf(1, 2, 3))
        val y2 = plantList(memory2, listOf(4, 5))
        val before = memory2.dump()
        val machine2 =
            freshMachine(
                setOf("x", "y", "temp", "cand"),
                listOperations(memory2),
                appendBangController,
                mapOf("x" to x2, "y" to y2),
            )
        runToHalt(machine2)
        val xSpliced = machine2.registers.getValue("x").content
        appendLines +
            listOf(
                "append!: before, the last pair of x points at e0:",
                before,
                "append!: after, it points at y:",
                memory2.dump(),
                "append!: the answer is x itself, now ${memory2.write(xSpliced)}, the same " +
                    "pointer ${wordToString(xSpliced)} the caller passed, and free is still " +
                    "${wordToString(pairPointer(memory2.free))}",
            )
    }
