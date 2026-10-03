// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.41: `find-variable` answers the lexical address
// of a variable in a compile-time environment, or false when the name is
// unbound. The environment is the frame list the compiler threads,
// innermost frame first.

package sicp.ch5.solutions

/** `find-variable`: the address of [name] in [cenv], or null when the
 *  name is unbound. */
public fun findVariable(
    name: String,
    cenv: List<List<String>>,
): LexicalAddress? {
    for ((frame, names) in cenv.withIndex()) {
        val offset = names.indexOf(name)
        if (offset >= 0) return LexicalAddress(frame, offset)
    }
    return null
}
