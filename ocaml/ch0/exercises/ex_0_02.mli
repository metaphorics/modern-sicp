(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 0.2's statement is in the book, section 0.5; [sum_cubes_rec]
    and [sum_cubes_fold] are the two forms it asks for. *)

(** [sum_cubes_rec a b] sums the cubes from [a] to [b] by recursion. *)
val sum_cubes_rec : int -> int -> int

(** [sum_cubes_fold a b] sums the cubes from [a] to [b] with
    [List.fold_left]. *)
val sum_cubes_fold : int -> int -> int
