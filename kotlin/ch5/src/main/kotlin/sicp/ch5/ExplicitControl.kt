// SPDX-License-Identifier: GPL-3.0-only
package sicp.ch5

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either
import sicp.guest.Admission
import sicp.guest.AdmissionError
import sicp.guest.Assignment
import sicp.guest.Binary
import sicp.guest.Block
import sicp.guest.Break
import sicp.guest.Call
import sicp.guest.CheckedProgram
import sicp.guest.Continue
import sicp.guest.DataClass
import sicp.guest.DataObject
import sicp.guest.Destructure
import sicp.guest.Elvis
import sicp.guest.Env
import sicp.guest.Expression
import sicp.guest.ExpressionStatement
import sicp.guest.For
import sicp.guest.FunctionDecl
import sicp.guest.GValue
import sicp.guest.GuestError
import sicp.guest.If
import sicp.guest.Index
import sicp.guest.Is
import sicp.guest.Lambda
import sicp.guest.Literal
import sicp.guest.LocalProperty
import sicp.guest.Member
import sicp.guest.Mode
import sicp.guest.NO_POSITION
import sicp.guest.Name
import sicp.guest.Node
import sicp.guest.OutputSink
import sicp.guest.PlainClass
import sicp.guest.Primitives
import sicp.guest.Property
import sicp.guest.Return
import sicp.guest.RunResult
import sicp.guest.SealedInterface
import sicp.guest.Statement
import sicp.guest.StringTemplate
import sicp.guest.This
import sicp.guest.TopProperty
import sicp.guest.TypeAlias
import sicp.guest.Unary
import sicp.guest.When
import sicp.guest.While
import sicp.guest.checkedLiteralValue
import sicp.guest.valueEquals
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Machine
import sicp.runtime.MachineOp
import sicp.runtime.OpCond
import sicp.runtime.Restore
import sicp.runtime.Save
import sicp.runtime.Source.LabelSrc
import sicp.runtime.Source.OpSrc
import sicp.runtime.Source.RegSrc
import sicp.runtime.Stmt
import sicp.runtime.Test

/**
 * The explicit-control evaluator of section 5.4, run by the register machine
 * of section 4.4 itself: `Machine.run()` is the execution path. The
 * controller dispatches on the checked syntax held in the `expr` register,
 * recursion is explicit continuation frames in `kont` plus the monitored
 * stack (`save proc` around argument evaluation, as the book's controller
 * shows), and the operations only destructure and combine — no hidden
 * evaluator. The named grammar forms dispatch through controller labels;
 * the remaining surface (member/index/when/for/templates and friends)
 * executes in the machine operation `structural-eval`, the operation layer
 * the book keeps for primitives.
 */
public object ExplicitControl {
    /** Runs an already-checked program on the explicit-control machine. */
    public fun run(checked: CheckedProgram): RunResult {
        val sink = OutputSink()
        val machine = assembled(checked, sink)
        val outcome: Either<GuestError, GValue> =
            either {
                machine.run()
                machine.registers["val"]?.content ?: GValue.VUnit
            }
        return outcome.fold(
            { error -> RunResult(sink.contents(), error, null) },
            { value -> RunResult(sink.contents(), null, value) },
        )
    }

    /** Admits and runs [source]; admission failures execute nothing. */
    public fun run(
        source: String,
        mode: Mode = Mode.CORE,
    ): Either<AdmissionError, RunResult> = either { run(Admission.admitOrRaise(source, mode)) }

    /** The machine a run drives, registers initialized at `start`, for the
     * monitoring exercises of 5.26-5.30. */
    public fun machine(checked: CheckedProgram): Machine = assembled(checked, OutputSink())

    private fun assembled(
        checked: CheckedProgram,
        sink: OutputSink,
    ): Machine {
        val evaluator = EceOps(checked, sink)
        val machine = Machine(REGS, evaluator.operations(), controller())
        evaluator.bind(machine)
        return machine
    }

    /** The register names of the explicit-control machine; overlay
     * controllers may extend this set when assembling their own [Machine]. */
    public val REGS: Set<String> = setOf("expr", "env", "val", "proc", "argl", "continue", "kont", "target")

    /** The operation table of the explicit-control evaluator, so an
     * exercise can assemble its own controller over the same operations and
     * add its own. */
    public fun operations(
        checked: CheckedProgram,
        sink: OutputSink = OutputSink(),
    ): Map<String, MachineOp> = EceOps(checked, sink).operations()

