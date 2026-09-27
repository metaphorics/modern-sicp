(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.83 *)

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
val install_raise : unit -> unit
val raise_one_level : value -> value
val ex_2_83 : unit -> string * string * string * (float * float)
