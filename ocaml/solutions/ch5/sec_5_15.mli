(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.15 and the monitor the monitoring exercises share: the
    section's simulator driven one instruction at a time, counting the
    instructions it executes, with the label information, register
    tracing, and breakpoints of exercises 5.16 to 5.19. *)

(** How a run ended. *)
type stop =
  | Completed (** The sequence ran out. *)
  | Breakpoint of string * int
  (** The machine stopped just before the [n]th instruction after [label]. *)

(** A monitored machine over the simulator of [Sicp_ch5.Sec_5_1]. *)
module Monitor : sig
  type machine

  (** [make ~registers ~operations ~controller] assembles [controller]
      with the simulator's checks, with the count at zero, no traced
      register, and no breakpoint. *)
  val make
    :  registers:string list
    -> operations:(string * Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.op) list
    -> controller:Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.instruction list
    -> (machine, Sicp_ch5.Sec_5_2.error) result

  (** [set_register m r v] loads [r]; a traced [r] reports the write. *)
  val set_register
    :  machine
    -> string
    -> Sicp_ch5.Sec_5_1.value
    -> (unit, Sicp_ch5.Sec_5_2.error) result

  val get_register
    :  machine
    -> string
    -> (Sicp_ch5.Sec_5_1.value, Sicp_ch5.Sec_5_2.error) result

  (** [start ?before m] resets the program counter, flag, and stack and
      runs until the sequence ends or a breakpoint stops the machine.
      [before i] is called just before instruction [i] executes and
      never changes the count. *)
  val start : ?before:(int -> unit) -> machine -> (stop, Sicp_ch5.Sec_5_2.error) result

  (** [proceed ?before m] executes the instruction the machine stopped
      at and continues as [start] does, without the reset. *)
  val proceed : ?before:(int -> unit) -> machine -> (stop, Sicp_ch5.Sec_5_2.error) result

  (** [take_instruction_count m] is the number of instructions executed
      since the last reset, and resets it: the book's count-and-reset
      message. *)
  val take_instruction_count : machine -> int

  (** [instruction_count m] is the count without the reset. *)
  val instruction_count : machine -> int

  (** [instruction_at m i] is the [i]th assembled instruction. *)
  val instruction_at
    :  machine
    -> int
    -> Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.instruction

  (** [instruction_labels m i] is 5.17's retained information: the
      labels that name instruction [i], in controller order. *)
  val instruction_labels : machine -> int -> string list

  (** [trace_register m r on] turns the tracing of register [r] on or
      off; [r] must be declared. *)
  val trace_register : machine -> string -> bool -> (unit, Sicp_ch5.Sec_5_2.error) result

  (** [traced_assignments m] is one line [r: old -> new] per write to a
      traced register, in order. *)
  val traced_assignments : machine -> string list

  (** [set_breakpoint m label n] stops the machine just before the
      [n]th instruction after [label], counted from one. *)
  val set_breakpoint : machine -> string -> int -> (unit, Sicp_ch5.Sec_5_2.error) result

  (** [cancel_breakpoint m label n] removes the breakpoint at [label]
      [n], if any. *)
  val cancel_breakpoint : machine -> string -> int -> unit

  (** [cancel_all_breakpoints m] removes every breakpoint. *)
  val cancel_all_breakpoints : machine -> unit
end

(** [ex_5_15 ()] counts the Fibonacci machine's instructions on
    [n = 3] and [n = 6] and shows the count reset. *)
val ex_5_15 : unit -> (string list, Sicp_ch5.Sec_5_2.error) result
