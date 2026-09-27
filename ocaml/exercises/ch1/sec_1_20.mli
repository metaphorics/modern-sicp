(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 1.20: the number of [remainder] calls eager evaluation
    makes computing [gcd 206 40], and the count normal order would
    make. The stub raises [Sicp_common.Pending.Pending_solution] until
    it is solved. *)

(** [gcd_traced a b] is [(gcd a b, remainder calls)] under OCaml's
    eager, applicative-order evaluation. *)
val gcd_traced : int -> int -> int * int

(** [ex_1_20 ()] is [gcd_traced 206 40]. *)
val ex_1_20 : unit -> int * int
