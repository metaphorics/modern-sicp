(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.34: Horner's rule as an accumulation. *)

let ex_2_34 x coefficient_sequence =
  let rec accumulate op initial sequence =
    match sequence with
    | [] -> initial
    | car :: cdr -> op car (accumulate op initial cdr)
  in
  accumulate
    (fun this_coeff higher_terms -> (higher_terms * x) + this_coeff)
    0
    coefficient_sequence
;;
