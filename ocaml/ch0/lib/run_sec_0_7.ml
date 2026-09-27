(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch0.Replay
module Sec_0_7 = Sicp_ch0.Sec_0_7

let show_option = function
  | Some n -> "Some " ^ string_of_int n
  | None -> "None"
;;

let () =
  Replay.expect (string_of_int (Sec_0_7.add_five 1)) "6";
  let doubled_and_shifted = Sec_0_7.compose (fun x -> x * 2) (fun x -> x + 3) in
  Replay.expect (string_of_int (doubled_and_shifted 4)) "14";
  let first, second = Sec_0_7.withdrawal_sequence () in
  Replay.expect
    ("(" ^ show_option first ^ ", " ^ show_option second ^ ")")
    "(Some 40, None)";
  let a, b, c = Sec_0_7.independent_withdrawals () in
  Replay.expect
    ("(" ^ show_option a ^ ", " ^ show_option b ^ ", " ^ show_option c ^ ")")
    "(Some 80, Some 70, Some 0)"
;;
