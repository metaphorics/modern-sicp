(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.38 *)

(** Exercise 3.38: Peter deposits 10, Paul withdraws 20, and Mary
    withdraws half of a shared balance that starts at 100. The
    statement's three @code{set!} commands become three read/write
    processes over one [int ref]; the exercise asks for the balances
    reachable under sequential orders and under interleaving. *)

(** One depositor: it reads the balance once and later writes the
    value [read_write] returns for that reading. *)
type process =
  { name : string
  ; read_write : int -> int
  }

val peter : process
val paul : process
val mary : process

(** [sequential_balances ()] is the sorted list of distinct final
    balances over the six sequential orders of the three processes. *)
val sequential_balances : unit -> int list

(** [interleaved_balances ()] is the sorted list of distinct final
    balances over every interleaving of the three processes' read and
    write events, in the order the processes run them. *)
val interleaved_balances : unit -> int list

(** [sample_concurrent_runs n] really runs the three processes in
    parallel [n] times and answers the sorted distinct final balances
    observed. Every observation is a member of
    [interleaved_balances ()]. *)
val sample_concurrent_runs : int -> int list

(** [ex_3_38 ()] is the sequential outcome list, the interleaved
    outcome list, and one sampled batch of real runs, in that order. *)
val ex_3_38 : unit -> int list * int list * int list
