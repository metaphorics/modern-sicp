(* SPDX-License-Identifier: GPL-3.0-only *)

module Env = Sicp_common.Env
module Value = Sicp_common.Value

let int_of v =
  match Value.view v with
  | Value.Int n -> n
  | _ -> Alcotest.fail "expected an int"
;;

let find env name = Option.map int_of (Env.find env name)
let found = Alcotest.(check (option int))

(* Lexical scope: the newest binding answers, and the extended
   environment leaves the outer one as it was. *)
let shadowing_is_lexical () =
  let outer = Env.extend [ "x", Value.int 1; "y", Value.int 2 ] Env.empty in
  let inner = Env.bind "x" (Value.int 10) outer in
  found "inner x" (Some 10) (find inner "x");
  found "inner sees outer y" (Some 2) (find inner "y");
  found "outer x unchanged" (Some 1) (find outer "x");
  found "unbound" None (find inner "z")
;;

(* A recursive group: a closure-like structure built while its own
   binding is still being computed refers to the finished value, the
   knot a statically constructive [let rec] ties (grammar section 4). *)
let recursive_value_knot () =
  let env, cells = Env.extend_recursive [ "ones" ] Env.empty in
  let self =
    match Env.find env "ones" with
    | Some v -> v
    | None -> Alcotest.fail "a recursive name is in scope in its own group"
  in
  let ones = Value.cons (Value.int 1) self in
  (match cells with
   | [ cell ] -> Env.fill cell ones
   | _ -> Alcotest.fail "one cell per name");
  let rec nth v n =
    match Value.view v with
    | Value.Cons (head, tail) -> if n = 0 then int_of head else nth tail (n - 1)
    | _ -> Alcotest.fail "the knot is a cons cycle"
  in
  Alcotest.(check int) "the third element of the cycle" 1 (nth ones 2);
  found
    "the name answers the filled value"
    (Some 1)
    (Option.map (fun v -> nth v 0) (Env.find env "ones"))
;;

(* References are the only mutation: every closure environment that
   holds a reference observes one cell. *)
let references_are_shared () =
  let cell = Value.ref_value (Value.int 0) in
  let a = Env.bind "r" cell Env.empty in
  let b = Env.bind "q" (Value.int 5) a in
  (match Option.map Value.view (Env.find a "r") with
   | Some (Value.Ref r) -> r := Value.int 7
   | _ -> Alcotest.fail "r is a reference");
  match Option.map Value.view (Env.find b "r") with
  | Some (Value.Ref r) ->
    Alcotest.(check int) "b observes the write through a" 7 (int_of !r)
  | _ -> Alcotest.fail "r is a reference"
;;

let () =
  Alcotest.run
    "env"
    [ ( "env"
      , [ Alcotest.test_case "shadowing is lexical" `Quick shadowing_is_lexical
        ; Alcotest.test_case "recursive values tie a knot" `Quick recursive_value_knot
        ; Alcotest.test_case "references are shared" `Quick references_are_shared
        ] )
    ]
;;
