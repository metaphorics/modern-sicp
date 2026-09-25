(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.28: the evaluator with its tail-recursion removed -- the
    naive [ev-sequence] of the 5.4.2 footnote -- rerunning the
    measurements of 5.26 and 5.27 to show both factorial versions now
    demand space that grows with n. *)

(** The naive sequence evaluation of the 5.4.2 footnote: no expression
    is in tail position, so a tail call pushes. *)
val naive_ev_sequence : string

(** The monitored controller with the naive sequence evaluation. *)
val controller : string

(** [run source] evaluates [source] on the non-tail-recursive monitored
    evaluator and answers the transcript. *)
val run : string -> (string list, Sicp_ch5.Sec_5_4.error) result

(** [ex_5_28 ()] reruns the 5.26 and 5.27 experiments on the
    non-tail-recursive evaluator: the iterative factorial's maximum
    depth, constant under the book's evaluator, now grows linearly,
    and the recursive factorial's depth keeps growing too -- both rows
    of the book's demonstration. *)
val ex_5_28 : unit -> (string list, Sicp_ch5.Sec_5_4.error) result
