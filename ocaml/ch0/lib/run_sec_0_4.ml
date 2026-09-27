(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch0.Replay
module Sec_0_4 = Sicp_ch0.Sec_0_4

let show_option = function
  | Some n -> "Some " ^ string_of_int n
  | None -> "None"
;;

let () =
  Replay.expect_float (Sec_0_4.area (Sec_0_4.Circle 2.0)) "12.5663706143591725";
  Replay.expect_float (Sec_0_4.area (Sec_0_4.Rectangle (3.0, 4.0))) "12.";
  Replay.expect (string_of_int (Sec_0_4.abs_value (-4))) "4";
  Replay.expect (string_of_int (Sec_0_4.abs_value 7)) "7";
  Replay.expect (show_option (Sec_0_4.head [ 3; 5 ])) "Some 3";
  Replay.expect (show_option (Sec_0_4.head [])) "None";
  Replay.expect (show_option (Sec_0_4.safe_divide 10 2)) "Some 5";
  Replay.expect (show_option (Sec_0_4.safe_divide 10 0)) "None";
  Replay.expect
    (string_of_int (Sec_0_4.total (Sec_0_4.Cons (1, Sec_0_4.Cons (2, Sec_0_4.Nil)))))
    "3";
  Replay.expect
    (string_of_int
       (Sec_0_4.tree_sum
          (Sec_0_4.Node
             ( Sec_0_4.Node (Sec_0_4.Leaf, 1, Sec_0_4.Leaf)
             , 6
             , Sec_0_4.Node (Sec_0_4.Leaf, 4, Sec_0_4.Leaf) ))))
    "11"
;;
