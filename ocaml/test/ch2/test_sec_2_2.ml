(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the reference solutions' public contracts for
   section 2.2. [sicp_ch2_solutions] implements the same contracts the
   exercises library states with pending stubs, so these tests double
   as the proof of each exercise's stated answer. *)

module S17 = Sicp_ch2_solutions.Sec_2_17
module S18 = Sicp_ch2_solutions.Sec_2_18
module S19 = Sicp_ch2_solutions.Sec_2_19
module S20 = Sicp_ch2_solutions.Sec_2_20
module S21 = Sicp_ch2_solutions.Sec_2_21
module S22 = Sicp_ch2_solutions.Sec_2_22
module S23 = Sicp_ch2_solutions.Sec_2_23
module S24 = Sicp_ch2_solutions.Sec_2_24
module S25 = Sicp_ch2_solutions.Sec_2_25
module S26 = Sicp_ch2_solutions.Sec_2_26
module S27 = Sicp_ch2_solutions.Sec_2_27
module S28 = Sicp_ch2_solutions.Sec_2_28
module S29 = Sicp_ch2_solutions.Sec_2_29
module S30 = Sicp_ch2_solutions.Sec_2_30
module S31 = Sicp_ch2_solutions.Sec_2_31
module S32 = Sicp_ch2_solutions.Sec_2_32
module S33 = Sicp_ch2_solutions.Sec_2_33
module S34 = Sicp_ch2_solutions.Sec_2_34
module S35 = Sicp_ch2_solutions.Sec_2_35
module S36 = Sicp_ch2_solutions.Sec_2_36
module S37 = Sicp_ch2_solutions.Sec_2_37
module S38 = Sicp_ch2_solutions.Sec_2_38
module S39 = Sicp_ch2_solutions.Sec_2_39
module S40 = Sicp_ch2_solutions.Sec_2_40
module S41 = Sicp_ch2_solutions.Sec_2_41
module S42 = Sicp_ch2_solutions.Sec_2_42
module S43 = Sicp_ch2_solutions.Sec_2_43
module S44 = Sicp_ch2_solutions.Sec_2_44
module S45 = Sicp_ch2_solutions.Sec_2_45
module S46 = Sicp_ch2_solutions.Sec_2_46
module S47 = Sicp_ch2_solutions.Sec_2_47
module S48 = Sicp_ch2_solutions.Sec_2_48
module S49 = Sicp_ch2_solutions.Sec_2_49
module S50 = Sicp_ch2_solutions.Sec_2_50
module S51 = Sicp_ch2_solutions.Sec_2_51
module S52 = Sicp_ch2_solutions.Sec_2_52

let int_list =
  Alcotest.testable
    (fun fmt l ->
       Format.fprintf fmt "[%s]" (String.concat "; " (List.map string_of_int l)))
    ( = )
;;

let eq_bool name a b = Alcotest.check Alcotest.bool name true (a = b)

let book_tree () =
  S24.Node [ S24.Leaf 1; S24.Node [ S24.Leaf 2; S24.Node [ S24.Leaf 3; S24.Leaf 4 ] ] ]
;;

let unit_frame_44 =
  { S44.origin = { S44.x = 0.0; y = 0.0 }
  ; edge1 = { S44.x = 1.0; y = 0.0 }
  ; edge2 = { S44.x = 0.0; y = 1.0 }
  }
;;

let unit_frame_45 =
  { S45.origin = { S45.x = 0.0; y = 0.0 }
  ; edge1 = { S45.x = 1.0; y = 0.0 }
  ; edge2 = { S45.x = 0.0; y = 1.0 }
  }
;;

let unit_frame_49 =
  { S49.origin = { S49.x = 0.0; y = 0.0 }
  ; edge1 = { S49.x = 1.0; y = 0.0 }
  ; edge2 = { S49.x = 0.0; y = 1.0 }
  }
;;

let unit_frame_50 =
  { S50.origin = { S50.x = 0.0; y = 0.0 }
  ; edge1 = { S50.x = 1.0; y = 0.0 }
  ; edge2 = { S50.x = 0.0; y = 1.0 }
  }
;;

let ex_2_17_last_pair () =
  Alcotest.check int_list "book example" [ 34 ] (S17.ex_2_17 [ 23; 72; 149; 34 ]);
  Alcotest.check int_list "singleton" [ 7 ] (S17.ex_2_17 [ 7 ])
;;

let ex_2_18_reverse () =
  Alcotest.check
    int_list
    "book example"
    [ 25; 16; 9; 4; 1 ]
    (S18.ex_2_18 [ 1; 4; 9; 16; 25 ]);
  Alcotest.check int_list "empty" [] (S18.ex_2_18 []);
  Alcotest.check int_list "involution" [ 3; 2; 1 ] (S18.ex_2_18 (S18.ex_2_18 [ 3; 2; 1 ]))
;;

let ex_2_19_change () =
  Alcotest.check Alcotest.int "book example" 292 (S19.ex_2_19 ());
  Alcotest.check Alcotest.int "one kind of coin" 1 (S19.cc 5.0 [ 5.0 ]);
  Alcotest.check Alcotest.int "impossible amount" 0 (S19.cc 3.0 [ 5.0 ]);
  Alcotest.check
    Alcotest.int
    "order independent"
    (S19.cc 11.0 [ 50.0; 25.0; 10.0; 5.0; 1.0 ])
    (S19.cc 11.0 [ 1.0; 5.0; 10.0; 25.0; 50.0 ])
;;

let ex_2_20_same_parity () =
  Alcotest.check
    int_list
    "book example 1"
    [ 1; 3; 5; 7 ]
    (S20.ex_2_20 1 [ 2; 3; 4; 5; 6; 7 ]);
  Alcotest.check int_list "book example 2" [ 2; 4; 6 ] (S20.ex_2_20 2 [ 3; 4; 5; 6; 7 ]);
  Alcotest.check int_list "negative parity" [ -3; -1; 1 ] (S20.ex_2_20 (-3) [ -1; 1; 2 ])
;;

let ex_2_21_square_list () =
  Alcotest.check int_list "direct" [ 1; 4; 9; 16 ] (S21.ex_2_21_direct [ 1; 2; 3; 4 ]);
  Alcotest.check int_list "map" [ 1; 4; 9; 16 ] (S21.ex_2_21_map [ 1; 2; 3; 4 ])
;;

let ex_2_22_louis_bug () =
  (* The iterative version conses each square onto the accumulated
     answer, so the order comes out reversed. *)
  Alcotest.check
    int_list
    "reversed answer"
    [ 16; 9; 4; 1 ]
    (S22.ex_2_22_louis [ 1; 2; 3; 4 ])
;;

let ex_2_23_for_each () =
  let seen = ref [] in
  S23.ex_2_23 (fun x -> seen := x :: !seen) [ 57; 321; 88 ];
  Alcotest.check int_list "visited left to right" [ 88; 321; 57 ] !seen
;;

let ex_2_24_tree_reading () =
  eq_bool "printed form" (S24.ex_2_24 ()) (book_tree ());
  let rec count_leaves t =
    match t with
    | S24.Leaf _ -> 1
    | S24.Node children -> List.fold_left (fun acc c -> acc + count_leaves c) 0 children
  in
  Alcotest.check Alcotest.int "four leaves as a tree" 4 (count_leaves (S24.ex_2_24 ()))
;;

let ex_2_25_pick_seven () =
  let a, b, c = S25.ex_2_25 () in
  Alcotest.check Alcotest.int "first 7" 7 a;
  Alcotest.check Alcotest.int "second 7" 7 b;
  Alcotest.check Alcotest.int "third 7" 7 c
;;

let ex_2_26_combinations () =
  let appends, lists = S26.ex_2_26 () in
  Alcotest.check int_list "x @ y" [ 1; 2; 3; 4; 5; 6 ] appends;
  eq_bool "[x; y]" lists [ [ 1; 2; 3 ]; [ 4; 5; 6 ] ]
;;

let ex_2_27_deep_reverse () =
  let x =
    S27.Node [ S27.Node [ S27.Leaf 1; S27.Leaf 2 ]; S27.Node [ S27.Leaf 3; S27.Leaf 4 ] ]
  in
  eq_bool
    "book example"
    (S27.ex_2_27 x)
    (S27.Node [ S27.Node [ S27.Leaf 4; S27.Leaf 3 ]; S27.Node [ S27.Leaf 2; S27.Leaf 1 ] ]);
  eq_bool "leaf fixed" (S27.ex_2_27 (S27.Leaf 5)) (S27.Leaf 5)
;;

let ex_2_28_fringe () =
  let x =
    S28.Node [ S28.Node [ S28.Leaf 1; S28.Leaf 2 ]; S28.Node [ S28.Leaf 3; S28.Leaf 4 ] ]
  in
  Alcotest.check int_list "book example" [ 1; 2; 3; 4 ] (S28.ex_2_28 x);
  Alcotest.check
    int_list
    "two trees"
    [ 1; 2; 3; 4; 1; 2; 3; 4 ]
    (S28.ex_2_28 (S28.Node [ x; x ]))
;;

let ex_2_29_mobiles () =
  let sub = S29.Mobile (S29.Branch (2, S29.Weight 3), S29.Branch (2, S29.Weight 3)) in
  let m = S29.Mobile (S29.Branch (3, S29.Hanging sub), S29.Branch (6, S29.Weight 3)) in
  Alcotest.check
    (Alcotest.pair Alcotest.int Alcotest.bool)
    "ex_2_29 sample"
    (9, true)
    (S29.ex_2_29 m);
  Alcotest.check Alcotest.int "total weight" 9 (S29.total_weight m);
  Alcotest.check Alcotest.bool "balanced" true (S29.balanced m);
  let crooked = S29.Mobile (S29.Branch (2, S29.Weight 1), S29.Branch (2, S29.Weight 3)) in
  Alcotest.check Alcotest.bool "unbalanced" false (S29.balanced crooked);
  let sub_pairs = (2, S29.Cons_repr.Weight 3), (2, S29.Cons_repr.Weight 3) in
  let m_pairs = (3, S29.Cons_repr.Hanging sub_pairs), (6, S29.Cons_repr.Weight 3) in
  Alcotest.check
    Alcotest.int
    "pair repr weight"
    (S29.total_weight m)
    (S29.Cons_repr.total_weight m_pairs);
  Alcotest.check Alcotest.bool "pair repr balance" true (S29.Cons_repr.balanced m_pairs)
;;

let ex_2_30_square_tree () =
  let input =
    S30.Node [ S30.Leaf 1; S30.Node [ S30.Leaf 2; S30.Node [ S30.Leaf 3; S30.Leaf 4 ] ] ]
  in
  let expected =
    S30.Node [ S30.Leaf 1; S30.Node [ S30.Leaf 4; S30.Node [ S30.Leaf 9; S30.Leaf 16 ] ] ]
  in
  eq_bool "direct" (S30.ex_2_30_direct input) expected;
  eq_bool "map" (S30.ex_2_30_map input) expected
;;

let ex_2_31_tree_map () =
  let input =
    S31.Node [ S31.Leaf 1; S31.Node [ S31.Leaf 2; S31.Node [ S31.Leaf 3; S31.Leaf 4 ] ] ]
  in
  let expected =
    S31.Node [ S31.Leaf 1; S31.Node [ S31.Leaf 4; S31.Node [ S31.Leaf 9; S31.Leaf 16 ] ] ]
  in
  eq_bool "square through tree_map" (S31.ex_2_31_square_tree input) expected;
  eq_bool
    "tree_map doubles"
    (S31.ex_2_31_tree_map (fun n -> n * 2) input)
    (S31.Node [ S31.Leaf 2; S31.Node [ S31.Leaf 4; S31.Node [ S31.Leaf 6; S31.Leaf 8 ] ] ])
;;

let ex_2_32_subsets () =
  let result = S32.ex_2_32 [ 1; 2; 3 ] in
  Alcotest.check Alcotest.int "eight subsets" 8 (List.length result);
  Alcotest.check int_list "empty first" [] (List.hd result);
  Alcotest.check int_list "full last" [ 1; 2; 3 ] (List.nth result 7);
  Alcotest.check Alcotest.int "empty set has one subset" 1 (List.length (S32.ex_2_32 []))
;;

let ex_2_33_accumulations () =
  Alcotest.check int_list "map" [ 1; 4; 9 ] (S33.ex_2_33_map (fun x -> x * x) [ 1; 2; 3 ]);
  Alcotest.check int_list "append" [ 1; 2; 3; 4 ] (S33.ex_2_33_append [ 1; 2 ] [ 3; 4 ]);
  Alcotest.check Alcotest.int "length" 4 (S33.ex_2_33_length [ 1; 2; 3; 4 ]);
  Alcotest.check int_list "map empty" [] (S33.ex_2_33_map (fun x -> x) [])
;;

let ex_2_34_horner () =
  Alcotest.check Alcotest.int "book example" 79 (S34.ex_2_34 2 [ 1; 3; 0; 5; 0; 1 ]);
  Alcotest.check Alcotest.int "constant" 7 (S34.ex_2_34 9 [ 7 ])
;;

let ex_2_35_count_leaves () =
  let x = S35.Node [ S35.Node [ S35.Leaf 1; S35.Leaf 2 ]; S35.Leaf 3; S35.Leaf 4 ] in
  Alcotest.check Alcotest.int "four leaves" 4 (S35.ex_2_35 x);
  Alcotest.check Alcotest.int "eight leaves" 8 (S35.ex_2_35 (S35.Node [ x; x ]))
;;

let ex_2_36_accumulate_n () =
  let s = [ [ 1; 2; 3 ]; [ 4; 5; 6 ]; [ 7; 8; 9 ]; [ 10; 11; 12 ] ] in
  Alcotest.check int_list "book example" [ 22; 26; 30 ] (S36.ex_2_36 ( + ) 0 s)
;;

let ex_2_37_matrices () =
  let m = [ [ 1; 2; 3; 4 ]; [ 4; 5; 6; 6 ]; [ 6; 7; 8; 9 ] ] in
  Alcotest.check Alcotest.int "ex_2_37 sample" 32 (S37.ex_2_37 ());
  Alcotest.check Alcotest.int "dot product" 32 (S37.dot_product [ 1; 2; 3 ] [ 4; 5; 6 ]);
  Alcotest.check
    int_list
    "matrix * vector"
    [ 10; 21; 30 ]
    (S37.matrix_times_vector m [ 1; 1; 1; 1 ]);
  Alcotest.check int_list "transpose row 1" [ 1; 4; 6 ] (List.hd (S37.transpose m));
  Alcotest.check int_list "transpose row 4" [ 4; 6; 9 ] (List.nth (S37.transpose m) 3);
  Alcotest.check
    int_list
    "product row 1"
    [ 30; 56; 80 ]
    (List.hd (S37.matrix_times_matrix m (S37.transpose m)))
;;

let ex_2_38_folds () =
  let fr_div, fl_div, fr_cons, fl_cons = S38.ex_2_38 () in
  Alcotest.check (Alcotest.float 1e-9) "fold_right /" 1.5 fr_div;
  Alcotest.check (Alcotest.float 1e-9) "fold_left /" (1.0 /. 6.0) fl_div;
  Alcotest.check int_list "fold_right cons" [ 1; 2; 3 ] fr_cons;
  Alcotest.check int_list "fold_left cons" [ 3; 2; 1 ] fl_cons;
  Alcotest.check Alcotest.int "sum agrees" 10 (S38.fold_left ( + ) 0 [ 1; 2; 3; 4 ])
;;

let ex_2_39_reverse_folds () =
  Alcotest.check
    int_list
    "right"
    [ 25; 16; 9; 4; 1 ]
    (S39.ex_2_39_right [ 1; 4; 9; 16; 25 ]);
  Alcotest.check
    int_list
    "left"
    [ 25; 16; 9; 4; 1 ]
    (S39.ex_2_39_left [ 1; 4; 9; 16; 25 ]);
  Alcotest.check int_list "right empty" [] (S39.ex_2_39_right []);
  Alcotest.check int_list "left empty" [] (S39.ex_2_39_left [])
;;

let ex_2_40_unique_pairs () =
  Alcotest.check
    (Alcotest.list (Alcotest.pair Alcotest.int Alcotest.int))
    "pairs of 3"
    [ 2, 1; 3, 1; 3, 2 ]
    (S40.ex_2_40_unique_pairs 3);
  let encoded =
    List.map (fun (i, j, s) -> (i * 100) + (j * 10) + s) (S40.ex_2_40_prime_sum_pairs 6)
  in
  Alcotest.check int_list "prime sum pairs" [ 213; 325; 415; 437; 527; 617; 661 ] encoded
;;

let ex_2_41_triples () =
  let answers = S41.ex_2_41 5 10 in
  List.iter
    (fun (i, j, k) ->
       Alcotest.check
         Alcotest.bool
         (Printf.sprintf "distinct and ordered %d %d %d" i j k)
         true
         (k < j && j < i && i + j + k = 10))
    answers;
  Alcotest.check Alcotest.bool "has (5, 4, 1)" true (List.mem (5, 4, 1) answers);
  Alcotest.check Alcotest.bool "has (5, 3, 2)" true (List.mem (5, 3, 2) answers);
  Alcotest.check Alcotest.int "no triple for 4 10" 0 (List.length (S41.ex_2_41 4 10))
;;

let ex_2_42_queens () =
  Alcotest.check Alcotest.int "6x6 has 4 solutions" 4 (List.length (S42.ex_2_42 6));
  Alcotest.check Alcotest.int "8x8 has 92 solutions" 92 (List.length (S42.ex_2_42 8));
  let first = List.hd (S42.ex_2_42 8) in
  Alcotest.check Alcotest.int "each solution has 8 queens" 8 (List.length first);
  Alcotest.check Alcotest.bool "empty board safe" true (S42.safe 1 S42.empty_board)
;;

let ex_2_43_louis () =
  Alcotest.check Alcotest.bool "louis agrees with the reference" true (S43.ex_2_43 ());
  Alcotest.check
    Alcotest.int
    "louis 6x6 has 4 solutions"
    4
    (List.length (S43.louis_queens 6))
;;

let ex_2_44_up_split () =
  (* lift the 2.49 wave into 2.44's own painter type *)
  let to_49 (f : S44.frame) : S49.frame =
    let conv v = { S49.x = v.S44.x; y = v.S44.y } in
    { S49.origin = conv f.S44.origin; edge1 = conv f.S44.edge1; edge2 = conv f.S44.edge2 }
  in
  let back_v v = { S44.x = v.S49.x; y = v.S49.y } in
  let wave44 frame =
    List.map (fun (a, b) -> back_v a, back_v b) (S49.ex_2_49_wave (to_49 frame))
  in
  (* the same recursion right_split obeys: c + 2 smaller, 31 copies
     of a 20-segment painter at level 4 *)
  Alcotest.check
    Alcotest.int
    "620 segments"
    620
    (List.length (S44.ex_2_44_up_split wave44 4 unit_frame_44));
  Alcotest.check
    Alcotest.int
    "level 0 is the painter"
    20
    (List.length (S44.ex_2_44_up_split wave44 0 unit_frame_44))
;;

let ex_2_45_split () =
  (* The recursion identity the combinator must satisfy, checked with
     simple list combiners over the module's own painter type. *)
  let v x y = { S45.x; y } in
  let p frame = [ frame.S45.origin, v 1.0 1.0 ] in
  let beside p1 p2 frame = p1 frame @ p2 frame in
  let double p1 p2 frame = p2 frame @ p1 frame in
  let split = S45.ex_2_45_split beside double in
  eq_bool "level 0 identity" (split p 0 unit_frame_45) (p unit_frame_45);
  eq_bool
    "level 1 recursion"
    (split p 1 unit_frame_45)
    (beside p (double p p) unit_frame_45);
  let smaller = split p 1 in
  eq_bool
    "level 2 recursion"
    (split p 2 unit_frame_45)
    (beside p (double smaller smaller) unit_frame_45)
;;

let ex_2_46_vectors () =
  let add, sub, scaled = S46.ex_2_46 () in
  eq_bool "add" add (S46.make_vect 4.0 6.0);
  eq_bool "sub" sub (S46.make_vect 2.0 2.0);
  eq_bool "scale" scaled (S46.make_vect 2.0 4.0)
;;

let ex_2_47_frames () =
  let f1, f2 = S47.ex_2_47 () in
  let o1, o2 = S47.origin_frame1 f1, S47.origin_frame2 f2 in
  let e11, e12 = S47.edge1_frame1 f1, S47.edge1_frame2 f2 in
  let e21, e22 = S47.edge2_frame1 f1, S47.edge2_frame2 f2 in
  Alcotest.check
    Alcotest.bool
    "origins agree"
    true
    ((o1.x, o1.y) = (o2.x, o2.y) && o1.x = 0.0);
  Alcotest.check Alcotest.bool "edge1s agree" true ((e11.x, e11.y) = (e12.x, e12.y));
  Alcotest.check Alcotest.bool "edge2s agree" true ((e21.x, e21.y) = (e22.x, e22.y))
;;

let ex_2_48_segments () =
  let s = S48.ex_2_48 () in
  Alcotest.check
    Alcotest.bool
    "start"
    true
    (S48.start_segment s = { S48.x = 1.0; y = 2.0 });
  Alcotest.check Alcotest.bool "end" true (S48.end_segment s = { S48.x = 3.0; y = 4.0 })
;;

let ex_2_49_painters () =
  let outline = S49.ex_2_49_outline unit_frame_49 in
  let x = S49.ex_2_49_x unit_frame_49 in
  let diamond = S49.ex_2_49_diamond unit_frame_49 in
  let wave = S49.ex_2_49_wave unit_frame_49 in
  Alcotest.check Alcotest.int "outline has 4" 4 (List.length outline);
  Alcotest.check Alcotest.int "x has 2" 2 (List.length x);
  Alcotest.check Alcotest.int "diamond has 4" 4 (List.length diamond);
  Alcotest.check Alcotest.int "wave has 20" 20 (List.length wave);
  let starts = List.map (fun (a, _) -> a.S49.x, a.S49.y) outline in
  Alcotest.check
    (Alcotest.slist (Alcotest.pair (Alcotest.float 0.0) (Alcotest.float 0.0)) compare)
    "outline starts on the corners"
    [ 0.0, 0.0; 0.0, 1.0; 1.0, 0.0; 1.0, 1.0 ]
    starts
;;

let ex_2_50_transforms () =
  (* painters built in 2.50's own vocabulary: the unit-square frame map
     spelled locally, since the transforms must preserve exactly this *)
  let outline (frame : S50.frame) : S50.segment list =
    let o, e1, e2 = frame.S50.origin, frame.S50.edge1, frame.S50.edge2 in
    let at u v =
      { S50.x = o.x +. (u *. e1.x) +. (v *. e2.x); y = o.y +. (u *. e1.y) +. (v *. e2.y) }
    in
    [ at 0.0 0.0, at 1.0 0.0
    ; at 1.0 0.0, at 1.0 1.0
    ; at 1.0 1.0, at 0.0 1.0
    ; at 0.0 1.0, at 0.0 0.0
    ]
  in
  let x_painter (frame : S50.frame) : S50.segment list =
    let o, e1, e2 = frame.S50.origin, frame.S50.edge1, frame.S50.edge2 in
    let at u v =
      { S50.x = o.x +. (u *. e1.x) +. (v *. e2.x); y = o.y +. (u *. e1.y) +. (v *. e2.y) }
    in
    [ at 0.0 0.0, at 1.0 1.0; at 1.0 0.0, at 0.0 1.0 ]
  in
  (* the X is symmetric under a 180-degree rotation and a horizontal
     flip; comparisons sort the endpoints because a rotation may
     reverse each segment's direction *)
  let sorted segs =
    let norm (a, b) =
      let pa, pb = (a.S50.x, a.S50.y), (b.S50.x, b.S50.y) in
      if compare pa pb > 0 then pb, pa else pa, pb
    in
    List.sort compare (List.map norm segs)
  in
  Alcotest.check
    Alcotest.bool
    "x is 180-symmetric"
    true
    (sorted (S50.ex_2_50_rotate_180 x_painter unit_frame_50)
     = sorted (x_painter unit_frame_50));
  Alcotest.check
    Alcotest.bool
    "x is flip-symmetric"
    true
    (sorted (S50.ex_2_50_flip_horiz x_painter unit_frame_50)
     = sorted (x_painter unit_frame_50));
  Alcotest.check
    Alcotest.int
    "flip keeps 4 segments"
    4
    (List.length (S50.ex_2_50_flip_horiz outline unit_frame_50))
;;

let unit_frame_51 =
  { S51.origin = { S51.x = 0.0; y = 0.0 }
  ; edge1 = { S51.x = 1.0; y = 0.0 }
  ; edge2 = { S51.x = 0.0; y = 1.0 }
  }
;;

let ex_2_51_below () =
  (* the outline painter, built in 2.51's own vocabulary *)
  let outline (frame : S51.frame) : S51.segment list =
    let o, e1, e2 = frame.S51.origin, frame.S51.edge1, frame.S51.edge2 in
    let at u v =
      { S51.x = o.x +. (u *. e1.x) +. (v *. e2.x); y = o.y +. (u *. e1.y) +. (v *. e2.y) }
    in
    [ at 0.0 0.0, at 1.0 0.0
    ; at 1.0 0.0, at 1.0 1.0
    ; at 1.0 1.0, at 0.0 1.0
    ; at 0.0 1.0, at 0.0 0.0
    ]
  in
  let direct = S51.ex_2_51_below outline outline unit_frame_51 in
  let rotated = S51.ex_2_51_below_rotate outline outline unit_frame_51 in
  Alcotest.check Alcotest.bool "both constructions agree" true (direct = rotated);
  Alcotest.check Alcotest.int "eight segments" 8 (List.length direct)
;;

let unit_frame_52 =
  { S52.origin = { S52.x = 0.0; y = 0.0 }
  ; edge1 = { S52.x = 1.0; y = 0.0 }
  ; edge2 = { S52.x = 0.0; y = 1.0 }
  }
;;

let ex_2_52_variations () =
  let plain = S52.wave unit_frame_52 in
  let smiling = S52.ex_2_52_wave unit_frame_52 in
  Alcotest.check
    Alcotest.int
    "smile adds 3 segments"
    (List.length plain + 3)
    (List.length smiling);
  (* one copy of each split per corner: 11 copies of the painter at
     level 2 instead of 19 *)
  Alcotest.check
    Alcotest.int
    "variant corner split 2 has 220 segments"
    220
    (List.length (S52.ex_2_52_corner_split S52.wave 2 unit_frame_52))
;;

let read_file path =
  let ic = open_in_bin path in
  Fun.protect
    ~finally:(fun () -> close_in ic)
    (fun () -> really_input_string ic (in_channel_length ic))
;;

let ex_2_52a_view_box () =
  let golden = read_file "golden_ex_2_52a.svg" in
  Alcotest.check Alcotest.bool "matches the golden file" true (S52.ex_2_52a () = golden)
;;

let () =
  Alcotest.run
    "sicp ch2 section 2.2"
    [ ( "exercises"
      , Alcotest.
          [ test_case "2.17 last-pair" `Quick ex_2_17_last_pair
          ; test_case "2.18 reverse" `Quick ex_2_18_reverse
          ; test_case "2.19 change" `Quick ex_2_19_change
          ; test_case "2.20 same-parity" `Quick ex_2_20_same_parity
          ; test_case "2.21 square-list" `Quick ex_2_21_square_list
          ; test_case "2.22 louis bug" `Quick ex_2_22_louis_bug
          ; test_case "2.23 for-each" `Quick ex_2_23_for_each
          ; test_case "2.24 tree reading" `Quick ex_2_24_tree_reading
          ; test_case "2.25 pick seven" `Quick ex_2_25_pick_seven
          ; test_case "2.26 combinations" `Quick ex_2_26_combinations
          ; test_case "2.27 deep-reverse" `Quick ex_2_27_deep_reverse
          ; test_case "2.28 fringe" `Quick ex_2_28_fringe
          ; test_case "2.29 mobiles" `Quick ex_2_29_mobiles
          ; test_case "2.30 square-tree" `Quick ex_2_30_square_tree
          ; test_case "2.31 tree-map" `Quick ex_2_31_tree_map
          ; test_case "2.32 subsets" `Quick ex_2_32_subsets
          ; test_case "2.33 accumulations" `Quick ex_2_33_accumulations
          ; test_case "2.34 horner" `Quick ex_2_34_horner
          ; test_case "2.35 count-leaves" `Quick ex_2_35_count_leaves
          ; test_case "2.36 accumulate-n" `Quick ex_2_36_accumulate_n
          ; test_case "2.37 matrices" `Quick ex_2_37_matrices
          ; test_case "2.38 folds" `Quick ex_2_38_folds
          ; test_case "2.39 reverse folds" `Quick ex_2_39_reverse_folds
          ; test_case "2.40 unique-pairs" `Quick ex_2_40_unique_pairs
          ; test_case "2.41 triples" `Quick ex_2_41_triples
          ; test_case "2.42 queens" `Quick ex_2_42_queens
          ; test_case "2.43 louis" `Quick ex_2_43_louis
          ; test_case "2.44 up-split" `Quick ex_2_44_up_split
          ; test_case "2.45 split" `Quick ex_2_45_split
          ; test_case "2.46 vectors" `Quick ex_2_46_vectors
          ; test_case "2.47 frames" `Quick ex_2_47_frames
          ; test_case "2.48 segments" `Quick ex_2_48_segments
          ; test_case "2.49 painters" `Quick ex_2_49_painters
          ; test_case "2.50 transforms" `Quick ex_2_50_transforms
          ; test_case "2.51 below" `Quick ex_2_51_below
          ; test_case "2.52 variations" `Quick ex_2_52_variations
          ; test_case "2.52a view box" `Quick ex_2_52a_view_box
          ] )
    ]
;;
