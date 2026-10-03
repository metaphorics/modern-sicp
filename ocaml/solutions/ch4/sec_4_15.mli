(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.15 *)

(** Exercise 4.15: no procedure decides halting.

    Suppose [halts p a] answers [true] exactly when [p a] halts, and
    define [try_ p = if halts p p then run_forever () else "halted"].
    If [halts try_ try_] is [true], [try_ try_] runs forever; if it is
    [false], [try_ try_] halts with ["halted"].  Either answer is wrong
    about [try_ try_], so no such [halts] exists.  The demonstration
    runs the diagonal program against the two constant candidates, the
    only total oracles a typed guest can write without inspecting a
    function, under a step budget that stands in for "runs forever". *)

(** [fuel_eval fuel] is the standard evaluator that stops with a
    [User_error] once it has dispatched [fuel] expressions. *)
val fuel_eval : int -> Sicp_ch4.Sec_4_1.eval_t

(** [prefix oracle] is the source binding [halts], [run_forever], and [try_]
    with [halts] bound to the expression [oracle]. *)
val diagonal : string -> string

(** [oracles] is the candidate [halts] expressions the demonstration
    refutes. *)
val oracles : string list

(** [diagonal oracle] is [prefix oracle] with [try_ try_] appended, and
    [claim_source oracle] is [prefix oracle] with [halts try_ try_] appended.
    [claim_eval] is the step-bounded evaluator the claim runs under, so the
    claim of the oracle that answers [true] is itself the budget error. *)

(** [ex_4_15 ()] answers, for each candidate oracle, what it claims
    about [try_ try_] and what [try_ try_] does within a budget of
    10000 steps. *)
val ex_4_15 : unit -> string list
