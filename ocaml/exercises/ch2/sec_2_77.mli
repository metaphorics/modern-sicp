(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.77 *)

type value =
  | Num of float
  | Cpx of float * float
  | Tagged of tagged

and tagged =
  { tag : string
  ; contents : value
  }

val attach_tag : string -> value -> tagged
val type_tag : tagged -> string
val contents_of : tagged -> value
val apply_generic : string -> value list -> value
val call_count : unit -> int
val reset_call_count : unit -> unit
val install_rectangular_package : unit -> unit
val real_part : value -> float
val imag_part : value -> float
val magnitude : value -> float
val angle : value -> float
val install_alyssa_complex_fix : unit -> unit
val ex_2_77 : unit -> bool * float * int
