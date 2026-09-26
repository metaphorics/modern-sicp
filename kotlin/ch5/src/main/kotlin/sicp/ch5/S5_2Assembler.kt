// SPDX-License-Identifier: GPL-3.0-only
// Section 5.2: the assembler (5.2.2) and the execution procedures it
// installs (5.2.3). `assemble` is the book's two stages in one pass:
// `extractLabels` scans the controller, separating the labels from the
// instructions and pointing each label at the instruction that follows it (a
// trailing label names the stop address one past the last instruction), then
// `executionProcedure` builds one closure per instruction and `Inst` pairs
// the closure with the instruction's text. While assembling, the assembler
// also gathers the summary of exercise 5.12 (the instruction census, the
// registers used, the labels) with the 5.12a per-type counts. The builders
// are free functions, so a variant assembler -- exercise 5.9's label-
// refusing operands, 5.11's restore disciplines, 5.10's new syntax --
// recomposes them instead of editing the machine.

package sicp.ch5

import arrow.core.raise.Raise
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
import sicp.runtime.VBool
import sicp.runtime.VInt
import sicp.runtime.Value

/** One execution procedure: the closure whose call simulates the
 *  instruction, raising [MachineError] through the [Raise] it is passed.
 *  The closures capture the machine's data paths (registers, flag, stack);
 *  the raise scope arrives per run. */
public typealias Exec = (Raise<MachineError>) -> Unit

/** One value procedure: the closure whose call computes a source's or
 *  operand's value, raising [MachineError] through the [Raise] it is
 *  passed. */
public typealias ValueProc = (Raise<MachineError>) -> Value

/** One assembled instruction: the instruction's text (handy for tracing,
 *  exercise 5.16) and its execution procedure. */
public class Inst(
    public val text: String,
    public val exec: Exec,
)

/** What assembling a controller yields: the instruction sequence, the label
 *  table it was resolved against, and exercise 5.12's summary with the
 *  5.12a per-type census. */
public class CompiledProgram(
    public val insts: List<Inst>,
    public val labels: Map<String, Int>,
    public val summary: AssemblySummary,
)

/** Exercise 5.12's summary, extended by 5.12a: the assembled instructions
 *  with duplicates removed and grouped by type, the registers the
 *  controller names overall, the entry-point registers (the `goto (reg
 *  r)` targets), the saved and restored registers, the assign sources of
 *  each register, the defined labels, and the per-type census. Every list
 *  is in a deterministic order: instruction texts and sources
 *  alphabetically, types in the book's listing order, registers
 *  alphabetically, labels in definition order. */
public class AssemblySummary(
    public val instructionCount: Int,
    public val instructionsByType: Map<String, List<String>>,
    public val registersUsed: List<String>,
    public val entryPointRegisters: List<String>,
    public val stackRegisters: List<String>,
    public val sourcesByRegister: Map<String, List<String>>,
    public val labels: List<String>,
    public val countsByType: Map<String, Int>,
)

/** Renders the summary the way the 5.12 report reads: one part per line.
 *  The last line is 5.12a's per-type census. */
public fun renderAssemblySummary(s: AssemblySummary): String =
    listOf(
        s.instructionsByType.entries.joinToString(
            prefix = "(instructions ",
            postfix = ")",
            separator = " ",
        ) { (kind, texts) -> "($kind ${texts.joinToString(" ")})" },
        "(registers ${s.registersUsed.joinToString(" ")})",
        "(entry-point registers ${s.entryPointRegisters.joinToString(" ")})",
        "(stack registers ${s.stackRegisters.joinToString(" ")})",
        s.sourcesByRegister.entries.joinToString(
            prefix = "(sources ",
            postfix = ")",
            separator = " ",
        ) { (name, sources) -> "($name ${sources.joinToString(" ")})" },
        "(labels ${s.labels.joinToString(" ")})",
        "(by type ${s.countsByType.entries.joinToString(" ") { (kind, count) -> "$kind $count" }})",
    ).joinToString(separator = "\n")

/** Renders one controller line the way the book writes it: the flat
 *  instruction form, `(assign val (op *) (reg n) (reg val))`. */
