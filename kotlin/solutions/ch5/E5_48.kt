// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.48: the compile-and-run interface. In this
// edition the run happens between the compile and run phases rather
// than inside a machine operation, observably identically: the
// compilation's instruction sequences install on the compiler's
// machine, the machine runs to its halt, and the answers compare with
// the direct run of the same checked source.

package sicp.ch5.solutions

import arrow.core.raise.either
import sicp.ch5.Compiler
import sicp.guest.GuestError
import sicp.guest.OutputSink

/** The two-phase session's report: the compiled run's answer beside the
 *  direct run's, and their agreement. */
public fun compileAndRunReport(): List<String> {
    val source = recursiveFactorialSource + "\nfun main() { println(factorial(5L)) }\n"
    val checked = admitProgram(source)
    val sink = OutputSink()
    val machine = Compiler.machine(checked, sink = sink)
    val outcome = either<GuestError, Unit> { machine.run() }
    outcome.fold({ error -> error("the compiled run faulted: ${error.category}") }, { })
    val compiled = sink.contents().split('\n').filter { it.isNotEmpty() }
    val direct = outputLines(sicp.ch4.Direct.run(checked))
    return listOf(
        "the compiled run answers: ${compiled.joinToString(" ")}",
        "the direct run answers: ${direct.joinToString(" ")}",
        "the two runs agree: ${compiled == direct}",
    )
}
