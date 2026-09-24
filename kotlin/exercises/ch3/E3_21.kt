// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.21

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 3.21: Ben inspects a queue two ways. `printQueue` walks the
 * front chain and prints the items; `frontCell`/`rearCell` expose the
 * hidden pair of pointers behind the representation. The original Lisp
 * interpreter had neither: it printed the queue's two pointers as one
 * pair, so a queue built by inserting a, b, c, d in order rendered as
 * ((a b c d) d), a one-item queue as ((a) a), and the pair of two empty
 * pointers as (() ()). Build that rendering from the cells: "(" + the
 * front chain + " " + the rear pointer's single item + ")", with ()
 * standing in for an empty pointer.
 */
public fun benView(queue: Queue): String = throw PendingSolution()
