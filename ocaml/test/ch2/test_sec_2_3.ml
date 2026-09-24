(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the reference solutions' public contracts for
   section 2.3. [sicp_ch2_solutions] implements the same contracts the
   exercises library states with pending stubs, so these tests double
   as the proof of each exercise's stated answer. *)

module S53 = Sicp_ch2_solutions.Sec_2_53
module S54 = Sicp_ch2_solutions.Sec_2_54
module S55 = Sicp_ch2_solutions.Sec_2_55
module S56 = Sicp_ch2_solutions.Sec_2_56
module S57 = Sicp_ch2_solutions.Sec_2_57
module S58 = Sicp_ch2_solutions.Sec_2_58
module S59 = Sicp_ch2_solutions.Sec_2_59
module S60 = Sicp_ch2_solutions.Sec_2_60
module S61 = Sicp_ch2_solutions.Sec_2_61
module S62 = Sicp_ch2_solutions.Sec_2_62
module S63 = Sicp_ch2_solutions.Sec_2_63
module S64 = Sicp_ch2_solutions.Sec_2_64
module S65 = Sicp_ch2_solutions.Sec_2_65
module S66 = Sicp_ch2_solutions.Sec_2_66
module S67 = Sicp_ch2_solutions.Sec_2_67
module S68 = Sicp_ch2_solutions.Sec_2_68
module S69 = Sicp_ch2_solutions.Sec_2_69
module S70 = Sicp_ch2_solutions.Sec_2_70
module S71 = Sicp_ch2_solutions.Sec_2_71
module S72 = Sicp_ch2_solutions.Sec_2_72

let int_list =
  Alcotest.testable
    (fun fmt l ->
       Format.fprintf fmt "[%s]" (String.concat "; " (List.map string_of_int l)))
    ( = )
;;

let string_list =
  Alcotest.testable (fun fmt l -> Format.fprintf fmt "[%s]" (String.concat "; " l)) ( = )
;;

let eq_bool name a b = Alcotest.check Alcotest.bool name true (a = b)

let ex_2_53_predictions () =
  Alcotest.(check (list string))
    "the seven printed predictions"
    [ {|[Sym "a"; Sym "b"; Sym "c"]|}
    ; {|[[Sym "george"]]|}
    ; {|[[Sym "y1"; Sym "y2"]]|}
    ; {|[Sym "y1"; Sym "y2"]|}
    ; "true"
    ; "None"
    ; {|Some [Sym "red"; Sym "shoes"; Sym "blue"; Sym "socks"]|}
    ]
    (S53.ex_2_53 ())
;;

let ex_2_54_equal_datum () =
  eq_bool "(this is a list) equals itself" (S54.ex_2_54 ()) true;
  eq_bool
    "(this is a list) does not equal (this (is a) list)"
    (S54.equal_datum
       (S54.words [ "this"; "is"; "a"; "list" ])
       (S54.Seq [ S54.Sym "this"; S54.Seq [ S54.Sym "is"; S54.Sym "a" ]; S54.Sym "list" ]))
    false;
  eq_bool
    "different lengths are not equal"
    (S54.equal_datum (S54.words [ "a"; "b" ]) (S54.words [ "a"; "b"; "c" ]))
    false
;;

let ex_2_55_quote () =
  Alcotest.(check string) "car of the double quote is quote" "quote" (S55.ex_2_55 ())
;;

let show56 exp =
  let rec go = function
    | S56.Const n -> Printf.sprintf "Const %d" n
    | S56.Var v -> Printf.sprintf "Var %S" v
    | S56.Sum (a, b) -> Printf.sprintf "Sum (%s, %s)" (go a) (go b)
    | S56.Prod (a, b) -> Printf.sprintf "Prod (%s, %s)" (go a) (go b)
    | S56.Power (a, b) -> Printf.sprintf "Power (%s, %s)" (go a) (go b)
  in
  go exp
;;

let ex_2_56_exponentiation () =
  Alcotest.(check string)
    "the derivative of x^3 is 3 * x^2"
    "Prod (Const 3, Power (Var \"x\", Const 2))"
    (show56 (S56.ex_2_56 ()))
;;

let show57 exp =
  let rec go = function
    | S57.Const n -> Printf.sprintf "Const %d" n
    | S57.Var v -> Printf.sprintf "Var %S" v
    | S57.Sum l -> Printf.sprintf "Sum [%s]" (String.concat "; " (List.map go l))
    | S57.Prod l -> Printf.sprintf "Prod [%s]" (String.concat "; " (List.map go l))
  in
  go exp
;;

let ex_2_57_n_ary () =
  Alcotest.(check string)
    "the derivative of x * y * (x + 3) with respect to x"
    {|Sum [Prod [Var "y"; Sum [Var "x"; Const 3]]; Prod [Var "x"; Var "y"]]|}
    (show57 (S57.ex_2_57 ()))
;;

let show58 exp =
  let rec go = function
    | S58.Num n -> Printf.sprintf "Num %d" n
    | S58.Var v -> Printf.sprintf "Var %S" v
    | S58.Plus (a, b) -> Printf.sprintf "Plus (%s, %s)" (go a) (go b)
    | S58.Times (a, b) -> Printf.sprintf "Times (%s, %s)" (go a) (go b)
  in
  go exp
;;

let ex_2_58_infix () =
  Alcotest.(check string)
    "part (a): fully parenthesized"
    "Num 4"
    (show58 (S58.ex_2_58_parenthesized ()));
  Alcotest.(check string)
    "part (b): standard precedence"
    "Num 4"
    (show58 (S58.ex_2_58_standard ()))
;;

let ex_2_59_union () =
  Alcotest.(check int_list)
    "union of {1,2,3} and {3,4,5}"
    [ 1; 2; 3; 4; 5 ]
    (S59.ex_2_59 ())
;;

let ex_2_60_duplicates () =
  let adjoined, unioned = S60.ex_2_60 () in
  Alcotest.(check int_list)
    "adjoining a present element still grows the list"
    [ 1; 2; 3; 2; 1; 3; 2; 2 ]
    adjoined;
  Alcotest.(check int_list)
    "union is append, doubling every count"
    [ 2; 3; 2; 1; 3; 2; 2; 2; 3; 2; 1; 3; 2; 2 ]
    unioned;
  Alcotest.(check int_list)
    "intersection keeps set1's duplicates present in set2"
    [ 2; 3; 2; 3; 2; 2 ]
    (S60.intersection_set S60.sample_set [ 3; 2 ])
;;

let ex_2_61_ordered_adjoin () =
  Alcotest.(check int_list)
    "4 inserted into {1,3,6,10}"
    [ 1; 3; 4; 6; 10 ]
    (S61.ex_2_61 ())
;;

let ex_2_62_ordered_union () =
  Alcotest.(check int_list)
    "union of {1,3,5,7} and {2,3,6,7}"
    [ 1; 2; 3; 5; 6; 7 ]
    (S62.ex_2_62 ())
;;

let ex_2_63_tree_to_list () =
  let sorted = [ 1; 3; 5; 7; 9; 11 ] in
  List.iter
    (fun (l1, l2) ->
       Alcotest.(check int_list) "tree_to_list_1 is sorted" sorted l1;
       Alcotest.(check int_list) "tree_to_list_2 is sorted" sorted l2)
    (S63.ex_2_63 ())
;;

let ex_2_64_list_to_tree () =
  let open S64 in
  let expected =
    Node
      ( Node (Empty, 1, Node (Empty, 3, Empty))
      , 5
      , Node (Node (Empty, 7, Empty), 9, Node (Empty, 11, Empty)) )
  in
  eq_bool "list_to_tree balances [1;3;5;7;9;11]" (S64.ex_2_64 ()) expected
;;

let ex_2_65_tree_sets () =
  let open S65 in
  let union, inter = S65.ex_2_65 () in
  eq_bool
    "union of tree{1,3,5,7} and tree{3,4,5}"
    union
    (Node
       ( Node (Empty, 1, Node (Empty, 3, Empty))
       , 4
       , Node (Empty, 5, Node (Empty, 7, Empty)) ));
  eq_bool
    "intersection of the same two trees"
    inter
    (Node (Empty, 3, Node (Empty, 5, Empty)))
;;

let ex_2_66_tree_lookup () =
  (match S66.ex_2_66 () with
   | Some { S66.key = 3; name = "margaret" } -> ()
   | _ -> Alcotest.fail "expected the record with key 3");
  eq_bool "a missing key finds nothing" (S66.lookup 99 S66.sample_tree) None
;;

let ex_2_67_decode () =
  Alcotest.(check string_list)
    "the sample message decodes to ADABBCA"
    [ "A"; "D"; "A"; "B"; "B"; "C"; "A" ]
    (S67.ex_2_67 ())
;;

let ex_2_68_encode () =
  eq_bool
    "encoding the decoded message gives back the original"
    (S68.ex_2_68 () = S68.sample_message)
    true;
  Alcotest.check_raises
    "encode_symbol errors on an unknown symbol"
    (Invalid_argument "encode_symbol: symbol not in tree")
    (fun () -> ignore (S68.encode_symbol "Z" S68.sample_tree))
;;

let ex_2_69_generate () =
  let open S69 in
  let expected =
    make_code_tree
      (make_leaf "A" 4)
      (make_code_tree
         (make_leaf "B" 2)
         (make_code_tree (make_leaf "D" 1) (make_leaf "C" 1)))
  in
  eq_bool "generate_huffman_tree reproduces the sample tree" (S69.ex_2_69 ()) expected;
  eq_bool
    "a single pair generates its own leaf"
    (S69.generate_huffman_tree [ "A", 1 ])
    (S69.make_leaf "A" 1)
;;

let ex_2_70_rock_song () =
  let huffman_bits, fixed_bits = S70.ex_2_70 () in
  Alcotest.(check int) "the Huffman encoding of the song" 84 huffman_bits;
  Alcotest.(check int) "the fixed-length encoding of the song" 108 fixed_bits;
  eq_bool "2.70a: the roundtrip battery holds" (S70.ex_2_70a ()) true
;;

let ex_2_71_skewed_bits () =
  Alcotest.(check (pair int int)) "n=5: 1 bit, 4 bits" (1, 4) (S71.ex_2_71 5);
  Alcotest.(check (pair int int)) "n=10: 1 bit, 9 bits" (1, 9) (S71.ex_2_71 10);
  Alcotest.(check (pair int int)) "n=3: 1 bit, 2 bits" (1, 2) (S71.ex_2_71 3)
;;

let ex_2_72_growth () =
  Alcotest.(check (pair int int)) "n=5 steps" (7, 9) (S72.ex_2_72 5);
  Alcotest.(check (pair int int)) "n=10 steps" (12, 19) (S72.ex_2_72 10);
  Alcotest.(check (pair int int)) "n=3 steps" (5, 5) (S72.ex_2_72 3)
;;

let () =
  Alcotest.run
    "sec_2_3"
    [ ( "exercises"
      , Alcotest.
          [ test_case "2.53 predictions" `Quick ex_2_53_predictions
          ; test_case "2.54 equal-datum" `Quick ex_2_54_equal_datum
          ; test_case "2.55 quote" `Quick ex_2_55_quote
          ; test_case "2.56 exponentiation" `Quick ex_2_56_exponentiation
          ; test_case "2.57 n-ary" `Quick ex_2_57_n_ary
          ; test_case "2.58 infix" `Quick ex_2_58_infix
          ; test_case "2.59 union" `Quick ex_2_59_union
          ; test_case "2.60 duplicates" `Quick ex_2_60_duplicates
          ; test_case "2.61 ordered adjoin" `Quick ex_2_61_ordered_adjoin
          ; test_case "2.62 ordered union" `Quick ex_2_62_ordered_union
          ; test_case "2.63 tree to list" `Quick ex_2_63_tree_to_list
          ; test_case "2.64 list to tree" `Quick ex_2_64_list_to_tree
          ; test_case "2.65 tree sets" `Quick ex_2_65_tree_sets
          ; test_case "2.66 tree lookup" `Quick ex_2_66_tree_lookup
          ; test_case "2.67 decode" `Quick ex_2_67_decode
          ; test_case "2.68 encode" `Quick ex_2_68_encode
          ; test_case "2.69 generate" `Quick ex_2_69_generate
          ; test_case "2.70 rock song" `Quick ex_2_70_rock_song
          ; test_case "2.71 skewed bits" `Quick ex_2_71_skewed_bits
          ; test_case "2.72 growth" `Quick ex_2_72_growth
          ] )
    ]
;;
