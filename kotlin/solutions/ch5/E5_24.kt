// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.24: implement the derived form as a new basic
// special form without reducing it to `if`. The exercise's answer is a
// loop that tests the clauses' predicates successively and evaluates the
// selected clause's body, realized here as an overlay controller over
// the shipped 5.4 machine: `ExplicitControl.controller()` with a `when`
// case spliced in front of the catch-all `structural-eval` entry, one
// `ev-when` loop, and one `fr-when` continuation case beside the
// engine's `fr-*` vocabulary. The clause loop never builds an `if`
// chain: the guard is evaluated by the machine, the case list advances
// in the frame, and the selected body re-enters `eval-dispatch` with the
// frame already popped, so the clause's recursive call stays in tail
// position. The exercise's observables are the session's answers and
// the two machines' instruction counts and stack use.

package sicp.ch5.solutions

import arrow.core.raise.Raise
import arrow.core.raise.either
import sicp.ch5.ExplicitControl
import sicp.guest.Expression
import sicp.guest.GValue
import sicp.guest.GuestError
import sicp.guest.Literal
import sicp.guest.LiteralKind
import sicp.guest.NO_POSITION
import sicp.guest.OutputSink
import sicp.guest.When
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Machine
import sicp.runtime.MachineOp
import sicp.runtime.OpCond
import sicp.runtime.Source.OpSrc
import sicp.runtime.Source.RegSrc
import sicp.runtime.Stmt
import sicp.runtime.Test

/** A syntax node as a machine word: the register model the evaluator's
 *  operation layer already uses. */
private fun nodeWord(expression: Expression): GValue = GValue.VNode(expression)

context(r: Raise<GuestError>)
private fun asNode(value: GValue): Expression =
    ((value as? GValue.VNode)?.node as? Expression) ?: r.raise(GuestError.UnassignedRegister(NO_POSITION))

/** The overlay's frame: the when case list beside the engine's frame
 *  fields, popped by this overlay's own reader. */
private fun whenFrame(
    node: Expression,
    env: GValue,
    rest: GValue,
): GValue =
    GValue.VObject(
        "frame",
        structural = true,
        fields =
            linkedMapOf(
                "kind" to GValue.VString("when"),
                "node" to nodeWord(node),
                "env" to env,
                "rest" to rest,
            ),
    )

context(r: Raise<GuestError>)
private fun frameField(
    frame: GValue,
    name: String,
): GValue =
    (frame as? GValue.VObject)?.fields?.get(name)
        ?: r.raise(GuestError.UnassignedRegister(NO_POSITION))

context(r: Raise<GuestError>)
private fun frameWhen(frame: GValue): When =
    ((frameField(frame, "node") as? GValue.VNode)?.node as? When)
        ?: r.raise(GuestError.UnassignedRegister(NO_POSITION))

/** The clause-loop operations: the when case list advances inside the
 *  frame, one clause at a time, exactly the loop the exercise asks for. */
public fun whenOverlayOperations(): Map<String, MachineOp> =
    mapOf(
        "is-when" to { args -> GValue.VBool(asNode(args[0]) is When) },
        "push-when-frame" to { args -> whenFrame(asNode(args[0]), args[1], args[2]) },
        "when-frame-is" to { args ->
            GValue.VBool((frameField(args[0], "kind") as? GValue.VString)?.value == "when")
        },
        "when-has-case" to { args -> GValue.VBool(frameWhen(args[0]).branches.isNotEmpty()) },
        "when-first-guard" to { args ->
            val guard =
                frameWhen(args[0]).branches.first().pattern
                    ?: raise(GuestError.UnassignedRegister(NO_POSITION))
            nodeWord(guard)
        },
        "when-first-body" to { args -> nodeWord(frameWhen(args[0]).branches.first().body) },
        "when-otherwise" to { args ->
            val expression = frameWhen(args[0])
            nodeWord(expression.otherwise ?: Literal("false", LiteralKind.BOOLEAN, expression.span))
        },
        "when-drop-case" to { args ->
            val frame = args[0]
            val expression = frameWhen(frame)
            whenFrame(
                expression.copy(branches = expression.branches.drop(1)),
                frameField(frame, "env"),
                frameField(frame, "rest"),
            )
        },
        "when-pop-frame" to { args -> frameField(args[0], "rest") },
        "when-frame-env" to { args -> frameField(args[0], "env") },
    )

private val whenLoop: List<Stmt> =
    listOf(
        Label("ev-when"),
        Assign("kont", OpSrc("push-when-frame", listOf(RegSrc("expr"), RegSrc("env"), RegSrc("kont")))),
        Label("when-next"),
        Test(OpCond("when-has-case", listOf(RegSrc("kont")))),
        Branch("when-guard"),
        Assign("expr", OpSrc("when-otherwise", listOf(RegSrc("kont")))),
        Assign("env", OpSrc("when-frame-env", listOf(RegSrc("kont")))),
        Assign("kont", OpSrc("when-pop-frame", listOf(RegSrc("kont")))),
        Goto(GotoTarget.Lbl("eval-dispatch")),
        Label("when-guard"),
        Assign("expr", OpSrc("when-first-guard", listOf(RegSrc("kont")))),
        Goto(GotoTarget.Lbl("eval-dispatch")),
        Label("fr-when"),
        Test(OpCond("is-true", listOf(RegSrc("val")))),
        Branch("when-body"),
        Assign("kont", OpSrc("when-drop-case", listOf(RegSrc("kont")))),
        Goto(GotoTarget.Lbl("when-next")),
        Label("when-body"),
        Assign("expr", OpSrc("when-first-body", listOf(RegSrc("kont")))),
        Assign("env", OpSrc("when-frame-env", listOf(RegSrc("kont")))),
        Assign("kont", OpSrc("when-pop-frame", listOf(RegSrc("kont")))),
        Goto(GotoTarget.Lbl("eval-dispatch")),
    )

