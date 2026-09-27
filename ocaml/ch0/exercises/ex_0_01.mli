(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 0.1's statement is in the book, section 0.2; the six vals
    below are the names it asks for. *)

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
