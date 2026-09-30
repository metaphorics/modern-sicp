(* SPDX-License-Identifier: GPL-3.0-only *)

module Data = Sicp_common.Constructor_data

let read text = Data.read ~filename:"fixture" text

(* Fixture data is constructor literals only: the reader keeps their
   structure and rejects every computing form. *)
let reads_literals () =
  match
    read
      "(* c *) [ Assert (List [ Atom \"a\"; Num 3 ]); Rule { name = \"r\"; body = (1, \
       -2.5) } ]"
  with
  | Ok
      (Data.List
         [ Data.Ctor
             ( "Assert"
             , [ Data.Ctor
                   ( "List"
                   , [ Data.List
                         [ Data.Ctor ("Atom", [ Data.String "a" ])
                         ; Data.Ctor ("Num", [ Data.Int 3 ])
                         ]
                     ] )
               ] )
         ; Data.Ctor
             ( "Rule"
             , [ Data.Record
                   [ ("name", Data.String "r")
                   ; ("body", Data.Tuple [ Data.Int 1; Data.Float -2.5 ])
                   ]
               ] )
         ]) -> ()
  | Ok d -> Alcotest.fail ("unexpected shape: " ^ Data.describe d)
  | Error e -> Alcotest.fail e
;;

let rejects_computation () =
  List.iter
    (fun text ->
       match read text with
       | Ok _ -> Alcotest.fail ("admitted " ^ text)
       | Error _ -> ())
    [ "x"
    ; "f 1"
    ; "fun x -> x"
    ; "let x = 1 in x"
    ; "1 + 2"
    ; "Some x"
    ; "M.C"
    ; "{ r with a = 1 }"
    ; "[ 1 ]; [ 2 ]"
    ]
;;

let () =
  Alcotest.run
    "constructor data"
    [ ( "reader"
      , [ Alcotest.test_case "literals" `Quick reads_literals
        ; Alcotest.test_case "no computation" `Quick rejects_computation
        ] )
    ]
;;
