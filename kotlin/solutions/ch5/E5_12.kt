// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.12: the assembler's summary. While assembling, the
// assembler can gather data about the controller: how often each
// instruction text occurs, which registers the controller names, which
// registers hold entry points or ride on the stack, the sources that
// assign to each register, and which labels it defines. The summary of the
// book's gcd machine is computed here from the same controller the 5.7
// machine runs, in the book's own machine notation.

package sicp.ch5.solutions

import sicp.guest.GValue
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.OpAct
import sicp.runtime.OpCond
import sicp.runtime.Perform
import sicp.runtime.Reg
import sicp.runtime.Restore
import sicp.runtime.Save
import sicp.runtime.Source
import sicp.runtime.Stmt
import sicp.runtime.Test
import sicp.runtime.assemble

/** A constant's datum, as the summary writes it. */
private fun constText(value: GValue): String =
    when (value) {
        is GValue.VLong -> value.value.toString()
        is GValue.VInt -> value.value.toString()
        is GValue.VDouble -> value.value.toString()
        is GValue.VBool -> value.value.toString()
        is GValue.VString -> value.value
        else -> render(value)
    }

/** An operand source's book spelling: `(reg b)`, `(const 0)`, `(label b)`,
 *  or the flat `(op rem) (reg a) (reg b)` of an operation call. */
private fun sourceText(src: Source): String =
    when (src) {
        is Source.RegSrc -> "(reg ${src.reg})"
        is Source.ConstSrc -> "(const ${constText(src.v)})"
        is Source.LabelSrc -> "(label ${src.name})"
        is Source.OpSrc -> "(op ${src.name}) ${src.args.joinToString(" ") { sourceText(it) }}"
    }

/** An operand source as one expression, the spelling the summary's source
 *  records use: a register reference stays one form, an operation call
 *  wraps its operator and operands in one list. */
private fun sourceExpressionText(src: Source): String =
    when (src) {
        is Source.OpSrc -> "(${sourceText(src)})"
        else -> sourceText(src)
    }

private fun condText(cond: sicp.runtime.Cond): String =
    when (cond) {
        is sicp.runtime.OpCond -> "(op ${cond.name}) ${cond.args.joinToString(" ") { sourceText(it) }}"
    }

private fun actionText(action: sicp.runtime.Action): String =
    when (action) {
        is sicp.runtime.OpAct -> "(op ${action.name}) ${action.args.joinToString(" ") { sourceText(it) }}"
    }

private fun targetText(target: GotoTarget): String =
    when (target) {
        is GotoTarget.Lbl -> "(label ${target.name})"
        is GotoTarget.ByReg -> "(reg ${target.reg})"
    }

/** One instruction's book spelling. */
public fun instructionText(instruction: Stmt): String =
    when (instruction) {
        is Label -> "(label ${instruction.name})"
        is Assign -> "(assign ${instruction.reg} ${sourceText(instruction.src)})"
        is Test -> "(test ${condText(instruction.cond)})"
        is Branch -> "(branch ${targetText(sicp.runtime.GotoTarget.Lbl(instruction.label))})"
        is Goto -> "(goto ${targetText(instruction.to)})"
        is Save -> "(save ${instruction.reg})"
        is Restore -> "(restore ${instruction.reg})"
        is Perform -> "(perform ${actionText(instruction.act)})"
    }

/** One instruction's type name: the summary's grouping key and the
 *  census's counting key. */
internal fun instructionKind(instruction: Stmt): String =
    when (instruction) {
        is Label -> "label"
        is Assign -> "assign"
        is Test -> "test"
        is Branch -> "branch"
        is Goto -> "goto"
        is Save -> "save"
        is Restore -> "restore"
        is Perform -> "perform"
    }

/** One summary line: the header, the items, and the closing paren, so an
 *  empty item list keeps the book's `... registers )` spelling. */
private fun summaryLine(
    header: String,
    items: List<String>,
): String = "($header ${items.joinToString(" ")})"

/** Every register name the controller mentions. */
internal fun controllerRegisters(controller: List<Stmt>): Set<Reg> {
    val names = linkedSetOf<Reg>()

    fun sourceRegisters(src: Source) {
        when (src) {
            is Source.RegSrc -> names.add(src.reg)
            is Source.ConstSrc -> Unit
            is Source.LabelSrc -> Unit
            is Source.OpSrc -> src.args.forEach { sourceRegisters(it) }
        }
    }
    for (instruction in controller) {
        when (instruction) {
            is Label -> {}

            is Assign -> {
                names.add(instruction.reg)
                sourceRegisters(instruction.src)
            }

            is Test -> {
                when (val condition = instruction.cond) {
                    is OpCond -> condition.args.forEach { sourceRegisters(it) }
                }
            }

            is Branch -> {}

            is Goto -> {
                if (instruction.to is GotoTarget.ByReg) names.add((instruction.to as GotoTarget.ByReg).reg)
            }

            is Save -> {
                names.add(instruction.reg)
            }

            is Restore -> {
                names.add(instruction.reg)
            }

            is Perform -> {
                when (val action = instruction.act) {
                    is OpAct -> action.args.forEach { sourceRegisters(it) }
                }
            }
        }
    }
    return names
}

/** The assembler's summary of [controller] in the book's machine
 *  notation: the instruction texts by type (duplicates removed), the
 *  registers, the entry-point and stack registers, the sources of every
 *  assigned register, and the labels. */
public fun assemblySummary(controller: List<Stmt>): String {
    val summary = assemble(controller).fold({ failure -> error("the controller does not assemble: ${failure.category}") }, { it })
    val instructions = controller.filterNot { it is Label }
    val kinds = linkedMapOf<String, MutableList<String>>()
    for (instruction in instructions) {
        val kind = instructionKind(instruction)
        val texts = kinds.getOrPut(kind) { mutableListOf() }
        val text = instructionText(instruction)
        if (text !in texts) texts.add(text)
    }
    val instructionGroups =
        kinds.map { (kind, texts) -> summaryLine(kind, texts.sorted()) }
    val registers = controllerRegisters(controller).sorted()
    val entryPoints =
        instructions
            .filterIsInstance<Goto>()
            .mapNotNull { (it.to as? GotoTarget.ByReg)?.reg }
            .distinct()
            .sorted()
    val stackRegisters =
        instructions
            .filter { it is Save || it is Restore }
            .map { (it as? Save)?.reg ?: (it as Restore).reg }
            .distinct()
            .sorted()
    val sources = linkedMapOf<Reg, MutableList<String>>()
    for (instruction in instructions.filterIsInstance<Assign>()) {
        val texts = sources.getOrPut(instruction.reg) { mutableListOf() }
        val text = sourceExpressionText(instruction.src)
        if (text !in texts) texts.add(text)
    }
    val sourceRecords = sources.map { (register, texts) -> summaryLine(register, texts) }
    return listOf(
        summaryLine("instructions", instructionGroups),
        summaryLine("registers", registers),
        summaryLine("entry-point registers", entryPoints),
        summaryLine("stack registers", stackRegisters),
        summaryLine("sources", sourceRecords),
        summaryLine("labels", summary.labels.keys.toList()),
    ).joinToString("\n")
}

/** The 5.12 summary of the gcd machine. */
public fun gcdMachineSummary(): String = assemblySummary(gcdController)
