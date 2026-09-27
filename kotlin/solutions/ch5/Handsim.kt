// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, section 5.1: the hand-transcription machinery shared by the
// section's exercise solutions. This is NOT the register-machine simulator
// of 5.2: there is no controller parser, no register objects, and no
// machine model. Each machine is transcribed by hand into the instruction
// list below, and `run` steps that list the way a reader steps the
// controller diagram, recording the edition's trace events.

package sicp.ch5.solutions

/** One value the section's machines hold: a register, the stack, or the
 *  flag. The machines of 5.1 use integers, reals (the sqrt machine),
 *  booleans (test results), and controller labels stored in `continue`. */
public sealed interface HandVal

public data class HNum(
    val value: Long,
) : HandVal

public data class HReal(
    val value: Double,
) : HandVal

public data class HBool(
    val value: Boolean,
) : HandVal

public data class HLabel(
    val name: String,
) : HandVal

/** Rendered the way the edition prints values: booleans as `#t`/`#f`
 *  (the printer contract), a label as its bare name, reals at the full
 *  host precision Kotlin's `Double.toString` gives. */
public fun render(value: HandVal): String =
    when (value) {
        is HNum -> value.value.toString()
        is HReal -> value.value.toString()
        is HBool -> if (value.value) "#t" else "#f"
        is HLabel -> value.name
    }

/** One operand source of an operation call: a register or a constant, the
 *  section's rule that operations read registers and constants only. */
public sealed interface HandArg

public data class HArgReg(
    val name: String,
) : HandArg

public data class HArgConst(
    val value: HandVal,
) : HandArg

public fun argReg(name: String): HArgReg = HArgReg(name)

public fun argNum(value: Long): HArgConst = HArgConst(HNum(value))

public fun argReal(value: Double): HArgConst = HArgConst(HReal(value))

/** One controller instruction: the instruction forms of 5.1.5 that the
 *  section's machines use. */
public sealed interface HandInstruction

public data class HLabelDef(
    val name: String,
) : HandInstruction

public data class HAssignReg(
    val target: String,
    val source: String,
) : HandInstruction

public data class HAssignConst(
    val target: String,
    val source: HandVal,
) : HandInstruction

public data class HAssignLabel(
    val target: String,
    val label: String,
) : HandInstruction

public data class HAssignOp(
    val target: String,
    val op: String,
    val args: List<HandArg>,
) : HandInstruction

public data class HTest(
    val op: String,
    val args: List<HandArg>,
) : HandInstruction

public data class HBranch(
    val label: String,
) : HandInstruction

public data class HGotoLabel(
    val label: String,
) : HandInstruction

public data class HGotoReg(
    val reg: String,
) : HandInstruction

public data class HPerform(
    val op: String,
    val args: List<HandArg>,
) : HandInstruction

public data class HSave(
    val reg: String,
) : HandInstruction

public data class HRestore(
    val reg: String,
) : HandInstruction

/** One entry of a machine's operations table. */
public fun interface HandOp {
    public operator fun invoke(args: List<HandVal>): HandVal
}

public fun handOp1(op: (HandVal) -> HandVal): HandOp = HandOp { args -> op(args[0]) }

public fun handOp2(op: (HandVal, HandVal) -> HandVal): HandOp = HandOp { args -> op(args[0], args[1]) }

private fun asDouble(value: HandVal): Double =
    when (value) {
        is HNum -> value.value.toDouble()
        is HReal -> value.value
        else -> error("hand simulation: expected a number, found ${render(value)}")
    }

/** The section's shared arithmetic operations: the primitive devices the
 *  machines assume (the arithmetic of 1.1.7, with `abs` where 5.3 uses it). */
