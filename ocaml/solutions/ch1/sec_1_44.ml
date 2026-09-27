(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program smooth in SICP section 1.3
   exercise 1.44 *)

(** Exercise 1.44: [smooth]'s [dx] mirrors [Sec_1_3.Newtons_method.dx];
    [n_fold_smoothed] is exercise 1.43's [repeated] applied to
    [smooth]. [noisy] adds a fast, small oscillation to [x *. x] so
    smoothing has something to remove. *)

let smoothing_dx = 0.00001
let smooth f x = (f (x -. smoothing_dx) +. f x +. f (x +. smoothing_dx)) /. 3.0
let compose f g x = f (g x)
let identity x = x

let repeated f n =
  let rec go n acc = if n = 0 then acc else go (n - 1) (compose f acc) in
  go n identity
;;

let n_fold_smoothed f n = repeated smooth n f
let noisy x = (x *. x) +. (0.01 *. sin (1000.0 *. x))
let ex_1_44 () = noisy 2.0, n_fold_smoothed noisy 5 2.0
