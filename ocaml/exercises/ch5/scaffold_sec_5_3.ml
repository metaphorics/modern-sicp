(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The runner touches every exercise of section 5.3 once, including the
   tailored addition 5.20a. Each touched stub raises
   [Pending_solution], so [dune build @scaffold] surfaces pending work
   and [dune runtest] never runs the stubs. *)

let () =
  ignore (Sicp_ch5_exercises.Sec_5_20.ex_5_20 ());
  ignore (Sicp_ch5_exercises.Sec_5_20.ex_5_20a ());
  ignore (Sicp_ch5_exercises.Sec_5_21.ex_5_21 ());
  ignore (Sicp_ch5_exercises.Sec_5_22.ex_5_22 ())
;;
