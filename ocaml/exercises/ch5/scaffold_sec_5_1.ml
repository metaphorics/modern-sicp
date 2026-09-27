(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The runner touches every exercise of section 5.1 once, including the
   tailored addition 5.1a. Each touched stub raises
   [Pending_solution], so [dune build @scaffold] surfaces pending work
   and [dune runtest] never runs the stubs. *)

let () =
  ignore (Sicp_ch5_exercises.Sec_5_1.ex_5_01 ());
  ignore (Sicp_ch5_exercises.Sec_5_1.ex_5_01a ());
  ignore (Sicp_ch5_exercises.Sec_5_2.ex_5_02 ());
  ignore (Sicp_ch5_exercises.Sec_5_3.ex_5_03 ());
  ignore (Sicp_ch5_exercises.Sec_5_4.ex_5_04 ());
  ignore (Sicp_ch5_exercises.Sec_5_5.ex_5_05 ());
  ignore (Sicp_ch5_exercises.Sec_5_6.ex_5_06 ())
;;
