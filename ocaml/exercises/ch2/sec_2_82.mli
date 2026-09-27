(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.82 *)

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
val coerce_all_to : value -> value list -> value list option
val apply_generic_n : string -> value list -> value
val ex_2_82_a : unit -> value
val ex_2_82_b : unit -> bool
