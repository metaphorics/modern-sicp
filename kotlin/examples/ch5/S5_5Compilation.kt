// SPDX-License-Identifier: GPL-3.0-only
// Section 5.5: running the compiler. The book's Figure 5.17 compilation
// of the factorial definition, replayed statement for statement from the
// compiler's own output, and the 5.5.7 compile-and-go conversation: the
// definition answers ok, the call answers 120, and the monitored stack
// shows the book's `(total-pushes = 0 maximum-depth = 0)` at the entry
// and 31 pushes at depth 14 for the call.

package sicp.ch5.examples

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.readProgram
import sicp.ch5.CompilerConfig
import sicp.ch5.CompilerState
import sicp.ch5.EvaluatorFault
import sicp.ch5.Linkage
import sicp.ch5.MachineError
import sicp.ch5.compileAndGo
import sicp.ch5.compileBlock
import sicp.ch5.ecevalController
import sicp.ch5.monitoredEcevalController
import sicp.ch5.renderStmt
import sicp.runtime.Stmt

private val factorialSource: String =
    """
    (define (factorial n)
      (if (= n 1)
          1
          (* (factorial (- n 1)) n)))
    """.trimIndent()

/** The compiled block of [source]: the entry name and the controller
 *  statements, compiled under a fresh state. */
private fun compileBlockOf(source: String): Either<MachineError, Pair<String, List<Stmt>>> =
    either {
        val cfg = CompilerConfig()
        val state = CompilerState()
        val forms =
            when (val read = either { readProgram(source) }) {
                is Either.Left -> raise(EvaluatorFault(read.value.toString()))
                is Either.Right -> read.value
            }
        compileBlock(cfg, state, forms)
    }

/** The book's Figure 5.17, statements only: the listing's comments are
 *  not statements. */
private val figure5_17Statements: List<String> =
    listOf(
        "(assign val (op make-compiled-procedure) (label entry2) (reg env))",
        "(goto (label after-lambda1))",
        "entry2",
        "(assign env (op compiled-procedure-env) (reg proc))",
        "(assign env (op extend-environment) (const (n)) (reg argl) (reg env))",
        "(save continue)",
        "(save env)",
        "(assign proc (op lookup-variable-value) (const =) (reg env))",
        "(assign val (const 1))",
        "(assign argl (op list) (reg val))",
        "(assign val (op lookup-variable-value) (const n) (reg env))",
        "(assign argl (op cons) (reg val) (reg argl))",
        "(test (op primitive-procedure?) (reg proc))",
        "(branch (label primitive-branch17))",
        "compiled-branch16",
        "(assign continue (label after-call15))",
        "(assign val (op compiled-procedure-entry) (reg proc))",
        "(goto (reg val))",
        "primitive-branch17",
        "(assign val (op apply-primitive-procedure) (reg proc) (reg argl))",
        "after-call15",
        "(restore env)",
        "(restore continue)",
        "(test (op false?) (reg val))",
        "(branch (label false-branch4))",
        "true-branch5",
        "(assign val (const 1))",
        "(goto (reg continue))",
        "false-branch4",
        "(assign proc (op lookup-variable-value) (const *) (reg env))",
        "(save continue)",
        "(save proc)",
        "(assign val (op lookup-variable-value) (const n) (reg env))",
        "(assign argl (op list) (reg val))",
        "(save argl)",
        "(assign proc (op lookup-variable-value) (const factorial) (reg env))",
        "(save proc)",
        "(assign proc (op lookup-variable-value) (const -) (reg env))",
        "(assign val (const 1))",
        "(assign argl (op list) (reg val))",
        "(assign val (op lookup-variable-value) (const n) (reg env))",
        "(assign argl (op cons) (reg val) (reg argl))",
        "(test (op primitive-procedure?) (reg proc))",
        "(branch (label primitive-branch8))",
        "compiled-branch7",
        "(assign continue (label after-call6))",
        "(assign val (op compiled-procedure-entry) (reg proc))",
        "(goto (reg val))",
        "primitive-branch8",
        "(assign val (op apply-primitive-procedure) (reg proc) (reg argl))",
        "after-call6",
        "(assign argl (op list) (reg val))",
        "(restore proc)",
        "(test (op primitive-procedure?) (reg proc))",
        "(branch (label primitive-branch11))",
        "compiled-branch10",
        "(assign continue (label after-call9))",
        "(assign val (op compiled-procedure-entry) (reg proc))",
        "(goto (reg val))",
        "primitive-branch11",
        "(assign val (op apply-primitive-procedure) (reg proc) (reg argl))",
        "after-call9",
        "(restore argl)",
        "(assign argl (op cons) (reg val) (reg argl))",
        "(restore proc)",
        "(restore continue)",
        "(test (op primitive-procedure?) (reg proc))",
        "(branch (label primitive-branch14))",
        "compiled-branch13",
        "(assign val (op compiled-procedure-entry) (reg proc))",
        "(goto (reg val))",
        "primitive-branch14",
        "(assign val (op apply-primitive-procedure) (reg proc) (reg argl))",
        "(goto (reg continue))",
        "after-call12",
        "after-if3",
        "after-lambda1",
        "(perform (op define-variable!) (const factorial) (reg val) (reg env))",
        "(assign val (const ok))",
    )

/** The figure is the compilation with linkage next: the block carries
 *  the synthetic entry label on top and the return linkage's
 *  `(goto (reg continue))` at the bottom, and both leave. */
private fun figureStatements(): List<String> =
    compileBlockOf(factorialSource).fold(
        { e -> error("the compilation failed: $e") },
        { (entry, block) ->
            check(entry == "compiled-entry-1") { "the entry label moved: $entry" }
            block.map { renderStmt(it) }.drop(1).dropLast(1)
        },
    )

/** Runs the compile-and-go conversation on [controller] and answers the
 *  transcript. */
private fun goSession(controller: List<Stmt>): List<String> =
    compileBlockOf(factorialSource).fold(
        { e -> error("the compilation failed: $e") },
        { (entry, block) ->
            either {
                val evaluator = compileAndGo(entry, block, "(factorial 5)", controller)
                evaluator.drive()
                evaluator.transcript
            }.fold(
                { e -> error("the session failed: $e") },
                { it },
            )
        },
    )

public class S5_5CompilationTest :
    FunSpec({
        test("the compiler replays Figure 5.17 statement for statement") {
            figureStatements() shouldBe figure5_17Statements
        }

        test("compile-and-go runs the book's session") {
            goSession(ecevalController) shouldBe
                listOf(
                    ";;; EC-Eval value:",
                    "ok",
                    ";;; EC-Eval input:",
                    ";;; EC-Eval value:",
                    "120",
                    ";;; EC-Eval input:",
                )
        }

        test("the monitored 5.5.7 session shows 0 pushes at the entry and 31 at depth 14 for the call") {
            goSession(monitoredEcevalController) shouldBe
                listOf(
                    "(total-pushes = 0 maximum-depth = 0)",
                    ";;; EC-Eval value:",
                    "ok",
                    ";;; EC-Eval input:",
                    "(total-pushes = 31 maximum-depth = 14)",
                    ";;; EC-Eval value:",
                    "120",
                    ";;; EC-Eval input:",
                )
        }
    })
