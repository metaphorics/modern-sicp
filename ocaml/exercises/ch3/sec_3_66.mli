(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.66 *)

(** Exercise 3.66: the order of the pairs stream. The map class is
    [T]: the exact positions that are enumerable are measured against
    the closed forms, and the pairs beyond reach are bounded by the
    same formulas. *)

(** [positions limit] is the one-based position of every pair among
    the first [limit] elements of [pairs integers integers]. *)
val positions : int -> (int * int, int) Hashtbl.t

(** [ex_3_66 ()] reports what the enumeration confirms: whether
    (1, j) sits at exactly 2j - 2 for j <= 100, whether (k, k) sits at
    exactly 2^k - 1 for 2 <= k <= 15, and the measured positions of
    (2, 10), (9, 10), and (10, 10). The measured diagonal confirms the
    doubling, so (99, 100) and (100, 100) follow the same law beyond
    any enumeration: they are preceded by on the order of 2^99
    pairs. *)
val ex_3_66 : unit -> bool * bool * int option * int option * int option