public fun renderStmt(stmt: Stmt): String =
    when (stmt) {
        is Label -> {
            stmt.name
        }

        is Assign -> {
            "(assign ${stmt.reg} ${renderSource(stmt.src, flat = true)})"
        }

        is Test -> {
            when (val cond = stmt.cond) {
                is OpCond -> "(test ${renderCall(cond.name, renderOperands(cond.args), flat = true)})"
            }
        }

        is Branch -> {
            "(branch (label ${stmt.label}))"
        }

        is Goto -> {
            when (val dest = stmt.to) {
                is GotoTarget.Lbl -> "(goto (label ${dest.name}))"
                is GotoTarget.ByReg -> "(goto (reg ${dest.reg}))"
            }
        }

        is Save -> {
            "(save ${stmt.reg})"
        }

        is Restore -> {
            "(restore ${stmt.reg})"
        }

        is Perform -> {
            when (val act = stmt.act) {
                is OpAct -> "(perform ${renderCall(act.name, renderOperands(act.args), flat = true)})"
            }
        }
    }

/** Renders one source or operand in the book's nested form,
 *  `((op *) (reg n) (reg val))`; with `flat`, an operation call renders the
 *  way it appears inside an instruction, `(op *) (reg n) (reg val)`. */
public fun renderSource(
    src: Source,
    flat: Boolean = false,
): String =
    when (src) {
        is Source.RegSrc -> "(reg ${src.reg})"
        is Source.ConstSrc -> "(const ${src.v})"
        is Source.LabelSrc -> "(label ${src.name})"
        is Source.OpSrc -> renderCall(src.name, renderOperands(src.args), flat)
    }

private fun renderOperands(args: List<Source>): String = args.joinToString(" ") { renderSource(it) }

/** An operation call: `(op *) (reg n) (reg val)` flat, `((op *) (reg n)
 *  (reg val))` nested; `(op print-stack-statistics)` with no operands. */
private fun renderCall(
    name: String,
    operands: String,
    flat: Boolean,
): String {
    val head = "(op $name)"
    val call = if (operands.isEmpty()) head else "$head $operands"
    return if (flat) call else "($call)"
}

/** The label scan: separates the labels from the instructions and points
 *  each label at the index of the instruction that follows it. A label used
 *  twice is exercise 5.8's assembly error. */
context(r: Raise<MachineError>)
public fun extractLabels(controller: List<Stmt>): Pair<List<Stmt>, Map<String, Int>> {
    val insts = ArrayList<Stmt>()
    val labels = LinkedHashMap<String, Int>()
    for (stmt in controller) {
        when (stmt) {
            is Label -> {
                if (stmt.name in labels) r.raise(MachineError.DuplicateLabel(stmt.name))
                labels[stmt.name] = insts.size
            }

            else -> {
                insts.add(stmt)
            }
        }
    }
    return insts to labels
}

/** The address a label names; an undefined label is an assembly error. */
context(r: Raise<MachineError>)
public fun lookupLabel(
    labels: Map<String, Int>,
    name: String,
): Int = labels[name] ?: r.raise(MachineError.UnknownLabel(name))

/** The registers a controller names: every assign target, every register
 *  source anywhere (assign sources, test and perform operands), every goto
 *  register, every save and restore register. Exercise 5.13 builds a
 *  machine's register list from this scan. */
public fun controllerRegisters(controller: List<Stmt>): Set<String> {
    val names = LinkedHashSet<String>()
    for (stmt in controller) {
        when (stmt) {
            is Assign -> {
                names.add(stmt.reg)
                sourceRegisters(stmt.src, names)
            }

            is Test -> {
                when (val cond = stmt.cond) {
                    is OpCond -> cond.args.forEach { sourceRegisters(it, names) }
                }
            }

            is Branch -> {}

            is Goto -> {
                when (val dest = stmt.to) {
                    is GotoTarget.ByReg -> {
                        names.add(dest.reg)
                    }

                    is GotoTarget.Lbl -> {}
                }
            }

            is Save -> {
                names.add(stmt.reg)
            }

            is Restore -> {
                names.add(stmt.reg)
            }

            is Label, is Perform -> {}
        }
    }
    return names
}

private fun sourceRegisters(
    src: Source,
    names: MutableSet<String>,
) {
    when (src) {
        is Source.RegSrc -> {
            names.add(src.reg)
        }

        is Source.OpSrc -> {
            src.args.forEach { sourceRegisters(it, names) }
        }

        is Source.ConstSrc, is Source.LabelSrc -> {}
    }
}

/** Exercise 5.12's census: the instructions grouped by type with
 *  duplicates removed, the registers used, the entry-point and stack
 *  registers, each register's assign sources, the defined labels, and
 *  (5.12a) the counts by type. */
