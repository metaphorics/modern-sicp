(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The runner touches every exercise of section 5.2 once, including the
   tailored addition 5.7a. Each touched stub raises
   [Pending_solution], so [dune build @scaffold] surfaces pending work
   and [dune runtest] never runs the stubs. *)

let () =
  ignore (Sicp_ch5_exercises.Sec_5_7.ex_5_07 ());
  ignore (Sicp_ch5_exercises.Sec_5_8.ex_5_08 ());
  ignore (Sicp_ch5_exercises.Sec_5_9.ex_5_09 ());
  ignore (Sicp_ch5_exercises.Sec_5_10.ex_5_10 ());
  ignore (Sicp_ch5_exercises.Sec_5_11.ex_5_11 ());
  ignore (Sicp_ch5_exercises.Sec_5_12.ex_5_12 ());
  ignore (Sicp_ch5_exercises.Sec_5_13.ex_5_13 ());
  ignore (Sicp_ch5_exercises.Sec_5_14.ex_5_14 ());
  ignore (Sicp_ch5_exercises.Sec_5_15.ex_5_15 ());
  ignore (Sicp_ch5_exercises.Sec_5_16.ex_5_16 ());
  ignore (Sicp_ch5_exercises.Sec_5_17.ex_5_17 ());
  ignore (Sicp_ch5_exercises.Sec_5_18.ex_5_18 ());
  ignore (Sicp_ch5_exercises.Sec_5_19.ex_5_19 ());
  ignore (Sicp_ch5_exercises.Sec_5_7.ex_5_07a ())
;;
