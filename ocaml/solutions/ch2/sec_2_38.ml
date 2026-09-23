(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.38: fold-left versus fold-right. *)

let fold_left op initial sequence =
  let rec iter result rest =
    match rest with
    | [] -> result
    | car :: cdr -> iter (op result car) cdr
  in
  iter initial sequence
;;

let rec fold_right op initial sequence =
  match sequence with
  | [] -> initial
  | car :: cdr -> op car (fold_right op initial cdr)
;;

let ex_2_38 () =
  ( fold_right ( /. ) 1.0 [ 1.0; 2.0; 3.0 ]
  , fold_left ( /. ) 1.0 [ 1.0; 2.0; 3.0 ]
  , fold_right (fun x y -> x :: y) [] [ 1; 2; 3 ]
  , fold_left (fun x y -> y :: x) [] [ 1; 2; 3 ] )
;;
