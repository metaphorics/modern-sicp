(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.1 *)

(** Exercise 5.5: the hand simulations of the Figure 5.11 and Figure
    5.12 machines, with the trace format and the controllers the traces
    transcribe. The test suite cross-checks these answers against the
    simulator. *)

(** [ex_5_05 ()] hand-simulates the factorial machine on [n = 3] and the
    Fibonacci machine on [n = 3]: one line per save, restore, taken
    branch, and return, then the answer register. *)
val ex_5_05 : unit -> (string list, Sicp_ch5.Sec_5_1.error) result

(** The recursive factorial machine of Figure 5.11: registers [n],
    [val], and [continue]. *)
val factorial_recursive_controller
  : Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.instruction list

(** The Fibonacci machine of Figure 5.12: registers [n], [val], and
    [continue]. *)
val fib_controller : Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.instruction list

(** The hand-simulation transcription shared with exercise 5.6's
    counts: its own program counter, flag, registers, and stack over
    the operation table [Sicp_ch5.Sec_5_1.arith_operations]. *)
module Handsim : sig
  (** One state of the transcription. [steps] counts executed
      instructions and [saves] counts pushes. *)
  type state =
    { pc : int
    ; flag : bool
    ; regs : (string * Sicp_ch5.Sec_5_1.value) list
    ; stack : Sicp_ch5.Sec_5_1.value list
    ; steps : int
    ; saves : int
    }

  (** One significant point of the trace. A stack is listed top
      first. *)
  type event =
    | Saved of string * Sicp_ch5.Sec_5_1.value list
    | Restored of string * Sicp_ch5.Sec_5_1.value * Sicp_ch5.Sec_5_1.value list
    | Branch_taken of string
    | Returned_to of string

  (** [initial regs] is the state at the first instruction with [regs]
      loaded, an empty stack, and zero counts. *)
  val initial : (string * Sicp_ch5.Sec_5_1.value) list -> state

  (** [run program state events] steps until the sequence ends and is
      the events in order with the final state. *)
  val run
    :  Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.program
    -> state
    -> event list
    -> (event list * state, Sicp_ch5.Sec_5_1.error) result

  (** [render e] is the trace line of [e]. *)
  val render : event -> string
end
