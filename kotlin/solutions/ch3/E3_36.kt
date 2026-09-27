// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.36

package sicp.ch3.exercises

/**
 * Exercise 3.36: the logging constraint. Each dispatch appends one line
 * to `transcript`, so a test can read off exactly when the connector
 * consulted the constraint and with which message. The setter the
 * caller passes (the book's `'user`) is never dispatched to, because it
 * sits in no connector's constraint list to be iterated over.
 */
public fun loggingConstraint(
    transcript: MutableList<String>,
    tag: String,
): Constraint =
    object : Constraint {
        override fun newValue() {
            transcript.add("$tag newValue")
        }

        override fun forgetValue() {
            transcript.add("$tag forgetValue")
        }
    }
