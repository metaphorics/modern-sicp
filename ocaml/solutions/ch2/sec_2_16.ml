(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise
   SICP section 2.1 exercise 2.16 *)

(** [sub_interval x x] is [make_interval (lower_bound x -. upper_bound
    x) (upper_bound x -. lower_bound x)], not [[0, 0]], whenever [x]
    has positive width: the formula treats the two occurrences of [x]
    as if they were independent measurements that merely happen to
    share bounds, exactly the mechanism behind exercise 2.14 and 2.15.
    In general, any expression that mentions the same uncertain
    quantity more than once loses precision this way, and no package
    built on the "interval = a pair of bounds, nothing else"
    representation can close this gap: the representation itself has
    already thrown away the information (that two intervals denote
    the same underlying quantity) needed to compute the tight answer.
    A package that tracked which sub-expressions shared a source
    variable, carrying intervals paired with a dependency record
    rather than bare bounds, could in principle do better, but that is
    a different, richer representation, not a fix to this one. *)

type interval = float * float

let make_interval a b : interval = a, b
let lower_bound ((a, _) : interval) = a
let upper_bound ((_, b) : interval) = b

let sub_interval x y =
  make_interval (lower_bound x -. upper_bound y) (upper_bound x -. lower_bound y)
;;

let ex_2_16 () =
  let a = make_interval 2.0 4.0 in
  sub_interval a a
;;
