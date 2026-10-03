// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.28: change sequence evaluation so a procedure's
// last expression no longer reuses its caller's continuation. The variant
// keeps a sequence continuation on the machine stack across every body
// expression, including the last. Each compound call also retains an
// explicit return frame. The recursive and iterative factorial programs
// are unchanged: the difference measured below belongs to the evaluator.

package sicp.ch5.solutions

import sicp.ch5.ExplicitControl
import sicp.guest.GValue
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Machine
import sicp.runtime.OpCond
import sicp.runtime.Restore
import sicp.runtime.Save
import sicp.runtime.Source.ConstSrc
import sicp.runtime.Source.OpSrc
import sicp.runtime.Source.RegSrc
import sicp.runtime.Stmt
import sicp.runtime.Test
import sicp.runtime.assemble

private fun frameKind(name: String): OpSrc = OpSrc("literal", listOf(ConstSrc(GValue.VString(name))))

/** The 5.4.2 controller variant. `frame-seq-next` and
 * `frame-seq-advance` are the original operations: the variant changes
 * when the continuation is saved, not how the block is traversed. */
public fun nonTailController(): List<Stmt> = nonTailControllerStatements

private val nonTailControllerStatements: List<Stmt> by lazy { buildNonTailController() }

private fun buildNonTailController(): List<Stmt> {
    val base = ExplicitControl.controller()

    fun label(name: String): Int =
        base.indexOfFirst { it is Label && it.name == name }.also {
            check(it >= 0) { "the evaluator controller has no $name entry" }
        }

    val dispatchEnd = base.indexOfFirst { it is Goto && it.to == GotoTarget.Lbl("halt") }
    val marker = label("fr-marker")
    val applyCompound = label("fr-apply-compound")
    val sequence = label("fr-seq")
    val returnFrame = label("fr-ret")
    check(dispatchEnd in 0 until marker && marker < applyCompound && applyCompound < sequence && sequence < returnFrame)

    val registerFrames =
        listOf(
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), frameKind("seq-restore")))),
            Branch("fr-seq-restore"),
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), frameKind("compound-return")))),
            Branch("fr-compound-return"),
        )
    // A return or break marker must restore the saved sequence and
    // compound-call frames before ordinary marker propagation pops them.
    val restoreMarker =
        listOf(
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), frameKind("seq-restore")))),
            Branch("fr-seq-restore"),
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), frameKind("compound-return")))),
            Branch("fr-compound-return"),
        )
    val compoundBody =
        listOf(
            Label("fr-apply-compound"),
            Assign(
                "kont",
                OpSrc("push-frame", listOf(frameKind("compound-return"), RegSrc("expr"), RegSrc("env"), RegSrc("kont"))),
            ),
            Assign("env", OpSrc("extend-environment", listOf(RegSrc("proc"), RegSrc("argl")))),
            Assign("expr", OpSrc("procedure-body", listOf(RegSrc("proc")))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
        )
    val sequenceBody =
        listOf(
            Label("fr-seq"),
            Test(OpCond("frame-seq-empty", listOf(RegSrc("kont")))),
            Branch("fr-seq-done"),
            Assign("expr", OpSrc("frame-seq-next", listOf(RegSrc("kont")))),
            Save("kont"),
            Assign(
                "kont",
                OpSrc("push-frame", listOf(frameKind("seq-restore"), RegSrc("expr"), RegSrc("env"), RegSrc("kont"))),
            ),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("fr-seq-done"),
            Assign("kont", OpSrc("pop-frame", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("fr-seq-restore"),
            Restore("kont"),
            Assign("kont", OpSrc("frame-seq-advance", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("fr-compound-return"),
            Assign("env", OpSrc("frame-env", listOf(RegSrc("kont")))),
            Assign("kont", OpSrc("pop-frame", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
        )

    val controller =
        base.subList(0, dispatchEnd) +
            registerFrames +
            base.subList(dispatchEnd, marker + 1) +
            restoreMarker +
            base.subList(marker + 1, applyCompound) +
            compoundBody +
            sequenceBody +
            base.subList(returnFrame, base.size)
    assemble(controller).fold(
        { error -> error("the non-tail controller does not assemble: ${error.category}") },
        { },
    )
    return controller
}

private fun measureNonTail(source: String): Stats {
    val checked = admitProgram(source)
    val machine = Machine(ExplicitControl.REGS, ExplicitControl.operations(checked), nonTailController())
    runToHalt(machine, budget = 50_000_000)
    return Stats(machine.stack.pushes.toInt(), machine.stack.maxDepth.toInt())
}

/** Both factorial versions on the altered evaluator. The rows are
 * machine measurements, not constants borrowed from the book's different
 * controller; growth in depth is the exercise's observable conclusion. */
public fun nonTailRecursiveMeasurements(): List<String> {
    val ns = listOf(1, 2, 3, 4, 5)
    val iterative = ns.map { n -> measureNonTail(measuredCall(iterativeFactorialSource, "factorial(${n}L)")) }
    val recursive = ns.map { n -> measureNonTail(measuredCall(recursiveFactorialSource, "factorial(${n}L)")) }
    val rows =
        ns.indices.flatMap { index ->
            listOf(
                renderStats("non-tail iterative factorial", ns[index], iterative[index]),
                renderStats("non-tail recursive factorial", ns[index], recursive[index]),
            )
        }
    val recursiveGrows = recursive.zipWithNext().all { (a, b) -> b.depth > a.depth }
    val iterativeGrows = iterative.zipWithNext().all { (a, b) -> b.depth > a.depth }
    return rows +
        listOf(
            "recursive maximum depth now grows with n: $recursiveGrows",
            "iterative maximum depth now grows with n: $iterativeGrows",
        )
}

/** The altered controller still computes a guest program's answer; only
 * its continuation discipline changes. */
public fun nonTailAnswersAgree(): Boolean {
    val source =
        recursiveFactorialSource +
            "\nfun main() {\n    println(factorial(5L))\n}\n"
    val checked = admitProgram(source)
    val sink = sicp.guest.OutputSink()
    val machine = Machine(ExplicitControl.REGS, ExplicitControl.operations(checked, sink), nonTailController())
    runToHalt(machine, budget = 50_000_000)
    val direct = sicp.ch4.Direct.run(checked)
    return direct.output == sink.contents() && sink.contents() == "120\n"
}
