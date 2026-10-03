// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.10

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.10: syntax in the kernel is not sacred -- redefine it. The
 * rewrite runs before dispatch: `defun(name, params, body)` becomes the
 * procedure-defining derived form and `fun(params, body)` becomes `GLam`,
 * structurally and recursively, so a `fun` inside a `defun` body comes out
 * as a lambda and a malformed shape passes through untouched for the
 * kernel to answer null. Without the rewrite the same program never
 * admits: `defun` is an undeclared name and admission rejects it before
 * any effect.
 *
 * Expected: the transformed demo answers 343 then 512; the untransformed
 * program is rejected with the `UndeclaredName` category.
 */
public fun defunTranscript(): String = throw PendingSolution()

/** The same program without the rewrite is rejected at admission.
 * => "UndeclaredName" */
public fun defunPlainRejection(): String = throw PendingSolution()

/** The rewrite is structural: a nested `fun` inside a `defun` body
 * becomes a lambda and the probe applies it. => "49\n" */
public fun nestedFunTranscript(): String = throw PendingSolution()
