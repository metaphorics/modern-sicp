// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
// Chapter 4, exercise 4.6

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.6: a one-binding `let` is derived from the kernel's `GLam`
 * and `GApp`: the initializer is evaluated in the current frame, then the
 * body runs in the new binding frame. Nested one-binding lets add nested
 * frames.
 *
 * Expected: the ordinary and derived forms both print 7; an initializer
 * that reads the outer x and a body that reads the new x print 7; nested
 * bindings print 3.
 */
public fun letEquivalenceTranscript(): String = throw PendingSolution()

/** The derived let computes the body in the new frame. => "7\n" */
public fun letBodyTranscript(): String = throw PendingSolution()

/** Lets nest as derived expressions at every depth. => "3\n" */
public fun letNestedTranscript(): String = throw PendingSolution()
