(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.89 *)

type dense = int list

val is_empty_termlist : dense -> bool
val first_term : dense -> int * int
val rest_terms : dense -> dense
val of_terms : (int * int) list -> dense
val to_terms : dense -> (int * int) list
val ex_2_89 : unit -> dense * (int * int) * (int * int) list
