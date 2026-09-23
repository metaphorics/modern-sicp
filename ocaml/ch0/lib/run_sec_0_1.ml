(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch0.Replay
module Sec_0_1 = Sicp_ch0.Sec_0_1

let () = Replay.expect (string_of_int (Sec_0_1.square 21)) "441"
