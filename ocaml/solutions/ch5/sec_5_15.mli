(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.15 and the simulator core the monitoring exercises
    share. *)

(** How a run ended. *)
type stop =
  | Completed (** The sequence ran out. *)
  | Breakpoint of string * int
  (** The machine stopped just before the [n]th instruction after [label]. *)

(** The monitoring simulator: the section's machine with an executed-
    instruction counter plus the inert hooks the later exercises turn
    on. *)
module Sim : sig
  type machine

  val make
    :  registers:string list
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

  (** [start ?before m] resets the pc and runs; [proceed ?before m]
      continues from the pc. [before] runs just before each executed
      instruction and never disturbs the count. *)
  val start : ?before:(int -> unit) -> machine -> (stop, Sicp_ch5.Sec_5_2.error) result

  val proceed : ?before:(int -> unit) -> machine -> (stop, Sicp_ch5.Sec_5_2.error) result

  (** [take_instruction_count m] is the count since the last reset,
      and the reset -- the book's count-and-reset message. *)
  val take_instruction_count : machine -> int

  val instruction_count : machine -> int

  (** [instruction_labels m i] is 5.17's retained information: the
      labels the assembly attached to instruction [i]. *)
  val instruction_labels : machine -> int -> string list

  val instruction_text : machine -> int -> Sicp_ch5.Sec_5_2.instruction
  val instruction_total : machine -> int

  (** [trace_register m r on] turns one register's assignment tracing
      on or off. *)
  val trace_register : machine -> string -> bool -> (unit, Sicp_ch5.Sec_5_2.error) result

  val traced_assignments : machine -> string list
  val set_breakpoint : machine -> string -> int -> (unit, Sicp_ch5.Sec_5_2.error) result

  val cancel_breakpoint
    :  machine
    -> string
    -> int
    -> (unit, Sicp_ch5.Sec_5_2.error) result

  val cancel_all_breakpoints : machine -> (unit, Sicp_ch5.Sec_5_2.error) result
end

(** The Fibonacci machine of Figure 5.12, the monitoring exercises'
    running example. *)
val fib_controller : string

(** [ex_5_15 ()] counts the Fibonacci machine's instructions on
    [n = 3] and [n = 6] and shows the count reset. *)
val ex_5_15 : unit -> (string list, Sicp_ch5.Sec_5_2.error) result
