// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.46

package sicp.ch2.exercises

/**
 * The exercise's constructor, selectors, and vector operations are
 * `Vect`, `makeVect`, `xcorVect`, `ycorVect`, `addVect`, `subVect`, and
 * `scaleVect`, all declared in this package's shared `Painters.kt`
 * (`frameCoordMap` needs them before the exercise order reaches this
 * file, exactly as the book's own prose uses vectors two subsections
 * before asking the reader to implement them). This file demonstrates
 * the abstraction's contract.
 */
public fun ex_2_46(): Vect = addVect(makeVect(1.0, 2.0), makeVect(3.0, 4.5))
