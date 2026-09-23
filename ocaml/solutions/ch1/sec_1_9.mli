(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program plus in SICP section 1.2 exercise 1.9 *)

(** Reference solution of exercise 1.9. *)

(** [plus_deferred a b] is [a + b] by a recursive process: the increment
    waits for the recursive call to return. *)
val plus_deferred : int -> int -> int

(** [plus_tail a b] is [a + b] by an iterative process: the self-call is
    the whole result, so OCaml runs it in constant space. *)
val plus_tail : int -> int -> int

(** [ex_1_09 ()] is [(plus_deferred 4 5, plus_tail 4 5)], both 9. *)
val ex_1_09 : unit -> int * int
