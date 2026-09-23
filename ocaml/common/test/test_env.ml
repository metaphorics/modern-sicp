(* SPDX-License-Identifier: GPL-3.0-only *)
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

let error = Alcotest.testable Eval_error.pp ( = )

let find env name =
  Option.map
    (fun v ->
       match Value.view v with
       | Value.Int n -> n
       | _ -> Alcotest.fail "test only binds ints")
    (Env.find_binding env name)
;;

let define env name n = Env.define env name (Value.int n)

let fresh () =
  let env = Env.empty () in
  define env "global" 0;
  env
;;

let finds_and_shadows () =
  let env = fresh () in
  Alcotest.check (Alcotest.option Alcotest.int) "unbound name" None (find env "x");
  define env "x" 1;
  Alcotest.check
    (Alcotest.option Alcotest.int)
    "bound in global frame"
    (Some 1)
    (find env "x");
  let inner =
    match Env.extend [ "x" ] [ Value.int 2 ] env with
    | Ok env -> env
    | Error e -> Alcotest.fail (Eval_error.to_string e)
  in
  Alcotest.check
    (Alcotest.option Alcotest.int)
    "newest frame wins"
    (Some 2)
    (find inner "x");
  Env.define inner "x" (Value.int 3);
  Alcotest.check
    (Alcotest.option Alcotest.int)
    "define lands in newest frame"
    (Some 3)
    (find inner "x");
  Alcotest.check
    (Alcotest.option Alcotest.int)
    "outer frame untouched by define"
    (Some 1)
    (find env "x")
;;

let set_mutates_the_nearest_binding () =
  let global = fresh () in
  define global "x" 1;
  let outer = global in
  let inner =
    match Env.extend [ "y" ] [ Value.int 10 ] outer with
    | Ok env -> env
    | Error e -> Alcotest.fail (Eval_error.to_string e)
  in
  (match Env.set inner "x" (Value.int 99) with
   | Ok () -> ()
   | Error e -> Alcotest.fail (Eval_error.to_string e));
  Alcotest.check
    (Alcotest.option Alcotest.int)
    "set reached the global frame"
    (Some 99)
    (find global "x");
  Alcotest.check
    (Alcotest.option Alcotest.int)
    "inner frame kept y"
    (Some 10)
    (find inner "y");
  match Env.set inner "z" (Value.int 0) with
  | Error e ->
    Alcotest.check error "set of an unbound name" (Eval_error.Unbound_variable "z") e
  | Ok () -> Alcotest.fail "set accepted an unbound name"
;;

let extend_validates_arity () =
  let env = fresh () in
  (match Env.extend [ "a"; "b" ] [ Value.int 1 ] env with
   | Error e ->
     Alcotest.check
       error
       "extend with mismatched lists"
       (Eval_error.Arity_mismatch { expected = 2; given = 1 })
       e
   | Ok _ -> Alcotest.fail "extend accepted mismatched lists");
  (match Env.extend [] [] env with
   | Ok _ -> ()
   | Error e -> Alcotest.fail (Eval_error.to_string e));
  let inner =
    match Env.extend [ "a" ] [ Value.int 1 ] env with
    | Ok env -> env
    | Error e -> Alcotest.fail (Eval_error.to_string e)
  in
  Alcotest.check
    (Alcotest.option Alcotest.int)
    "empty extend is a plain push"
    (Some 1)
    (find inner "a")
;;

let () =
  Alcotest.run
    "sicp_common.env"
    [ ( "env"
      , Alcotest.
          [ test_case "finds and shadows" `Quick finds_and_shadows
          ; test_case
              "set mutates the nearest binding"
              `Quick
              set_mutates_the_nearest_binding
          ; test_case "extend validates arity" `Quick extend_validates_arity
          ] )
    ]
;;
