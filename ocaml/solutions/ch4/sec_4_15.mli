(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.15: the halting argument made concrete. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [run_forever_trace ()] runs the diagonal program under the
      always-yes halts? and answers its observation trace.
      [halts_trace ()] runs the same program under the always-no
      halts?. *)
val run_forever_trace : unit -> string list

val halts_trace : unit -> string list

(** [ex_4_15 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_15 : unit -> string list
