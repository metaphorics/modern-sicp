(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.50: the metacircular evaluator compiled and run.

    The evaluator is [Sicp_ch5.Metacircular.evaluator], the section 4.1
    evaluator written in the admitted subset.  A guest program is an
    [expr] built with its constructors; [Metacircular.with_guest] makes
    the complete unit, which is checked like any other source and then
    compiled by the section 5.5 compiler and run on its machine.  The
    same unit also runs on the explicit-control evaluator and on the
    direct evaluator of 4.1, and all three print the same answer, which
    the native run of the same guest source by the pinned OCaml
    compiler confirms.

    The guest language has addition but no multiplication, so the
    guest factorial multiplies by repeated addition; the levels are
    timed on the same algorithm written directly in the subset.  Machine
    steps are the clock: level 0 is the compiled program, level 1 the
    explicit-control evaluator interpreting it, and level 2 the compiled
    metacircular evaluator interpreting the guest program.  Each level
    of interpretation multiplies the cost by a roughly constant
    factor. *)

(** [guest_factorial] is the guest factorial of [5]. *)
val guest_factorial : string

(** [guest_counter] ticks a counter held in a guest reference three
    times and reads it. *)
val guest_counter : string

(** [subset_factorial] is the same factorial as a subset program; the
    levels are timed on it. *)
val subset_factorial : string

(** [ex_5_50 ()] reports the answers of the metacircular evaluator
    under the three engines with the native oracle's agreement, and the
    machine steps of the three interpretation levels.  A disagreeing
    engine is an error, not a line. *)
val ex_5_50 : unit -> (string list, Sicp_common.Eval_error.t) result
