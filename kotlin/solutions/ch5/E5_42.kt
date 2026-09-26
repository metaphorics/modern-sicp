// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.42: lexical addressing in the code generators.
// `compile-variable` and `compile-assignment` emit lexical-address
// instructions when `find-variable` locates the name and fall back to
// the global search when it does not. The nested example shows the
// addresses, and the applied example runs to the book's 180 through
// the machine's lexical operations.

package sicp.ch5.solutions

import sicp.ch5.CompilerConfig

private val nestedExample: String =
    """
    (define (f x y)
      (lambda (a b c d e)
        (lambda (y z) (* x y z))))
    """.trimIndent()

private val appliedExample: String =
    """
    (define (f x y)
      (lambda (a b c d e)
        ((lambda (y z) (* x y z))
         (* a b x)
         (+ c d x))))
    (define (run f) (f 1 2 3 4 5))
    """.trimIndent()

/** Compiles the nested example lexically and runs the applied example
 *  on the lexical machine. */
public fun lexicalAddressRuns(): List<String> {
    val cfg = CompilerConfig(lexical = true)
    val accesses =
        compiledStatements(cfg, nestedExample).filter { it.contains("(op lexical-address-lookup)") }
    val values =
        valuesOf(runCompiled(cfg, appliedExample, "(run (f 3 4))")).filter { it != "ok" }
    return accesses + listOf("lexical run: ${values.joinToString(" ")}")
}
