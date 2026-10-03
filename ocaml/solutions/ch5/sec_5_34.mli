(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.34: the iterative factorial compiled.

    The self-call of [iter] is the last expression of its body, so it
    compiles with linkage [Return]: its compiled branch jumps to
    [compiled-apply] without assigning [continue] and nothing is saved
    around it.  The recursive factorial's call is followed by a
    multiplication, so its branch assigns [continue] to a return label
    and every pending level holds [continue] and [env].  The measured
    depth 2 of the iterative run is not the body but the top-level call
    item's own argument-list setup, which saves [env] and [argl] around
    the call operand at every [n]; the recursive depths add two per
    pending level on top of that constant. *)

(** [iterative] is the exercise's iterative factorial. *)
val iterative : string

(** [ex_5_34 ()] shows the compiled branch of each call in the
    compilation of [iterative], the number of saves it contains, and the
    maximum stack depth of the iterative and the recursive factorial at
    [n] = 3, 4, and 5. *)
val ex_5_34 : unit -> (string list, Sicp_common.Eval_error.t) result
