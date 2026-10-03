(* SPDX-License-Identifier: GPL-3.0-only *)

module Value = Sicp_common.Value

let bool_result name expected = function
  | Ok b -> Alcotest.(check bool) name expected b
  | Error _ -> Alcotest.fail (name ^ ": unexpected type error")
;;

let rejects name = function
  | Ok _ -> Alcotest.fail (name ^ ": the pair is outside the admitted comparison types")
  | Error _ -> ()
;;

(* Scalar equality follows the host: floats are IEEE, so a NaN equals
   nothing, itself included; structured values are not comparable. *)
let scalar_equality () =
  bool_result "ints" true (Value.equal_scalars (Value.int 3) (Value.int 3));
  bool_result "strings" false (Value.equal_scalars (Value.string "a") (Value.string "b"));
  bool_result
    "nan"
    false
    (Value.equal_scalars (Value.float Float.nan) (Value.float Float.nan));
  bool_result
    "signed zeros"
    true
    (Value.equal_scalars (Value.float 0.0) (Value.float (-0.0)));
  rejects "mixed" (Value.equal_scalars (Value.int 1) (Value.float 1.0));
  rejects
    "tuples"
    (Value.equal_scalars (Value.tuple [ Value.int 1 ]) (Value.tuple [ Value.int 1 ]));
  rejects "ordered bools" (Value.compare_scalars (Value.bool true) (Value.bool false))
;;

(* Hash-table keys compare structurally over immutable data (grammar
   section 6): a rebuilt constructor tree finds the same entry. *)
let tables_key_structurally () =
  let key n = Value.construct "Point" [ Value.int n; Value.string "p" ] in
  let t = Value.table () in
  Value.table_replace t (key 1) (Value.int 10);
  Value.table_replace t (key 2) (Value.int 20);
  Value.table_replace t (key 1) (Value.int 11);
  Alcotest.(check int) "replace keeps one binding per key" 2 (Value.table_length t);
  (match Option.map Value.view (Value.table_find t (key 1)) with
   | Some (Value.Int 11) -> ()
   | _ -> Alcotest.fail "a rebuilt key finds the replaced value");
  Value.table_remove t (key 2);
  Alcotest.(check bool) "removed" true (Value.table_find t (key 2) = None);
  Alcotest.(check int) "one left" 1 (Value.table_length t)
;;

let () =
  Alcotest.run
    "value"
    [ ( "value"
      , [ Alcotest.test_case "scalar equality follows the host" `Quick scalar_equality
        ; Alcotest.test_case "tables key structurally" `Quick tables_key_structurally
        ] )
    ]
;;
