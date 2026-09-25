// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.10

package sicp.ch4.exercises

import sicp.runtime.PendingSolution
import sicp.runtime.Value

/**
 * Exercise 4.10: syntax in the interpreter is not sacred -- redefine it.
 * This host answers at the reader level: [syntaxize] is a `Value`->
 * `Value` transform that rewrites `(defun name (params...) body...)` into
 * the procedure-defining `define` sugar and `(fun (params...) body...)`
 * into `lambda`, before the parser ever sees the program. The rewrite is
 * structural and recursive, so a `fun` nested inside a `defun` body comes
 * out as a `lambda`, and a malformed `defun`/`fun` passes through
 * untouched for the interpreter to report. Pins: the transformed demo
 * `(defun cube (x) (* x x x)) (cube 7) ...` reads `"343\n512\n"`, the
 * same program without the transform reads
 * `"Error: unbound variable: defun\n"`.
 */
public fun syntaxize(v: Value): Value = throw PendingSolution()

/** The demo program under the defun/fun transform. => "343\n512\n" */
public fun defunTranscript(): String = throw PendingSolution()

/** The same program without the transform. => "Error: unbound variable: defun\n" */
public fun defunPlainTranscript(): String = throw PendingSolution()
