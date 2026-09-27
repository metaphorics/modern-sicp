(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.22: Louis Reasoner's iterative square-list. *)

let ex_2_22_louis items =
  let rec iter things answer =
    match things with
    | [] -> answer
    | car :: cdr -> iter cdr ((car * car) :: answer)
  in
  iter items []
;;
