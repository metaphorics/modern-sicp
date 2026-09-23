(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The runner touches every exercise once. The two exercises whose
   solved forms diverge by design ([ex_1_05_p] and
   [ex_1_06_sqrt_iter]) are called only through their argument
   positions that return immediately, never through the diverging
   calls themselves. *)

let () =
  ignore (Sicp_ch1_exercises.Sec_1_1.ex_1_01 ());
  ignore (Sicp_ch1_exercises.Sec_1_2.ex_1_02 ());
  ignore (Sicp_ch1_exercises.Sec_1_3.ex_1_03 1 2 3);
  ignore (Sicp_ch1_exercises.Sec_1_4.ex_1_04 3 4);
  ignore (Sicp_ch1_exercises.Sec_1_5.ex_1_05_test 0 9);
  ignore (Sicp_ch1_exercises.Sec_1_6.ex_1_06_new_if true 0 5);
  ignore (Sicp_ch1_exercises.Sec_1_7.ex_1_07_sqrt 9.0);
  ignore (Sicp_ch1_exercises.Sec_1_7.ex_1_07_sqrt_improved 9.0);
  ignore (Sicp_ch1_exercises.Sec_1_8.ex_1_08_cube_root 27.0)
;;
