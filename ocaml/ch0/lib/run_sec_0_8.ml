(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch0.Replay
module Sec_0_8 = Sicp_ch0.Sec_0_8

let show_option = function
  | Some n -> "Some " ^ string_of_int n
  | None -> "None"
;;

let show_result = function
  | Ok s -> "Ok " ^ Printf.sprintf "%S" s
  | Error e -> "Error " ^ Printf.sprintf "%S" e
;;

let show_list xs = "[" ^ String.concat "; " (List.map string_of_int xs) ^ "]"

let () =
  Replay.expect (show_option (int_of_string_opt "42")) "Some 42";
  Replay.expect (show_option (int_of_string_opt "x")) "None";
  Replay.expect (show_result (Sec_0_8.half_report "20")) "Ok \"10 is half of the input\"";
  Replay.expect (show_result (Sec_0_8.half_report "7")) "Error \"7 is odd\"";
  Replay.expect (show_result (Sec_0_8.half_report "x")) "Error \"not a number: x\"";
  let head, rest = Sec_0_8.dequeue [ 1; 2 ] in
  Replay.expect ("(" ^ string_of_int head ^ ", " ^ show_list rest ^ ")") "(1, [2])";
  (try ignore (Sec_0_8.dequeue []) with
   | Sec_0_8.Empty_queue -> Replay.expect "exception Empty_queue" "exception Empty_queue");
  Replay.expect_float (Sec_0_8.checked_sqrt 9) "3."
;;
