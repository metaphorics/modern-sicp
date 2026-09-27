(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The runner touches every listing of section 2.3 in the order the book
   presents it, asserting each value the book shows. *)

module Replay = Sicp_ch1.Replay
module S = Sicp_ch2.Sec_2_3.Symbols
module D0 = Sicp_ch2.Sec_2_3.Deriv_naive
module D = Sicp_ch2.Sec_2_3.Deriv
module Unordered = Sicp_ch2.Sec_2_3.Unordered_list_set
module Ordered = Sicp_ch2.Sec_2_3.Ordered_list_set
module Tree_set = Sicp_ch2.Sec_2_3.Tree_set
module Db = Sicp_ch2.Sec_2_3.Record_db
module H = Sicp_ch2.Sec_2_3.Huffman

let expect (actual : string) (shown : string) = Replay.expect actual shown
let expect_int computed shown = expect (string_of_int computed) shown
let expect_bool computed shown = expect (string_of_bool computed) shown
let show_int_list l = "[" ^ String.concat "; " (List.map string_of_int l) ^ "]"

let show_pair_list l =
  "["
  ^ String.concat
      "; "
      (List.map (fun (s, n) -> "(\"" ^ s ^ "\", " ^ string_of_int n ^ ")") l)
  ^ "]"
;;

let show_symbols l =
  "[" ^ String.concat "; " (List.map (fun (S.Sym s) -> "Sym \"" ^ s ^ "\"") l) ^ "]"
;;

let show_option_symbols = function
  | None -> "None"
  | Some l -> "Some " ^ show_symbols l
;;

let rec show_expr exp =
  match exp with
  | D.Const n -> "Const " ^ string_of_int n
  | D.Var v -> "Var \"" ^ v ^ "\""
  | D.Sum (a, b) -> "Sum (" ^ show_expr a ^ ", " ^ show_expr b ^ ")"
  | D.Prod (a, b) -> "Prod (" ^ show_expr a ^ ", " ^ show_expr b ^ ")"
;;

let rec show_tree = function
  | Tree_set.Empty -> "Empty"
  | Tree_set.Node (l, e, r) ->
    "Node (" ^ show_tree l ^ ", " ^ string_of_int e ^ ", " ^ show_tree r ^ ")"
;;

let show_leaf leaf =
  match leaf with
  | H.Leaf (s, w) -> "Leaf (\"" ^ s ^ "\", " ^ string_of_int w ^ ")"
  | H.Node _ -> "Node (…)"
;;

let show_leaf_set leaves = "[" ^ String.concat "; " (List.map show_leaf leaves) ^ "]"

let show_record_opt = function
  | None -> "None"
  | Some { Db.key; name } ->
    "Some {key = " ^ string_of_int key ^ "; name = \"" ^ name ^ "\"}"
;;

