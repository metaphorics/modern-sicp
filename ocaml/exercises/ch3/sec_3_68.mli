(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.68 *)

(** Exercise 3.68: Louis Reasoner's pairs, whole first row and all.
    The map class is [T]: the version is implemented and the answer to
    "does this work?" is measured, not argued. *)

(** [positions_book limit] is the one-based position of each pair
    among the first [limit] elements of the book's [pairs]. *)
val positions_book : int -> (int * int, int) Hashtbl.t

(** Louis's version: the whole first row interleaved with the
    recursively defined remainder. *)
val pairs_louis
  :  int Sicp_ch3.Sec_3_5.Streams.stream
  -> int Sicp_ch3.Sec_3_5.Streams.stream
  -> (int * int) Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_68 ()] is the first sixteen pairs Louis's definition yields,
    the first sixteen of the book's, whether the two prefixes hold the
    same set of pairs, and whether they list them in the same order.
    The prefixes agree as sets but not in order, so the definition
    does produce elements -- while (i, 1) drifts to position
    2^(i-1) + 1, exponential delay the book's decomposition avoids. *)
val ex_3_68 : unit -> (int * int) list * (int * int) list * bool * bool
