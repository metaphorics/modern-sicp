(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.30: forcing in the sequence clause.  The text's original
    sequence evaluates every expression and forces none; Cy D. Fect's
    proposal forces every expression but the last.  The section's lazy
    evaluator already follows Cy, because the subset's [;] runs its
    left expression; the original is a variant sequence clause.

    Part a: Ben is right about [for_each].  The call [proc x] applies a
    compound procedure whose body is evaluated, and its printing
    reaches a primitive application, whose operands are forced, so the
    same program prints the same lines under both sequences.  Part b:
    [p1 (ref [1])] answers [1 2] under both.  [p2 (ref [1])] answers
    [1] under the original, because the assignment is the delayed
    operand [e] of [p], evaluated to its thunk and discarded, and
    [1 2] under Cy's, which forces it.  Part c: forcing a sequence
    position changes nothing where the effect flows through an
    application.  Part d: the edition keeps Cy's rule, the one the
    subset's [;] already promises. *)

(** [ex_4_30 ()] answers the [for_each] transcript under the original
    sequence and under Cy's, then the [p1]/[p2] transcript under each. *)
val ex_4_30 : unit -> string list
