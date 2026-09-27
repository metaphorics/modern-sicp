// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.41: lexical addresses.

package sicp.ch5.solutions

import sicp.ch5.findVariable

/** Answers the three find-variable cases from the book's environment. */
public fun findVariableLookups(): List<String> {
    val env = listOf(listOf("y", "z"), listOf("a", "b", "c", "d", "e"), listOf("x", "y"))
    return listOf("c", "x", "w").map { name ->
        val address = findVariable(name, env)?.let { "(${it.first} ${it.second})" } ?: "not-found"
        "$name: $address"
    }
}
