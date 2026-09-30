// SPDX-License-Identifier: GPL-3.0-only

package sicp.ch2.exercises

/** Marks a deliberately incomplete student exercise without depending on the old data runtime. */
internal class PendingExercise : IllegalStateException("pending exercise: implement the body to satisfy its documented expectation")
