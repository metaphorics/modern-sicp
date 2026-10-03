(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.79 *)

type value =
  | Num of float
  | Ratpair of int * int
  | Cpx of float * float
  | Bool of bool
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
val install_real_package : unit -> unit
val install_rational_package : unit -> unit
val install_complex_package : unit -> unit
val equ : value -> value -> bool
val ex_2_79 : unit -> bool * bool * bool * bool * bool
