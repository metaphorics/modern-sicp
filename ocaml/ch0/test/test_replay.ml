(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* An Alcotest suite over the same values the primer's replay executables
   pin, so the book's listings are proved twice: once by the replay
   executable each section links (`dune runtest`, exit-code proof), and
   once here (`dune runtest`, per-assertion proof with a readable report
   when something regresses). *)

module Sec_0_1 = Sicp_ch0.Sec_0_1
module Sec_0_2 = Sicp_ch0.Sec_0_2
module Sec_0_3 = Sicp_ch0.Sec_0_3
module Sec_0_4 = Sicp_ch0.Sec_0_4
module Sec_0_5 = Sicp_ch0.Sec_0_5
module Sec_0_6 = Sicp_ch0.Sec_0_6
module Sec_0_7 = Sicp_ch0.Sec_0_7
module Sec_0_8 = Sicp_ch0.Sec_0_8
module Sec_0_9 = Sicp_ch0.Sec_0_9
module Sec_0_10 = Sicp_ch0.Sec_0_10

let float3 = Alcotest.float 0.0001
let sec_0_1_square () = Alcotest.(check int) "square 21" 441 (Sec_0_1.square 21)

let sec_0_2_average () =
  Alcotest.(check float3) "average_of_two_ints 5 3" 4.0 (Sec_0_2.average_of_two_ints 5 3)
;;

let sec_0_2_warm_enough () =
  Alcotest.(check bool) "warm_enough 20." true (Sec_0_2.warm_enough 20.0);
  Alcotest.(check bool) "warm_enough 10." false (Sec_0_2.warm_enough 10.0)
;;

let sec_0_2_greet () =
  Alcotest.(check string) "greet" "hello, sicp!" (Sec_0_2.greet "sicp")
;;

let sec_0_3_bindings () =
  Alcotest.(check int) "sum_of_squares 3 4" 25 (Sec_0_3.sum_of_squares 3 4);
  Alcotest.(check int) "factorial 6" 720 (Sec_0_3.factorial 6);
  Alcotest.(check int) "f 3" 20 (Sec_0_3.f 3);
  Alcotest.(check int) "inc 41" 42 (Sec_0_3.inc 41);
  Alcotest.(check int) "double 8" 16 (Sec_0_3.double 8);
  Alcotest.(check string)
    "pad_to ~width:8 \"sicp\""
    "....sicp"
    (Sec_0_3.pad_to ~width:8 "sicp")
;;

let sec_0_4_shapes () =
  Alcotest.(check float3)
    "area (Circle 2.0)"
    12.5663706143591725
    (Sec_0_4.area (Sec_0_4.Circle 2.0));
  Alcotest.(check float3)
    "area (Rectangle (3.0, 4.0))"
    12.0
    (Sec_0_4.area (Sec_0_4.Rectangle (3.0, 4.0)))
;;

let sec_0_4_pattern_forms () =
  Alcotest.(check int) "abs_value (-4)" 4 (Sec_0_4.abs_value (-4));
  Alcotest.(check int) "abs_value 7" 7 (Sec_0_4.abs_value 7);
  Alcotest.(check (option int)) "head [3; 5]" (Some 3) (Sec_0_4.head [ 3; 5 ]);
  Alcotest.(check (option int)) "head []" None (Sec_0_4.head []);
  Alcotest.(check (option int)) "safe_divide 10 2" (Some 5) (Sec_0_4.safe_divide 10 2);
  Alcotest.(check (option int)) "safe_divide 10 0" None (Sec_0_4.safe_divide 10 0)
;;

let sec_0_4_hand_rolled_data () =
  Alcotest.(check int)
    "total (Cons (1, Cons (2, Nil)))"
    3
    (Sec_0_4.total (Sec_0_4.Cons (1, Sec_0_4.Cons (2, Sec_0_4.Nil))));
  Alcotest.(check int)
    "tree_sum"
    11
    (Sec_0_4.tree_sum
       (Sec_0_4.Node
          ( Sec_0_4.Node (Sec_0_4.Leaf, 1, Sec_0_4.Leaf)
          , 6
          , Sec_0_4.Node (Sec_0_4.Leaf, 4, Sec_0_4.Leaf) )))
