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

(** The recursive factorial machine of Figure 5.11. *)
val factorial_recursive_controller : string

(** The Fibonacci machine of Figure 5.12. *)
val fib_controller : string

(** The hand-simulation transcription shared with exercise 5.6's
    counts. *)
module Handsim : sig
  (** One significant point of the trace. *)
  type event =
    | Saved of string * Sicp_ch5.Sec_5_1.value list
    | Restored of string * Sicp_ch5.Sec_5_1.value * Sicp_ch5.Sec_5_1.value list
    | Branch_taken of string
    | Returned_to of string

  (** One state of the transcription. *)
  type state =
    { pc : int
    ; flag : bool
    ; regs : (string * Sicp_ch5.Sec_5_1.value) list
    ; stack : Sicp_ch5.Sec_5_1.value list
    ; steps : int
    ; saves : int
    }

  val initial : (string * Sicp_ch5.Sec_5_1.value) list -> state

  (** [run program state events] steps until the sequence ends. *)
  val run
    :  Sicp_ch5.Sec_5_1.program
    -> state
    -> event list
    -> (event list * state, Sicp_ch5.Sec_5_1.error) result
end
