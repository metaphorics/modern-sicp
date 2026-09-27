// SPDX-License-Identifier: GPL-3.0-only
// Section 5.4: the book's controller, in the book's fragments. Each
// fragment is the fragment's name and its controller as `Stmt` data, the
// same instruction list the 5.2 simulator assembles; the base controller
// is their concatenation. An exercise replaces a fragment (5.25's argument
// loop, 5.28's sequence evaluation, 5.30's checking entries) or appends
// its own (5.23's transformers, 5.24's clause loop) and hands the composed
// list to `makeEvaluator`.

package sicp.ch5

import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.OpAct
import sicp.runtime.Perform
import sicp.runtime.Restore
import sicp.runtime.Save
import sicp.runtime.Source
import sicp.runtime.Stmt
import sicp.runtime.Test
import sicp.runtime.VBool
import sicp.runtime.VSym

/** `(const symbol)` and `(const boolean)` sources, the controller's
 *  self-describing constants. */
public fun constSym(name: String): Source.ConstSrc = Source.ConstSrc(VSym(name))

public fun constBool(b: Boolean): Source.ConstSrc = Source.ConstSrc(VBool(b))

private fun fragment(
    name: String,
    vararg stmts: Stmt,
): Pair<String, List<Stmt>> = name to stmts.toList()

private fun dispatchTest(
    op: String,
    label: String,
): List<Stmt> = listOf(Test(sicp.runtime.OpCond(op, listOf<Source>(reg("exp")))), Branch(label))

/** The `eval-dispatch` core: the book's first eight tests in order, each
 *  branching to its entry. Exercise 5.23 and 5.24 splice their
 *  derived-form tests between these and [applicationDispatchTest], since
 *  an application test would otherwise consume every derived form. */
public val evalDispatchTests: List<Stmt> =
    dispatchTest("self-evaluating?", "ev-self-eval") +
        dispatchTest("variable?", "ev-variable") +
        dispatchTest("quoted?", "ev-quoted") +
        dispatchTest("assignment?", "ev-assignment") +
        dispatchTest("definition?", "ev-definition") +
        dispatchTest("if?", "ev-if") +
        dispatchTest("lambda?", "ev-lambda") +
        dispatchTest("begin?", "ev-begin")

/** The dispatch's last test: every remaining expression is an
 *  application. */
public val applicationDispatchTest: List<Stmt> = dispatchTest("application?", "ev-application")

/** The dispatch's fallthrough: the typed unknown-expression error. */
public val unknownExpressionTypeGoto: List<Stmt> = listOf(Goto(GotoTarget.Lbl("unknown-expression-type")))

private val evalDispatch: List<Stmt> =
    listOf(Label("eval-dispatch")) + evalDispatchTests + applicationDispatchTest + unknownExpressionTypeGoto

private val unknownExpressionType: List<Stmt> =
    listOf(
        Label("unknown-expression-type"),
        Assign("val", constSym("unknown-expression-type-error")),
        Goto(GotoTarget.Lbl("signal-error")),
        Label("unknown-procedure-type"),
        Restore("continue"),
        Assign("val", constSym("unknown-procedure-type-error")),
        Goto(GotoTarget.Lbl("signal-error")),
        Label("signal-error"),
        Perform(OpAct("signal-error", listOf(reg("val")))),
    )

/** The controller fragments of the base evaluator, in printed order: each
 *  pair is the fragment's name and its controller. */
