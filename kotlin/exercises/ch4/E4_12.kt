// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.12

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

// Exercise 4.12: the book asks for abstractions that capture the pattern
// beneath lookup and assignment: one per-frame search and one frame-chain
// walk, with the environment operations redefined on top. The per-frame
// search is the scan over one frame's association-list representation
// (Exercise 4.11's); the chain walk finds the nearest frame whose scan
// hits; lookup and assignment are defined purely on the two scans, and a
// definition adds without scanning. The typed unbound and premature-read
// answers are the `Either` errors of the map's Kotlin note.
//
// Same pins as 4.11: 2, then 10 through an inner frame's assignment, and
// the kernel's null for the binding that died with its call frame.

/** The define/lookup/assign demo through the scan abstractions.
 * => "2\n10\n10\n" */
public fun scannedTranscript(): String = throw PendingSolution()

/** `z` died with its call frame: the chain walk finds nothing and the
 * probe reads the kernel's fault. => "2\nnull\n" */
public fun scannedFreshFrameTranscript(): String = throw PendingSolution()

/** The arity contract holds on the scanned frames: an ill-shaped
 * application answers the kernel's fault. => "null\n" */
public fun scannedArityTranscript(): String = throw PendingSolution()
