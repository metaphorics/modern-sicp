(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program fixed-point in SICP section 1.3
   exercise 1.35 *)

(** Exercise 1.35: a local [fixed_point], the same search the section
    text runs, bounded so a badly chosen guess fails loudly instead of
    looping forever. *)

let tolerance = 0.00001

let fixed_point ?(max_iterations = 100_000) f first_guess =
  let close_enough v1 v2 = Float.abs (v1 -. v2) < tolerance in
  let rec try_guess guess n =
    if n > max_iterations
    then failwith "fixed_point: did not converge"
    else (
      let next = f guess in
      if close_enough guess next then next else try_guess next (n + 1))
  in
  try_guess first_guess 0
;;

let golden_ratio () = fixed_point (fun x -> 1.0 +. (1.0 /. x)) 1.0
let ex_1_35 () = golden_ratio ()
