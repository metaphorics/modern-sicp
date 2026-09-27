(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.19: counting change over a coin list. *)

val us_coins : float list
val uk_coins : float list
val first_denomination : float list -> float
val except_first_denomination : float list -> float list
val no_more : float list -> bool
val cc : float -> float list -> int
val ex_2_19 : unit -> int
