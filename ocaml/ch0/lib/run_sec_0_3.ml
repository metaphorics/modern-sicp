(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch0.Replay
module Sec_0_3 = Sicp_ch0.Sec_0_3

let () =
  let size = 2 in
  Replay.expect (string_of_int (5 + 3 + size)) "10";
  let x = 5 in
  let x = x + 1 in
  Replay.expect (string_of_int x) "6";
  Replay.expect (string_of_int (Sec_0_3.sum_of_squares 3 4)) "25";
  Replay.expect (string_of_int (Sec_0_3.factorial 6)) "720";
  Replay.expect (string_of_int (Sec_0_3.f 3)) "20";
  Replay.expect (string_of_int (Sec_0_3.inc 41)) "42";
  Replay.expect (string_of_int (Sec_0_3.double 8)) "16";
  Replay.expect (Sec_0_3.pad_to ~width:8 "sicp") "....sicp";
  Replay.expect (string_of_int (List.length [ 1; 2; 3 ])) "3"
;;
