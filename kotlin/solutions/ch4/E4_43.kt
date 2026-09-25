// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.43

package sicp.ch4.solutions

import sicp.ch4.AmbEvaluator

/** Told that Mary Ann's last name is Moore, the puzzle has exactly one
 * solution: Lorna's father is Downing. */
public fun yachtToldAnswers(): List<String> = answerLinesFaulted(::AmbEvaluator, "$AMB_BASE_PRELUDE\n$YACHT_TOLD_PROGRAM", "(yacht-puzzle)")

/** Untold, Moore's daughter is a choice too, and Lorna's father is
 * Parker or Downing: two solutions. */
public fun yachtUntoldAnswers(): List<String> =
    answerLinesFaulted(::AmbEvaluator, "$AMB_BASE_PRELUDE\n$YACHT_UNTOLD_PROGRAM", "(yacht-puzzle-untold)")
