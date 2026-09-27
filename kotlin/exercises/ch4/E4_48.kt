// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.48

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.48: extend the grammar. This edition adds an adjective
 * list and a `maybe-adjectives` choice of zero or more adjectives
 * between the article and the noun, so a noun phrase can carry any
 * number of modifiers.
 *
 * Expected answer: "the quick brown cat sleeps" parses to (sentence
 * (noun-phrase (article the) ((adjective quick) (adjective brown))
 * (noun cat)) (verb sleeps)), and the adjective-free "the cat sleeps"
 * parses with an empty modifier list.
 */
public fun adjectiveParse(): String = throw PendingSolution()

public fun noAdjectiveParse(): String = throw PendingSolution()
