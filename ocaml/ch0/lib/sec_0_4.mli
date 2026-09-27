(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Pattern matching and variants: the named definitions behind the
    listings of section 0.4. *)

(** A first sum type: geometric shapes that carry their own dimensions. *)
type shape =
  | Circle of float
  | Rectangle of float * float

(** [area shape] dispatches on the constructor; the compiler checks that
    every constructor is handled. [area (Circle 2.0)] is
    [12.5663706143591725] and [area (Rectangle (3.0, 4.0))] is [12.]. *)
val area : shape -> float

(** [abs_value n] is the absolute value of [n], written as a [match] with a
    [when] guard, the translation of Scheme's [cond]. *)
val abs_value : int -> int

(** [head xs] is [Some] the first element of [xs], or [None] when [xs] is
    empty. *)
val head : 'a list -> 'a option

(** [safe_divide n d] is [Some (n / d)], or [None] when [d] is zero. *)
val safe_divide : int -> int -> int option

(** A hand-rolled list of integers, the warm-up reimplementation the reader
    extends in the book's data chapters. *)
type int_list =
  | Nil
  | Cons of int * int_list

(** [total l] adds the integers of the hand-rolled list [l]. *)
val total : int_list -> int

(** A binary tree of integers. *)
type tree =
  | Leaf
  | Node of tree * int * tree

(** [tree_sum t] adds the labels of [t]; [tree_sum] on the section's small
    tree is [11]. *)
val tree_sum : tree -> int
