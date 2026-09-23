(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch0.Replay
module Sec_0_9 = Sicp_ch0.Sec_0_9

let () =
  Replay.expect (string_of_int (Sec_0_9.Int_summer.sum [ 1; 2; 3 ])) "6";
  Replay.expect_float (Sec_0_9.Float_summer.sum [ 1.5; 2.5 ]) "4.";
  Replay.expect (Sec_0_9.describe_zero Sec_0_9.int_package) "0";
  Replay.expect (Sec_0_9.describe_zero Sec_0_9.float_package) "0";
  Replay.expect (Sec_0_9.sum_with (module Sec_0_9.Int_number) [ 1; 2; 3 ]) "6";
  Replay.expect (Sec_0_9.sum_with (module Sec_0_9.Float_number) [ 1.5; 2.5 ]) "4"
;;
