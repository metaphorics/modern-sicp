// SPDX-License-Identifier: GPL-3.0-only
// The conformance corpus, capability `core` (D23): every row the manifest
// assigns to all four editions runs through this edition's reader, parser,
// evaluator, and printer, and the transcript compares byte-for-byte with
// spec/scheme-subset/expected/, from the third line (the first two are the
// license header) to the final LF. `metacircular.scm` -- the chapter's own
// evaluator written in the object language -- is one of the rows.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import java.io.File

private val specRoot = File("../../spec/scheme-subset")

private data class CorpusRow(
    val capability: String,
    val program: String,
    val expected: String,
    val mode: String,
    val evaluators: String,
)

private fun manifestRows(): List<CorpusRow> =
    File(specRoot, "manifest.txt")
        .readLines()
        .filter { it.isNotBlank() && !it.startsWith("#") && !it.startsWith(";") }
        .map { line ->
            val cells = line.split("\t")
            CorpusRow(cells[0], cells[1], cells[2], cells[3], cells[4])
        }

private fun expectedText(row: CorpusRow): String {
    val lines = File(specRoot, "expected/${row.expected}").readLines()
    return lines.drop(2).joinToString(separator = "\n", postfix = "\n")
}

/** The bytecode-identical transcript of one corpus program, `sink` reused. */
private fun runCorpusProgram(program: String): String {
    val sink = sicp.ch4.OutputSink()
    return sicp.ch4.runProgram(File(specRoot, "programs/$program").readText(), sicp.ch4.setupEnvironment(sink), sink)
}

public class CorpusCoreTest :
    FunSpec({
        val rows = manifestRows().filter { it.capability == "core" && it.evaluators == "all" }

        test("the manifest assigns core rows to every edition, including this one") {
            rows.size shouldBe 21
            rows.map { it.program }.contains("core/metacircular.scm") shouldBe true
        }

        for (row in rows) {
            val name = row.program.substringAfterLast('/').removeSuffix(".scm")
            test("corpus $name output matches the expected bytes") {
                runCorpusProgram(row.program) shouldBe expectedText(row)
            }
        }
    })
