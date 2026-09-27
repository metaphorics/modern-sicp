// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.34: the iterative factorial's compilation. The
// essential difference from the recursive version sits in `iter`'s
// self-call: it is the last expression of the body, compiled with
// target `val` and linkage `return`, so `compile-proc-appl` emits the
// two-instruction direct transfer -- `(assign val (op compiled-
// procedure-entry) (reg proc))` then `(goto (reg val))` -- with no save
// of `continue` for the call. The measured depths at n = 3, 4, 5 are
// equal: the iterative process runs in constant stack space.

package sicp.ch5.solutions

import sicp.ch5.CompilerConfig

private val factorialIterSource: String =
    """
    (define (factorial n)
      (define (iter product counter)
        (if (> counter n)
            product
            (iter (* counter product)
                  (+ counter 1))))
      (iter 1 1))
    """.trimIndent()

/** The tail-call shape: a compiled-branch dispatch whose compiled path
 *  is the direct transfer with no `continue` assignment before it. A
 *  non-tail call sets `continue` to its after-call label on the line
 *  before the entry read, so only transfers without that predecessor
 *  count: the tail calls of `iter` and of `factorial`'s body. */
private fun tailTransferCount(stmts: List<String>): Int {
    var count = 0
    for (i in 0 until stmts.size - 3) {
        val entry = stmts[i]
        val transfer = stmts[i + 1]
        val label = stmts[i + 2]
        val setup = stmts.getOrNull(i - 1) ?: ""
        if (entry == "(assign val (op compiled-procedure-entry) (reg proc))" &&
            transfer == "(goto (reg val))" &&
            label.startsWith("primitive-branch") &&
            !setup.startsWith("(assign continue (label ")
        ) {
            count += 1
        }
    }
    return count
}

/** The compilation's annotation and the measured depths: the direct
 *  transfers appear (the tail calls of `iter` and of `factorial`'s
 *  body), and the monitored session holds one maximum depth for every
 *  n. */
public fun iterativeFactorialCompilation(): List<String> {
    val cfg = CompilerConfig()
    val stmts = compiledStatements(cfg, factorialIterSource)
    val depths =
        listOf(3, 4, 5).map { n ->
            lastStats(runCompiledMonitored(cfg, factorialIterSource, "(factorial $n)")).depth
        }
    return listOf(
        "direct tail transfers: ${tailTransferCount(stmts)}",
        "saves in the whole compilation: ${stmts.count { it.startsWith("(save ") }}",
        "depth at n = 3: ${depths[0]}",
        "depth at n = 4: ${depths[1]}",
        "depth at n = 5: ${depths[2]}",
        "the depths are equal: ${depths[0] == depths[1] && depths[1] == depths[2]}",
    )
}
