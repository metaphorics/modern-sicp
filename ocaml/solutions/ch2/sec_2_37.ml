(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.37: matrix operations as sequence operations. *)

let rec accumulate op initial sequence =
  match sequence with
  | [] -> initial
  | car :: cdr -> op car (accumulate op initial cdr)
;;

let rec accumulate_n op init seqs =
  match seqs with
  | first :: _ when first = [] -> []
  | _ ->
    accumulate op init (List.map List.hd seqs)
    :: accumulate_n op init (List.map List.tl seqs)
;;

let dot_product v w = accumulate ( + ) 0 (List.map2 ( * ) v w)
let matrix_times_vector m v = List.map (fun row -> dot_product row v) m
let transpose mat = accumulate_n (fun x y -> x :: y) [] mat

let matrix_times_matrix m n =
  let cols = transpose n in
  List.map (fun row -> List.map (fun col -> dot_product row col) cols) m
;;

let ex_2_37 () = dot_product [ 1; 2; 3 ] [ 4; 5; 6 ]
