(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.78 *)

type value =
  | Num of float
  | Tagged of tagged

and tagged =
  { tag : string
  ; contents : value
  }

val type_tag : value -> string
val contents_of : value -> value
val attach_tag : string -> value -> value
val ex_2_78 : unit -> string * bool * string
