// SPDX-License-Identifier: GPL-3.0-only
// The conformance corpus (D23): every core row of the shared manifest is
// checked guest source that runs through the direct, analyzed,
// explicit-control, and compiled engines, and the four engines must agree
// on every observable -- the output stream, the typed fault category, and
// main's completion -- so no execution path can drift from another. The
// native comparison and the expected transcripts are the parent's
// conformance gate over spec/host-subsets/kotlin/driver.json.

package sicp.ch5.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.Analyzed
import sicp.ch4.Direct
import sicp.ch5.Compiler
import sicp.ch5.ExplicitControl
import sicp.guest.Admission
import sicp.guest.CheckedProgram
import sicp.guest.GValue
import sicp.guest.Mode
import sicp.guest.RunResult
import sicp.guest.renderPrinted
import java.io.File

/** The corpus directory, found from the working directory upward so the test
 * does not depend on which directory the runner starts in. */
private val corpusRoot: File = locateCorpus()

private fun locateCorpus(): File {
    var directory: File? = File(System.getProperty("user.dir")).absoluteFile
    while (directory != null) {
        val candidate = File(directory, "spec/host-subsets/kotlin")
        if (File(candidate, "manifest.tsv").isFile) return candidate
        directory = directory.parentFile
    }
    throw AssertionError("the corpus manifest spec/host-subsets/kotlin/manifest.tsv is not above ${System.getProperty("user.dir")}")
}

private data class CorpusRow(
    val id: String,
    val fixture: String,
)

private fun coreRows(): List<CorpusRow> =
    File(corpusRoot, "manifest.tsv")
        .readLines()
        .filter { it.isNotBlank() && !it.startsWith("#") }
        .map { line ->
            val cells = line.split("\t")
            if (cells.size < 2) throw AssertionError("malformed manifest row: $line")
            CorpusRow(cells[0], cells[1])
        }.filter { it.id.startsWith("core/") }

private fun checkedOf(row: CorpusRow): CheckedProgram {
    val source = File(corpusRoot, row.fixture).readText()
    return Admission.admit(source, Mode.CORE).fold(
        { e -> throw AssertionError("${row.id} rejected before any effect: ${e.category}: ${e.message}") },
        { it },
    )
}

private fun renderedMainValue(value: GValue): String = "${value::class.simpleName}:${renderPrinted(value) ?: value.toString()}"

private fun observations(result: RunResult): Triple<String, String?, String?> =
    Triple(result.output, result.error?.category, result.mainValue?.let(::renderedMainValue))

public class CorpusCoreTest :
    FunSpec({
        val rows = coreRows()

        test("every core row of the manifest is a checked program of this edition") {
            rows.size shouldBe 21
            rows.map { it.id }.toSet().size shouldBe rows.size
            for (row in rows) {
                File(corpusRoot, row.fixture).exists() shouldBe true
                checkedOf(row).mode shouldBe Mode.CORE
            }
        }

        test("the four engines agree on every core row, the self-interpreter included") {
            for (row in rows) {
                val program = checkedOf(row)
                val direct = Direct.run(program)
                val analyzed = Analyzed.run(program)
                val eceval = ExplicitControl.run(program)
                val compiled = Compiler.compileAndRun(program)
                observations(analyzed) shouldBe observations(direct)
                observations(eceval) shouldBe observations(direct)
                observations(compiled) shouldBe observations(direct)
            }
        }
    })
