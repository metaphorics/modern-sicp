(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The runner touches every listing of section 2.2 in the order the book
   presents it, asserting each value the book shows. It also writes this
   edition's generated figures under book/figures/generated/ch2/. *)

module Replay = Sicp_ch1.Replay
module Pic = Sicp_ch2.Sec_2_2.Picture
module Seq = Sicp_ch2.Sec_2_2.Seq_ops
module Trees = Sicp_ch2.Sec_2_2.Trees
module List_ops = Sicp_ch2.Sec_2_2.List_ops

(* [expect computed shown] proves the book's result comment [shown]
   against the computed value. *)
let expect (actual : string) (shown : string) = Replay.expect actual shown
let expect_int computed shown = expect (string_of_int computed) shown
let expect_bool computed shown = expect (string_of_bool computed) shown
let show_int_list l = "[" ^ String.concat "; " (List.map string_of_int l) ^ "]"
let expect_int_list computed shown = expect (show_int_list computed) shown
let show_float_list l = "[" ^ String.concat "; " (List.map string_of_float l) ^ "]"

let show_tree tree =
  let rec go tree =
    match tree with
    | Trees.Leaf n -> "Leaf " ^ string_of_int n
    | Trees.Node children -> "Node [" ^ String.concat "; " (List.map go children) ^ "]"
  in
  go tree
;;

let count painter frame = List.length (painter frame)

let unit_frame =
  Pic.Frame.make_frame
    (Pic.Vect.make_vect 0.0 0.0)
    (Pic.Vect.make_vect 1.0 0.0)
    (Pic.Vect.make_vect 0.0 1.0)
;;

let generated = "../../book/figures/generated/ch2"

let write_figure name painter =
  Pic.write_svg ~path:(Filename.concat generated name) ~size:400 unit_frame painter
;;

let () =
  (* 2.2.1 Representing Sequences *)
  expect_int_list List_ops.one_through_four "[1; 2; 3; 4]";
  expect_int (List_ops.list_ref List_ops.squares 3) "16";
  expect_int (List_ops.length List_ops.odds) "4";
  expect_int (List_ops.length_iter List_ops.odds) "4";
  expect_int_list
    (List_ops.append List_ops.squares List_ops.odds)
    "[1; 4; 9; 16; 25; 1; 3; 5; 7]";
  expect_int_list
    (List_ops.append List_ops.odds List_ops.squares)
    "[1; 3; 5; 7; 1; 4; 9; 16; 25]";
  expect_int_list (List_ops.scale_list [ 1; 2; 3; 4; 5 ] 10) "[10; 20; 30; 40; 50]";
  expect
    (show_float_list (List_ops.map Float.abs [ -10.0; 2.5; -11.6; 17.0 ]))
    "[10.; 2.5; 11.6; 17.]";
  expect_int_list (List_ops.map (fun x -> x * x) [ 1; 2; 3; 4 ]) "[1; 4; 9; 16]";
  expect_int_list (List_ops.scale_list_map [ 1; 2; 3; 4; 5 ] 10) "[10; 20; 30; 40; 50]";
  (* 2.2.2 Hierarchical Structures *)
  expect_int (List_ops.length Trees.x_list) "3";
  expect (show_tree Trees.x) "Node [Node [Leaf 1; Leaf 2]; Leaf 3; Leaf 4]";
  expect_int (Trees.count_leaves Trees.x) "4";
  expect_int (List_ops.length Trees.xx) "2";
  expect_int (Trees.count_leaves (Trees.Node Trees.xx)) "8";
  let scale_me =
    Trees.Node
      [ Trees.Leaf 1
      ; Trees.Node [ Trees.Leaf 2; Trees.Node [ Trees.Leaf 3; Trees.Leaf 4 ] ]
      ; Trees.Leaf 5
      ; Trees.Node [ Trees.Leaf 6; Trees.Leaf 7 ]
      ]
  in
  let scaled_shape =
    "Node [Leaf 10; Node [Leaf 20; Node [Leaf 30; Leaf 40]]; Leaf 50; Node [Leaf 60; \
     Leaf 70]]"
  in
  expect (show_tree (Trees.scale_tree scale_me 10)) scaled_shape;
  expect (show_tree (Trees.scale_tree_map scale_me 10)) scaled_shape;
  (* 2.2.3 Sequence Operations *)
  expect_int_list (Seq.map (fun x -> x * x) [ 1; 2; 3; 4; 5 ]) "[1; 4; 9; 16; 25]";
  expect_int_list (Seq.filter (fun n -> n mod 2 <> 0) [ 1; 2; 3; 4; 5 ]) "[1; 3; 5]";
  expect_int (Seq.accumulate ( + ) 0 [ 1; 2; 3; 4; 5 ]) "15";
  expect_int (Seq.accumulate ( * ) 1 [ 1; 2; 3; 4; 5 ]) "120";
  expect_int_list
    (Seq.accumulate (fun x acc -> x :: acc) [] [ 1; 2; 3; 4; 5 ])
    "[1; 2; 3; 4; 5]";
  expect_int_list (Seq.enumerate_interval 2 7) "[2; 3; 4; 5; 6; 7]";
  let book_tree =
    Trees.Node
      [ Trees.Leaf 1
      ; Trees.Node [ Trees.Leaf 2; Trees.Node [ Trees.Leaf 3; Trees.Leaf 4 ] ]
      ; Trees.Leaf 5
      ]
  in
  expect_int_list (Seq.enumerate_tree book_tree) "[1; 2; 3; 4; 5]";
  expect_int
    (Seq.sum_odd_squares scale_me)
    (string_of_int (Seq.sum_odd_squares_tree scale_me));
  expect_int_list (Seq.even_fibs 10) "[0; 2; 8; 34]";
  expect_int_list (Seq.even_fibs_slow 10) "[0; 2; 8; 34]";
  expect_int_list
    (Seq.list_fib_squares 10)
    "[0; 1; 1; 4; 9; 25; 64; 169; 441; 1156; 3025]";
  expect_int (Seq.product_of_squares_of_odd_elements [ 1; 2; 3; 4; 5 ]) "225";
  expect_int
    (Seq.salary_of_highest_paid_programmer
       [ { is_programmer = true; salary = 680 }; { is_programmer = false; salary = 700 } ])
    "680";
  let encoded_pairs =
    List.map (fun (i, j, s) -> (i * 100) + (j * 10) + s) (Seq.prime_sum_pairs 6)
  in
  expect_int_list encoded_pairs "[213; 325; 415; 437; 527; 617; 661]";
  let perms = List.map show_int_list (Seq.permutations [ 1; 2; 3 ]) in
  expect
    (String.concat " " perms)
    "[1; 2; 3] [1; 3; 2] [2; 1; 3] [2; 3; 1] [3; 1; 2] [3; 2; 1]";
  (* 2.2.4 A Picture Language *)
  expect_int (count Pic.wave unit_frame) "34";
  expect_int (count Pic.wave2 unit_frame) "68";
  expect_int (count Pic.wave4 unit_frame) "136";
  expect_int (count Pic.rogers unit_frame) "13";
  (* The frame coordinate map sends the unit square's origin to the
     frame's origin. *)
  expect_bool
    (Pic.frame_coord_map unit_frame (Pic.Vect.make_vect 0.0 0.0)
     = Pic.Frame.origin_frame unit_frame)
    "true";
  (* The book's two constructions of flipped-pairs and square-limit are
     the same painters: painting them yields the same segments. *)
  expect_bool (Pic.flipped_pairs Pic.wave unit_frame = Pic.wave4 unit_frame) "true";
  expect_bool
    (Pic.square_limit Pic.wave 4 unit_frame
     = Pic.square_limit_of_four Pic.wave 4 unit_frame)
    "true";
  expect_int (count (Pic.right_split Pic.wave 4) unit_frame) "1054";
  expect_int (count (Pic.up_split Pic.wave 4) unit_frame) "1054";
  expect_bool (String.length (Pic.render_to_string unit_frame Pic.wave4) > 1000) "true";
  write_figure "wave.svg" Pic.wave;
  write_figure "wave4.svg" Pic.wave4;
  write_figure "flipped_pairs.svg" (Pic.flipped_pairs Pic.wave);
  write_figure "right_split.svg" (Pic.right_split Pic.wave 4);
  write_figure "corner_split.svg" (Pic.corner_split Pic.wave 4);
  write_figure "square_limit.svg" (Pic.square_limit Pic.wave 4);
  write_figure "squash_inwards.svg" (Pic.squash_inwards Pic.wave);
  write_figure "corner_split_rogers.svg" (Pic.corner_split Pic.rogers 4)
;;
