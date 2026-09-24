// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.36

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 3.36: a logging constraint for tracing the connector's
 * observer wiring. Each time a connector dispatches to it, append one
 * line to `transcript`: "<tag> newValue" from `newValue` and "<tag>
 * forgetValue" from `forgetValue`. Connect it to a fresh connector,
 * call `setValue(10L, User)` and then `forgetValue(User)`, and read the
 * transcript in order to trace who consults whom.
 */
public fun loggingConstraint(
    transcript: MutableList<String>,
    tag: String,
): Constraint = throw PendingSolution()
