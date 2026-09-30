// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.35a

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.35a (added by this edition): count the search choices the
 * triples search consumes. The counting rule: every delivery of an
 * alternative from a choice point counts one choice -- the first
 * alternative on entering the choice point, and each later alternative
 * a failure resumption delivers. The counter is cumulative across the
 * search, so each element of the answer is the total choices taken by
 * the time that triple is delivered; a triple's own share is the delta
 * from the previous element.
 *
 * Expected answers: between 1 and 20 the totals at the six triples are
 * 1386, 2705, 2965, 3870, 4064, 4765; between 1 and 9 the single triple
 * [3, 4, 5] arrives after 330 choices.
 */
public fun choicesTakenWithin20(): List<Long> = throw PendingSolution()

public fun choicesTakenWithin9(): List<Long> = throw PendingSolution()
