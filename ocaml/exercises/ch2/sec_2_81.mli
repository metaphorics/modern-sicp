(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.81 *)

type value =
  | Num of float
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
val put_coercion : string -> string -> (value -> value) -> unit
val get_coercion : string -> string -> (value -> value) option

exception Loop_detected of int

val apply_generic_loop : int -> string -> value list -> value
val apply_generic_fixed : string -> value list -> value
val install_louis_setup : unit -> unit
val ex_2_81_a : unit -> int
val ex_2_81_c : unit -> bool
