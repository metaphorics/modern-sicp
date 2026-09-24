(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The QCheck2 property behind section 0.10 and the exercise 0.2 rationale:
   the two [sum_cubes] forms of [Sicp_ch0_solutions.Ex_0_02] agree on every
   generated range. Reversed ranges [a > b] are included on purpose — both
   forms answer [0] on the empty range — alongside the single-element and
   zero-straddling ranges the Alcotest suite in [test_solutions.ml] pins. *)

let fold_agreement =
  QCheck2.Test.make
    ~name:"sum_cubes_rec and sum_cubes_fold agree on generated ranges"
    QCheck2.Gen.(pair (int_range (-100) 100) (int_range (-100) 100))
    (fun (a, b) ->
       Sicp_ch0_solutions.Ex_0_02.sum_cubes_rec a b
       = Sicp_ch0_solutions.Ex_0_02.sum_cubes_fold a b)
;;

let () = QCheck_base_runner.run_tests_main [ fold_agreement ]
