(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.18 *)

(** Exercise 3.18: detecting a cycle with a visited-by-identity scan. *)

open Sicp_ch3.Sec_3_3.Mpairs

(** [contains_cycle x] is whether some cdr chain from [x] reaches a pair
    it has already visited. *)
val contains_cycle : mobj -> bool

(** [ex_3_18 ()] is [(the answer for the plain list (a b c), for the
    (a b c) ring of exercise 3.13, for the one-pair list whose cdr
    points at itself, and for the empty list)]. *)
val ex_3_18 : unit -> bool * bool * bool * bool
