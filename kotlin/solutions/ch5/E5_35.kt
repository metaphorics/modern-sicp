// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.35: the expression compiled to Figure 5.18 is
// `(define (f x) (+ x (g (+ x 2))))`. The figure's numbering is a
// session artifact: the book's label counter had generated fourteen
// labels before this compilation. Seeding the compiler's counter at 14
// reproduces the figure statement for statement, in the book's own
// spellings: the entry rides in a `(label entry16)` operation input and
// the parameter list in one `(const (x))` list constant.

package sicp.ch5.solutions

import sicp.ch5.CompilerConfig
import sicp.ch5.CompilerState
import sicp.ch5.Linkage
import sicp.ch5.compileProgram

private val source: String = "(define (f x) (+ x (g (+ x 2))))"

/** Figure 5.18, statements only: the listing's prose comments are not
 *  statements. */
private val figure5_18Statements: List<String> =
    listOf(
        "(assign val (op make-compiled-procedure) (label entry16) (reg env))",
        "(goto (label after-lambda15))",
        "entry16",
        "(assign env (op compiled-procedure-env) (reg proc))",
        "(assign env (op extend-environment) (const (x)) (reg argl) (reg env))",
        "(assign proc (op lookup-variable-value) (const +) (reg env))",
        "(save continue)",
        "(save proc)",
        "(save env)",
        "(assign proc (op lookup-variable-value) (const g) (reg env))",
        "(save proc)",
        "(assign proc (op lookup-variable-value) (const +) (reg env))",
        "(assign val (const 2))",
        "(assign argl (op list) (reg val))",
        "(assign val (op lookup-variable-value) (const x) (reg env))",
        "(assign argl (op cons) (reg val) (reg argl))",
        "(test (op primitive-procedure?) (reg proc))",
        "(branch (label primitive-branch19))",
        "compiled-branch18",
        "(assign continue (label after-call17))",
        "(assign val (op compiled-procedure-entry) (reg proc))",
        "(goto (reg val))",
        "primitive-branch19",
        "(assign val (op apply-primitive-procedure) (reg proc) (reg argl))",
        "after-call17",
        "(assign argl (op list) (reg val))",
        "(restore proc)",
        "(test (op primitive-procedure?) (reg proc))",
        "(branch (label primitive-branch22))",
        "compiled-branch21",
        "(assign continue (label after-call20))",
        "(assign val (op compiled-procedure-entry) (reg proc))",
        "(goto (reg val))",
        "primitive-branch22",
        "(assign val (op apply-primitive-procedure) (reg proc) (reg argl))",
        "after-call20",
        "(assign argl (op list) (reg val))",
        "(restore env)",
        "(assign val (op lookup-variable-value) (const x) (reg env))",
        "(assign argl (op cons) (reg val) (reg argl))",
        "(restore proc)",
        "(restore continue)",
        "(test (op primitive-procedure?) (reg proc))",
        "(branch (label primitive-branch25))",
        "compiled-branch24",
        "(assign val (op compiled-procedure-entry) (reg proc))",
        "(goto (reg val))",
        "primitive-branch25",
        "(assign val (op apply-primitive-procedure) (reg proc) (reg argl))",
        "(goto (reg continue))",
        "after-call23",
        "after-lambda15",
        "(perform (op define-variable!) (const f) (reg val) (reg env))",
        "(assign val (const ok))",
    )

/** Compiles the expression with the counter seeded at 14 and answers
 *  the compiler's own output beside the figure it matches. */
public fun figure5_18Compilation(): List<String> {
    val cfg = CompilerConfig()
    val state = CompilerState(seed = 14)
    val forms = readForms(source)
    val stmts =
        arrow.core.raise
            .either {
                compileProgram(cfg, state, forms).stmts.map { sicp.ch5.renderStmt(it) }
            }.fold(
                { e -> error("the compilation failed: $e") },
                { it },
            )
    return listOf(
        "compiled to the figure: $source",
        stmts.joinToString("\n"),
        "figure matches: ${stmts == figure5_18Statements}",
    )
}
