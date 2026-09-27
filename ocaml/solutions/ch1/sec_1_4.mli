(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program ex_1_04 in SICP section 1.1 *)

(** Reference solution of exercise 1.4. *)

(** [ex_1_04 a b] is [a + b] when [b > 0] and [a - b] otherwise; the
    conditional yields the operator function itself. *)
val ex_1_04 : int -> int -> int