public val handArithOps: Map<String, HandOp> =
    mapOf(
        "+" to
            handOp2 { a, b ->
                if (a is HNum && b is HNum) HNum(a.value + b.value) else HReal(asDouble(a) + asDouble(b))
            },
        "-" to
            handOp2 { a, b ->
                if (a is HNum && b is HNum) HNum(a.value - b.value) else HReal(asDouble(a) - asDouble(b))
            },
        "*" to
            handOp2 { a, b ->
                if (a is HNum && b is HNum) HNum(a.value * b.value) else HReal(asDouble(a) * asDouble(b))
            },
        "/" to
            handOp2 { a, b ->
                if (a is HNum && b is HNum) HNum(a.value / b.value) else HReal(asDouble(a) / asDouble(b))
            },
        "=" to handOp2 { a, b -> HBool(asDouble(a) == asDouble(b)) },
        "<" to handOp2 { a, b -> HBool(asDouble(a) < asDouble(b)) },
        ">" to handOp2 { a, b -> HBool(asDouble(a) > asDouble(b)) },
        "abs" to
            handOp1 { a ->
                if (a is HNum) HNum(kotlin.math.abs(a.value)) else HReal(kotlin.math.abs(asDouble(a)))
            },
    )

/** A `print` operation that appends the rendered value to [out], the way
 *  the driver-loop machines of 5.1.2 observe a result. */
public fun handPrintOp(out: StringBuilder): HandOp =
    HandOp { args ->
        out.append(render(args[0]))
        args[0]
    }

/** One hand-transcribed machine: an instruction list plus its operations
 *  table. Labels resolve once, at construction: a label names the position
 *  of the instruction that follows it, counting instructions only, and a
 *  trailing label names the stop address one past the last instruction,
 *  the section's exit convention. The label lines themselves occupy no
 *  slot: the machine executes the resolved instruction sequence. */
public class HandSim(
    instructions: List<HandInstruction>,
    public val operations: Map<String, HandOp>,
) {
    /** The controller's instructions, labels resolved away. */
    public val code: List<HandInstruction>

    /** Each label's position in [code]. */
    public val labels: Map<String, Int>

    init {
        val (text, table) = resolveLabels(instructions)
        code = text
        labels = table
    }
}

private fun resolveLabels(instructions: List<HandInstruction>): Pair<List<HandInstruction>, Map<String, Int>> {
    val code = mutableListOf<HandInstruction>()
    val table = HashMap<String, Int>()
    var pending = emptyList<String>()
    for (instruction in instructions) {
        if (instruction is HLabelDef) {
            pending = pending + instruction.name
            continue
        }
        for (name in pending) {
            table[name] = code.size
        }
        pending = emptyList()
        code.add(instruction)
    }
    for (name in pending) {
        table[name] = code.size
    }
    return code.toList() to table
}

/** How a hand simulation ends: halted at the stop address, or stopped by
 *  the step budget (a transcription whose controller loops). */
public sealed interface HandOutcome

public class HandHalted(
    public val registers: Map<String, HandVal>,
    public val stack: List<HandVal>,
    public val steps: Int,
    public val saves: Int,
    public val maxDepth: Int,
    public val events: List<String>,
) : HandOutcome {
    /** The value the machine answers in [reg] when it stops. */
    public fun answer(reg: String): HandVal = registers.getValue(reg)
}

public data object OutOfFuel : HandOutcome

/** How a run ends when the controller reaches a state no correct machine
 *  reaches: the hand model names the impossible state instead of halting. */
public class HandStuck(
    public val reason: String,
) : HandOutcome

/** One run's mutable state: pc, flag, the register file, the stack, the
 *  step and save counters, and the trace events gathered so far. */
