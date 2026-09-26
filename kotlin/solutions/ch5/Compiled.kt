// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, section 5.5: the measurement and compilation harness the
// compiler exercises share (5.31 to 5.50). Every run compiles with the
// section's own compiler, installs the block on the 5.5.7 machine, and
// reads the transcript, the step counter, or the monitored stack, so
// every number the exercises pin is machine-run output.

package sicp.ch5.solutions

import arrow.core.raise.either
import sicp.ch4.readProgram
import sicp.ch5.CompilerConfig
import sicp.ch5.CompilerState
import sicp.ch5.Linkage
import sicp.ch5.ObjectPrimitive
import sicp.ch5.compileAndGo
import sicp.ch5.compileBlock
import sicp.ch5.compileProgram
import sicp.ch5.ecevalController
import sicp.ch5.renderStmt
import sicp.runtime.SchemeError
import sicp.runtime.Stmt
import sicp.runtime.Value

/** Reads the forms of [source]; a parse failure is a measurement bug. */
internal fun readForms(source: String): List<Value> {
    val parsed = either<SchemeError, List<Value>> { readProgram(source) }
    return parsed.fold(
        { e -> error("the driver source did not parse: $e") },
        { it },
    )
}

/** Compiles [source] (one form) under [cfg] and answers its statements
 *  as the book's controller lines. [linkage] is the top-level linkage:
 *  bare top-level combinations run through the book's `compile-and-go`,
 *  which compiles the expression with target `val` and linkage `return`,
 *  so 5.31 passes [Linkage.Return]; every definition comparison keeps
 *  the default [Linkage.Next]. */
internal fun compiledStatements(
    cfg: CompilerConfig = CompilerConfig(),
    source: String,
    linkage: Linkage = Linkage.Next,
): List<String> =
    either {
        val state = CompilerState()
        val forms = readForms(source)
        val seq = compileProgram(cfg, state, forms, linkage)
        seq.stmts.map { renderStmt(it) }
    }.fold(
        { e -> error("the compilation failed: $e") },
        { it },
    )

/** Compiles [source] under [cfg] and answers the statement count and
 *  the save/restore count. */
internal fun compileCounts(
    cfg: CompilerConfig = CompilerConfig(),
    source: String,
): Pair<Int, Int> {
    val stmts = compiledStatements(cfg, source)
    val saves = stmts.count { it.startsWith("(save ") || it.startsWith("(restore ") }
    return stmts.size to saves
}

/** Compiles [compiled] under [cfg], runs it by compile-and-go on
 *  [controller], and answers the transcript. */
internal fun runCompiled(
    cfg: CompilerConfig = CompilerConfig(),
    compiled: String,
    driver: String,
    controller: List<Stmt> = ecevalController,
    extraPrimitives: Map<String, ObjectPrimitive> = emptyMap(),
    runtimeSupport: Boolean = false,
): List<String> =
    either {
        val state = CompilerState()
        val forms = readForms(compiled)
        val (entry, block) = compileBlock(cfg, state, forms)
        val evaluator =
            compileAndGo(
                entry,
                block,
                driver,
                controller,
                extraPrimitives = extraPrimitives,
                runtimeSupport = runtimeSupport,
            )
        evaluator.drive()
        evaluator.transcript
    }.fold(
        { e -> error("the compiled run failed: $e") },
        { it },
    )

/** [runCompiled] with the monitored 5.5.7 driver. */
internal fun runCompiledMonitored(
    cfg: CompilerConfig = CompilerConfig(),
    compiled: String,
    driver: String,
): List<String> = runCompiled(cfg, compiled, driver, sicp.ch5.monitoredEcevalController)

/** The step count of the compile-and-go run of [compiled] on [driver]. */
internal fun runCompiledSteps(
    cfg: CompilerConfig = CompilerConfig(),
    compiled: String,
    driver: String,
    runtimeSupport: Boolean = false,
): Long =
    either {
        val state = CompilerState()
        val forms = readForms(compiled)
        val (entry, block) = compileBlock(cfg, state, forms)
        val evaluator = compileAndGo(entry, block, driver, runtimeSupport = runtimeSupport)
        evaluator.drive()
        evaluator.steps
    }.fold(
        { e -> error("the compiled run failed: $e") },
        { it },
    )

/** The printed values of a driver transcript: every line that follows a
 *  value announcement. */
internal fun valuesOf(transcript: List<String>): List<String> {
    val out = ArrayList<String>()
    var announce = false
    for (line in transcript) {
        when {
            line == ";;; EC-Eval value:" -> {
                announce = true
            }

            announce -> {
                out.add(line)
                announce = false
            }
        }
    }
    return out
}

/** The last statistics line's counters of a monitored transcript. */
internal fun lastStats(transcript: List<String>): Stats =
    statsOf(transcript).lastOrNull() ?: error("the session printed no stack statistics")

/** The lines of [transcript] that are stack statistics. */
internal fun statLines(transcript: List<String>): List<String> = transcript.filter { it.startsWith("(total-pushes") }
