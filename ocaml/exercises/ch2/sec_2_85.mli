(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.85 *)

type value =
  | Int of int
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
val tower_level : string -> int
val install_raise_and_project : unit -> unit
val install_homogeneous_add : unit -> unit
val raise_one_level : value -> value
val apply_generic : string -> value list -> value
val drop : value -> value
val apply_generic_drop : string -> value list -> value
val ex_2_85 : unit -> string * string * string * string

(* Addition by this edition, extending SICP section 2.5 exercise 2.85 *)

type edge = string * string

val has_cycle : edge list -> bool
val ex_2_85a : unit -> bool * bool * bool
