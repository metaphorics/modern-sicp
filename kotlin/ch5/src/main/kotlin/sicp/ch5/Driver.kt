// SPDX-License-Identifier: GPL-3.0-only
package sicp.ch5

import arrow.core.Either
import arrow.core.raise.either
import sicp.guest.Admission
import sicp.guest.AdmissionError
import sicp.guest.GValue
import sicp.guest.GuestError
import sicp.guest.Mode
import sicp.guest.OutputSink
import sicp.guest.RunResult
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Machine
import sicp.runtime.MachineOp
import sicp.runtime.OpAct
import sicp.runtime.OpCond
import sicp.runtime.Perform
import sicp.runtime.Restore
import sicp.runtime.Save
import sicp.runtime.Source
import sicp.runtime.Source.ConstSrc
import sicp.runtime.Source.OpSrc
import sicp.runtime.Source.RegSrc
import sicp.runtime.Stmt
import sicp.runtime.Test

/**
 * The Kotlin teaching driver for the conformance contract: one JSON object
 * on stdout — `termination` and `observation` transcript — with all
 * diagnostics on stderr. Guest output is captured as the observation; a
 * source admission failure is `rejected` before any effect; a guest runtime
 * fault is `error`.
 */
public object Driver {
    @JvmStatic
    public fun main(args: Array<String>) {
        if (args.contains("--native-compile")) {
            val source = flagValue(args, "--source") ?: kotlin.system.exitProcess(2)
            val work = flagValue(args, "--work") ?: kotlin.system.exitProcess(2)
            kotlin.system.exitProcess(NativeOracle.compile(source, work))
        }
        if (args.contains("--native-run")) {
            val work = flagValue(args, "--work") ?: kotlin.system.exitProcess(2)
            kotlin.system.exitProcess(NativeOracle.run(work))
        }
        val options = parseOptions(args)
        if (options == null) {
            System.err.println("usage: ch5 --case <id> --engine <name> --source <path> [--work <dir>]")
            kotlin.system.exitProcess(2)
        }
        val outcome = runCase(options)
        if (outcome == null) {
            System.err.println("unknown engine: ${options.engine}")
            kotlin.system.exitProcess(2)
        }
        println(jsonObject(outcome.termination, outcome.transcript))
    }

    private fun flagValue(
        args: Array<String>,
        flag: String,
    ): String? {
        val index = args.indexOf(flag)
        if (index < 0 || index + 1 >= args.size) return null
        return args[index + 1]
    }

    private class Options(
        val caseId: String,
        val engine: String,
        val source: String,
    )

    private fun parseOptions(args: Array<String>): Options? {
        var caseId: String? = null
        var engine: String? = null
        var source: String? = null
        var index = 0
        while (index + 1 < args.size) {
            val key = args[index]
            val value = args[index + 1]
            if (key == "--case") caseId = value
            if (key == "--engine") engine = value
            if (key == "--source") source = value
            index += 2
        }
        val id = caseId ?: return null
        val name = engine ?: return null
        val path = source ?: return null
        return Options(id, name, path)
    }

    private fun runCase(options: Options): ReferenceModels.Observation? {
        val text = java.io.File(options.source).readText()
        if (options.engine == "machine") return machineEngine(text)
        if (options.engine == "reference") return referenceFor(options.caseId, text)
        val mode = modeFor(options.caseId)
        return when (options.engine) {
            "direct" -> observationOfRun(sicp.ch4.Direct.run(text, mode))
            "analyzed" -> observationOfRun(sicp.ch4.Analyzed.run(text, mode))
            "eceval" -> observationOfRun(ExplicitControl.run(text, mode))
            "compiled" -> compiledObservation(text, mode)
            "lazy" -> lazyObservation(text)
            "search" -> searchObservation(text)
            "query" -> QueryCase.observe(text).fold(::rejection) { it }
            else -> null
        }
    }

    private fun modeFor(caseId: String): Mode = Mode.CORE

    private fun referenceFor(
        caseId: String,
        text: String,
    ): ReferenceModels.Observation =
        when {
            caseId.startsWith("lazy/") -> ReferenceModels.lazyCase(text).fold(::rejection) { it }
            caseId.startsWith("amb/") -> ReferenceModels.searchCase(text).fold(::rejection) { it }
            caseId.startsWith("machine/") -> ReferenceModels.machineCase(text)
            else -> ReferenceModels.queryCase(text).fold(::rejection) { it }
        }

