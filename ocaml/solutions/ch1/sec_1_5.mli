(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs p and test in SICP section 1.1
   exercise 1.5 *)

(** Reference solution of exercise 1.5. *)

(** [ex_1_05_p ()] is the nonterminating procedure: a call reduces to
    itself and never returns. *)
val ex_1_05_p : unit -> 'a

(** [ex_1_05_test x y] is [0] when [x = 0] and [y] otherwise; both
    branches must share one type, so [test] is a comparison of
    integers. Evaluating [ex_1_05_test 0 (ex_1_05_p ())] under
    OCaml's applicative order diverges; under normal order it would
    return [0]. The tests never run the diverging form. *)
val ex_1_05_test : int -> int -> int
