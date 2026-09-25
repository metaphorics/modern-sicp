// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.46

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.46: the amb evaluator evaluates operands left to right.
 * The enumeration order of a two-choice list pins the order: the first
 * choice cycles only after the second has been exhausted, which is the
 * only order under which `parse` works -- `parse-word` consumes
 * `*unparsed*` left to right, so a right-to-left evaluator would consume
 * the sentence backwards and fail.
 *
 * Expected answer: ((1 3), (1 4), (2 3), (2 4)) -- the left operand's
 * first value pairs with both of the right operand's before the left
 * operand advances.
 */
public fun operandOrderEnumeration(): List<String> = throw PendingSolution()
