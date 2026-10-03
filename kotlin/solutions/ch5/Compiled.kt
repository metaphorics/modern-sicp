// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, section 5.5: the measurement and analysis harness the
// compiler exercises share (5.31 to 5.50). Every run compiles with the
// section's own compiler, installs the block on the compiled-code
// machine, and reads the transcript, the instruction counter, or the
// monitored stack, so every number the exercises pin is machine-run
// output. The register analysis over emitted statements answers the
// needs/modifies questions: which registers a statement reads and
// writes, and which `save`/`restore` pairs are superfluous because the
// saved register survives the pair intact.

package sicp.ch5.solutions

import arrow.core.raise.either
import sicp.ch5.Compiler
import sicp.ch5.CompilerOptions
import sicp.guest.Admission
import sicp.guest.CheckedProgram
import sicp.guest.GuestError
import sicp.guest.Mode
import sicp.guest.RunResult
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Machine
import sicp.runtime.OpAct
import sicp.runtime.OpCond
import sicp.runtime.Perform
import sicp.runtime.Restore
import sicp.runtime.Save
import sicp.runtime.Source
import sicp.runtime.Stmt
import sicp.runtime.Test

/** Admits [source]; a parse or check failure is a harness bug. */
internal fun admitProgram(source: String): CheckedProgram =
    Admission.admit(source, Mode.CORE).fold({ error -> error("the source did not admit: $error") }, { it })

/** Compiles [source] and answers its instruction sequence; the compile
 *  failure is a harness bug with its category. */
internal fun compiledStatements(
    source: String,
    options: CompilerOptions = CompilerOptions(),
): List<Stmt> =
    Compiler.compile(admitProgram(source), options).fold(
        { error -> error("the compilation failed: $error") },
        { it },
    )

/** The registers a source operand reads. */
internal fun sourceReads(src: Source): Set<String> =
    when (src) {
        is Source.RegSrc -> setOf(src.reg)
        is Source.ConstSrc -> emptySet()
        is Source.LabelSrc -> emptySet()
        is Source.OpSrc -> src.args.fold(emptySet()) { names, argument -> names + sourceReads(argument) }
    }

/** The registers an operation call reads. */
private fun opReads(args: List<Source>): Set<String> = args.fold(emptySet()) { names, argument -> names + sourceReads(argument) }

/** The registers one instruction reads. */
internal fun readsOf(instruction: Stmt): Set<String> =
    when (instruction) {
        is Label -> {
            emptySet()
        }

        is Assign -> {
            sourceReads(instruction.src)
        }

        is Test -> {
            when (val condition = instruction.cond) {
                is OpCond -> opReads(condition.args)
            }
        }

        is Branch -> {
            emptySet()
        }

        is Goto -> {
            if (instruction.to is GotoTarget.ByReg) setOf((instruction.to as GotoTarget.ByReg).reg) else emptySet()
        }

        is Save -> {
            setOf(instruction.reg)
        }

        is Restore -> {
            emptySet()
        }

        is Perform -> {
            when (val action = instruction.act) {
                is OpAct -> opReads(action.args)
            }
        }
    }

/** The registers one instruction writes. */
internal fun writesOf(instruction: Stmt): Set<String> =
    when (instruction) {
        is Assign -> setOf(instruction.reg)
        is Restore -> setOf(instruction.reg)
        else -> emptySet()
    }

/** One `save`/`restore` pair: where it sits and whether the saved
 *  register is modified between the two instructions. */
internal data class SavePair(
    val saveIndex: Int,
    val restoreIndex: Int,
    val register: String,
    val clobbered: Boolean,
)

/** Pairs every `save` with the matching `restore` (the stack discipline
 *  of the compiled sequences) and records whether anything writes the
 *  saved register in between: a pair whose register survives intact is
 *  the superfluous save of the exercise. */
internal fun savePairs(stmts: List<Stmt>): List<SavePair> {
    val pairs = mutableListOf<SavePair>()
    val open = ArrayDeque<Pair<Int, String>>()
    for ((index, instruction) in stmts.withIndex()) {
        when (instruction) {
            is Save -> {
                open.addLast(index to instruction.reg)
            }

            is Restore -> {
                val saved = open.removeLastOrNull() ?: continue
                val clobbered = (saved.first + 1 until index).any { between -> saved.second in writesOf(stmts[between]) }
                pairs.add(SavePair(saved.first, index, saved.second, clobbered))
            }

            else -> {}
        }
    }
    return pairs
}

/** The compiled-code machine of one checked program, wired to the
 *  compiler's operation table: the monitored run the stack exercises
 *  read. */
internal fun compiledMachine(
    source: String,
    options: CompilerOptions = CompilerOptions(),
): Machine = Compiler.machine(admitProgram(source), options)

/** Runs the compiled-code machine of [source] to its halt and answers
 *  the result; a fault is a harness bug with its category. */
internal fun runCompiled(source: String): RunResult = Compiler.compileAndRun(admitProgram(source))

/** The printed values of one run: its output lines in program order. */
internal fun outputLines(result: RunResult): List<String> = result.output.split("\n").filter { it.isNotEmpty() }

/** The monitored run's counters the measuring exercises print. */
internal fun compiledStatistics(source: String): String {
    val machine = compiledMachine(source)
    val outcome = either<GuestError, Unit> { machine.run() }
    outcome.fold({ error -> error("the compiled run faulted: ${error.category}") }, { })
    return statisticsLine(machine)
}

/** The compiled run's instruction count, the 5.34 annotation's input. */
internal fun compiledInstructions(source: String): Long {
    val machine = compiledMachine(source)
    val outcome = either<GuestError, Unit> { machine.run() }
    outcome.fold({ error -> error("the compiled run faulted: ${error.category}") }, { })
    return machine.instructions
}

/** The registers one instruction list modifies and needs, the 5.31
 *  analysis's summary over a whole compilation. */
internal fun registersOf(stmts: List<Stmt>): Pair<Set<String>, Set<String>> =
    stmts.fold(emptySet<String>() to emptySet<String>()) { (writes, reads), instruction ->
        (writes + writesOf(instruction)) to (reads + readsOf(instruction))
    }
