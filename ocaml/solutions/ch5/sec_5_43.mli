(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.43: internal definitions scanned out.

    The edition's internal definitions are [let rec] groups, and the
    compiler already treats them the way the exercise asks: the group's
    cells are allocated in one frame before any right-hand side runs,
    then filled in order.  No binding is added to a frame after its code
    starts, so the frame the compile-time environment predicts is the
    frame the code runs in, and lexical addressing stays sound. *)

(** [source] defines a procedure with an internal mutually recursive
    group. *)
val source : string

(** [ex_5_43 ()] lists the group operations of the procedure's
    compilation in order and runs [source] with and without lexical
    addressing, both answering [3]. *)
val ex_5_43 : unit -> (string list, Sicp_common.Eval_error.t) result
