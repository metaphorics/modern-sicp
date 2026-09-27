(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.80 *)

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
val install_scheme_number_package : unit -> unit
val install_rational_package : unit -> unit
val install_complex_package : unit -> unit
val is_zero : value -> bool
val ex_2_80 : unit -> bool * bool * bool * bool * bool * bool
