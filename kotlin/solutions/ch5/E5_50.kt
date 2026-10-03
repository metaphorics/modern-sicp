// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.50: use the compiler to compile the metacircular
// evaluator and run it on the register machine. In this edition the
// evaluator is the canonical guest self-interpreter of the corpus
// (spec/host-subsets/kotlin/corpus/selfinterp/evaluator.kt): guest source
// like any other program, admitted and type-checked before any effect,
// then executed three ways -- direct execution, the teaching
// (explicit-control) evaluator, and the compiled machine. The machine
// run interprets the kernel, which interprets the target program the
// kernel's `main` builds: real self-interpretation, three levels deep,
// and the exercise's observable is that all three executions answer
// alike.

package sicp.ch5.solutions

import arrow.core.raise.either
import sicp.ch4.Direct
import sicp.ch5.Compiler
import sicp.ch5.ExplicitControl
import sicp.guest.Admission
import sicp.guest.CheckedProgram
import sicp.guest.GValue
import sicp.guest.Mode
import sicp.guest.RunResult
import java.io.File

/** The canonical guest self-interpreter, read from the corpus at the
 *  repository root: one evaluator source for every consumer (the 5.50
 *  runs here and the 5.52 C translation), never a forked copy. */
public val metacircularEvaluatorSource: String = readCanonicalEvaluator()

private fun readCanonicalEvaluator(): String {
    var directory: File? = File(System.getProperty("user.dir")).absoluteFile
    while (directory != null) {
        val candidate = File(directory, "spec/host-subsets/kotlin/corpus/selfinterp/evaluator.kt")
        if (candidate.isFile) return candidate.readText()
        directory = directory.parentFile
    }
    error(
        "the canonical self-interpreter is missing: " +
            "spec/host-subsets/kotlin/corpus/selfinterp/evaluator.kt",
    )
}

/** The evaluator source admitted once and shared by the three runs. */
public fun admitMetacircular(): CheckedProgram =
    Admission.admit(metacircularEvaluatorSource, Mode.CORE).fold(
        { error -> error("the evaluator source did not admit: $error") },
        { it },
    )

/** One run's answer, the kernel's printed observation. The evaluator's
 *  `main` returns Unit and prints the interpreted factorial, so the answer
 *  is the transcript line, identical on all three engines. */
private fun answerOf(result: RunResult): String =
    result.output.trim().ifEmpty {
        "no completion: ${result.error?.category ?: "search exhausted"}"
    }

/** The three executions of the corpus evaluator and their agreement:
 *  each answers the kernel's 120, and the self-interpretation holds
 *  exactly when the three answers coincide. */
public fun compiledMetacircularRuns(): List<String> {
    val checked = admitMetacircular()
    val direct = Direct.run(checked)
    val teaching = ExplicitControl.run(checked)
    val compiled = Compiler.compileAndRun(checked)
    val answers = listOf(answerOf(direct), answerOf(teaching), answerOf(compiled))
    val clean = listOf(direct, teaching, compiled).all { it.error == null }
    return answers.mapIndexed { index, answer ->
        val names = listOf("direct answer", "explicit-control answer", "compiled machine answer")
        "${names[index]}: $answer"
    } + "the three executions agree: ${answers.distinct().size == 1 && answers[0] == "120" && clean}"
}
