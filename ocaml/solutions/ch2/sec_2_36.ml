(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.36: accumulate-n. *)

let rec accumulate op initial sequence =
  match sequence with
  | [] -> initial
  | car :: cdr -> op car (accumulate op initial cdr)
;;

let rec ex_2_36 op init seqs =
  match seqs with
  | first :: _ when first = [] -> []
  | _ ->
    accumulate op init (List.map List.hd seqs) :: ex_2_36 op init (List.map List.tl seqs)
;;
