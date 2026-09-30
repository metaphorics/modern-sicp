(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.15 *)

(** Exercise 4.15: no procedure decides halting. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [fuel_eval fuel] is the standard evaluator that stops with a
    [User_error] once it has dispatched [fuel] expressions. *)
val fuel_eval : int -> Sicp_ch4.Sec_4_1.eval_t

(** [prefix oracle] is the source binding [halts], [run_forever], and [try_]
    with [halts] bound to the expression [oracle]. *)
val diagonal : string -> string

(** [diagonal oracle] is [prefix oracle] with [try_ try_] appended, and
    [claim_source oracle] is [prefix oracle] with [halts try_ try_] appended.
    [claim_eval] is the step-bounded evaluator the claim runs under, so the
    claim of the oracle that answers [true] is itself the budget error. *)

(** [ex_4_15 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_15 : unit -> string list
