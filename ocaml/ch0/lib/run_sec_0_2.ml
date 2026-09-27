(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch0.Replay
module Sec_0_2 = Sicp_ch0.Sec_0_2

let () =
  Replay.expect (string_of_int 486) "486";
  Replay.expect (string_of_int (5 + 3 + 4)) "12";
  Replay.expect (string_of_int (10 - 9)) "1";
  Replay.expect (string_of_int ((2 * 4) + (4 - 6))) "6";
  Replay.expect (string_of_int (10 / 5)) "2";
  Replay.expect (string_of_int (21 * 35)) "735";
  Replay.expect (string_of_int (10 / 3)) "3";
  Replay.expect (string_of_int (10 mod 3)) "1";
  Replay.expect_float (10.0 /. 3.0) "3.33333333333333348";
  Replay.expect_float (Float.of_int 5) "5.";
  Replay.expect (string_of_int (int_of_float 3.7)) "3";
  Replay.expect (Printf.sprintf "%S" (string_of_int 42)) "\"42\"";
  Replay.expect (Printf.sprintf "%S" ("lis" ^ "p")) "\"lisp\"";
  Replay.expect (string_of_int (String.length "scheme")) "6";
  Replay.expect (string_of_bool (1 < 2)) "true";
  Replay.expect (string_of_bool (not (1 = 2))) "true";
  Replay.expect_float (Sec_0_2.average_of_two_ints 5 3) "4.";
  Replay.expect (string_of_bool (Sec_0_2.warm_enough 20.0)) "true";
  Replay.expect (string_of_bool (Sec_0_2.warm_enough 10.0)) "false";
  Replay.expect (Sec_0_2.greet "sicp") "hello, sicp!"
;;