private class HandRun(
    val sim: HandSim,
    initialRegisters: Map<String, HandVal>,
) {
    val registers = HashMap(initialRegisters)
    val stack = ArrayDeque<HandVal>()
    val events = mutableListOf<String>()
    var pc = 0
    var flag = false
    var steps = 0
    var saves = 0
    var maxDepth = 0
    var stuck: String? = null

    /** Execute one instruction; answer false when the run is out of fuel
     *  or the controller reached a state no correct machine reaches. */
    fun step(fuel: Int): Boolean {
        if (stuck != null || steps >= fuel) {
            return false
        }
        when (val instruction = sim.code[pc]) {
            is HLabelDef -> {
                error("hand simulation: a label line survived resolution")
            }

            is HAssignReg -> {
                registers[instruction.target] = fetch(registers, instruction.source)
                advance()
            }

            is HAssignConst -> {
                registers[instruction.target] = instruction.source
                advance()
            }

            is HAssignLabel -> {
                registers[instruction.target] = HLabel(instruction.label)
                advance()
            }

            is HAssignOp -> {
                registers[instruction.target] = call(instruction.op, instruction.args)
                advance()
            }

            is HTest -> {
                flag = asBool(call(instruction.op, instruction.args))
                advance()
            }

            is HBranch -> {
                if (!flag) {
                    advance()
                } else {
                    events.add("branch taken to ${instruction.label}")
                    pc = jump(instruction.label)
                    steps += 1
                }
            }

            is HGotoLabel -> {
                pc = jump(instruction.label)
                steps += 1
            }

            is HGotoReg -> {
                when (val target = fetch(registers, instruction.reg)) {
                    is HLabel -> {
                        events.add("return to ${target.name}")
                        pc = jump(target.name)
                        steps += 1
                    }

                    else -> {
                        error("hand simulation: ${instruction.reg} holds ${render(target)}, not a label")
                    }
                }
            }

            is HPerform -> {
                call(instruction.op, instruction.args)
                advance()
            }

            is HSave -> {
                val value = fetch(registers, instruction.reg)
                stack.addFirst(value)
                saves += 1
                if (stack.size > maxDepth) {
                    maxDepth = stack.size
                }
                events.add("(save ${instruction.reg}) stack=${stackText()}")
                advance()
            }

            is HRestore -> {
                val value = stack.removeFirstOrNull()
                if (value == null) {
                    stuck = "restore ${instruction.reg} from an empty stack"
                    return false
                }
                registers[instruction.reg] = value
                events.add("(restore ${instruction.reg}) ${instruction.reg}=${render(value)} stack=${stackText()}")
                advance()
            }
        }
        return true
    }

    private fun advance() {
        pc += 1
        steps += 1
    }

    private fun jump(label: String): Int = sim.labels[label] ?: error("hand simulation: no label $label")

    private fun asBool(answer: HandVal): Boolean =
        when (answer) {
            is HBool -> answer.value
            else -> error("hand simulation: the test answered ${render(answer)}, not a boolean")
        }

    private fun call(
        op: String,
        args: List<HandArg>,
    ): HandVal {
        val operation = sim.operations[op] ?: error("hand simulation: no operation $op")
        return operation(
            args.map { arg ->
                when (arg) {
                    is HArgReg -> fetch(registers, arg.name)
                    is HArgConst -> arg.value
                }
            },
        )
    }

    private fun stackText(): String = stack.joinToString(" ", prefix = "(", postfix = ")") { render(it) }
}

private fun fetch(
    registers: Map<String, HandVal>,
    name: String,
): HandVal = registers[name] ?: error("hand simulation: the register $name is unset")

/** Step the transcription from [initialRegisters] until the controller
 *  reaches its stop address. Every executed instruction counts one step;
 *  label entries and untaken branches produce no trace event. */
public fun HandSim.run(
    initialRegisters: Map<String, HandVal>,
    fuel: Int = 1_000_000,
): HandOutcome {
    val run = HandRun(this, initialRegisters)
    while (run.pc < code.size) {
        if (!run.step(fuel)) {
            val reason = run.stuck
            return if (reason != null) HandStuck(reason) else OutOfFuel
        }
    }
    return HandHalted(
        registers = run.registers,
        stack = run.stack.toList(),
        steps = run.steps,
        saves = run.saves,
        maxDepth = run.maxDepth,
        events = run.events.toList(),
    )
}
