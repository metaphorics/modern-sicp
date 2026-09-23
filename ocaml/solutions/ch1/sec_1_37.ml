(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program cont-frac in SICP section 1.3
   exercise 1.37 *)

(** Exercise 1.37, part b: building outward from the innermost term
    [n k / d k] is already an iterative, right-to-left fold; there is
    no separate recursive-then-iterative rewrite needed. *)

let cont_frac n d k =
  let rec go i result = if i = 0 then result else go (i - 1) (n i /. (d i +. result)) in
  go k 0.0
;;

let one_over_phi = 2.0 /. (1.0 +. sqrt 5.0)

let smallest_k_for_4_decimal_places () =
  let rec search k =
    let v = cont_frac (fun _ -> 1.0) (fun _ -> 1.0) k in
    if Float.abs (v -. one_over_phi) < 0.00005 then k else search (k + 1)
  in
  search 1
;;

let ex_1_37 () =
  let k = smallest_k_for_4_decimal_places () in
  cont_frac (fun _ -> 1.0) (fun _ -> 1.0) k, k
;;
