(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.84 *)

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

(** [tower_level tag] is 2.83's tower order, stated as a comparable
    integer: "compatible with the rest of the system" per the
    exercise, so adding a new level above "complex" only needs a
    higher number, not a rewritten comparison. *)
val tower_level : string -> int

val install_raise : unit -> unit
val raise_one_level : value -> value
val install_homogeneous_add : unit -> unit

(** [apply_generic_tower op args] is [apply_generic], modified so
    that a two-argument call with mismatched tower levels raises the
    lower argument and retries, until the levels match or an ["add"]
    entry is found directly. *)
val apply_generic_tower : string -> value list -> value

(** [ex_2_84 ()] adds the integer 3 to the complex number (2, 3),
    returning the sum's real and imaginary parts: the integer must
    climb three levels before ["add"; \["complex"; "complex"\]\]
    applies. *)
val ex_2_84 : unit -> float * float