public fun summarize(
    instructions: List<Stmt>,
    labels: Map<String, Int>,
): AssemblySummary {
    val byTypeOrder = linkedMapOf<String, MutableList<String>>()
    val byTypeCount = linkedMapOf<String, Int>()
    val sources = linkedMapOf<String, MutableSet<String>>()
    val entryPoints = sortedSetOf<String>()
    val stackRegs = sortedSetOf<String>()
    for (stmt in instructions) {
        val kind =
            when (stmt) {
                is Assign -> {
                    val sourceText = renderSource(stmt.src)
                    sources.getOrPut(stmt.reg) { sortedSetOf() }.add(sourceText)
                    "assign"
                }

                is Test -> {
                    "test"
                }

                is Branch -> {
                    "branch"
                }

                is Goto -> {
                    when (val dest = stmt.to) {
                        is GotoTarget.ByReg -> {
                            entryPoints.add(dest.reg)
                        }

                        is GotoTarget.Lbl -> {}
                    }
                    "goto"
                }

                is Save -> {
                    stackRegs.add(stmt.reg)
                    "save"
                }

                is Restore -> {
                    stackRegs.add(stmt.reg)
                    "restore"
                }

                is Perform -> {
                    "perform"
                }

                is Label -> {
                    continue
                }
            }
        byTypeOrder.getOrPut(kind) { mutableListOf() }.add(renderStmt(stmt))
        byTypeCount[kind] = (byTypeCount[kind] ?: 0) + 1
    }
    return AssemblySummary(
        instructionCount = instructions.size,
        instructionsByType = byTypeOrder.mapValues { it.value.distinct().sorted() },
        registersUsed = controllerRegisters(instructions).sorted(),
        entryPointRegisters = entryPoints.sorted(),
        stackRegisters = stackRegs.sorted(),
        sourcesByRegister = sources.mapValues { it.value.sorted() },
        labels = labels.keys.toList(),
        countsByType = byTypeCount,
    )
}

/** The assembler: scans the controller and pairs each instruction's text
 *  with its execution procedure, gathering the 5.12 summary. With
 *  [strictLabels], using a label as an operation operand is an assembly
 *  error (exercise 5.9); without it, the book's base accepts labels
 *  wherever a primitive expression may stand. */
context(r: Raise<MachineError>)
public fun assemble(
    controller: List<Stmt>,
    machine: Machine,
    strictLabels: Boolean = false,
): CompiledProgram {
    val (insts, labels) = extractLabels(controller)
    val compiled =
        insts.map { inst -> Inst(renderStmt(inst), executionProcedure(inst, machine, labels, strictLabels)) }
    return CompiledProgram(compiled, labels, summarize(insts, labels))
}

/** The book's `make-execution-procedure`: one generator per instruction
 *  type. A `Label` never reaches here; the scan removed it. */
context(r: Raise<MachineError>)
public fun executionProcedure(
    stmt: Stmt,
    machine: Machine,
    labels: Map<String, Int>,
    strictLabels: Boolean = false,
): Exec =
    when (stmt) {
        is Assign -> assignProc(stmt, machine, labels, strictLabels)
        is Test -> testProc(stmt, machine, labels, strictLabels)
        is Branch -> branchProc(stmt, machine, labels)
        is Goto -> gotoProc(stmt, machine, labels)
        is Save -> saveProc(stmt, machine)
        is Restore -> restoreProc(stmt, machine)
        is Perform -> performProc(stmt, machine, labels, strictLabels)
        is Label -> throw IllegalStateException("a label names a place, it does not execute")
    }

/** `make-assign`: stores the source's value into the target register and
 *  advances `pc`. */
context(r: Raise<MachineError>)
public fun assignProc(
    inst: Assign,
    machine: Machine,
    labels: Map<String, Int>,
    strictLabels: Boolean = false,
): Exec {
    val target = machine.registerFor(inst.reg)
    val source = primitiveExp(inst.src, machine, labels, strictLabels)
    return { r ->
        target.store(source(r))
        machine.pc += 1
    }
}

/** `make-test`: sets the flag from the condition and advances `pc`. */
context(r: Raise<MachineError>)
public fun testProc(
    inst: Test,
    machine: Machine,
    labels: Map<String, Int>,
    strictLabels: Boolean = false,
): Exec {
    val condition =
        when (val cond = inst.cond) {
            is OpCond -> operationValue(cond.name, cond.args, machine, labels, strictLabels)
        }
    return { r ->
        machine.flag.store(condition(r))
        machine.pc += 1
    }
}

/** `make-branch`: the destination must be a label (the ADT enforces it);
 *  the branch consults the flag, which a test must have set. */