/** The overlay controller: the shipped controller with the clause loop
 *  spliced in front of the catch-all `structural-eval` entry, the
 *  `fr-when` case registered in `continue-dispatch`, and the loop's own
 *  labels kept clear of the `halt` stop address. */
public fun whenOverlayController(): List<Stmt> {
    val base = ExplicitControl.controller()
    val structural = base.indexOfFirst { it is Label && it.name == "ev-structural" }
    if (structural < 0) error("the shipped controller has no ev-structural entry")
    val dispatchEnd =
        base.indexOfFirst { it is Goto && it.to == GotoTarget.Lbl("halt") }
    val haltIndex = base.indexOfFirst { it is Label && it.name == "halt" }
    if (dispatchEnd < 0 || haltIndex < 0) error("the shipped controller has no dispatch halt or halt label")
    val intercept =
        listOf(
            Test(OpCond("is-when", listOf(RegSrc("expr")))),
            Branch("ev-when"),
        )
    val registration =
        listOf(
            Test(OpCond("when-frame-is", listOf(RegSrc("kont")))),
            Branch("fr-when"),
        )
    return base.subList(0, structural) +
        intercept +
        base.subList(structural, dispatchEnd) +
        registration +
        base.subList(dispatchEnd, haltIndex) +
        whenLoop +
        base.subList(haltIndex, base.size)
}

private fun overlayMachine(
    checked: sicp.guest.CheckedProgram,
    sink: OutputSink,
): Machine =
    Machine(
        ExplicitControl.REGS,
        ExplicitControl.operations(checked, sink) + whenOverlayOperations(),
        whenOverlayController(),
    )

private fun runOverlay(source: String): Pair<OutputSink, Machine> {
    val checked = admitProgram(source)
    val sink = OutputSink()
    val machine = overlayMachine(checked, sink)
    val outcome = either<GuestError, Unit> { machine.run() }
    outcome.fold({ error -> error("the clause-loop machine faulted: ${error.category}") }, { })
    return sink to machine
}

private fun runBase(source: String): Machine {
    val checked = admitProgram(source)
    val machine = ExplicitControl.machine(checked)
    val outcome = either<GuestError, Unit> { machine.run() }
    outcome.fold({ error -> error("the base machine faulted: ${error.category}") }, { })
    return machine
}

/** The clause-loop session: the same guard `when` the derived-form
 *  exercise runs, answered by the basic-form loop. */
private val clauseLoopSession: String =
    """
    fun classify(n: Long): String {
        return when {
            n == 0L -> "zero"
            n == 1L -> "one"
            else -> "many"
        }
    }

    fun truth(): Boolean {
        return when {
            1L == 1L -> true
            else -> false
        }
    }

    fun falsity(): Boolean {
        return when {
            1L == 2L -> true
            else -> false
        }
    }

    fun main() {
        println(classify(0L))
        println(classify(1L))
        println(classify(7L))
        println(truth())
        println(falsity())
    }
    """.trimIndent()

/** The tail-recursion probe: the selected clause's recursive call must
 *  stay in tail position, so the depth does not grow with n. */
private fun tailSession(n: Int): String =
    """
    fun loop(count: Long): Long {
        return when {
            count == 0L -> 0L
            else -> loop(count - 1L)
        }
    }

    fun main() {
        println(loop(${n}L))
    }
    """.trimIndent()

/** The clause loop's session beside the two machines' cost: the answers
 *  the basic form gives, the instruction counts and stack depths of the
 *  overlay and the shipped controller on the same session, and the tail
 *  verdict of the selected clause. */
public fun condBasicFormRuns(): List<String> {
    val (sink, overlay) = runOverlay(clauseLoopSession)
    val base = runBase(clauseLoopSession)
    val answers = sink.contents().split("\n").filter { it.isNotEmpty() }
    val tails = (10..200 step 50).map { n -> runOverlay(tailSession(n)).second.stack.maxDepth }
    return answers +
        listOf(
            "overlay instructions = ${overlay.instructions}, base instructions = ${base.instructions}",
            "overlay stack depth = ${overlay.stack.maxDepth}, base stack depth = ${base.stack.maxDepth}",
            "selected clause tail depth independent of n: ${tails.distinct().size == 1}",
        )
}

/** The tail depths the exercise watches, one per probe size. */
public fun condTailPositionDepths(ns: List<Int>): List<Int> =
    ns.map { n ->
        runOverlay(tailSession(n))
            .second.stack.maxDepth
            .toInt()
    }
