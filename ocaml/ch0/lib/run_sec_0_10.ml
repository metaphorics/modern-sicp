(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch0.Replay
module Sec_0_10 = Sicp_ch0.Sec_0_10

let () = Replay.expect (string_of_int (Sec_0_10.add 2 3)) "5"
