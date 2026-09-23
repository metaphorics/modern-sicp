(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.33:
   map, append, and length as accumulations *)

val ex_2_33_map : ('a -> 'b) -> 'a list -> 'b list
val ex_2_33_append : 'a list -> 'a list -> 'a list
val ex_2_33_length : 'a list -> int