public val evaluatorControllerFragments: List<Pair<String, List<Stmt>>> =
    listOf(
        fragment(
            "driver",
            Label("read-eval-print-loop"),
            Perform(OpAct("initialize-stack", emptyList())),
            Perform(OpAct("prompt-for-input", emptyList())),
            Assign("exp", opSrc("read")),
            Assign("env", opSrc("get-global-environment")),
            Assign("continue", labelSrc("print-result")),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("print-result"),
            Perform(OpAct("announce-output", emptyList())),
            Perform(OpAct("user-print", listOf(reg("val")))),
            Goto(GotoTarget.Lbl("read-eval-print-loop")),
        ),
        fragment("eval-dispatch", *evalDispatch.toTypedArray()),
        fragment(
            "ev-self-eval",
            Label("ev-self-eval"),
            Assign("val", reg("exp")),
            Goto(GotoTarget.ByReg("continue")),
        ),
        fragment(
            "ev-variable",
            Label("ev-variable"),
            Assign("val", opSrc("lookup-variable-value", reg("exp"), reg("env"))),
            Goto(GotoTarget.ByReg("continue")),
        ),
        fragment(
            "ev-quoted",
            Label("ev-quoted"),
            Assign("val", opSrc("text-of-quotation", reg("exp"))),
            Goto(GotoTarget.ByReg("continue")),
        ),
        fragment(
            "ev-lambda",
            Label("ev-lambda"),
            Assign("unev", opSrc("lambda-parameters", reg("exp"))),
            Assign("exp", opSrc("lambda-body", reg("exp"))),
            Assign("val", opSrc("make-procedure", reg("unev"), reg("exp"), reg("env"))),
            Goto(GotoTarget.ByReg("continue")),
        ),
        fragment(
            "ev-application",
            Label("ev-application"),
            Save("continue"),
            Save("env"),
            Assign("unev", opSrc("operands", reg("exp"))),
            Save("unev"),
            Assign("exp", opSrc("operator", reg("exp"))),
            Assign("continue", labelSrc("ev-appl-did-operator")),
            Goto(GotoTarget.Lbl("eval-dispatch")),
        ),
        fragment(
            "ev-appl-did-operator",
            Label("ev-appl-did-operator"),
            Restore("unev"),
            Restore("env"),
            Assign("argl", opSrc("empty-arglist")),
            Assign("proc", reg("val")),
            Test(opCond("no-operands?", reg("unev"))),
            Branch("apply-dispatch"),
            Save("proc"),
        ),
        fragment(
            "argument-loop",
            Label("ev-appl-operand-loop"),
            Save("argl"),
            Assign("exp", opSrc("first-operand", reg("unev"))),
            Test(opCond("last-operand?", reg("unev"))),
            Branch("ev-appl-last-arg"),
            Save("env"),
            Save("unev"),
            Assign("continue", labelSrc("ev-appl-accumulate-arg")),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("ev-appl-accumulate-arg"),
            Restore("unev"),
            Restore("env"),
            Restore("argl"),
            Assign("argl", opSrc("adjoin-arg", reg("val"), reg("argl"))),
            Assign("unev", opSrc("rest-operands", reg("unev"))),
            Goto(GotoTarget.Lbl("ev-appl-operand-loop")),
            Label("ev-appl-last-arg"),
            Assign("continue", labelSrc("ev-appl-accum-last-arg")),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("ev-appl-accum-last-arg"),
            Restore("argl"),
            Assign("argl", opSrc("adjoin-arg", reg("val"), reg("argl"))),
            Restore("proc"),
            Goto(GotoTarget.Lbl("apply-dispatch")),
        ),
        fragment(
            "apply-dispatch",
            Label("apply-dispatch"),
            Test(opCond("primitive-procedure?", reg("proc"))),
            Branch("primitive-apply"),
            Test(opCond("compound-procedure?", reg("proc"))),
            Branch("compound-apply"),
            Goto(GotoTarget.Lbl("unknown-procedure-type")),
        ),
        fragment(
            "primitive-apply",
            Label("primitive-apply"),
            Assign("val", opSrc("apply-primitive-procedure", reg("proc"), reg("argl"))),
            Restore("continue"),
            Goto(GotoTarget.ByReg("continue")),
        ),
        fragment(
            "compound-apply",
            Label("compound-apply"),
            Assign("unev", opSrc("procedure-parameters", reg("proc"))),
            Assign("env", opSrc("procedure-environment", reg("proc"))),
            Assign("env", opSrc("extend-environment", reg("unev"), reg("argl"), reg("env"))),
            Assign("unev", opSrc("procedure-body", reg("proc"))),
            Goto(GotoTarget.Lbl("ev-sequence")),
        ),
        fragment(
            "begin",
            Label("ev-begin"),
            Assign("unev", opSrc("begin-actions", reg("exp"))),
            Save("continue"),
            Goto(GotoTarget.Lbl("ev-sequence")),
        ),
        fragment(
            "ev-sequence",
            Label("ev-sequence"),
            Assign("exp", opSrc("first-exp", reg("unev"))),
            Test(opCond("last-exp?", reg("unev"))),
            Branch("ev-sequence-last-exp"),
            Save("unev"),
            Save("env"),
            Assign("continue", labelSrc("ev-sequence-continue")),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("ev-sequence-continue"),
            Restore("env"),
            Restore("unev"),
            Assign("unev", opSrc("rest-exps", reg("unev"))),
            Goto(GotoTarget.Lbl("ev-sequence")),
            Label("ev-sequence-last-exp"),
            Restore("continue"),
            Goto(GotoTarget.Lbl("eval-dispatch")),
        ),
        fragment(
            "if",
            Label("ev-if"),
            Save("exp"),
            Save("env"),
            Save("continue"),
            Assign("continue", labelSrc("ev-if-decide")),
            Assign("exp", opSrc("if-predicate", reg("exp"))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("ev-if-decide"),
            Restore("continue"),
            Restore("env"),
            Restore("exp"),
            Test(opCond("true?", reg("val"))),
            Branch("ev-if-consequent"),
            Label("ev-if-alternative"),
            Assign("exp", opSrc("if-alternative", reg("exp"))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("ev-if-consequent"),
            Assign("exp", opSrc("if-consequent", reg("exp"))),
            Goto(GotoTarget.Lbl("eval-dispatch")),
        ),
        fragment(
            "assignment",
            Label("ev-assignment"),
            Assign("unev", opSrc("assignment-variable", reg("exp"))),
            Save("unev"),
            Assign("exp", opSrc("assignment-value", reg("exp"))),
            Save("env"),
            Save("continue"),
            Assign("continue", labelSrc("ev-assignment-1")),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("ev-assignment-1"),
            Restore("continue"),
            Restore("env"),
            Restore("unev"),
            Perform(OpAct("set-variable-value!", listOf(reg("unev"), reg("val"), reg("env")))),
            Assign("val", constSym("ok")),
            Goto(GotoTarget.ByReg("continue")),
        ),
        fragment(
            "definition",
            Label("ev-definition"),
            Assign("unev", opSrc("definition-variable", reg("exp"))),
            Save("unev"),
            Assign("exp", opSrc("definition-value", reg("exp"))),
            Save("env"),
            Save("continue"),
            Assign("continue", labelSrc("ev-definition-1")),
            Goto(GotoTarget.Lbl("eval-dispatch")),
            Label("ev-definition-1"),
            Restore("continue"),
            Restore("env"),
            Restore("unev"),
            Perform(OpAct("define-variable!", listOf(reg("unev"), reg("val"), reg("env")))),
            Assign("val", constSym("ok")),
            Goto(GotoTarget.ByReg("continue")),
        ),
        fragment("errors", *unknownExpressionType.toTypedArray()),
    )

/** The fragment named [name]; an unknown name is a host bug. */
public fun evaluatorFragment(name: String): List<Stmt> {
    val hit = evaluatorControllerFragments.first { it.first == name }
    return hit.second
}

/** The book's base controller: the fragments in printed order. */
public val baseEvaluatorController: List<Stmt> =
    evaluatorControllerFragments.flatMap { it.second }

/** The monitored driver of 5.4.4: `print-result` performs
 *  `print-stack-statistics` before announcing the value, so every
 *  interaction reports its own total pushes and maximum depth, and the
 *  driver initializes the stack once per interaction. */
public val monitoredDriver: List<Stmt> =
    listOf(
        Label("read-eval-print-loop"),
        Perform(OpAct("initialize-stack", emptyList())),
        Perform(OpAct("prompt-for-input", emptyList())),
        Assign("exp", opSrc("read")),
        Assign("env", opSrc("get-global-environment")),
        Assign("continue", labelSrc("print-result")),
        Goto(GotoTarget.Lbl("eval-dispatch")),
        Label("print-result"),
        Perform(OpAct("print-stack-statistics", emptyList())),
        Perform(OpAct("announce-output", emptyList())),
        Perform(OpAct("user-print", listOf(reg("val")))),
        Goto(GotoTarget.Lbl("read-eval-print-loop")),
    )

/** The base controller with the monitored driver in place of the plain
 *  one: the controller the stack-measuring exercises (5.26 to 5.29) and
 *  the book's monitored session run on. */
public val monitoredEvaluatorController: List<Stmt> =
    evaluatorControllerFragments.flatMap { (name, stmts) ->
        if (name == "driver") monitoredDriver else stmts
    }