    /** The controller: eval-dispatch on the expression in `expr`, the
     * continuation dispatch over frames in `kont`, and `halt` reached with
     * `continue` as the book's final register shows. Labels are the
     * extension vocabulary: `eval-dispatch`, `ev-*` evaluation cases,
     * `continue-dispatch`, `fr-*` continuation cases, `halt`. */
    public fun controller(): List<Stmt> =
        listOf(
            Label("start"),
            Assign("expr", OpSrc("entry-expression", emptyList())),
            Assign("env", OpSrc("make-global-env", emptyList())),
            Assign("kont", OpSrc("empty-kont", emptyList())),
            Assign("continue", LabelSrc("halt")),
            Label("eval-dispatch"),
            Assign("val", OpSrc("node-kind", listOf(RegSrc("expr")))),
            Test(OpCond("kind-is", listOf(RegSrc("val"), literal("literal")))),
            Branch("ev-literal"),
            Test(OpCond("kind-is", listOf(RegSrc("val"), literal("name")))),
            Branch("ev-variable"),
            Test(OpCond("kind-is", listOf(RegSrc("val"), literal("lambda")))),
            Branch("ev-lambda"),
            Test(OpCond("kind-is", listOf(RegSrc("val"), literal("if")))),
            Branch("ev-if"),
            Test(OpCond("kind-is", listOf(RegSrc("val"), literal("call")))),
            Branch("ev-call"),
            Test(OpCond("kind-is", listOf(RegSrc("val"), literal("binary")))),
            Branch("ev-binary"),
            Test(OpCond("kind-is", listOf(RegSrc("val"), literal("unary")))),
            Branch("ev-unary"),
            Test(OpCond("kind-is", listOf(RegSrc("val"), literal("block")))),
            Branch("ev-block"),
            Test(OpCond("kind-is", listOf(RegSrc("val"), literal("return")))),
            Branch("ev-return"),
            Test(OpCond("kind-is", listOf(RegSrc("val"), literal("break")))),
            Branch("ev-break"),
            Test(OpCond("kind-is", listOf(RegSrc("val"), literal("continue")))),
            Branch("ev-continue"),
            Test(OpCond("kind-is", listOf(RegSrc("val"), literal("local")))),
            Branch("ev-local"),
            Test(OpCond("kind-is", listOf(RegSrc("val"), literal("assign")))),
            Branch("ev-assign"),
            Test(OpCond("kind-is", listOf(RegSrc("val"), literal("while")))),
            Branch("ev-while"),
            Test(OpCond("kind-is", listOf(RegSrc("val"), literal("exprstmt")))),
            Branch("ev-exprstmt"),
            Goto(GotoTarget.Lbl("ev-structural")),
            Label("ev-literal"),
            Assign("val", OpSrc("literal-value", listOf(RegSrc("expr")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("ev-variable"),
            Assign("val", OpSrc("lookup", listOf(RegSrc("expr"), RegSrc("env")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("ev-lambda"),
            Assign("val", OpSrc("make-closure", listOf(RegSrc("expr"), RegSrc("env")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("ev-if"),
            Assign("kont", OpSrc("push-frame", listOf(literal("if"), RegSrc("expr"), RegSrc("env"), RegSrc("kont")))),
            Assign("expr", OpSrc("if-condition", listOf(RegSrc("expr")))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("ev-binary"),
            Assign("kont", OpSrc("push-frame", listOf(literal("bin-left"), RegSrc("expr"), RegSrc("env"), RegSrc("kont")))),
            Assign("expr", OpSrc("left-operand", listOf(RegSrc("expr")))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("ev-unary"),
            Assign("kont", OpSrc("push-frame", listOf(literal("un"), RegSrc("expr"), RegSrc("env"), RegSrc("kont")))),
            Assign("expr", OpSrc("operand", listOf(RegSrc("expr")))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("ev-call"),
            Test(OpCond("is-simple-call", listOf(RegSrc("expr")))),
            Branch("ev-call-simple"),
            Goto(GotoTarget.Lbl("ev-structural")),
            Label("ev-call-simple"),
            Assign("kont", OpSrc("push-frame", listOf(literal("call-fn"), RegSrc("expr"), RegSrc("env"), RegSrc("kont")))),
            Assign("expr", OpSrc("callee", listOf(RegSrc("expr")))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("ev-block"),
            Test(OpCond("is-lambda-block", listOf(RegSrc("expr")))),
            Branch("ev-block-lambda"),
            Label("ev-block-body"),
            Assign("kont", OpSrc("push-seq", listOf(RegSrc("expr"), RegSrc("env"), RegSrc("kont")))),
            Assign("val", OpSrc("unit-value", emptyList())),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("ev-block-lambda"),
            Assign("val", OpSrc("make-block-closure", listOf(RegSrc("expr"), RegSrc("env")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("ev-return"),
            Test(OpCond("has-operand", listOf(RegSrc("expr")))),
            Branch("ev-return-value"),
            Assign("val", OpSrc("unit-value", emptyList())),
            Assign("val", OpSrc("make-marker", listOf(literal("return"), RegSrc("val")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("ev-return-value"),
            Test(OpCond("is-simple-tail-call", listOf(RegSrc("expr"), RegSrc("kont")))),
            Branch("ev-tail-call"),
            Assign("kont", OpSrc("push-frame", listOf(literal("ret"), RegSrc("expr"), RegSrc("env"), RegSrc("kont")))),
            Assign("expr", OpSrc("return-operand", listOf(RegSrc("expr")))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("ev-tail-call"),
            Assign("kont", OpSrc("push-frame", listOf(literal("tail"), RegSrc("expr"), RegSrc("env"), RegSrc("kont")))),
            Assign("expr", OpSrc("return-operand", listOf(RegSrc("expr")))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("ev-break"),
            Assign("val", OpSrc("make-marker", listOf(literal("break"), OpSrc("unit-value", emptyList())))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("ev-continue"),
            Assign("val", OpSrc("make-marker", listOf(literal("continue"), OpSrc("unit-value", emptyList())))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("ev-local"),
            Assign("kont", OpSrc("push-frame", listOf(literal("local"), RegSrc("expr"), RegSrc("env"), RegSrc("kont")))),
            Assign("expr", OpSrc("initializer", listOf(RegSrc("expr")))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("ev-assign"),
            Test(OpCond("is-name-target", listOf(RegSrc("expr")))),
            Branch("ev-assign-name"),
            Goto(GotoTarget.Lbl("ev-structural")),
            Label("ev-assign-name"),
            Assign("kont", OpSrc("push-frame", listOf(literal("assign"), RegSrc("expr"), RegSrc("env"), RegSrc("kont")))),
            Assign("expr", OpSrc("assign-value", listOf(RegSrc("expr")))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("ev-while"),
            Assign("val", OpSrc("unit-value", emptyList())),
            Assign("kont", OpSrc("push-frame", listOf(literal("while"), RegSrc("expr"), RegSrc("env"), RegSrc("kont")))),
            Assign("kont", OpSrc("while-set-phase-cond", listOf(RegSrc("kont")))),
            Assign("expr", OpSrc("while-condition", listOf(RegSrc("expr")))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("ev-exprstmt"),
            Assign("kont", OpSrc("push-frame", listOf(literal("exprstmt"), RegSrc("expr"), RegSrc("env"), RegSrc("kont")))),
            Assign("expr", OpSrc("stmt-expression", listOf(RegSrc("expr")))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("ev-structural"),
            Assign("val", OpSrc("structural-eval", listOf(RegSrc("expr"), RegSrc("env")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("continue-dispatch"),
            Test(OpCond("is-marker", listOf(RegSrc("val")))),
            Branch("fr-marker"),
            Test(OpCond("kont-empty", listOf(RegSrc("kont")))),
            Branch("halt"),
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), literal("if")))),
            Branch("fr-if"),
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), literal("bin-left")))),
            Branch("fr-bin-left"),
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), literal("bin-right")))),
            Branch("fr-bin-right"),
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), literal("un")))),
            Branch("fr-un"),
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), literal("call-fn")))),
            Branch("fr-call-fn"),
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), literal("call-args")))),
            Branch("fr-call-args"),
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), literal("call-arg")))),
            Branch("fr-call-arg"),
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), literal("return-unwrap")))),
            Branch("fr-unwrap"),
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), literal("seq")))),
            Branch("fr-seq"),
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), literal("ret")))),
            Branch("fr-ret"),
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), literal("local")))),
            Branch("fr-local"),
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), literal("assign")))),
            Branch("fr-assign"),
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), literal("exprstmt")))),
            Branch("fr-exprstmt"),
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), literal("while")))),
            Branch("fr-while"),
            Goto(GotoTarget.Lbl("halt")),
            Label("fr-marker"),
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), literal("return-unwrap")))),
            Branch("fr-unwrap"),
            Test(OpCond("is-return-marker", listOf(RegSrc("val")))),
            Branch("fr-marker-propagate"),
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), literal("while")))),
            Branch("fr-marker-while"),
            Label("fr-marker-propagate"),
            Test(OpCond("kont-empty", listOf(RegSrc("kont")))),
            Branch("fr-unclaimed"),
            Assign("kont", OpSrc("pop-frame", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("fr-marker")),
            Label("fr-marker-while"),
            Test(OpCond("is-break", listOf(RegSrc("val")))),
            Branch("fr-marker-break"),
            Assign("expr", OpSrc("while-cond-of", listOf(RegSrc("kont")))),
            Assign("kont", OpSrc("while-set-phase-cond", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("fr-marker-break"),
            Assign("val", OpSrc("unit-value", emptyList())),
            Assign("kont", OpSrc("pop-frame", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("fr-unclaimed"),
            Assign("val", OpSrc("error-value", emptyList())),
            Goto(GotoTarget.ByReg("continue")),
            Label("fr-unwrap"),
            Assign("val", OpSrc("unwrap-marker", listOf(RegSrc("val")))),
            Restore("env"),
            Assign("kont", OpSrc("pop-frame", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("fr-if"),
            Test(OpCond("is-true", listOf(RegSrc("val")))),
            Branch("fr-if-yes"),
            Assign("expr", OpSrc("frame-else", listOf(RegSrc("kont")))),
            Assign("env", OpSrc("frame-env", listOf(RegSrc("kont")))),
            Assign("kont", OpSrc("pop-frame", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("fr-if-yes"),
            Assign("expr", OpSrc("frame-then", listOf(RegSrc("kont")))),
            Assign("env", OpSrc("frame-env", listOf(RegSrc("kont")))),
            Assign("kont", OpSrc("pop-frame", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("fr-bin-left"),
            Assign("kont", OpSrc("frame-to-bin-right", listOf(RegSrc("kont"), RegSrc("val")))),
            Assign("env", OpSrc("frame-env", listOf(RegSrc("kont")))),
            Assign("expr", OpSrc("frame-right", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("fr-bin-right"),
            Assign("val", OpSrc("combine-binary", listOf(RegSrc("kont"), RegSrc("val")))),
            Assign("kont", OpSrc("pop-frame", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("fr-un"),
            Assign("val", OpSrc("combine-unary", listOf(RegSrc("kont"), RegSrc("val")))),
            Assign("kont", OpSrc("pop-frame", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("fr-call-fn"),
            Assign("proc", OpSrc("ensure-proc", listOf(RegSrc("val")))),
            Assign("env", OpSrc("frame-env", listOf(RegSrc("kont")))),
            Assign("kont", OpSrc("frame-to-call-args", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("fr-call-args")),
            Label("fr-call-args"),
            Assign("env", OpSrc("frame-env", listOf(RegSrc("kont")))),
            Test(OpCond("frame-args-empty", listOf(RegSrc("kont")))),
            Branch("fr-apply"),
            Save("proc"),
            Assign("expr", OpSrc("first-arg", listOf(RegSrc("kont")))),
            Assign("kont", OpSrc("mark-arg-scheduled", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("fr-call-arg"),
            Restore("proc"),
            Assign("kont", OpSrc("accumulate-arg", listOf(RegSrc("kont"), RegSrc("val")))),
            Goto(GotoTarget.Lbl("fr-call-args")),
            Label("fr-apply"),
            Assign("argl", OpSrc("frame-argl", listOf(RegSrc("kont")))),
            Assign("kont", OpSrc("pop-frame", listOf(RegSrc("kont")))),
            Test(OpCond("frame-kind-is", listOf(RegSrc("kont"), literal("tail")))),
            Branch("fr-tail-apply"),
            Assign("kont", OpSrc("push-unwrap", listOf(RegSrc("kont")))),
            Save("env"),
            Test(OpCond("is-primitive", listOf(RegSrc("proc")))),
            Branch("fr-apply-primitive"),
            Goto(GotoTarget.Lbl("fr-apply-compound")),
            Label("fr-tail-apply"),
            Assign("kont", OpSrc("tail-target", listOf(RegSrc("kont")))),
            Test(OpCond("is-primitive", listOf(RegSrc("proc")))),
            Branch("fr-apply-primitive"),
            Goto(GotoTarget.Lbl("fr-apply-compound")),
            Label("fr-apply-primitive"),
            Assign("val", OpSrc("apply-primitive", listOf(RegSrc("proc"), RegSrc("argl")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("fr-apply-compound"),
            Assign("env", OpSrc("extend-environment", listOf(RegSrc("proc"), RegSrc("argl")))),
            Assign("expr", OpSrc("procedure-body", listOf(RegSrc("proc")))),
            Test(OpCond("is-block", listOf(RegSrc("expr")))),
            Branch("ev-block-body"),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("fr-seq"),
            Assign("env", OpSrc("frame-env", listOf(RegSrc("kont")))),
            Test(OpCond("frame-seq-empty", listOf(RegSrc("kont")))),
            Branch("fr-seq-done"),
            Assign("expr", OpSrc("frame-seq-next", listOf(RegSrc("kont")))),
            Assign("kont", OpSrc("frame-seq-advance", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("fr-seq-done"),
            Assign("kont", OpSrc("pop-frame", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("fr-ret"),
            Assign("val", OpSrc("make-marker", listOf(literal("return"), RegSrc("val")))),
            Assign("kont", OpSrc("pop-frame", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("fr-local"),
            Assign("env", OpSrc("declare-local", listOf(RegSrc("kont"), RegSrc("val")))),
            Assign("val", OpSrc("unit-value", emptyList())),
            Assign("kont", OpSrc("pop-frame", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("fr-assign"),
            Assign("val", OpSrc("do-assign", listOf(RegSrc("kont"), RegSrc("val")))),
            Assign("kont", OpSrc("pop-frame", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("fr-exprstmt"),
            Assign("kont", OpSrc("pop-frame", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("fr-while"),
            Assign("env", OpSrc("frame-env", listOf(RegSrc("kont")))),
            Test(OpCond("while-phase-cond", listOf(RegSrc("kont")))),
            Branch("fr-while-after-cond"),
            Test(OpCond("is-break", listOf(RegSrc("val")))),
            Branch("fr-while-end"),
            Assign("expr", OpSrc("while-cond-of", listOf(RegSrc("kont")))),
            Assign("kont", OpSrc("while-set-phase-cond", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("fr-while-after-cond"),
            Test(OpCond("is-true", listOf(RegSrc("val")))),
            Branch("fr-while-start-body"),
            Label("fr-while-end"),
            Assign("val", OpSrc("unit-value", emptyList())),
            Assign("kont", OpSrc("pop-frame", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("fr-while-start-body"),
            Assign("kont", OpSrc("while-start-body", listOf(RegSrc("kont")))),
            Goto(GotoTarget.Lbl("continue-dispatch")),
            Label("halt"),
        )

    private fun literal(text: String): OpSrc = OpSrc("literal", listOf(sicp.runtime.Source.ConstSrc(GValue.VString(text))))
}

/** The operation layer: destructure and combine over values in registers.
 * Nothing here recurses into evaluation except the documented
 * `structural-eval` operation for the grammar forms outside the controller's
 * named dispatch. */
internal class EceOps(
    private val checked: CheckedProgram,
    private val sink: OutputSink,
) {
    private val globals = Env.root()
    private val classMethods: Map<String, Map<String, FunctionDecl>> =
        checked.syntax.declarations
            .filterIsInstance<PlainClass>()
            .associate { it.name to it.methods.associateBy(FunctionDecl::name) }
    private val collectionCallbacks = setOf("map", "filter", "fold", "any", "all")
    private var bound: Machine? = null

    fun bind(machine: Machine) {
        bound = machine
    }

    fun operations(): Map<String, MachineOp> {
        val ops = linkedMapOf<String, MachineOp>()
        ops["literal"] = { args -> args[0] }
        ops["entry-expression"] = { _ -> GValue.VNode(Call(Name("main", NO_POSITION), emptyList(), emptyList(), NO_POSITION)) }
        ops["make-global-env"] = { _ -> GValue.VEnvVal(install(globals)) }
        ops["node-kind"] = { args -> GValue.VString(kindOf(asNode(args[0]))) }
        ops["kind-is"] = { args -> GValue.VBool((args[0] as GValue.VString).value == (args[1] as GValue.VString).value) }
        ops["literal-value"] = { args ->
            val literal = asNode(args[0]) as Literal
            checkedLiteralValue(literal, checked.types[literal])
        }
        ops["lookup"] = { args -> lookup(asNode(args[0]) as Name, asEnv(args[1])) }
        ops["make-closure"] = { args -> closure(asNode(args[0]) as Lambda, asEnv(args[1])) }
        ops["is-lambda-block"] = { args -> GValue.VBool(asNode(args[0]) in checked.lambdaCoercions) }
        ops["is-block"] = { args -> GValue.VBool(asNode(args[0]) is Block) }
        ops["make-block-closure"] = { args -> blockClosure(asNode(args[0]) as Block, asEnv(args[1])) }
        ops["empty-kont"] = { _ -> GValue.VNull }
        ops["kont-empty"] = { args -> GValue.VBool(args[0] is GValue.VNull) }
        ops["push-frame"] = { args -> pushFrame(args) }
        ops["push-seq"] = { args -> pushSeq(asNode(args[0]) as Block, asEnv(args[1]), args[2]) }
        ops["push-unwrap"] = { args -> frame(listOf("kind" to GValue.VString("return-unwrap"), "rest" to args[0])) }
        ops["pop-frame"] = { args -> frameRest(args[0]) }
        ops["frame-kind-is"] = { args ->
            GValue.VBool(args[0] is GValue.VObject && frameKind(args[0]) == (args[1] as GValue.VString).value)
        }
        ops["frame-env"] = { args -> GValue.VEnvVal(frameEnv(args[0])) }
        ops["frame-then"] = { args -> GValue.VNode((frameNode(args[0]) as If).yes) }
        ops["frame-else"] = { args -> GValue.VNode(orUnit((frameNode(args[0]) as If).no)) }
        ops["if-condition"] = { args -> GValue.VNode((asNode(args[0]) as If).condition) }
        ops["left-operand"] = { args -> GValue.VNode((asNode(args[0]) as Binary).left) }
        ops["operand"] = { args -> GValue.VNode((asNode(args[0]) as Unary).operand) }
        ops["frame-right"] = { args -> GValue.VNode((frameNode(args[0]) as Binary).right) }
        ops["frame-to-bin-right"] = { args -> retagFrame(args[0], "bin-right", args[1]) }
        ops["combine-binary"] = { args -> combineBinary(args[0], args[1]) }
        ops["combine-unary"] = { args -> Primitives.unary((frameNode(args[0]) as Unary).operator, args[1], frameNode(args[0]).span) }
        ops["callee"] = { args -> GValue.VNode((asNode(args[0]) as Call).callee) }
        ops["is-simple-call"] = { args ->
            val callee = (asNode(args[0]) as Call).callee
            GValue.VBool(callee is Name || callee is Lambda || callee is Call)
        }
        ops["is-simple-tail-call"] = { args -> GValue.VBool(isSimpleTailCall(asNode(args[0]) as Return, args[1])) }
        ops["tail-target"] = { args -> tailTarget(args[0]) }
        ops["ensure-proc"] = { args -> ensureProc(args[0]) }
        ops["frame-to-call-args"] = { args -> frameToCallArgs(args[0]) }
        ops["frame-args-empty"] = { args -> GValue.VBool(frameValues(args[0]).isEmpty()) }
        ops["first-arg"] = { args -> GValue.VNode((frameValues(args[0]).first() as GValue.VNode).node) }
        ops["mark-arg-scheduled"] = { args -> retagFrame(args[0], "call-arg", frameValue(args[0])) }
        ops["accumulate-arg"] = { args -> accumulateArg(args[0], args[1]) }
        ops["frame-argl"] = { args -> frameArgl(args[0]) }
        ops["is-primitive"] = { args -> GValue.VBool(args[0] is GValue.VFunction) }
        ops["apply-primitive"] = { args -> applyPrimitive(args[0], args[1]) }
        ops["extend-environment"] = { args -> GValue.VEnvVal(extendEnv(args[0], args[1])) }
        ops["procedure-body"] = { args -> GValue.VNode(procBody(args[0])) }
        ops["frame-seq-empty"] = { args -> GValue.VBool(frameValues(args[0]).isEmpty()) }
        ops["frame-seq-next"] = { args -> GValue.VNode((frameValues(args[0]).first() as GValue.VNode).node) }
        ops["frame-seq-advance"] = { args -> advanceSeq(args[0]) }
        ops["has-operand"] = { args -> GValue.VBool((asNode(args[0]) as Return).value != null) }
        ops["return-operand"] = { args -> GValue.VNode(orUnit((asNode(args[0]) as Return).value)) }
        ops["initializer"] = { args -> GValue.VNode((asNode(args[0]) as LocalProperty).initializer) }
        ops["is-name-target"] = { args -> GValue.VBool((asNode(args[0]) as Assignment).target is Name) }
        ops["assign-value"] = { args -> GValue.VNode((asNode(args[0]) as Assignment).value) }
        ops["while-condition"] = { args -> GValue.VNode((asNode(args[0]) as While).condition) }
        ops["while-cond-of"] = { args -> GValue.VNode((frameNode(args[0]) as While).condition) }
        ops["while-phase-cond"] = { args -> GValue.VBool(frameOp(args[0]) == "cond") }
        ops["while-set-phase-cond"] = { args -> retagFrame(args[0], "while", frameValue(args[0]), "cond") }
        ops["while-start-body"] = { args -> whileStartBody(args[0]) }
        ops["stmt-expression"] = { args -> GValue.VNode((asNode(args[0]) as ExpressionStatement).expression) }
        ops["unit-value"] = { _ -> GValue.VUnit }
        ops["make-marker"] = { args -> GValue.VObject("marker", false, mutableMapOf("kind" to args[0], "value" to args[1])) }
        ops["is-marker"] = { args ->
            val marker = args[0]
            GValue.VBool(marker is GValue.VObject && marker.className == "marker")
        }
        ops["is-break"] = { args -> GValue.VBool(markerKind(args[0]) == "break") }
        ops["is-return-marker"] = { args -> GValue.VBool(markerKind(args[0]) == "return") }
        ops["unwrap-marker"] = { args ->
            val value = args[0]
            if (value !is GValue.VObject || value.className != "marker") {
                value
            } else if (markerKind(value) == "return") {
                value.fields["value"] ?: GValue.VUnit
            } else {
                raise(GuestError.UnassignedRead(NO_POSITION))
            }
        }
        ops["error-value"] = { _ -> raise(GuestError.UnassignedRead(NO_POSITION)) }
        ops["is-true"] = { args -> GValue.VBool(Primitives.truth(args[0], NO_POSITION)) }
        ops["structural-eval"] = { args -> structuralEval(asNode(args[0]), asEnv(args[1])) }
        ops["declare-local"] = { args -> declareLocal(args[0], args[1]) }
        ops["do-assign"] = { args -> doAssign(args[0], args[1]) }
        return ops
    }

    private fun kind(kind: String): GValue = GValue.VString(kind)

    private fun asNode(value: GValue): Node = (value as GValue.VNode).node

    private fun asEnv(value: GValue): Env = (value as GValue.VEnvVal).env

    private fun orUnit(expression: Expression?): Node = expression ?: Block(emptyList(), NO_POSITION)

    private fun kindOf(node: Node): String =
        when (node) {
            is Literal -> "literal"
            is Name -> "name"
            is Lambda -> "lambda"
            is If -> "if"
            is Call -> "call"
            is Binary -> "binary"
            is Unary -> "unary"
            is Block -> "block"
            is Return -> "return"
            is Break -> "break"
            is Continue -> "continue"
            is LocalProperty -> "local"
            is Assignment -> "assign"
            is While -> "while"
            is ExpressionStatement -> "exprstmt"
            else -> "structural"
        }

    context(r: Raise<GuestError>)
    private fun lookup(
        name: Name,
        env: Env,
    ): GValue {
        val cell =
            env.lookup(name.text) ?: globals.lookup(name.text)
                ?: return GValue.VFunction(name.text, 0) { args -> Primitives.call(name.text, args, sink, name.span) }
        val value = cell.value
        if (value is GValue.VUnassigned) r.raise(GuestError.UnassignedRead(name.span))
        return value
    }

    private fun closure(
        lambda: Lambda,
        env: Env,
    ): GValue = makeClosure(lambda.parameters.map { it.name }, lambda.body, env)

    private fun blockClosure(
        block: Block,
        env: Env,
    ): GValue = makeClosure(emptyList(), block, env)

    private fun makeClosure(
        names: List<String>,
        body: Block,
        env: Env,
    ): GValue =
        GValue.VObject(
            "compound-procedure",
            false,
            mutableMapOf(
                "params" to GValue.VList(names.map { GValue.VString(it) }.toMutableList(), false),
                "body" to GValue.VNode(body),
                "env" to GValue.VEnvVal(env),
            ),
        )

    private fun frame(fields: List<Pair<String, GValue>>): GValue {
        val defaults =
            linkedMapOf(
                "kind" to GValue.VUnit,
                "node" to GValue.VUnit,
                "env" to GValue.VUnit,
                "value" to GValue.VUnit,
                "values" to GValue.VList(mutableListOf(), false),
                "argl" to GValue.VList(mutableListOf(), false),
                "op" to GValue.VString(""),
                "rest" to GValue.VNull,
            )
        defaults.putAll(fields)
        return GValue.VObject("frame", false, mutableMapOf(*defaults.entries.map { it.key to it.value }.toTypedArray()))
    }

    private fun pushFrame(args: List<GValue>): GValue =
        frame(
            listOf(
                "kind" to args[0],
                "node" to args[1],
                "env" to args[2],
                "value" to GValue.VUnit,
                "values" to GValue.VList(mutableListOf(), false),
                "argl" to GValue.VList(mutableListOf(), false),
                "op" to GValue.VString(""),
                "rest" to args[3],
            ),
        )

    private fun pushSeq(
        block: Block,
        env: Env,
        kont: GValue,
    ): GValue =
        frame(
            listOf(
                "kind" to GValue.VString("seq"),
                "node" to GValue.VNode(block),
                "env" to GValue.VEnvVal(env),
                "value" to GValue.VUnit,
                "values" to GValue.VList(block.statements.map { GValue.VNode(it) }.toMutableList(), false),
                "argl" to GValue.VList(mutableListOf(), false),
                "op" to GValue.VString(""),
                "rest" to kont,
            ),
        )

    private fun fields(of: GValue): Map<String, GValue> = (of as GValue.VObject).fields

    private fun frameKind(kont: GValue): String = (fields(kont)["kind"] as GValue.VString).value

    private fun frameNode(kont: GValue): Node {
        val node = fields(kont)["node"] ?: GValue.VUnit
        return (node as GValue.VNode).node
    }

    private fun frameEnv(kont: GValue): Env = (fields(kont)["env"] as GValue.VEnvVal).env

    private fun frameValue(kont: GValue): GValue = fields(kont)["value"] ?: GValue.VUnit

    private fun frameValues(kont: GValue): List<GValue> = (fields(kont)["values"] as GValue.VList).items

    private fun frameArgl(kont: GValue): GValue = fields(kont)["argl"] ?: GValue.VList(mutableListOf(), false)

    private fun frameOp(kont: GValue): String = ((fields(kont)["op"] as GValue.VString)).value

    private fun frameRest(kont: GValue): GValue = fields(kont)["rest"] ?: GValue.VNull

    private fun isSimpleTailCall(
        statement: Return,
        kont: GValue,
    ): Boolean {
        val callee = (statement.value as? Call)?.callee ?: return false
        if (callee !is Name && callee !is Lambda && callee !is Call) return false
        var frame = kont
        while (frame is GValue.VObject && frame.className == "frame") {
            when (frameKind(frame)) {
                "call-arg", "seq-restore", "compound-return" -> return false
                "return-unwrap" -> return true
            }
            frame = frameRest(frame)
        }
        return false
    }

    context(r: Raise<GuestError>)
    private fun tailTarget(kont: GValue): GValue {
        var frame = kont
        while (frame is GValue.VObject && frame.className == "frame") {
            if (frameKind(frame) == "return-unwrap") return frame
            frame = frameRest(frame)
        }
        return r.raise(GuestError.ShapeFault(NO_POSITION))
    }

    private fun retagFrame(
        kont: GValue,
        kind: String,
        value: GValue,
        op: String = "",
    ): GValue {
        val copy = LinkedHashMap(fields(kont))
        copy["kind"] = GValue.VString(kind)
        copy["value"] = value
        copy["op"] = GValue.VString(op)
        return GValue.VObject("frame", false, mutableMapOf(*copy.entries.map { it.key to it.value }.toTypedArray()))
    }

    private fun frameToCallArgs(kont: GValue): GValue {
        val node = frameNode(kont) as Call
        val copy = LinkedHashMap(fields(kont))
        copy["kind"] = GValue.VString("call-args")
        copy["values"] = GValue.VList(node.arguments.map { GValue.VNode(it.value) }.toMutableList(), false)
        copy["argl"] = GValue.VList(mutableListOf(), false)
        return GValue.VObject("frame", false, mutableMapOf(*copy.entries.map { it.key to it.value }.toTypedArray()))
    }

    private fun accumulateArg(
        kont: GValue,
        value: GValue,
    ): GValue {
        val copy = LinkedHashMap(fields(kont))
        copy["kind"] = GValue.VString("call-args")
        val argl = (fields(kont)["argl"] as GValue.VList).items + value
        copy["argl"] = GValue.VList(argl.toMutableList(), false)
        val rest = frameValues(kont).drop(1)
        copy["values"] = GValue.VList(rest.toMutableList(), false)
        return GValue.VObject("frame", false, mutableMapOf(*copy.entries.map { it.key to it.value }.toTypedArray()))
    }

    private fun advanceSeq(kont: GValue): GValue {
        val copy = LinkedHashMap(fields(kont))
        copy["values"] = GValue.VList(frameValues(kont).drop(1).toMutableList(), false)
        return GValue.VObject("frame", false, mutableMapOf(*copy.entries.map { it.key to it.value }.toTypedArray()))
    }

    private fun whileStartBody(kont: GValue): GValue {
        val node = frameNode(kont) as While
        val loop = retagFrame(kont, "while", frameValue(kont), "body")
        val statements = node.body.statements.map { GValue.VNode(it) }
        return frame(
            listOf(
                "kind" to GValue.VString("seq"),
                "node" to GValue.VNode(node.body),
                "env" to GValue.VEnvVal(frameEnv(kont)),
                "value" to GValue.VUnit,
                "values" to GValue.VList(statements.toMutableList(), false),
                "argl" to GValue.VList(mutableListOf(), false),
                "op" to GValue.VString(""),
                "rest" to loop,
            ),
        )
    }

    context(r: Raise<GuestError>)
    private fun ensureProc(value: GValue): GValue {
        if (value is GValue.VFunction) return value
        if (value is GValue.VObject && value.className == "compound-procedure") return value
        return r.raise(GuestError.UnassignedRead(NO_POSITION))
    }

    private fun procBody(proc: GValue): Node = ((proc as GValue.VObject).fields["body"] as GValue.VNode).node

    private fun extendEnv(
        proc: GValue,
        argl: GValue,
    ): Env {
        val fields = (proc as GValue.VObject).fields
        val params = (fields["params"] as GValue.VList).items.map { (it as GValue.VString).value }
        val env = Env.child((fields["env"] as GValue.VEnvVal).env)
        val values = (argl as GValue.VList).items
        for ((index, name) in params.withIndex()) env.define(name, values.getOrElse(index) { GValue.VUnit })
        return env
    }

    context(r: Raise<GuestError>)
    private fun applyPrimitive(
        proc: GValue,
        argl: GValue,
    ): GValue = Primitives.invoke(proc, (argl as GValue.VList).items, NO_POSITION)

    private fun markerKind(value: GValue): String {
        if (value !is GValue.VObject || value.className != "marker") return ""
        return ((value.fields["kind"] as GValue.VString)).value
    }

    context(r: Raise<GuestError>)
    private fun combineBinary(
        kont: GValue,
        right: GValue,
    ): GValue {
        val node = frameNode(kont) as Binary
        val left = frameValue(kont)
        return Primitives.binary(node.operator, left, right, node.span)
    }

    private fun declareLocal(
        kont: GValue,
        value: GValue,
    ): GValue {
        val node = frameNode(kont) as LocalProperty
        val env = frameEnv(kont)
        env.define(node.name, value)
        return GValue.VEnvVal(env)
    }

    private fun doAssign(
        kont: GValue,
        value: GValue,
    ): GValue {
        val node = frameNode(kont) as Assignment
        val target = node.target as Name
        val env = frameEnv(kont)
        val cell = env.lookup(target.text) ?: globals.lookup(target.text) ?: return GValue.VUnit
        cell.value = value
        return value
    }

    /** The operation layer for the grammar forms outside the controller's
     * named dispatch: member/index/when/for/templates, object construction,
     * destructuring, and non-name assignment targets. Independent of the
     * direct evaluator. */
    context(r: Raise<GuestError>)
    private fun structuralEval(
        node: Node,
        env: Env,
    ): GValue =
        when (node) {
            is Member -> {
                val receiver = evalExpr(node.receiver, env)
                val method = (receiver as? GValue.VObject)?.let { classMethods[it.className]?.get(node.name) }
                when {
                    node.safe && receiver is GValue.VNull -> {
                        GValue.VNull
                    }

                    receiver is GValue.VObject && receiver.fields.containsKey(node.name) -> {
                        receiver.fields[node.name] ?: r.raise(GuestError.UnassignedRead(node.span))
                    }

                    receiver is GValue.VObject && method != null -> {
                        boundMethod(receiver, method)
                    }

                    else -> {
                        Primitives.property(receiver, node.name, node.span)
                    }
                }
            }

            is Index -> {
                Primitives.readIndex(evalExpr(node.receiver, env), evalExpr(node.index, env), node.span)
            }

            is When -> {
                evalWhen(node, env)
            }

            is StringTemplate -> {
                evalTemplate(node, env)
            }

            is Elvis -> {
                evalExpr(node.left, env).takeIf { it !is GValue.VNull } ?: evalExpr(node.right, env)
            }

            is Is -> {
                GValue.VBool(Primitives.isTypeValue(evalExpr(node.value, env), node.type) != node.negated)
            }

            is This -> {
                lookup(Name("this", node.span), env)
            }

            is sicp.guest.CallableReference -> {
                lookup(Name(node.name, node.span), env)
            }

            is For -> {
                evalFor(node, env)
            }

            is Destructure -> {
                evalDestructure(node, env)
            }

            is FunctionDecl -> {
                env.define(node.name, closureOf(node, env))
                GValue.VUnit
            }

            is Assignment -> {
                evalAssignment(node, env)
            }

            is While -> {
                evalWhile(node, env)
            }

            is Break -> {
                throw Marker("break", GValue.VUnit)
            }

            is Continue -> {
                throw Marker("continue", GValue.VUnit)
            }

            is LocalProperty -> {
                env.define(node.name, evalExpr(node.initializer, env))
                GValue.VUnit
            }

            is Return -> {
                throw Marker("return", node.value?.let { evalExpr(it, env) } ?: GValue.VUnit)
            }

            is Block -> {
                evalExpr(node, env)
            }

            is Expression -> {
                evalExpr(node, env)
            }

            else -> {
                GValue.VUnit
            }
        }

    context(r: Raise<GuestError>)
    private fun evalExpr(
        node: Expression,
        env: Env,
    ): GValue =
        when (node) {
            is Literal -> {
                checkedLiteralValue(node, checked.types[node])
            }

            is Name -> {
                lookup(node, env)
            }

            is Lambda -> {
                closure(node, env)
            }

            is Binary -> {
                val left = evalExpr(node.left, env)
                if (node.operator ==
                    "&&"
                ) {
                    return GValue.VBool(Primitives.truth(left, node.span) && Primitives.truth(evalExpr(node.right, env), node.span))
                }
                if (node.operator ==
                    "||"
                ) {
                    return GValue.VBool(Primitives.truth(left, node.span) || Primitives.truth(evalExpr(node.right, env), node.span))
                }
                Primitives.binary(node.operator, left, evalExpr(node.right, env), node.span)
            }

            is Unary -> {
                Primitives.unary(node.operator, evalExpr(node.operand, env), node.span)
            }

            is If -> {
                if (Primitives.truth(evalExpr(node.condition, env), node.span)) {
                    evalExpr(node.yes, env)
                } else {
                    node.no?.let { evalExpr(it, env) }
                        ?: GValue.VUnit
                }
            }

            is Call -> {
                evalCall(node, env)
            }

            is Block -> {
                if (node in checked.lambdaCoercions) blockClosure(node, env) else evalBlock(node, env)
            }

            is Return -> {
                throw Marker("return", node.value?.let { evalExpr(it, env) } ?: GValue.VUnit)
            }

            else -> {
                structuralEval(node, env)
            }
        }

    context(r: Raise<GuestError>)
    private fun evalBlock(
        block: Block,
        env: Env,
    ): GValue {
        val frame = Env.child(env)
        var result: GValue = GValue.VUnit
        for (statement in block.statements) result = evalStatement(statement, frame)
        return result
    }

    context(r: Raise<GuestError>)
    private fun evalStatement(
        statement: Statement,
        env: Env,
    ): GValue =
        when (statement) {
            is LocalProperty -> {
                env.define(statement.name, evalExpr(statement.initializer, env))
                GValue.VUnit
            }

            is ExpressionStatement -> {
                evalExpr(statement.expression, env)
            }

            is Assignment -> {
                evalAssignment(statement, env)
            }

            is While -> {
                evalWhile(statement, env)
            }

            is Return -> {
                throw Marker("return", statement.value?.let { evalExpr(it, env) } ?: GValue.VUnit)
            }

            is Break -> {
                throw Marker("break", GValue.VUnit)
            }

            is Continue -> {
                throw Marker("continue", GValue.VUnit)
            }

            else -> {
                structuralEval(statement, env)
            }
        }

    context(r: Raise<GuestError>)
    private fun evalAssignment(
        node: Assignment,
        env: Env,
    ): GValue {
        val value = evalExpr(node.value, env)
        val target = node.target
        if (target is Name) {
            val cell = env.lookup(target.text) ?: globals.lookup(target.text) ?: r.raise(GuestError.UnassignedRead(target.span))
            cell.value = value
            return value
        }
        if (target is Member) {
            val receiver = evalExpr(target.receiver, env) as? GValue.VObject ?: r.raise(GuestError.UnassignedRead(target.span))
            receiver.fields[target.name] = value
            return value
        }
        if (target is Index) {
            Primitives.writeIndex(evalExpr(target.receiver, env), evalExpr(target.index, env), value, node.span)
            return value
        }
        return r.raise(GuestError.UnassignedRead(node.span))
    }

    context(r: Raise<GuestError>)
    private fun evalWhile(
        node: While,
        env: Env,
    ): GValue {
        while (Primitives.truth(evalExpr(node.condition, env), node.condition.span)) {
            try {
                evalBlock(node.body, env)
            } catch (marker: Marker) {
                if (marker.kind == "break") return GValue.VUnit
                if (marker.kind == "return") throw marker
            }
        }
        return GValue.VUnit
    }

    context(r: Raise<GuestError>)
    private fun evalWhen(
        node: When,
        env: Env,
    ): GValue {
        val subject = node.subject?.let { evalExpr(it, env) }
        for (branch in node.branches) {
            if (branchMatches(branch, subject, env)) return evalExpr(branch.body, env)
        }
        return node.otherwise?.let { evalExpr(it, env) } ?: GValue.VUnit
    }

    context(r: Raise<GuestError>)
    private fun branchMatches(
        branch: sicp.guest.WhenBranch,
        subject: GValue?,
        env: Env,
    ): Boolean {
        val typePattern = branch.typePattern
        if (typePattern != null) return subject != null && Primitives.isTypeValue(subject, typePattern)
        val pattern = branch.pattern ?: return false
        if (subject == null) return Primitives.truth(evalExpr(pattern, env), pattern.span)
        return valueEquals(evalExpr(pattern, env), subject)
    }

    context(r: Raise<GuestError>)
    private fun evalTemplate(
        node: StringTemplate,
        env: Env,
    ): GValue {
        val out = StringBuilder()
        for (fragment in node.fragments) {
            out.append(sicp.guest.renderPrinted(evalExpr(fragment, env)) ?: r.raise(GuestError.UnassignedRead(node.span)))
        }
        return GValue.VString(out.toString())
    }

    context(r: Raise<GuestError>)
    private fun evalFor(
        node: For,
        env: Env,
    ): GValue {
        val items = forItems(node, env)
        for (item in items) {
            val frame = Env.child(env)
            frame.define(node.name, item)
            try {
                evalBlock(node.body, frame)
            } catch (marker: Marker) {
                if (marker.kind == "break") return GValue.VUnit
                if (marker.kind == "return") throw marker
            }
        }
        return GValue.VUnit
    }

    context(r: Raise<GuestError>)
    private fun forItems(
        node: For,
        env: Env,
    ): Iterable<GValue> {
        val endExpression = node.end
        if (endExpression != null) {
            val start = evalExpr(node.iterable, env)
            val end = evalExpr(endExpression, env)
            return Primitives.rangeValues(start, end, node.span).asIterable()
        }
        return when (val source = evalExpr(node.iterable, env)) {
            is GValue.VList -> source.items
            is GValue.VLazyList -> (sicp.guest.materialize(source) as GValue.VList).items
            else -> r.raise(GuestError.UnassignedRead(node.span))
        }
    }

    context(r: Raise<GuestError>)
    private fun evalDestructure(
        node: Destructure,
        env: Env,
    ): GValue {
        val source = evalExpr(node.initializer, env)
        val values =
            when (source) {
                is GValue.VPair -> listOf(source.first, source.second)
                is GValue.VObject -> source.fields.values.toList()
                else -> r.raise(GuestError.UnassignedRead(node.span))
            }
        for ((name, value) in node.names.zip(values)) env.define(name, value)
        return GValue.VUnit
    }

    context(r: Raise<GuestError>)
    private fun evalCall(
        node: Call,
        env: Env,
    ): GValue {
        val callee = node.callee
        if (callee is Member) {
            val receiver = evalExpr(callee.receiver, env)
            if (callee.safe && receiver is GValue.VNull) return GValue.VNull
            if (callee.name == "copy" && receiver is GValue.VObject) return copyObject(receiver, node, env)
            val method = (receiver as? GValue.VObject)?.let { classMethods[it.className]?.get(callee.name) }
            if (receiver is GValue.VObject && method != null) {
                return applyValue(boundMethod(receiver, method), node.arguments.map { evalExpr(it.value, env) }, node.span)
            }
            val arguments = node.arguments.map { evalExpr(it.value, env) }
            if (receiver is GValue.VList && callee.name in collectionCallbacks) {
                return collectionCall(receiver, callee.name, arguments, node.span)
            }
            return Primitives.member(receiver, callee.name, arguments, node.span)
        }
        val arguments = node.arguments.map { evalExpr(it.value, env) }
        if (callee is Name) {
            val bound = env.lookup(callee.text) ?: globals.lookup(callee.text)
            if (bound != null && bound.value !is GValue.VUnassigned) return applyValue(bound.value, arguments, node.span)
            return Primitives.call(callee.text, arguments, sink, node.span)
        }
        return applyValue(evalExpr(callee, env), arguments, node.span)
    }

    context(r: Raise<GuestError>)
    private fun collectionCall(
        receiver: GValue.VList,
        name: String,
        arguments: List<GValue>,
        at: sicp.guest.Span,
    ): GValue {
        val callback = arguments.lastOrNull() ?: r.raise(GuestError.ShapeFault(at))
        val one = mutableListOf<GValue>(GValue.VUnit)
        return when (name) {
            "map" -> {
                val output = ArrayList<GValue>(receiver.items.size)
                for (item in receiver.items) {
                    one[0] = item
                    output.add(applyValue(callback, one, at))
                }
                GValue.VList(output, false)
            }

            "filter" -> {
                val output = ArrayList<GValue>()
                for (item in receiver.items) {
                    one[0] = item
                    if (Primitives.truth(applyValue(callback, one, at), at)) output.add(item)
                }
                GValue.VList(output, false)
            }

            "fold" -> {
                var acc = arguments.firstOrNull() ?: r.raise(GuestError.ShapeFault(at))
                val pair = mutableListOf(acc, GValue.VUnit)
                for (item in receiver.items) {
                    pair[0] = acc
                    pair[1] = item
                    acc = applyValue(callback, pair, at)
                }
                acc
            }

            "any" -> {
                for (item in receiver.items) {
                    one[0] = item
                    if (Primitives.truth(applyValue(callback, one, at), at)) return GValue.VBool(true)
                }
                GValue.VBool(false)
            }

            else -> {
                for (item in receiver.items) {
                    one[0] = item
                    if (!Primitives.truth(applyValue(callback, one, at), at)) return GValue.VBool(false)
                }
                GValue.VBool(true)
            }
        }
    }

    context(r: Raise<GuestError>)
    private fun applyValue(
        target: GValue,
        arguments: List<GValue>,
        span: sicp.guest.Span,
    ): GValue {
        if (target is GValue.VFunction) return Primitives.invoke(target, arguments, span)
        if (target !is GValue.VObject || target.className != "compound-procedure") return r.raise(GuestError.UnassignedRead(span))
        val params = (target.fields["params"] as GValue.VList).items.map { (it as GValue.VString).value }
        val body = (target.fields["body"] as GValue.VNode).node
        val frame = Env.child((target.fields["env"] as GValue.VEnvVal).env)
        for ((index, name) in params.withIndex()) frame.define(name, arguments.getOrElse(index) { GValue.VUnit })
        try {
            return if (body is Block) evalBlock(body, frame) else evalExpr(body as Expression, frame)
        } catch (marker: Marker) {
            if (marker.kind == "return") return marker.value
            return r.raise(GuestError.UnassignedRead(span))
        }
    }

    context(r: Raise<GuestError>)
    private fun copyObject(
        receiver: GValue.VObject,
        call: Call,
        env: Env,
    ): GValue {
        val fields = LinkedHashMap(receiver.fields)
        for (argument in call.arguments) {
            val name = argument.name ?: r.raise(GuestError.UnassignedRead(argument.span))
            fields[name] = evalExpr(argument.value, env)
        }
        return GValue.VObject(receiver.className, structural = true, fields)
    }

    private fun closureOf(
        node: FunctionDecl,
        env: Env,
    ): GValue {
        val names = node.parameters.map { it.name }
        return GValue.VObject(
            "compound-procedure",
            false,
            mutableMapOf(
                "params" to GValue.VList(names.map { GValue.VString(it) }.toMutableList(), false),
                "body" to GValue.VNode(node.body),
                "env" to GValue.VEnvVal(env),
            ),
        )
    }

    private fun boundMethod(
        receiver: GValue.VObject,
        method: FunctionDecl,
    ): GValue = closureOf(method, Env.child(globals).apply { define("this", receiver) })

    private fun constructor(
        name: String,
        properties: List<Property>,
        data: Boolean,
    ): GValue.VFunction =
        GValue.VFunction(name, properties.size) { arguments ->
            val fields = linkedMapOf<String, GValue>()
            for ((property, value) in properties.zip(arguments)) fields[property.name] = value
            GValue.VObject(name, data, fields)
        }

    context(r: Raise<GuestError>)
    private fun install(env: Env): Env {
        for (declaration in checked.vocabulary + checked.syntax.declarations) {
            when (declaration) {
                is DataClass -> env.define(declaration.name, constructor(declaration.name, declaration.properties, data = true))
                is DataObject -> env.define(declaration.name, GValue.VObject(declaration.name, true, mutableMapOf()))
                is PlainClass -> env.define(declaration.name, constructor(declaration.name, declaration.properties, data = false))
                is SealedInterface -> Unit
                is TypeAlias -> Unit
                is FunctionDecl -> env.define(declaration.name, closureOf(declaration, env))
                is TopProperty -> env.define(declaration.property.name, GValue.VUnassigned)
            }
        }
        for (declaration in checked.syntax.declarations) {
            if (declaration is TopProperty) {
                env.define(declaration.property.name, evalExpr(declaration.initializer, env))
            }
        }
        return env
    }
}

private class Marker(
    val kind: String,
    val value: GValue,
) : RuntimeException(null, null, false, false)
