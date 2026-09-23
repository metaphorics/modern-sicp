(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program nth-root in SICP section 1.3
   exercise 1.45 *)

(** Exercise 1.45: [repeated] and [average_damp] from earlier
    exercises, folded in locally; [nth_root] damps
    [y -> x / y^(n - 1)] [n_damps] times before searching for its
    fixed point. [damps_needed] is the experiment the exercise asks
    for: try increasing damp counts against a guess of [1.] until the
    result's [n]th power lands within [0.001] of [x], and stay there
    (a lucky early match that does not hold for its own recheck does
    not count). *)

let compose f g x = f (g x)
let identity x = x

let repeated f n =
  let rec go n acc = if n = 0 then acc else go (n - 1) (compose f acc) in
  go n identity
;;

let average_damp f x = (x +. f x) /. 2.0
let tolerance = 0.00001

let fixed_point f first_guess =
  let close_enough v1 v2 = Float.abs (v1 -. v2) < tolerance in
  let rec try_guess guess n =
    if n > 100_000
    then None
    else (
      let next = f guess in
      if Float.is_nan next || Float.is_infinite next
      then None
      else if close_enough guess next
      then Some next
      else try_guess next (n + 1))
  in
  try_guess first_guess 0
;;

let nth_root n n_damps x =
  let raw y = x /. (y ** float_of_int (n - 1)) in
  match fixed_point (repeated average_damp n_damps raw) 1.0 with
  | Some v -> v
  | None -> Float.nan
;;

let damps_needed n =
  let x = 2.0 in
  let converges damps =
    let v = nth_root n damps x in
    Float.is_finite v && Float.abs ((v ** float_of_int n) -. x) < 0.001
  in
  let rec search damps = if converges damps then damps else search (damps + 1) in
  search 1
;;

let ex_1_45 n = nth_root n (damps_needed n) 2.0
