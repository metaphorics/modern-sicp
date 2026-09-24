(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.83 *)

(** The tower of Figure 2.25, contents at each level: an integer, a
    rational pair, a real, or a complex pair. *)
type value =
  | Int of int
  | Rat of int * int
  | Real of float
  | Cpx of float * float
  | Tagged of tagged

and tagged =
  { tag : string
  ; contents : value
  }

val attach_tag : string -> value -> tagged
val type_tag : tagged -> string
val contents_of : tagged -> value
val put : string -> string list -> (value list -> value) -> unit
val get : string -> string list -> (value list -> value) option
val apply_generic : string -> value list -> value

(** Installs ["raise"] for ["integer"], ["rational"], and ["real"],
    each one step up the tower; "complex" has no supertype, so it
    gets no entry, matching the exercise's "for each type (except
    complex)". *)
val install_raise : unit -> unit

val raise_one_level : value -> value

(** [ex_2_83 ()] raises the integer 3 to the top of the tower one
    step at a time, returning the type tag after each step and the
    final complex pair. *)
val ex_2_83 : unit -> string * string * string * (float * float)
