(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.28: the evaluator with its tail recursion removed,
    rerunning the measurements of Exercises 5.26 and 5.27. *)

(** [controller] is the base controller with the 5.4.2 footnote's
    [ev-sequence], whose last expression runs across a saved
    [continue], and with a procedure body run the same way, so no
    expression is in tail position. *)
val controller : Sec_5_23.controller

(** [ex_5_28 ()] measures both factorials for n = 1 to 5 on the
    non-tail-recursive evaluator and answers the table, the fitted
    linear maximum depth of each, and whether the iterative version's
    depth now grows with n. *)
val ex_5_28 : unit -> (string list, Sicp_common.Eval_error.t) result
