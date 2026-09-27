// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.46

package sicp.ch4.solutions

import sicp.ch4.AmbEvaluator

/** The enumeration order of two choices pins the evaluator's operand
 * order: left to right. */
public fun operandOrderEnumeration(): List<String> =
    answerLinesFaulted(::AmbEvaluator, AMB_BASE_PRELUDE, "(list (an-element-of '(1 2)) (an-element-of '(3 4)))")
