(* SPDX-License-Identifier: GPL-3.0-only *)
module Ast = Sicp_common.Ast
module Env = Sicp_common.Env
module Value = Sicp_common.Value

let check_value detail value expected =
  Alcotest.check Alcotest.string detail expected (Value.to_string value)
;;

let check_display detail value expected =
  Alcotest.check Alcotest.string detail expected (Value.display value)
;;

let atoms () =
  check_value "int 0" (Value.int 0) "0";
  check_value "negative int" (Value.int (-5)) "-5";
  check_value "true" (Value.bool true) "#t";
  check_value "false" (Value.bool false) "#f";
  check_value "symbol" (Value.symbol "car") "car";
  check_value "empty list" Value.nil "()"
;;

(* Every float case is an example the shared printer contract names, or the
   boundary the contract draws between positional and exponential form. *)
let floats () =
  check_value "integral float" (Value.float 3.0) "3.0";
  check_value "441.0" (Value.float 441.0) "441.0";
  check_value "2.5" (Value.float 2.5) "2.5";
  check_value "0.001" (Value.float 0.001) "0.001";
  check_value "negative fixed" (Value.float (-2.5)) "-2.5";
  check_value "zero" (Value.float 0.0) "0.0";
  check_value "negative zero" (Value.float (-0.0)) "-0.0";
  check_value "1e20 stays positional" (Value.float 1e20) "100000000000000000000.0";
  check_value "1e21 goes exponential" (Value.float 1e21) "1.0e21";
  check_value "1.0e22" (Value.float 1e22) "1.0e22";
  check_value "1.5e22" (Value.float 1.5e22) "1.5e22";
  check_value "2.5e-7" (Value.float 2.5e-7) "2.5e-7";
  check_value "1e-6 boundary is positional" (Value.float 1e-6) "0.000001";
  check_value "1e-7 goes exponential" (Value.float 1e-7) "1.0e-7"
;;

let strings () =
  check_value "plain string" (Value.string "abc") "\"abc\"";
  check_value "escaped quote" (Value.string "a\"b") "\"a\\\"b\"";
  check_value "escaped backslash" (Value.string "a\\b") "\"a\\\\b\"";
  check_display "display prints raw" (Value.string "a\"b\\c") "a\"b\\c";
  check_display "display of int is unchanged" (Value.int 42) "42"
;;

let pairs () =
  let open Value in
  check_value "proper list" (pair (int 1) (pair (int 2) (pair (int 3) nil))) "(1 2 3)";
  check_value "dotted pair" (pair (int 1) (int 2)) "(1 . 2)";
  check_value
    "chain ending in dotted pair"
    (pair (symbol "a") (pair (symbol "b") (symbol "c")))
    "(a b . c)";
  check_value
    "dotted pair inside list"
    (pair (pair (symbol "a") (pair (int 1) (int 2))) (pair (symbol "d") nil))
    "((a 1 . 2) d)";
  check_value
    "nested pairs"
    (pair (pair (int 1) (int 2)) (pair (pair (int 3) (int 4)) nil))
    "((1 . 2) (3 . 4))"
;;

let procedures () =
  let open Value in
  let car_primitive = primitive ~name:"car" (fun _ -> Ok nil) in
  check_value "primitive prints its name" car_primitive "#[primitive-procedure car]";
  let global = Env.empty () in
  let named =
    compound
      ~name:(Some "square")
      ~parameters:[ "x" ]
      ~body:[ Ast.variable "x" ]
      ~env:global
  in
  check_value "named compound" named "#[compound-procedure square]";
  let anonymous =
    compound ~name:None ~parameters:[ "x" ] ~body:[ Ast.variable "x" ] ~env:global
  in
  check_value "anonymous compound" anonymous "#[compound-procedure]"
;;

let equality () =
  let open Value in
  Alcotest.check Alcotest.bool "eq ints" true (physical_equal (int 3) (int 3));
  Alcotest.check
    Alcotest.bool
    "eq symbols"
    true
    (physical_equal (symbol "a") (symbol "a"));
  Alcotest.check Alcotest.bool "eq nil" true (physical_equal nil nil);
  let shared = pair (int 1) (int 2) in
  Alcotest.check Alcotest.bool "same cons cell is eq" true (physical_equal shared shared);
  Alcotest.check
    Alcotest.bool
    "distinct cons cells are not eq"
    false
    (physical_equal shared (pair (int 1) (int 2)));
  Alcotest.check
    Alcotest.bool
    "equal? over structure"
    true
    (structural_equal shared (pair (int 1) (int 2)));
  Alcotest.check
    Alcotest.bool
    "equal? over nested structure"
    true
    (structural_equal
       (pair (int 1) (pair (symbol "a") nil))
       (pair (int 1) (pair (symbol "a") nil)));
  let p1 = primitive ~name:"car" (fun _ -> Ok nil) in
  let p2 = primitive ~name:"car" (fun _ -> Ok (int 9)) in
  Alcotest.check
    Alcotest.bool
    "primitives compare by name under equal?"
    true
    (structural_equal p1 p2);
  Alcotest.check Alcotest.bool "primitives are not eq" false (physical_equal p1 p2);
  let global = Env.empty () in
  let c1 = compound ~name:None ~parameters:[] ~body:[] ~env:global in
  let c2 = compound ~name:None ~parameters:[] ~body:[] ~env:global in
  Alcotest.check
    Alcotest.bool
    "compounds in distinct envs are not equal?"
    false
    (structural_equal c1 c2);
  Alcotest.check Alcotest.bool "one compound equals itself" true (structural_equal c1 c1)
;;

let () =
  Alcotest.run
    "sicp_common.value"
    [ ( "value"
      , Alcotest.
          [ test_case "atoms" `Quick atoms
          ; test_case "floats" `Quick floats
          ; test_case "strings" `Quick strings
          ; test_case "pairs" `Quick pairs
          ; test_case "procedures" `Quick procedures
          ; test_case "equality" `Quick equality
          ] )
    ]
;;
