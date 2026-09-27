(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 0.2: implement [sum_cubes] over a range twice, once by
    recursion and once with [List.fold_left], and state in one sentence
    when the fold version is preferable.

    When the fold is preferable: the fold's helper is tail recursive, so
    prefer it whenever the range can be wide, since the recursive form's
    depth grows as [b - a]. *)

(** [sum_cubes_rec a b] sums the cubes from [a] to [b] by recursion;
    [sum_cubes_rec 1 10] is [3025]. *)
val sum_cubes_rec : int -> int -> int

(** [sum_cubes_fold a b] sums the cubes from [a] to [b] with
    [List.fold_left]; it agrees with [sum_cubes_rec] everywhere,
    including the empty range. *)
val sum_cubes_fold : int -> int -> int
