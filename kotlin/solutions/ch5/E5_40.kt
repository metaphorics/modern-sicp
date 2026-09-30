// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.40: the compile-time environment in the code
// generators. The exercise's dump reads the environment the compiler
// threads for each variable reference; the dump here walks the same
// frame-list model the lexical addressing uses, name by name.

package sicp.ch5.solutions

/** The variable references of a probe read against the compile-time
 *  environment, one line each: the name, its address, and the frame it
 *  resolved in. */
public fun compileTimeEnvironmentDump(): List<String> {
    val frames = lexicalEnvironment(listOf(listOf("n"), listOf("product", "counter")))
    val names = listOf("product", "counter", "n", "missing")
    return names.map { name ->
        val address = findVariable(name, frames)
        "$name -> ${address?.let { "frame ${it.frame}, offset ${it.offset}" } ?: "unbound"}"
    }
}
