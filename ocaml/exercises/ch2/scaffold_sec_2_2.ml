(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The runner touches every exercise of section 2.2 once, including the
   tailored addition 2.52a. Every call raises Pending_solution until the
   exercise is solved; building this executable proves the scaffolds
   link against the stated signatures. *)

open Sicp_ch2_exercises

(* [void_painter] unifies with every exercise module's painter type. *)
let void_painter _frame = []

(* A unit frame in each exercise module's own picture vocabulary, so the
   painter-returning calls can be applied to completion. *)
let frame_44 =
  let v x y = { Sec_2_44.x; Sec_2_44.y } in
  { Sec_2_44.origin = v 0.0 0.0; Sec_2_44.edge1 = v 1.0 0.0; Sec_2_44.edge2 = v 0.0 1.0 }
;;

let frame_45 =
  let v x y = { Sec_2_45.x; Sec_2_45.y } in
  { Sec_2_45.origin = v 0.0 0.0; Sec_2_45.edge1 = v 1.0 0.0; Sec_2_45.edge2 = v 0.0 1.0 }
;;

let frame_49 =
  let v x y = { Sec_2_49.x; Sec_2_49.y } in
  { Sec_2_49.origin = v 0.0 0.0; Sec_2_49.edge1 = v 1.0 0.0; Sec_2_49.edge2 = v 0.0 1.0 }
;;

let frame_50 =
  let v x y = { Sec_2_50.x; Sec_2_50.y } in
  { Sec_2_50.origin = v 0.0 0.0; Sec_2_50.edge1 = v 1.0 0.0; Sec_2_50.edge2 = v 0.0 1.0 }
;;

let frame_51 =
  let v x y = { Sec_2_51.x; Sec_2_51.y } in
  { Sec_2_51.origin = v 0.0 0.0; Sec_2_51.edge1 = v 1.0 0.0; Sec_2_51.edge2 = v 0.0 1.0 }
;;

let frame_52 =
  let v x y = { Sec_2_52.x; Sec_2_52.y } in
  { Sec_2_52.origin = v 0.0 0.0; Sec_2_52.edge1 = v 1.0 0.0; Sec_2_52.edge2 = v 0.0 1.0 }
;;

(* A unit frame in exercise 2.49's and 2.52's own picture vocabulary. *)

let () =
  ignore (Sec_2_17.ex_2_17 [ 1 ]);
  ignore (Sec_2_18.ex_2_18 [ 1 ]);
  ignore (Sec_2_19.ex_2_19 ());
  ignore (Sec_2_20.ex_2_20 1 []);
  ignore (Sec_2_21.ex_2_21_direct []);
  ignore (Sec_2_21.ex_2_21_map []);
  ignore (Sec_2_22.ex_2_22_louis []);
  ignore (Sec_2_23.ex_2_23 ignore []);
  ignore (Sec_2_24.ex_2_24 ());
  ignore (Sec_2_25.ex_2_25 ());
  ignore (Sec_2_26.ex_2_26 ());
  ignore (Sec_2_27.ex_2_27 (Sec_2_27.Node [ Sec_2_27.Leaf 1 ]));
  ignore (Sec_2_28.ex_2_28 (Sec_2_28.Node [ Sec_2_28.Leaf 1 ]));
  let mobile =
    Sec_2_29.Mobile
      (Sec_2_29.Branch (2, Sec_2_29.Weight 3), Sec_2_29.Branch (2, Sec_2_29.Weight 3))
  in
  ignore (Sec_2_29.ex_2_29 mobile);
  ignore (Sec_2_29.total_weight mobile);
  ignore (Sec_2_29.balanced mobile);
  ignore
    (Sec_2_29.Cons_repr.total_weight
       ((2, Sec_2_29.Cons_repr.Weight 3), (2, Sec_2_29.Cons_repr.Weight 3)));
  ignore (Sec_2_30.ex_2_30_direct (Sec_2_30.Node [ Sec_2_30.Leaf 1 ]));
  ignore (Sec_2_30.ex_2_30_map (Sec_2_30.Node [ Sec_2_30.Leaf 1 ]));
  ignore (Sec_2_31.ex_2_31_tree_map (fun n -> n) (Sec_2_31.Node [ Sec_2_31.Leaf 1 ]));
  ignore (Sec_2_31.ex_2_31_square_tree (Sec_2_31.Node [ Sec_2_31.Leaf 1 ]));
  ignore (Sec_2_32.ex_2_32 []);
  ignore (Sec_2_33.ex_2_33_map (fun x -> x) []);
  ignore (Sec_2_33.ex_2_33_append [] []);
  ignore (Sec_2_33.ex_2_33_length []);
  ignore (Sec_2_34.ex_2_34 1 []);
  ignore (Sec_2_35.ex_2_35 (Sec_2_35.Node [ Sec_2_35.Leaf 1 ]));
  ignore (Sec_2_36.ex_2_36 ( + ) 0 []);
  ignore (Sec_2_37.ex_2_37 ());
  ignore (Sec_2_37.dot_product [] []);
  ignore (Sec_2_37.matrix_times_vector [] []);
  ignore (Sec_2_37.transpose []);
  ignore (Sec_2_37.matrix_times_matrix [] []);
  ignore (Sec_2_38.ex_2_38 ());
  ignore (Sec_2_39.ex_2_39_right []);
  ignore (Sec_2_39.ex_2_39_left []);
  ignore (Sec_2_40.ex_2_40_unique_pairs 1);
  ignore (Sec_2_40.ex_2_40_prime_sum_pairs 1);
  ignore (Sec_2_41.ex_2_41 1 1);
  ignore (Sec_2_42.ex_2_42 1);
  ignore (Sec_2_43.ex_2_43 ());
  ignore (Sec_2_44.ex_2_44_up_split void_painter 0 frame_44);
  ignore (Sec_2_45.ex_2_45_split (fun p _q -> p) (fun p _q -> p) void_painter 0 frame_45);
  ignore (Sec_2_46.ex_2_46 ());
  ignore (Sec_2_47.ex_2_47 ());
  ignore (Sec_2_48.ex_2_48 ());
  ignore (Sec_2_49.ex_2_49_outline frame_49);
  ignore (Sec_2_49.ex_2_49_x frame_49);
  ignore (Sec_2_49.ex_2_49_diamond frame_49);
  ignore (Sec_2_49.ex_2_49_wave frame_49);
  ignore (Sec_2_50.ex_2_50_flip_horiz void_painter frame_50);
  ignore (Sec_2_50.ex_2_50_rotate_180 void_painter frame_50);
  ignore (Sec_2_50.ex_2_50_rotate_270 void_painter frame_50);
  ignore (Sec_2_51.ex_2_51_below void_painter void_painter frame_51);
  ignore (Sec_2_51.ex_2_51_below_rotate void_painter void_painter frame_51);
  ignore (Sec_2_52.ex_2_52_wave frame_52);
  ignore (Sec_2_52.ex_2_52_corner_split void_painter 1 frame_52);
  ignore (Sec_2_52.ex_2_52_square_limit void_painter 1 frame_52);
  ignore (Sec_2_52.ex_2_52a ())
;;
