// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.47

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.47: Louis Reasoner's `parseVerbPhrase`. Louis's version
 * delivers the same first parse as the text's -- the verb word is
 * parsed inside the first alternative -- but its resumption behavior
 * diverges: once the input is spent, the second alternative recurses
 * into `parseVerbPhrase` before anything is consumed, so the search
 * descends forever (measured under a 500-choice budget). Interchanging
 * the alternatives makes even the first parse diverge, measured under a
 * 300-choice budget, because the recursion then runs before any word is
 * consumed at all.
 *
 * Expected answers: Louis's first parse is (sentence (simple-noun-phrase
 * (article the) (noun cat)) (verb eats)), then `choice budget exhausted
 * after 500 choices`; the interchanged version answers
 * `choice budget exhausted after 300 choices`.
 */
public fun louisFirstParse(): String = throw PendingSolution()

public fun louisTryAgainFault(): String = throw PendingSolution()

public fun interchangedFault(): String = throw PendingSolution()
