(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The runner touches every exercise of section 2.3 once, including the
   tailored addition 2.70a. Every call raises Pending_solution until the
   exercise is solved; building this executable proves the scaffolds
   link against the stated signatures. *)

open Sicp_ch2_exercises

let () =
  ignore (Sec_2_53.ex_2_53 ());
  ignore (Sec_2_54.ex_2_54 ());
  ignore (Sec_2_55.ex_2_55 ());
  ignore (Sec_2_56.ex_2_56 ());
  ignore (Sec_2_57.ex_2_57 ());
  ignore (Sec_2_58.ex_2_58_parenthesized ());
  ignore (Sec_2_58.ex_2_58_standard ());
  ignore (Sec_2_59.ex_2_59 ());
  ignore (Sec_2_60.ex_2_60 ());
  ignore (Sec_2_61.ex_2_61 ());
  ignore (Sec_2_62.ex_2_62 ());
  ignore (Sec_2_63.ex_2_63 ());
  ignore (Sec_2_64.ex_2_64 ());
  ignore (Sec_2_65.ex_2_65 ());
  ignore (Sec_2_66.ex_2_66 ());
  ignore (Sec_2_67.ex_2_67 ());
  ignore (Sec_2_68.ex_2_68 ());
  ignore (Sec_2_69.ex_2_69 ());
  ignore (Sec_2_70.ex_2_70 ());
  ignore (Sec_2_70.ex_2_70a ());
  ignore (Sec_2_71.ex_2_71 5);
  ignore (Sec_2_72.ex_2_72 5)
;;
