(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.27: the recursive factorial on the monitored stack, for
    comparison with Exercise 5.26. *)

(** [recursive_source n] is the 1.2.1 recursive factorial, ending with
    the call at [n]. *)
val recursive_source : int -> string

(** [ex_5_27 ()] measures the recursive factorial for n = 1 to 6 and
    fills the book's table: the fitted linear maximum depth and total
    pushes, each verified on every point. *)
val ex_5_27 : unit -> (string list, Sicp_common.Eval_error.t) result
