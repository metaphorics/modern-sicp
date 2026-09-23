(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs p and test in SICP section 1.1
   exercise 1.5 *)

(** Exercise 1.5: [p] loops forever; [test] is an ordinary function,
    so OCaml's applicative order evaluates the argument [p ()] before
    the call and the evaluation of [test 0 (p ())] never
    terminates. *)

let rec ex_1_05_p () = ex_1_05_p ()
let ex_1_05_test x y = if x = 0 then 0 else y
