(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 0.1: translate five Scheme interactions of section 1.1 into
    OCaml and reproduce them in utop, reporting the inferred type of each.

    The utop replies, one per interaction: [val sum_with_product : int =
    26], [val square : int -> int = <fun>] and [- : int = 49], [- : int =
    11] for the [if], and [- : int = 25] for the anonymous square. *)

(** [sum_with_product] is [+ 2 ( * 4 6)] as [2 + (4 * 6)], inferred [int],
    value [26]. *)
val sum_with_product : int

(** [square x] is [(define (square x) ( * x x))], inferred [int -> int]. *)
val square : int -> int

(** [seven_squared] is [(square 7)], value [49]. *)
val seven_squared : int

(** [larger a b] is [(if (> a b) a b)], inferred [int -> int -> int]. *)
val larger : int -> int -> int

(** [larger_of_4_and_11] is [(if (> 4 11) 4 11)], value [11]. *)
val larger_of_4_and_11 : int

(** [anonymous_square] is [((lambda (x) ( * x x)) 5)], value [25]. *)
val anonymous_square : int
