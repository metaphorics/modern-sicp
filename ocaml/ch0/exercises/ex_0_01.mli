(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 0.1: translate five Scheme interactions of section 1.1 into
    OCaml and reproduce them in utop, reporting the inferred type of each.

    The five interactions are [+ 2 ( * 4 6)], the [square] definition and
    the call [(square 7)], [(if (> 4 11) 4 11)], and the anonymous square
    [((lambda (x) ( * x x)) 5)]. The stub raises
    [Sicp_common.Pending.Pending_solution] until it is solved. *)

(** [sum_with_product] is [+ 2 ( * 4 6)] as an OCaml expression. *)
val sum_with_product : int

(** [square x] is [(define (square x) ( * x x))]. *)
val square : int -> int

(** [seven_squared] is [(square 7)]. *)
val seven_squared : int

(** [larger a b] is [(if (> a b) a b)]. *)
val larger : int -> int -> int

(** [larger_of_4_and_11] is [(if (> 4 11) 4 11)]. *)
val larger_of_4_and_11 : int

(** [anonymous_square] is [((lambda (x) ( * x x)) 5)]. *)
val anonymous_square : int