context(r: Raise<MachineError>)
public fun branchProc(
    inst: Branch,
    machine: Machine,
    labels: Map<String, Int>,
): Exec {
    val address = lookupLabel(labels, inst.label)
    return { r ->
        machine.pc =
            when (val flagValue = machine.flag.content) {
                is VBool -> if (flagValue.b) address else machine.pc + 1
                else -> r.raise(MachineError.BranchWithoutTest)
            }
    }
}

/** `make-goto`: the destination is a label or a register holding a label
 *  address; there is no condition to check. */
context(r: Raise<MachineError>)
public fun gotoProc(
    inst: Goto,
    machine: Machine,
    labels: Map<String, Int>,
): Exec =
    when (val dest = inst.to) {
        is GotoTarget.Lbl -> {
            val address = lookupLabel(labels, dest.name)
            val exec: Exec = { _ -> machine.pc = address }
            exec
        }

        is GotoTarget.ByReg -> {
            val target = machine.registerFor(dest.reg)
            val exec: Exec =
                { r ->
                    machine.pc =
                        when (val value = target.content) {
                            is VInt -> value.n.toInt()
                            else -> r.raise(MachineError.BadGotoTarget(dest.reg, value))
                        }
                }
            exec
        }
    }

/** `make-save`: pushes the register's contents and advances `pc`. */
context(r: Raise<MachineError>)
public fun saveProc(
    inst: Save,
    machine: Machine,
): Exec {
    val source = machine.registerFor(inst.reg)
    return { r ->
        machine.stack.push(source.content)
        machine.pc += 1
    }
}

/** `make-restore`: pops into the register; an empty stack underflows with
 *  the register's name. Exercise 5.11's disciplines replace this builder. */
context(r: Raise<MachineError>)
public fun restoreProc(
    inst: Restore,
    machine: Machine,
): Exec {
    val target = machine.registerFor(inst.reg)
    return { r ->
        target.store(machine.stack.pop(inst.reg, r))
        machine.pc += 1
    }
}

/** `make-perform`: runs the action for its side effect and advances
 *  `pc`. */
context(r: Raise<MachineError>)
public fun performProc(
    inst: Perform,
    machine: Machine,
    labels: Map<String, Int>,
    strictLabels: Boolean = false,
): Exec {
    val action =
        when (val act = inst.act) {
            is OpAct -> operationValue(act.name, act.args, machine, labels, strictLabels)
        }
    return { r ->
        action(r)
        machine.pc += 1
    }
}

/** `make-primitive-exp`: a value proc for a `reg`, `const`, or `label`
 *  expression. A label here names its instruction address. */
context(r: Raise<MachineError>)
public fun primitiveExp(
    exp: Source,
    machine: Machine,
    labels: Map<String, Int>,
    strictLabels: Boolean = false,
): ValueProc =
    when (exp) {
        is Source.RegSrc -> {
            val source = machine.registerFor(exp.reg)
            val read: ValueProc = { _ -> source.content }
            read
        }

        is Source.ConstSrc -> {
            val value = exp.v
            val read: ValueProc = { _ -> value }
            read
        }

        is Source.LabelSrc -> {
            val address = VInt(lookupLabel(labels, exp.name).toLong())
            val read: ValueProc = { _ -> address }
            read
        }

        is Source.OpSrc -> {
            operationValue(exp.name, exp.args, machine, labels, strictLabels)
        }
    }

/** `make-operation-exp`: looks the operation up now, builds a value proc
 *  per operand now, and at run time applies the operation to the operand
 *  values. With [strictLabels], an operand written `(label x)` is the
 *  assembly error of exercise 5.9. */
context(r: Raise<MachineError>)
public fun operationValue(
    name: String,
    args: List<Source>,
    machine: Machine,
    labels: Map<String, Int>,
    strictLabels: Boolean = false,
): ValueProc {
    val prim = lookupPrim(machine, name)
    val argProcs = args.map { operandValue(it, machine, labels, strictLabels) }
    return { r -> prim(r, argProcs.map { it(r) }) }
}

/** One operation operand: a label operand under [strictLabels] is the
 *  exercise 5.9 error; everything else is a primitive expression. */
context(r: Raise<MachineError>)
private fun operandValue(
    exp: Source,
    machine: Machine,
    labels: Map<String, Int>,
    strictLabels: Boolean,
): ValueProc =
    when {
        exp is Source.LabelSrc && strictLabels -> r.raise(MachineError.LabelOperand(exp.name))
        else -> primitiveExp(exp, machine, labels, strictLabels)
    }

/** `lookup-prim`: finds the operation name in the machine's table. */
context(r: Raise<MachineError>)
public fun lookupPrim(
    machine: Machine,
    name: String,
): Op = machine.operations[name] ?: r.raise(MachineError.UnknownOperation(name))
