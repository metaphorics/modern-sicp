// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.49

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.49: Alyssa's generator. Her `parseWord` ignores the input
 * and answers an `anElementOf` choice from the word list, and `parse`
 * clears `unparsed` instead of filling it, so the parsing programs
 * generate sentences instead -- proceeding left to right through each
 * sentence, the same order 4.46 pinned for the evaluator's choices.
 *
 * Expected answer: the first six generated sentences all start
 * (sentence (simple-noun-phrase (article the) (noun student)) and vary
 * only the verb phrase -- studies, then studies with one, two, three,
 * four, and five `for the student` prepositional phrases -- the
 * footnote's boring recursion into the grammar's first alternatives.
 */
public fun generatedSentences(): List<String> = throw PendingSolution()
