(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The runner touches every exercise of section 2.4 once, including the
   tailored addition 2.74a. Every call raises Pending_solution until the
   exercise is solved; building this executable proves the scaffolds
   link against the stated signatures. *)

open Sicp_ch2_exercises

let () =
  ignore (Sec_2_73.ex_2_73 ());
  ignore (Sec_2_74.find_employee_record [] "");
  ignore (Sec_2_74.find_employee_record_opt [] "");
  ignore Sec_2_74.sample_divisions;
  ignore (Sec_2_74.ex_2_74 ());
  ignore (Sec_2_74.ex_2_74a ());
  ignore (Sec_2_75.ex_2_75 5.0 0.0 Sec_2_75.Real_part);
  ignore (Sec_2_76.ex_2_76 ())
;;
