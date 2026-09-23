(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Reference solution of exercise 1.20; the normal-order argument is
    in [ex_1_20.md]. *)

(** [gcd_traced a b] is [(gcd a b, remainder calls)] under eager
    evaluation. *)
val gcd_traced : int -> int -> int * int

(** [ex_1_20 ()] is [gcd_traced 206 40], equal to [(2, 4)]. *)
val ex_1_20 : unit -> int * int
