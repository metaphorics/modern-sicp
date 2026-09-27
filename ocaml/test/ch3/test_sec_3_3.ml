(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the reference solutions' public contracts for
   section 3.3. [sicp_ch3_solutions] implements the same contracts the
   exercises library states with pending stubs, so these tests double
   as the proof of each exercise's stated answer. *)

module Sec_3_12 = Sicp_ch3_solutions.Sec_3_12
module Sec_3_13 = Sicp_ch3_solutions.Sec_3_13
module Sec_3_14 = Sicp_ch3_solutions.Sec_3_14
module Sec_3_15 = Sicp_ch3_solutions.Sec_3_15
module Sec_3_16 = Sicp_ch3_solutions.Sec_3_16
module Sec_3_17 = Sicp_ch3_solutions.Sec_3_17
module Sec_3_18 = Sicp_ch3_solutions.Sec_3_18
module Sec_3_19 = Sicp_ch3_solutions.Sec_3_19
module Sec_3_20 = Sicp_ch3_solutions.Sec_3_20
module Sec_3_21 = Sicp_ch3_solutions.Sec_3_21
module Sec_3_22 = Sicp_ch3_solutions.Sec_3_22
module Sec_3_23 = Sicp_ch3_solutions.Sec_3_23
module Sec_3_24 = Sicp_ch3_solutions.Sec_3_24
module Sec_3_25 = Sicp_ch3_solutions.Sec_3_25
module Sec_3_26 = Sicp_ch3_solutions.Sec_3_26
module Sec_3_27 = Sicp_ch3_solutions.Sec_3_27
module Sec_3_28 = Sicp_ch3_solutions.Sec_3_28
module Sec_3_29 = Sicp_ch3_solutions.Sec_3_29
module Sec_3_30 = Sicp_ch3_solutions.Sec_3_30
module Sec_3_31 = Sicp_ch3_solutions.Sec_3_31
module Sec_3_32 = Sicp_ch3_solutions.Sec_3_32
module Sec_3_33 = Sicp_ch3_solutions.Sec_3_33
module Sec_3_34 = Sicp_ch3_solutions.Sec_3_34
module Sec_3_35 = Sicp_ch3_solutions.Sec_3_35
module Sec_3_36 = Sicp_ch3_solutions.Sec_3_36
module Sec_3_37 = Sicp_ch3_solutions.Sec_3_37

let mobj_opt = function
  | Some v -> Sicp_ch3.Sec_3_3.Mpairs.show v
  | None -> "None"
;;

let ex_3_12_append_replays_transcript () =
  let z, cdr1, w, cdr2 = Sec_3_12.ex_3_12 () in
  Alcotest.(check string) "z prints as (a b c d)" "(a b c d)" z;
  Alcotest.(check string) "pure append leaves x's own tail alone" "(b)" cdr1;
  Alcotest.(check string) "w prints as (a b c d)" "(a b c d)" w;
  Alcotest.(check string) "append! splices x's last pair onto y" "(b c d)" cdr2
;;

let ex_3_13_cycle_never_ends () =
  let returns_to_start, last_pair_gives_up = Sec_3_13.ex_3_13 () in
  Alcotest.(check bool) "a lap around the ring returns to the start" true returns_to_start;
  Alcotest.(check bool) "last_pair never finds an end" true last_pair_gives_up
;;

let ex_3_13a_cycle_safe_printing () =
  let z3, z2, plain = Sec_3_13.ex_3_13a () in
  Alcotest.(check string) "three-pair ring prints with #cycle" "(a b c . #cycle)" z3;
  Alcotest.(check string) "two-pair ring prints with #cycle" "(a b . #cycle)" z2;
  Alcotest.(check string) "an acyclic list prints plainly" "(a b)" plain
;;

let ex_3_14_mystery_reverses_in_place () =
  let before, after, w = Sec_3_14.ex_3_14 () in
  Alcotest.(check string) "v before the call" "(a b c d)" before;
  Alcotest.(check string) "v after: only its first pair survives" "(a)" after;
  Alcotest.(check string) "w holds the reversal" "(d c b a)" w
;;

let ex_3_15_shared_vs_unshared_structure () =
  let z1_shared, z2_shared, z1_shown, z2_shown = Sec_3_15.ex_3_15 () in
  Alcotest.(check bool) "z1's two slots are the same pair" true z1_shared;
  Alcotest.(check bool) "z2's two slots are distinct pairs" false z2_shared;
  Alcotest.(check string) "set_to_wow on z1 changes both slots" "((wow b) wow b)" z1_shown;
  Alcotest.(check string) "set_to_wow on z2 changes only the car" "((wow b) a b)" z2_shown
;;

let ex_3_16_bens_flawed_count () =
  let three, four, seven, b3, b4, b7, ring = Sec_3_16.ex_3_16 () in
  Alcotest.(check int) "three unshared pairs" 3 three;
  Alcotest.(check int) "once-shared pair counted twice" 4 four;
  Alcotest.(check int) "twice-shared pairs counted seven times" 7 seven;
  Alcotest.(check (option int)) "bounded walk agrees on three" (Some 3) b3;
  Alcotest.(check (option int)) "bounded walk agrees on four" (Some 4) b4;
  Alcotest.(check (option int)) "bounded walk agrees on seven" (Some 7) b7;
  Alcotest.(check (option int)) "the ring never finishes" None ring
;;

let ex_3_17_distinct_pairs_agree () =
  let a, b, c, d = Sec_3_17.ex_3_17 () in
  Alcotest.(check int) "unshared structure" 3 a;
  Alcotest.(check int) "once-shared structure" 3 b;
  Alcotest.(check int) "twice-shared structure" 3 c;
  Alcotest.(check int) "the ring, three distinct pairs" 3 d
;;

let ex_3_18_detects_cycles () =
  let plain, ring, self, nil = Sec_3_18.ex_3_18 () in
  Alcotest.(check bool) "a plain list has no cycle" false plain;
  Alcotest.(check bool) "the closed ring has a cycle" true ring;
  Alcotest.(check bool) "a self-pointing pair has a cycle" true self;
  Alcotest.(check bool) "the empty list has no cycle" false nil
;;

let ex_3_19_constant_space_detection_agrees () =
  let plain, ring, self, suffix = Sec_3_19.ex_3_19 () in
  Alcotest.(check bool) "a plain list has no cycle" false plain;
  Alcotest.(check bool) "the closed ring has a cycle" true ring;
  Alcotest.(check bool) "a self-pointing pair has a cycle" true self;
  Alcotest.(check bool) "a shared, acyclic suffix is not a cycle" false suffix
;;

let ex_3_20_traces_aliasing () =
  let car_after, same_object = Sec_3_20.ex_3_20 () in
  Alcotest.(check string) "(car x) after (set-car! (cdr z) 17)" "17" car_after;
  Alcotest.(check bool) "both slots of z designate x itself" true same_object
;;

let ex_3_21_naive_view_goes_stale () =
  let v1, v2, v3, v4, v5, v6, v7 = Sec_3_21.ex_3_21 () in
  Alcotest.(check string) "after insert a" "((a) a)" v1;
  Alcotest.(check string) "after insert b" "((a b) b)" v2;
  Alcotest.(check string) "after the first delete" "((b) b)" v3;
  Alcotest.(check string) "after the second delete, the naive view is stale" "(() b)" v4;
  Alcotest.(check string) "print_queue shows a proper empty queue" "()" v5;
  Alcotest.(check bool) "the queue answers empty" true v6;
  Alcotest.(check string) "the stale rear pointer persists" "(() b)" v7
;;

let ex_3_22_queue_object () =
  let front_after_delete, q1_empty, second_front, second_empty = Sec_3_22.ex_3_22 () in
  Alcotest.(check string) "q1's front after insert, insert, delete" "b" front_after_delete;
  Alcotest.(check bool) "q1 is not empty" false q1_empty;
  Alcotest.(check string) "a fresh queue's front after its own insert" "c" second_front;
  Alcotest.(check bool) "that fresh queue is not empty either" false second_empty
;;

let ex_3_23_deque_constant_time_ends () =
  let after_three, after_deletes, front, rear, empty = Sec_3_23.ex_3_23 () in
  Alcotest.(check string)
    "rear-insert b, front-insert a, rear-insert c"
    "a b c"
    after_three;
  Alcotest.(check string)
    "front-delete then rear-delete leaves the middle"
    "b"
    after_deletes;
  Alcotest.(check string) "front is the sole survivor" "b" front;
  Alcotest.(check string) "rear is the sole survivor too" "b" rear;
  Alcotest.(check bool) "the deque is not empty" false empty
;;

let ex_3_24_tolerant_table () =
  let exact, within, outside, other = Sec_3_24.ex_3_24 () in
  Alcotest.(check string) "exact key match" "1" (mobj_opt exact);
  Alcotest.(check string) "one step inside the tolerance" "1" (mobj_opt within);
  Alcotest.(check string) "two steps outside the tolerance" "None" (mobj_opt outside);
  Alcotest.(check string) "the second row, key inside tolerance" "2" other
;;

let ex_3_25_table_of_arbitrary_key_lists () =
  let abc, abd, ae, x, missing, abc_again, abd_after = Sec_3_25.ex_3_25 () in
  Alcotest.(check string) "value under [a; b; c]" "1" (mobj_opt abc);
  Alcotest.(check string) "value under the sibling path [a; b; d]" "2" (mobj_opt abd);
  Alcotest.(check string) "value under the shorter path [a; e]" "3" (mobj_opt ae);
  Alcotest.(check string) "value under the top-level path [x]" "4" (mobj_opt x);
  Alcotest.(check string)
    "a path never inserted answers nothing"
    "None"
    (mobj_opt missing);
  Alcotest.(check string) "an overwrite through the same path" "5" (mobj_opt abc_again);
  Alcotest.(check string)
    "the sibling path is untouched by that overwrite"
    "2"
    (mobj_opt abd_after)
;;

let ex_3_26_binary_tree_table () =
  let out_of_order, overwritten, missing = Sec_3_26.ex_3_26 () in
  Alcotest.(check string)
    "an out-of-order build still answers every key"
    "ten thirty fifty seventy eighty"
    out_of_order;
  Alcotest.(check string)
    "inserting under an existing key overwrites it"
    "new"
    overwritten;
  Alcotest.(check string) "a key never inserted answers nothing" "None" (mobj_opt missing)
;;

let ex_3_27_memoized_fib () =
  let memo, plain, computes, hits, plain_calls, smaller = Sec_3_27.ex_3_27 () in
  Alcotest.(check int) "memoized fib 25" 75025 memo;
  Alcotest.(check int) "plain fib 25 agrees" 75025 plain;
  Alcotest.(check int) "one compute per distinct argument, 0..25" 26 computes;
  Alcotest.(check int) "every repeated argument is a hit" 23 hits;
  Alcotest.(check int) "the plain recursion's call count" 242785 plain_calls;
  Alcotest.(check bool) "memoization takes far fewer steps" true smaller
;;

let ex_3_28_or_gate_primitive () =
  let (o00, o01, o10, o11), l00, l01, l10, l11 = Sec_3_28.ex_3_28 () in
  Alcotest.(check int) "0 or 0" 0 o00;
  Alcotest.(check int) "0 or 1" 1 o01;
  Alcotest.(check int) "1 or 0" 1 o10;
  Alcotest.(check int) "1 or 1" 1 o11;
  Alcotest.(check int) "logical_or 0 0" 0 l00;
  Alcotest.(check int) "logical_or 0 1" 1 l01;
  Alcotest.(check int) "logical_or 1 0" 1 l10;
  Alcotest.(check int) "logical_or 1 1" 1 l11
;;

let ex_3_29_or_gate_from_and_and_not () =
  let outputs, delay10, delay01, expected_delay = Sec_3_29.ex_3_29 () in
  Alcotest.(check (list int)) "the or truth table" [ 0; 1; 1; 1 ] outputs;
  Alcotest.(check int) "measured delay for 1, 0" 7 delay10;
  Alcotest.(check int) "measured delay for 0, 1" 7 delay01;
  Alcotest.(check int) "two inverter delays plus one and-gate delay" 7 expected_delay
;;

let ex_3_30_ripple_carry_adder () =
  let (s1, c1), (s2, c2), (s3, c3) = Sec_3_30.ex_3_30 () in
  Alcotest.(check (pair int int)) "5 + 3 on four bits" (8, 0) (s1, c1);
  Alcotest.(check (pair int int)) "7 + 7 on four bits" (14, 0) (s2, c2);
  Alcotest.(check (pair int int)) "15 + 1 on four bits overflows" (0, 1) (s3, c3)
;;

let ex_3_31_initial_action_run () =
  let with_initial_run, without = Sec_3_31.ex_3_31 () in
  Alcotest.(check int) "an action run at attach reflects the input" 1 with_initial_run;
  Alcotest.(check int) "without that initial run the output stays stale" 0 without
;;

let ex_3_32_segment_order_matters () =
  let fifo, lifo = Sec_3_32.ex_3_32 () in
  Alcotest.(check int) "the book's FIFO agenda answers correctly" 0 fifo;
  Alcotest.(check int) "a LIFO agenda answers the stale, wrong value" 1 lifo
;;

let ex_3_33_averager_constraint () =
  let avg, solved_b, a_in_force, c_at_end = Sec_3_33.ex_3_33 () in
  Alcotest.(check int) "the average of 6 and 14" 10 avg;
  Alcotest.(check int) "the addend the network derives for c = 12, a = 4" 20 solved_b;
  Alcotest.(check int) "a stays at the value it was set to" 4 a_in_force;
  Alcotest.(check int) "c stays at the value it was set to" 12 c_at_end
;;

let ex_3_34_flawed_squarer () =
  let a_derived, a_forced, b_after = Sec_3_34.ex_3_34 () in
  Alcotest.(check bool) "setting b alone never derives a" false a_derived;
  Alcotest.(check int) "a must be forced by hand" 5 a_forced;
  Alcotest.(check int) "b then holds a's square, consistently" 25 b_after
;;

let ex_3_35_squarer_as_primitive_constraint () =
  let a_from_b, b_from_a = Sec_3_35.ex_3_35 () in
  Alcotest.(check int) "setting b = 49 derives a = 7" 7 a_from_b;
  Alcotest.(check int) "setting a = 6 after a forget derives b = 36" 36 b_from_a;
  Alcotest.(check bool)
    "a negative product is a programming error"
    true
    (Sec_3_35.negative_raises ())
;;

let ex_3_36_connector_dispatch_trace () =
  let set_trace, forget_trace = Sec_3_36.ex_3_36 () in
  Alcotest.(check bool)
    "setting dispatches through the constraint's me"
    true
    (List.length set_trace > 0);
  Alcotest.(check bool)
    "forgetting dispatches through the constraint's me too"
    true
    (List.length forget_trace > 0)
;;

let ex_3_37_expression_style_combinators () =
  let f_at_25c, c_at_212f, difference, solved_x = Sec_3_37.ex_3_37 () in
  Alcotest.(check int) "25 Celsius is 77 Fahrenheit" 77 f_at_25c;
  Alcotest.(check int) "212 Fahrenheit is 100 Celsius" 100 c_at_212f;
  Alcotest.(check int) "10 - 4 through c_sub" 6 difference;
  Alcotest.(check int) "x - 4 = 3 solves to x = 7" 7 solved_x
;;

let () =
  Alcotest.run
    "sec_3_3"
    [ ( "sec_3_3"
      , [ Alcotest.test_case
            "3.12 append replays the transcript"
            `Quick
            ex_3_12_append_replays_transcript
        ; Alcotest.test_case "3.13 make-cycle never ends" `Quick ex_3_13_cycle_never_ends
        ; Alcotest.test_case
            "3.13a cycle-safe printing"
            `Quick
            ex_3_13a_cycle_safe_printing
        ; Alcotest.test_case
            "3.14 mystery reverses in place"
            `Quick
            ex_3_14_mystery_reverses_in_place
        ; Alcotest.test_case
            "3.15 shared vs unshared structure"
            `Quick
            ex_3_15_shared_vs_unshared_structure
        ; Alcotest.test_case "3.16 Ben's flawed count" `Quick ex_3_16_bens_flawed_count
        ; Alcotest.test_case
            "3.17 distinct pairs agree"
            `Quick
            ex_3_17_distinct_pairs_agree
        ; Alcotest.test_case "3.18 detects cycles" `Quick ex_3_18_detects_cycles
        ; Alcotest.test_case
            "3.19 constant-space detection agrees"
            `Quick
            ex_3_19_constant_space_detection_agrees
        ; Alcotest.test_case "3.20 traces aliasing" `Quick ex_3_20_traces_aliasing
        ; Alcotest.test_case
            "3.21 naive view goes stale"
            `Quick
            ex_3_21_naive_view_goes_stale
        ; Alcotest.test_case "3.22 queue object" `Quick ex_3_22_queue_object
        ; Alcotest.test_case
            "3.23 deque constant-time ends"
            `Quick
            ex_3_23_deque_constant_time_ends
        ; Alcotest.test_case "3.24 tolerant table" `Quick ex_3_24_tolerant_table
        ; Alcotest.test_case
            "3.25 table of arbitrary key lists"
            `Quick
            ex_3_25_table_of_arbitrary_key_lists
        ; Alcotest.test_case "3.26 binary-tree table" `Quick ex_3_26_binary_tree_table
        ; Alcotest.test_case "3.27 memoized fib" `Quick ex_3_27_memoized_fib
        ; Alcotest.test_case "3.28 or-gate primitive" `Quick ex_3_28_or_gate_primitive
        ; Alcotest.test_case
            "3.29 or-gate from and and not"
            `Quick
            ex_3_29_or_gate_from_and_and_not
        ; Alcotest.test_case "3.30 ripple-carry adder" `Quick ex_3_30_ripple_carry_adder
        ; Alcotest.test_case "3.31 initial action run" `Quick ex_3_31_initial_action_run
        ; Alcotest.test_case
            "3.32 segment order matters"
            `Quick
            ex_3_32_segment_order_matters
        ; Alcotest.test_case "3.33 averager constraint" `Quick ex_3_33_averager_constraint
        ; Alcotest.test_case "3.34 flawed squarer" `Quick ex_3_34_flawed_squarer
        ; Alcotest.test_case
            "3.35 squarer as primitive constraint"
            `Quick
            ex_3_35_squarer_as_primitive_constraint
        ; Alcotest.test_case
            "3.36 connector dispatch trace"
            `Quick
            ex_3_36_connector_dispatch_trace
        ; Alcotest.test_case
            "3.37 expression-style combinators"
            `Quick
            ex_3_37_expression_style_combinators
        ] )
    ]
;;
