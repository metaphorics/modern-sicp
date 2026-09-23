(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.26: predicting the three list combinations. *)

let ex_2_26 () =
  let x = [ 1; 2; 3 ] in
  let y = [ 4; 5; 6 ] in
  x @ y, [ x; y ]
;;
