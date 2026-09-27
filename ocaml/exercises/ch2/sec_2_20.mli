(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.20:
   same-parity in the rest-argument idiom *)

(** [ex_2_20 first rest] is [first] together with the elements of
    [rest] having the same even-odd parity as [first]. *)
val ex_2_20 : int -> int list -> int list
