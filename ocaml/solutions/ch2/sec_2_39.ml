(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.39: reverse via folds. *)

let ex_2_39_right sequence =
  let rec fold_right op initial items =
    match items with
    | [] -> initial
    | car :: cdr -> op car (fold_right op initial cdr)
  in
  fold_right (fun x y -> y @ [ x ]) [] sequence
;;

let ex_2_39_left sequence =
  let fold_left op initial items =
    let rec iter result rest =
      match rest with
      | [] -> result
      | car :: cdr -> iter (op result car) cdr
    in
    iter initial items
  in
  fold_left (fun x y -> y :: x) [] sequence
;;