;;

let sec_0_5_library_forms () =
  Alcotest.(check (list int)) "squares" [ 1; 4; 9; 16 ] Sec_0_5.squares;
  Alcotest.(check (list int)) "evens" [ 2; 4; 6; 8; 10 ] Sec_0_5.evens
;;

let sec_0_5_hand_rolled () =
  Alcotest.(check (list int))
    "my_map (fun x -> x * x) [1; 2; 3]"
    [ 1; 4; 9 ]
    (Sec_0_5.my_map (fun x -> x * x) [ 1; 2; 3 ]);
  Alcotest.(check (list int))
    "my_filter (fun x -> x > 2) [1; 2; 3; 4]"
    [ 3; 4 ]
    (Sec_0_5.my_filter (fun x -> x > 2) [ 1; 2; 3; 4 ]);
  Alcotest.(check int)
    "my_fold_left ( + ) 0 [1; 2; 3; 4]"
    10
    (Sec_0_5.my_fold_left ( + ) 0 [ 1; 2; 3; 4 ])
;;

let sec_0_6_records () =
  Alcotest.(check float3) "origin.x" 0.0 Sec_0_6.origin.x;
  Alcotest.(check float3) "right_three.x" 3.0 Sec_0_6.right_three.x;
  Alcotest.(check float3) "right_three.y" 0.0 Sec_0_6.right_three.y
;;

let sec_0_6_mutable_fields_and_refs () =
  let counter = Sec_0_6.fresh_counter () in
  Sec_0_6.bump counter;
  Sec_0_6.bump counter;
  Alcotest.(check int) "counter.count after two bumps" 2 counter.count;
  Alcotest.(check int) "after_deposit ()" 120 (Sec_0_6.after_deposit ());
  Alcotest.(check int) "alias_cell ()" 15 (Sec_0_6.alias_cell ())
;;

let sec_0_6_copying_and_identity () =
  let original_x, moved_x = Sec_0_6.copy_point () in
  Alcotest.(check float3) "copy_point original.x" 1.0 original_x;
  Alcotest.(check float3) "copy_point moved.x" 9.0 moved_x;
  let distinct, shared = Sec_0_6.cell_identity () in
  Alcotest.(check bool) "cell_identity distinct" false distinct;
  Alcotest.(check bool) "cell_identity shared" true shared
;;

let sec_0_7_closures () =
  Alcotest.(check int) "add_five 1" 6 (Sec_0_7.add_five 1);
  let doubled_and_shifted = Sec_0_7.compose (fun x -> x * 2) (fun x -> x + 3) in
  Alcotest.(check int)
    "compose (fun x -> x*2) (fun x -> x+3) 4"
    14
    (doubled_and_shifted 4)
;;

let sec_0_7_private_state () =
  let first, second = Sec_0_7.withdrawal_sequence () in
  Alcotest.(check (option int)) "withdrawal_sequence first" (Some 40) first;
  Alcotest.(check (option int)) "withdrawal_sequence second" None second;
  let a, b, c = Sec_0_7.independent_withdrawals () in
  Alcotest.(check (option int)) "independent_withdrawals a" (Some 80) a;
  Alcotest.(check (option int)) "independent_withdrawals b" (Some 70) b;
  Alcotest.(check (option int)) "independent_withdrawals c" (Some 0) c
;;

let sec_0_8_exceptions () =
  let head, rest = Sec_0_8.dequeue [ 1; 2 ] in
  Alcotest.(check int) "dequeue head" 1 head;
  Alcotest.(check (list int)) "dequeue rest" [ 2 ] rest;
  Alcotest.check_raises "dequeue [] raises Empty_queue" Sec_0_8.Empty_queue (fun () ->
    ignore (Sec_0_8.dequeue []));
  Alcotest.(check float3) "checked_sqrt 9" 3.0 (Sec_0_8.checked_sqrt 9)
;;

