(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.68 *)

(** Exercise 3.68: Louis Reasoner's pairs, whole first row and all.
    The map class is [T]: the version is implemented and the answer to
    "does this work?" is measured, not argued. *)

(** [positions_book limit] is the one-based position of each pair
    among the first [limit] elements of the book's [pairs]. *)
val positions_book : int -> (int * int, int) Hashtbl.t

(** Louis's version: the whole first row interleaved with the
    recursively defined remainder. On infinite streams it diverges
    before the first element -- forcing it runs the eager recursive
    call forever -- so the value is built only to be named, never
    forced. *)
val pairs_louis
  :  int Sicp_ch3.Sec_3_5.Streams.stream
  -> int Sicp_ch3.Sec_3_5.Streams.stream
  -> (int * int) Sicp_ch3.Sec_3_5.Streams.stream

(** [louis_first_pair_within fuel] is the first pair Louis's definition
    would produce given [fuel] recursion steps, and is always [None]:
    each step descends one level of the recursive call, which runs
    before [interleave] can return a pair. *)
val louis_first_pair_within : int -> (int * int) option

(** [ex_3_68 ()] is Louis's first pair within a 200_000-step budget
    ([None] -- the recursion burns the whole budget producing nothing),
    the budget it burned, and the book's first sixteen pairs, which
    come out fine. *)
val ex_3_68 : unit -> (int * int) option * int * (int * int) list
