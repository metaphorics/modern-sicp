(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.11: the three disciplines a [Restore] can follow. *)

(** Which [Restore] semantics a machine follows. *)
type discipline =
  | Untagged
  (** The book simulator's: [Restore] takes the last word saved, whatever its source. *)
  | Tagged
  (** [Save] files the register's name with the word; a mismatched restore is refused. *)
  | Per_register (** Each register owns a stack. *)

(** [make ~discipline ~registers ~operations ~controller] is a machine
    of [discipline]. An [Untagged] machine is the simulator's own; the
    other two run [controller] with each [Save r] and [Restore r]
    replaced by one instruction whose operation keeps the discipline's
    stack, so instruction counts are unchanged. A refused restore is a
    [Bad_instruction]. *)
val make
  :  discipline:discipline
  -> registers:string list
  -> operations:(string * Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.op) list
  -> controller:Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.instruction list
  -> (Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.machine, Sicp_ch5.Sec_5_2.error) result

(** The Figure 5.12 machine with one instruction eliminated: the
    afterfib-n-2 exchange replaced by a single untagged
    [Restore "n"]. *)
val fib_one_fewer_controller : Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.instruction list

(** [ex_5_11 ()] demonstrates all three disciplines: the untagged
    machine and its one-instruction-smaller Fibonacci machine, the
    tagged machine's typed refusal, and the per-register stacks. *)
val ex_5_11 : unit -> (string list, Sicp_ch5.Sec_5_2.error) result
