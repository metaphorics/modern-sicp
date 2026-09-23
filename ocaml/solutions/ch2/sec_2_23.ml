(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.23: for-each. *)

let rec ex_2_23 proc items =
  match items with
  | [] -> ()
  | car :: cdr ->
    proc car;
    ex_2_23 proc cdr
;;
