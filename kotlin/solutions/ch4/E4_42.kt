// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.42

package sicp.ch4.solutions

import sicp.ch4.AmbEvaluator

/** The liars puzzle's placements: exactly one, then exhaustion. */
public fun liarsSolutions(): List<String> = answerLinesFaulted(::AmbEvaluator, "$AMB_BASE_PRELUDE\n$LIARS_PROGRAM", "(liars)")
