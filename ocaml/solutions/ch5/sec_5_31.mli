(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.31: which of the evaluator's saves survive the compiler's
    [preserving] for four combinations.

    The book's quoted operands become integer literals and its operator
    call [(f)] becomes [(pick true)], a call whose value is the
    procedure.  The compiler answers by construction: each compilation
    keeps exactly the saves its register analysis demands. *)

(** [combinations] pairs each combination, as shown, with the checked
    unit that declares its names and ends in it, in the exercise's
    order. *)
val combinations : (string * string) list

(** [ex_5_31 ()] is one line per combination listing the [save] and
    [restore] instructions of its compilation with target [val] and
    linkage [Next]. *)
val ex_5_31 : unit -> (string list, Sicp_common.Eval_error.t) result
