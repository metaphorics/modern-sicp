(* SPDX-License-Identifier: GPL-3.0-only *)

(** The metacircular evaluator of section 4.1 as guest source: an OCaml
    host-subset program (the [core/metacircular] conformance case
    without its driver) defining the guest syntax [expr] and [pattern],
    the values [value], and [eval], [lookup], [bind_pattern], and
    [show].  Exercises 5.50 and 5.52 compile and run this source; it
    never reaches a host evaluator. *)

(** [evaluator] is the evaluator's declarations. *)
val evaluator : string

(** [with_guest guest] is a complete unit: [evaluator], then
    [let guest_program = guest] (an [expr] written with the evaluator's
    constructors), then a driver printing [show (eval [] guest_program)]. *)
val with_guest : string -> string