let () =
  (* 2.3.1 Quotation: compound data made of symbols *)
  expect
    (show_symbols [ S.Sym "a"; S.Sym "b"; S.Sym "c"; S.Sym "d" ])
    {|[Sym "a"; Sym "b"; Sym "c"; Sym "d"]|};
  expect (show_int_list [ 23; 45; 17 ]) "[23; 45; 17]";
  expect
    (show_pair_list [ "Norah", 12; "Molly", 9; "Anna", 7; "Lauren", 6; "Charlotte", 4 ])
    {|[("Norah", 12); ("Molly", 9); ("Anna", 7); ("Lauren", 6); ("Charlotte", 4)]|};
  (* The program counterpart of the book's lookalike list. *)
  let rec fact n = if n = 1 then 1 else n * fact (n - 1) in
  expect_int (fact 5) "120";
  (* Symbols and their values live on different sides of the compiler. *)
  let a = 1 in
  let b = 2 in
  expect (show_int_list [ a; b ]) "[1; 2]";
  expect (show_symbols [ S.Sym "a"; S.Sym "b" ]) {|[Sym "a"; Sym "b"]|};
  expect
    ("("
     ^ (match S.Sym "a" with
        | S.Sym s -> "Sym \"" ^ s ^ "\"")
     ^ ", "
     ^ string_of_int b
     ^ ")")
    {|(Sym "a", 2)|};
  expect
    (show_symbols (List.tl [ S.Sym "a"; S.Sym "b"; S.Sym "c" ]))
    {|[Sym "b"; Sym "c"]|};
  let first = List.hd [ S.Sym "a"; S.Sym "b"; S.Sym "c" ] in
  expect_bool (first = S.Sym "a") "true";
  expect (show_symbols []) "[]";
  (* memq *)
  expect
    (show_option_symbols
       (S.memq (S.Sym "apple") [ S.Sym "pear"; S.Sym "banana"; S.Sym "prune" ]))
    "None";
  expect
    (show_option_symbols
       (S.memq (S.Sym "apple") [ S.Sym "x"; S.Sym "y"; S.Sym "apple"; S.Sym "pear" ]))
    {|Some [Sym "apple"; Sym "pear"]|};
  (* 2.3.2 Symbolic differentiation, first stage: no simplification *)
  let open D0 in
  expect
    (show_expr @@ Sicp_ch2.Sec_2_3.Deriv_naive.deriv (Sum (Var "x", Const 3)) "x")
    "Sum (Const 1, Const 0)";
  expect
    (show_expr @@ Sicp_ch2.Sec_2_3.Deriv_naive.deriv (Prod (Var "x", Var "y")) "x")
    "Sum (Prod (Var \"x\", Const 0), Prod (Const 1, Var \"y\"))";
  expect
    (show_expr
     @@ Sicp_ch2.Sec_2_3.Deriv_naive.deriv
          (Prod (Prod (Var "x", Var "y"), Sum (Var "x", Const 3)))
          "x")
    "Sum (Prod (Prod (Var \"x\", Var \"y\"), Sum (Const 1, Const 0)), Prod (Sum (Prod \
     (Var \"x\", Const 0), Prod (Const 1, Var \"y\")), Sum (Var \"x\", Const 3)))";
  (* Second stage: the constructors simplify. *)
  expect (show_expr (D.deriv (D.Sum (D.Var "x", D.Const 3)) "x")) "Const 1";
  expect (show_expr (D.deriv (D.Prod (D.Var "x", D.Var "y")) "x")) "Var \"y\"";
  expect
    (show_expr
       (D.deriv
          (D.Prod (D.Prod (D.Var "x", D.Var "y"), D.Sum (D.Var "x", D.Const 3)))
          "x"))
    {|Sum (Prod (Var "x", Var "y"), Prod (Var "y", Sum (Var "x", Const 3)))|};
  (* 2.3.3 Sets as unordered lists *)
  expect_bool (Unordered.element_of_set 2 [ 3; 2; 1 ]) "true";
  expect_bool (Unordered.element_of_set 4 [ 3; 2; 1 ]) "false";
  expect (show_int_list (Unordered.adjoin_set 4 [ 3; 2; 1 ])) "[4; 3; 2; 1]";
  expect (show_int_list (Unordered.adjoin_set 2 [ 3; 2; 1 ])) "[3; 2; 1]";
  expect
    (show_int_list (Unordered.intersection_set [ 1; 2; 3; 4 ] [ 3; 4; 5; 6 ]))
    "[3; 4]";
  (* Sets as ordered lists *)
  expect_bool (Ordered.element_of_set 3 [ 1; 3; 6; 10 ]) "true";
  expect_bool (Ordered.element_of_set 4 [ 1; 3; 6; 10 ]) "false";
  expect_bool (Ordered.element_of_set 0 [ 1; 3; 6; 10 ]) "false";
  expect (show_int_list (Ordered.intersection_set [ 1; 3; 5; 7 ] [ 2; 3; 6; 7 ])) "[3; 7]";
  (* Sets as binary trees: adjoining 1 through 7 in sequence builds the
     unbalanced tree of Figure 2.17. *)
  let seven =
    List.fold_left
      (fun acc n -> Tree_set.adjoin_set n acc)
      Tree_set.Empty
      [ 1; 2; 3; 4; 5; 6; 7 ]
  in
  expect
    (show_tree seven)
    {|Node (Empty, 1, Node (Empty, 2, Node (Empty, 3, Node (Empty, 4, Node (Empty, 5, Node (Empty, 6, Node (Empty, 7, Empty)))))))|};
  expect_bool (Tree_set.element_of_set 5 seven) "true";
  expect_bool (Tree_set.element_of_set 8 seven) "false";
  let e3 = Tree_set.make_tree 3 Tree_set.Empty Tree_set.Empty in
  expect_int (Tree_set.entry e3) "3";
  expect
    (show_tree (Tree_set.left_branch (Tree_set.make_tree 7 e3 Tree_set.Empty)))
    "Node (Empty, 3, Empty)";
  (* Information retrieval *)
  let records = [ { Db.key = 1; name = "ada" }; { Db.key = 2; name = "grace" } ] in
  expect (show_record_opt (Db.lookup 2 records)) {|Some {key = 2; name = "grace"}|};
  expect (show_record_opt (Db.lookup 9 records)) "None";
  (* 2.3.4 Huffman encoding trees *)
  expect
    (show_leaf_set (H.make_leaf_set [ "A", 4; "B", 2; "C", 1; "D", 1 ]))
    {|[Leaf ("D", 1); Leaf ("C", 1); Leaf ("B", 2); Leaf ("A", 4)]|};
  (* The book's decoding example: the sequence 10001010 names BAC. *)
  expect
    (String.concat
       ""
       (H.decode [ One; Zero; Zero; Zero; One; Zero; One; Zero ] H.sample_tree))
    "BAC";
  (* The code for D is 1011. *)
  expect (String.concat "" (H.decode [ One; Zero; One; One ] H.sample_tree)) "D"
;;