let sec_0_8_result_chain () =
  let string_result = Alcotest.result Alcotest.string Alcotest.string in
  Alcotest.check
    string_result
    "half_report \"20\""
    (Ok "10 is half of the input")
    (Sec_0_8.half_report "20");
  Alcotest.check
    string_result
    "half_report \"7\""
    (Error "7 is odd")
    (Sec_0_8.half_report "7");
  Alcotest.check
    string_result
    "half_report \"x\""
    (Error "not a number: x")
    (Sec_0_8.half_report "x")
;;

let sec_0_9_functors () =
  Alcotest.(check int) "Int_summer.sum [1; 2; 3]" 6 (Sec_0_9.Int_summer.sum [ 1; 2; 3 ]);
  Alcotest.(check float3)
    "Float_summer.sum [1.5; 2.5]"
    4.0
    (Sec_0_9.Float_summer.sum [ 1.5; 2.5 ])
;;

let sec_0_9_first_class_packages () =
  Alcotest.(check string)
    "describe_zero int_package"
    "0"
    (Sec_0_9.describe_zero Sec_0_9.int_package);
  Alcotest.(check string)
    "describe_zero float_package"
    "0"
    (Sec_0_9.describe_zero Sec_0_9.float_package);
  Alcotest.(check string)
    "sum_with (module Int_number) [1; 2; 3]"
    "6"
    (Sec_0_9.sum_with (module Sec_0_9.Int_number) [ 1; 2; 3 ]);
  Alcotest.(check string)
    "sum_with (module Float_number) [1.5; 2.5]"
    "4"
    (Sec_0_9.sum_with (module Sec_0_9.Float_number) [ 1.5; 2.5 ])
;;

let sec_0_10_add () = Alcotest.(check int) "add 2 3" 5 (Sec_0_10.add 2 3)

let () =
  Alcotest.run
    "sicp_ch0 replay"
    [ "0.1 toolchain", [ Alcotest.test_case "square" `Quick sec_0_1_square ]
    ; ( "0.2 expressions, values, types"
      , [ Alcotest.test_case "average_of_two_ints" `Quick sec_0_2_average
        ; Alcotest.test_case "warm_enough" `Quick sec_0_2_warm_enough
        ; Alcotest.test_case "greet" `Quick sec_0_2_greet
        ] )
    ; ( "0.3 bindings, functions, modules"
      , [ Alcotest.test_case "named definitions" `Quick sec_0_3_bindings ] )
    ; ( "0.4 pattern matching and variants"
      , [ Alcotest.test_case "shapes" `Quick sec_0_4_shapes
        ; Alcotest.test_case "pattern forms" `Quick sec_0_4_pattern_forms
        ; Alcotest.test_case "hand-rolled data" `Quick sec_0_4_hand_rolled_data
        ] )
    ; ( "0.5 lists and higher-order functions"
      , [ Alcotest.test_case "library forms" `Quick sec_0_5_library_forms
        ; Alcotest.test_case "hand-rolled forms" `Quick sec_0_5_hand_rolled
        ] )
    ; ( "0.6 records, mutable fields, refs"
      , [ Alcotest.test_case "records" `Quick sec_0_6_records
        ; Alcotest.test_case
            "mutable fields and refs"
            `Quick
            sec_0_6_mutable_fields_and_refs
        ; Alcotest.test_case "copying and identity" `Quick sec_0_6_copying_and_identity
        ] )
    ; ( "0.7 closures and lexical scope"
      , [ Alcotest.test_case "closures" `Quick sec_0_7_closures
        ; Alcotest.test_case "private state" `Quick sec_0_7_private_state
        ] )
    ; ( "0.8 errors"
      , [ Alcotest.test_case "exceptions" `Quick sec_0_8_exceptions
        ; Alcotest.test_case "result chain" `Quick sec_0_8_result_chain
        ] )
    ; ( "0.9 modules as values"
      , [ Alcotest.test_case "functors" `Quick sec_0_9_functors
        ; Alcotest.test_case "first-class packages" `Quick sec_0_9_first_class_packages
        ] )
    ; "0.10 testing and formatting", [ Alcotest.test_case "add" `Quick sec_0_10_add ]
    ]
;;
