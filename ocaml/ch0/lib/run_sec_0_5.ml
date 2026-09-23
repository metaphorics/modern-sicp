(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch0.Replay
module Sec_0_5 = Sicp_ch0.Sec_0_5

let show_list xs = "[" ^ String.concat "; " (List.map string_of_int xs) ^ "]"

let () =
  Replay.expect (show_list Sec_0_5.squares) "[1; 4; 9; 16]";
  Replay.expect (show_list Sec_0_5.evens) "[2; 4; 6; 8; 10]";
  Replay.expect (show_list (0 :: [ 1; 2; 3 ])) "[0; 1; 2; 3]";
  Replay.expect (show_list ([ 1; 2 ] @ [ 3; 4 ])) "[1; 2; 3; 4]";
  Replay.expect (show_list (Sec_0_5.my_map (fun x -> x * x) [ 1; 2; 3 ])) "[1; 4; 9]";
  Replay.expect (show_list (Sec_0_5.my_filter (fun x -> x > 2) [ 1; 2; 3; 4 ])) "[3; 4]";
  Replay.expect (string_of_int (List.fold_left ( + ) 0 [ 1; 2; 3; 4 ])) "10";
  Replay.expect (string_of_int (Sec_0_5.my_fold_left ( + ) 0 [ 1; 2; 3; 4 ])) "10";
  Replay.expect
    (show_list
       ([ 1; 2; 3; 4; 5 ]
        |> List.map (fun x -> x * x)
        |> List.filter (fun x -> x mod 2 = 1)))
    "[1; 9; 25]"
;;
