(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.11: the three disciplines a [restore] can follow. *)

(** Which [restore] semantics a machine follows. *)
type discipline =
  | Untagged
  (** The book simulator's: restore takes the last value saved, whatever its source. *)
  | Tagged
  (** [save] files the register's name with the value; a mismatched restore is refused. *)
  | Per_register (** Each register owns a stack. *)

(** [ex_5_11 ()] demonstrates all three disciplines: the untagged
    machine and its one-instruction-smaller Fibonacci machine, the
    tagged machine's typed refusal, and the per-register stacks. *)
val ex_5_11 : unit -> (string list, Sicp_ch5.Sec_5_2.error) result

(** The Fibonacci machine of Figure 5.12. *)
val fib_controller : string

(** The Figure 5.12 machine with one instruction eliminated: the
    afterfib-n-2 exchange replaced by a single untagged [restore n]. *)
val fib_one_fewer_controller : string

(** The three-restore simulator the demonstration runs on. *)
module Sim : sig
  type machine

  val make
    :  discipline:discipline
    -> registers:string list
    -> operations:(string * Sicp_ch5.Sec_5_2.op) list
    -> controller:string
    -> (machine, Sicp_ch5.Sec_5_2.error) result

  val set_register
    :  machine
    -> string
    -> Sicp_ch5.Sec_5_2.value
    -> (unit, Sicp_ch5.Sec_5_2.error) result

  val get_register
    :  machine
    -> string
    -> (Sicp_ch5.Sec_5_2.value, Sicp_ch5.Sec_5_2.error) result

  val run : machine -> (unit, Sicp_ch5.Sec_5_2.error) result
  val initialize_stack : machine -> (unit, Sicp_ch5.Sec_5_2.error) result
end
