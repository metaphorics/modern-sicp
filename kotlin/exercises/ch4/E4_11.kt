// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.11

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

// Exercise 4.11: represent frames as association lists instead of the
// runtime's name-to-value maps. The frame the four environment operations
// see is one list of name-value pairs: a definition conses a fresh pair
// onto the front, assignment scans outwards and rewrites the value slot
// of the nearest pair, lookup returns the first pair's value, and
// extending the environment builds a fresh frame from the parameters --
// with the arity contract. The solution carries the alist representation
// through the kernel's frame seam, so every procedure call and variable
// touch runs over it. Pins: define z 2 then lookup => 2; assign z 10
// through an inner frame then lookup => 10; a `z` that lived only in a
// returned call frame reads the kernel's null; an ill-shaped application
// still answers the kernel's fault.

/** The define/lookup/assign demo on alist frames. => "2\n10\n10\n" */
public fun alistTranscript(): String = throw PendingSolution()

/** `z` died with its call frame. => "2\nnull\n" */
public fun alistFreshFrameTranscript(): String = throw PendingSolution()

/** The arity contract holds on alist frames. => "null\n" */
public fun alistArityTranscript(): String = throw PendingSolution()
