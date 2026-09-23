(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch0.Replay
module Sec_0_6 = Sicp_ch0.Sec_0_6

let () =
  Replay.expect_float Sec_0_6.origin.x "0.";
  Replay.expect_float Sec_0_6.right_three.x "3.";
  Replay.expect_float Sec_0_6.right_three.y "0.";
  let counter = Sec_0_6.fresh_counter () in
  Sec_0_6.bump counter;
  Sec_0_6.bump counter;
  Replay.expect (string_of_int counter.count) "2";
  Replay.expect (string_of_int (Sec_0_6.after_deposit ())) "120";
  Replay.expect (string_of_int (Sec_0_6.alias_cell ())) "15";
  let original_x, moved_x = Sec_0_6.copy_point () in
  Replay.expect_float original_x "1.";
  Replay.expect_float moved_x "9.";
  let distinct, shared = Sec_0_6.cell_identity () in
  Replay.expect (string_of_bool distinct) "false";
  Replay.expect (string_of_bool shared) "true"
;;
