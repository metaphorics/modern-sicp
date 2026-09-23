(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 0.2: implement [sum_cubes] over a range twice, once by
    recursion and once with [List.fold_left], and state in one sentence
    when the fold version is preferable.

    [sum_cubes a b] is the sum of the cubes of the integers from [a]
    through [b] inclusive; [(sum-cubes 1 10)] is [3025] in the book. Both
    stubs raise [Sicp_common.Pending.Pending_solution] until solved. *)

(** [sum_cubes_rec a b] sums the cubes from [a] to [b] by recursion. *)
val sum_cubes_rec : int -> int -> int

(** [sum_cubes_fold a b] sums the cubes from [a] to [b] with
    [List.fold_left]. *)
val sum_cubes_fold : int -> int -> int