    private fun rejection(error: AdmissionError): ReferenceModels.Observation {
        System.err.println("${error.position}: ${error.category}: ${error.message}")
        return ReferenceModels.Observation("rejected", "")
    }

    private fun observationOfRun(result: Either<AdmissionError, RunResult>): ReferenceModels.Observation =
        result.fold(::rejection) { runObservation(it) }

    private fun runObservation(result: RunResult): ReferenceModels.Observation =
        if (result.error != null) {
            ReferenceModels.Observation("error", result.output)
        } else {
            ReferenceModels.Observation("value", result.output)
        }

    private fun compiledObservation(
        text: String,
        mode: Mode,
    ): ReferenceModels.Observation =
        either<AdmissionError, RunResult> {
            val checked = Admission.admitOrRaise(text, mode)
            Compiler.compileAndRun(checked)
        }.fold(::rejection) { runObservation(it) }

    private fun lazyObservation(text: String): ReferenceModels.Observation =
        sicp.ch4.LazyModule
            .run(text)
            .fold(::rejection) { runObservation(it.result) }

    private fun searchObservation(text: String): ReferenceModels.Observation =
        sicp.ch4.SearchModule
            .run(text)
            .fold(::rejection) { runObservation(it.result) }

    /** The teaching machine engine: the case's controller description assembles
     * to real section 4.4 instructions and runs on the register machine. */
    private fun machineEngine(text: String): ReferenceModels.Observation {
        val parsed = MachineScript.parse(text) ?: return ReferenceModels.Observation("rejected", "")
        val sink = OutputSink()
        val machine = Machine(parsed.registerNames.toSet(), MachineScript.operations(sink), parsed.instructions)
        for ((name, value) in parsed.registers) {
            val register = machine.registers[name] ?: continue
            register.content = GValue.VLong(value)
        }
        val outcome =
            either<GuestError, GValue> {
                machine.run()
                GValue.VUnit
            }
        val finals = parsed.registerNames.joinToString(" ") { "$it=${renderRegister(machine, it)}" }
        val transcript = (sink.contents().lines().filter { it.isNotEmpty() } + "final: $finals").joinToString("\n") + "\n"
        return if (outcome.isLeft()) {
            ReferenceModels.Observation("error", transcript)
        } else {
            ReferenceModels.Observation("value", transcript)
        }
    }

    private fun renderRegister(
        machine: Machine,
        name: String,
    ): String =
        when (val content = machine.registers[name]?.content ?: GValue.VUnit) {
            is GValue.VLong -> content.value.toString()
            is GValue.VInt -> content.value.toString()
            else -> "0"
        }

    private fun jsonObject(
        termination: String,
        transcript: String,
    ): String = """{"termination": "${escape(termination)}", "stdout": "${escape(transcript)}"}"""

    private fun escape(text: String): String {
        val out = StringBuilder()
        for (character in text) {
            when (character) {
                '\\' -> out.append("\\\\")
                '"' -> out.append("\\\"")
                '\n' -> out.append("\\n")
                '\r' -> out.append("\\r")
                '\t' -> out.append("\\t")
                else -> out.append(character)
            }
        }
        return out.toString()
    }
}

/** The machine case script: parse the controller description once and use
 * it for both the instruction assembly and the final-state rendering. */
internal object MachineScript {
    class Parsed(
        val registerNames: List<String>,
        val registers: Map<String, Long>,
        val instructions: List<Stmt>,
    )

