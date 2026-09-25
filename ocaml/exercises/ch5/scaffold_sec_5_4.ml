(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The runner touches every exercise of section 5.4 once, including the
   tailored addition 5.24a. Each touched stub raises
   [Pending_solution], so [dune build @scaffold] surfaces pending work
   and [dune runtest] never runs the stubs. *)

let () =
  ignore (Sicp_ch5_exercises.Sec_5_23.ex_5_23 ());
  ignore (Sicp_ch5_exercises.Sec_5_24.ex_5_24 ());
  ignore (Sicp_ch5_exercises.Sec_5_24.ex_5_24a ());
  ignore (Sicp_ch5_exercises.Sec_5_25.ex_5_25 ());
  ignore (Sicp_ch5_exercises.Sec_5_26.ex_5_26 ());
  ignore (Sicp_ch5_exercises.Sec_5_27.ex_5_27 ());
  ignore (Sicp_ch5_exercises.Sec_5_28.ex_5_28 ());
  ignore (Sicp_ch5_exercises.Sec_5_29.ex_5_29 ());
  ignore (Sicp_ch5_exercises.Sec_5_30.ex_5_30 ())
;;
