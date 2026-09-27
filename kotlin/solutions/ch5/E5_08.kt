// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.8: the assembler's answer to a controller that
// defines the same label twice. The book's receive-based label scan keeps
// whichever definition it folds onto (its right-to-left fold keeps the
// last, a left-to-right scan the first), so the machine assembles and then
// jumps arbitrarily; this edition's scan refuses the controller at
// assembly time with the typed duplicate-label error.

package sicp.ch5.solutions

import arrow.core.raise.either
import sicp.ch5.arithOperations
import sicp.ch5.constV
import sicp.ch5.makeMachine
import sicp.runtime.Assign
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Stmt

/** The book's snippet: two definitions of `here`, and `there` defined only
 *  after the second `here`. */
private val doublyDefinedLabelController: List<Stmt> =
    listOf(
        Label("start"),
        Goto(GotoTarget.Lbl("here")),
        Label("here"),
        Assign("a", constV(0)),
        Goto(GotoTarget.Lbl("there")),
        Label("here"),
        Assign("a", constV(1)),
        Goto(GotoTarget.Lbl("here")),
        Label("there"),
    )

/** Assembling the snippet: the typed duplicate-label error, never a
 *  machine. */
public fun duplicateLabelOutcome(): String {
    val outcome = either { makeMachine(listOf("a"), arithOperations, doublyDefinedLabelController) }
    return outcome.fold({ e -> e.toString() }, { "assembled" })
}