    fun parse(text: String): Parsed? {
        val registerNames = mutableListOf<String>()
        val registers = linkedMapOf<String, Long>()
        val instructions = mutableListOf<Stmt>()
        val labels = linkedMapOf<String, Int>()
        for (rawLine in text.lineSequence()) {
            val line = rawLine.trim()
            if (line.isEmpty() || line.startsWith("#")) continue
            if (line.startsWith("registers")) {
                registerNames.addAll(line.removePrefix("registers").trim().split(Regex("\\s+")))
                continue
            }
            if (line.startsWith("const")) {
                val parts = line.removePrefix("const").trim().split(Regex("\\s+"))
                val value = parts.getOrNull(1)?.toLongOrNull() ?: return null
                registers[parts[0]] = value
                continue
            }
            if (line.endsWith(":")) {
                labels[line.dropLast(1)] = instructions.size
                continue
            }
            instructions.add(instruction(line) ?: return null)
        }
        val labeled = instructions.toMutableList()
        var offset = 0
        for ((name, index) in labels) {
            labeled.add(index + offset, Label(name))
            offset++
        }
        return Parsed(registerNames, registers, labeled)
    }

    fun operations(sink: OutputSink): Map<String, MachineOp> {
        val ops = linkedMapOf<String, MachineOp>()
        ops["print"] = { args ->
            val value = args.firstOrNull() ?: GValue.VLong(0L)
            sink.write("${renderValue(value)}\n")
            GValue.VUnit
        }
        ops["zero"] = { args -> GValue.VBool(asLong(args.getOrElse(0) { GValue.VLong(0L) }) == 0L) }
        ops["eq"] =
            { args -> GValue.VBool(asLong(args.getOrElse(0) { GValue.VLong(0L) }) == asLong(args.getOrElse(1) { GValue.VLong(0L) })) }
        ops["lt"] =
            { args -> GValue.VBool(asLong(args.getOrElse(0) { GValue.VLong(0L) }) < asLong(args.getOrElse(1) { GValue.VLong(0L) })) }
        ops["add"] =
            { args -> GValue.VLong(asLong(args.getOrElse(0) { GValue.VLong(0L) }) + asLong(args.getOrElse(1) { GValue.VLong(0L) })) }
        ops["sub"] =
            { args -> GValue.VLong(asLong(args.getOrElse(0) { GValue.VLong(0L) }) - asLong(args.getOrElse(1) { GValue.VLong(0L) })) }
        ops["mul"] =
            { args -> GValue.VLong(asLong(args.getOrElse(0) { GValue.VLong(0L) }) * asLong(args.getOrElse(1) { GValue.VLong(0L) })) }
        ops["rem"] =
            { args -> GValue.VLong(asLong(args.getOrElse(0) { GValue.VLong(0L) }) % asLong(args.getOrElse(1) { GValue.VLong(0L) })) }
        ops["const"] = { args -> args[0] }
        ops["reg"] = { args -> args[0] }
        ops["op"] = { args -> args[0] }
        return ops
    }

    private fun renderValue(value: GValue): String =
        when (value) {
            is GValue.VLong -> value.value.toString()
            is GValue.VInt -> value.value.toString()
            else -> value.toString()
        }

    private fun asLong(value: GValue): Long =
        when (value) {
            is GValue.VLong -> value.value
            is GValue.VInt -> value.value.toLong()
            else -> 0L
        }

    private fun instruction(line: String): Stmt? {
        val words = line.split(Regex("\\s+"))
        val head = words[0]
        if (head == "assign") return assignInstruction(words)
        if (head == "test") return Test(OpCond(words[1], operandSources(words.subList(2, words.size))))
        if (head == "branch") return Branch(words[1])
        if (head == "goto") return Goto(GotoTarget.Lbl(words[1]))
        if (head == "save") return Save(words[1])
        if (head == "restore") return Restore(words[1])
        if (head == "perform") return Perform(OpAct(words[1], operandSources(words.subList(2, words.size))))
        return null
    }

    private fun assignInstruction(words: List<String>): Stmt {
        val target = words[1]
        val source = words.subList(3, words.size)
        return Assign(target, sourceFrom(source))
    }

    private fun sourceFrom(source: List<String>): Source {
        if (source[0] == "const") return ConstSrc(GValue.VLong(source[1].toLong()))
        if (source[0] == "reg") return RegSrc(source[1])
        return OpSrc(source[1], operandSources(source.subList(2, source.size)))
    }

    private fun operandSources(operands: List<String>): List<Source> =
        operands.map { operand ->
            if (operand.startsWith("#")) ConstSrc(GValue.VLong(operand.drop(1).toLong())) else RegSrc(operand)
        }
}
