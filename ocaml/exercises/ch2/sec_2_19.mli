(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.19:
   change-counting over a coin list *)

val us_coins : float list
val uk_coins : float list

(** The value of the first coin of the list. The list must be
    nonempty. *)
val first_denomination : float list -> float

(** The list without its first coin. *)
val except_first_denomination : float list -> float list

(** Whether the coin list is exhausted. *)
val no_more : float list -> bool

(** [cc amount coin_values] counts the ways to make [amount] from the
    coins in [coin_values], each usable any number of times. *)
val cc : float -> float list -> int

(** The book's interaction: [cc 100.0 us_coins]. *)
val ex_2_19 : unit -> int
