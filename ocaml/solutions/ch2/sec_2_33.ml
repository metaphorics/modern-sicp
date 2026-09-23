(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.33: map, append, and length as accumulations. *)

let rec accumulate op initial sequence =
  match sequence with
  | [] -> initial
  | car :: cdr -> op car (accumulate op initial cdr)
;;

let ex_2_33_map f sequence = accumulate (fun x y -> f x :: y) [] sequence
let ex_2_33_append seq1 seq2 = accumulate (fun x y -> x :: y) seq2 seq1
let ex_2_33_length sequence = accumulate (fun _ count -> 1 + count) 0 sequence
