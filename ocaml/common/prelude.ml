(* SPDX-License-Identifier: GPL-3.0-only *)

let unqualified =
  [ "print_string"
  ; "print_endline"
  ; "print_int"
  ; "print_newline"
  ; "string_of_int"
  ; "string_of_float"
  ; "float_of_int"
  ; "sqrt"
  ]
;;

let qualified =
  [ "Array.make"
  ; "Array.get"
  ; "Array.set"
  ; "Array.length"
  ; "List.map"
  ; "List.filter"
  ; "List.fold_left"
  ; "List.fold_right"
  ; "List.length"
  ; "List.rev"
  ; "List.append"
  ; "List.sort"
  ; "Hashtbl.create"
  ; "Hashtbl.find_opt"
  ; "Hashtbl.replace"
  ; "Hashtbl.remove"
  ; "Hashtbl.length"
  ]
;;

let ( >>= ) = Result.bind
let ( let* ) = Result.bind
let type_error name detail = Error (Eval_error.Type_error (name ^ ": " ^ detail))

let rec list_of_value v =
  match Value.view v with
  | Value.Nil -> Some []
  | Value.Cons (head, tail) -> Option.map (fun rest -> head :: rest) (list_of_value tail)
  | _ -> None
;;

let value_of_list items = List.fold_right Value.cons items Value.nil

let one _name = function
  | [ v ] -> Ok v
  | args -> Error (Eval_error.Arity_mismatch { expected = 1; given = List.length args })
;;

let two _name = function
  | [ a; b ] -> Ok (a, b)
  | args -> Error (Eval_error.Arity_mismatch { expected = 2; given = List.length args })
;;

let three _name = function
  | [ a; b; c ] -> Ok (a, b, c)
  | args -> Error (Eval_error.Arity_mismatch { expected = 3; given = List.length args })
;;

let as_int name v =
  match Value.view v with
  | Value.Int n -> Ok n
  | _ -> type_error name "expects an int"
;;

let as_float name v =
  match Value.view v with
  | Value.Float f -> Ok f
  | _ -> type_error name "expects a float"
;;

let as_string name v =
  match Value.view v with
  | Value.String s -> Ok s
  | _ -> type_error name "expects a string"
;;

let as_array name v =
  match Value.view v with
  | Value.Array a -> Ok a
  | _ -> type_error name "expects an array"
;;

let as_list name v =
  match list_of_value v with
  | Some items -> Ok items
  | None -> type_error name "expects a list"
;;

let as_table name v =
  match Value.view v with
  | Value.Table _ -> Ok v
  | _ -> type_error name "expects a table"
;;

let as_procedure name v =
  match Value.view v with
  | Value.Closure _ | Value.Primitive _ | Value.Partial _ | Value.Compiled _ -> Ok v
  | _ -> type_error name "expects a procedure"
;;

let bounds name index length =
  Error
    (Eval_error.Bounds_error
       (Printf.sprintf "%s: index %d out of range for length %d" name index length))
;;

let initial_env ~emit () =
  let prim name arity apply = name, Value.primitive ~name ~arity apply in
  let bindings =
    [ prim "print_string" 1 (fun _apply args ->
        one "print_string" args
        >>= as_string "print_string"
        |> Result.map (fun s ->
          emit s;
          Value.unit))
    ; prim "print_endline" 1 (fun _apply args ->
        one "print_endline" args
        >>= as_string "print_endline"
        |> Result.map (fun s ->
          emit (s ^ "\n");
          Value.unit))
    ; prim "print_int" 1 (fun _apply args ->
        one "print_int" args
        >>= as_int "print_int"
        |> Result.map (fun n ->
          emit (string_of_int n);
          Value.unit))
    ; prim "print_newline" 1 (fun _apply args ->
        one "print_newline" args
        |> Result.map (fun _ ->
          emit "\n";
          Value.unit))
    ; prim "string_of_int" 1 (fun _apply args ->
        one "string_of_int" args
        >>= as_int "string_of_int"
        |> Result.map (fun n -> Value.string (string_of_int n)))
    ; prim "string_of_float" 1 (fun _apply args ->
        one "string_of_float" args
        >>= as_float "string_of_float"
        |> Result.map (fun f -> Value.string (Stdlib.string_of_float f)))
    ; prim "float_of_int" 1 (fun _apply args ->
        one "float_of_int" args
        >>= as_int "float_of_int"
        |> Result.map (fun n -> Value.float (Stdlib.float_of_int n)))
    ; prim "sqrt" 1 (fun _apply args ->
        one "sqrt" args
        >>= as_float "sqrt"
        |> Result.map (fun f -> Value.float (Stdlib.sqrt f)))
    ; prim "Array.make" 2 (fun _apply args ->
        two "Array.make" args
        >>= fun (length, initial) ->
        as_int "Array.make" length
        >>= fun length ->
        if length < 0
        then Error (Eval_error.Bounds_error "Array.make: negative length")
        else Ok (Value.array (Array.make length initial)))
    ; prim "Array.get" 2 (fun _apply args ->
        two "Array.get" args
        >>= fun (array, index) ->
        let* array = as_array "Array.get" array in
        let* index = as_int "Array.get" index in
        if index < 0 || index >= Array.length array
        then bounds "Array.get" index (Array.length array)
        else Ok array.(index))
    ; prim "Array.set" 3 (fun _apply args ->
        three "Array.set" args
        >>= fun (array, index, value) ->
        let* array = as_array "Array.set" array in
        let* index = as_int "Array.set" index in
        if index < 0 || index >= Array.length array
        then bounds "Array.set" index (Array.length array)
        else (
          array.(index) <- value;
          Ok Value.unit))
    ; prim "Array.length" 1 (fun _apply args ->
        one "Array.length" args
        >>= as_array "Array.length"
        |> Result.map (fun a -> Value.int (Array.length a)))
    ; prim "List.map" 2 (fun apply args ->
        two "List.map" args
        >>= fun (f, items) ->
        let* f = as_procedure "List.map" f in
        let* items = as_list "List.map" items in
        let rec map acc = function
          | [] -> Ok (value_of_list (List.rev acc))
          | item :: rest -> Result.bind (apply f [ item ]) (fun v -> map (v :: acc) rest)
        in
        map [] items)
    ; prim "List.filter" 2 (fun apply args ->
        two "List.filter" args
        >>= fun (f, items) ->
        let* f = as_procedure "List.filter" f in
        let* items = as_list "List.filter" items in
        let rec filter acc = function
          | [] -> Ok (value_of_list (List.rev acc))
          | item :: rest ->
            Result.bind (apply f [ item ]) (fun answer ->
              match Value.view answer with
              | Value.Bool true -> filter (item :: acc) rest
              | Value.Bool false -> filter acc rest
              | _ -> type_error "List.filter" "predicate did not answer a bool")
        in
        filter [] items)
    ; prim "List.fold_left" 3 (fun apply args ->
        three "List.fold_left" args
        >>= fun (f, initial, items) ->
        let* f = as_procedure "List.fold_left" f in
        let* items = as_list "List.fold_left" items in
        let rec fold acc = function
          | [] -> Ok acc
          | item :: rest -> Result.bind (apply f [ acc; item ]) (fun v -> fold v rest)
        in
        fold initial items)
    ; prim "List.fold_right" 3 (fun apply args ->
        three "List.fold_right" args
        >>= fun (f, initial, items) ->
        let* f = as_procedure "List.fold_right" f in
        let* items = as_list "List.fold_right" items in
        let rec fold acc = function
          | [] -> Ok acc
          | item :: rest -> Result.bind (fold acc rest) (fun v -> apply f [ item; v ])
        in
        fold initial items)
    ; prim "List.length" 1 (fun _apply args ->
        one "List.length" args
        >>= as_list "List.length"
        |> Result.map (fun items -> Value.int (List.length items)))
    ; prim "List.rev" 1 (fun _apply args ->
        one "List.rev" args
        >>= as_list "List.rev"
        |> Result.map (fun items -> value_of_list (List.rev items)))
    ; prim "List.append" 2 (fun _apply args ->
        two "List.append" args
        >>= fun (left, right) ->
        let* left = as_list "List.append" left in
        let* right = as_list "List.append" right in
        Ok (value_of_list (left @ right)))
    ; prim "List.sort" 2 (fun apply args ->
        two "List.sort" args
        >>= fun (compare, items) ->
        let* compare = as_procedure "List.sort" compare in
        let* items = as_list "List.sort" items in
        let order x y =
          Result.bind
            (apply compare [ x; y ])
            (fun answer ->
               match Value.view answer with
               | Value.Int n -> Ok n
               | _ -> type_error "List.sort" "comparator did not answer an int")
        in
        let rec insert x = function
          | [] -> Ok [ x ]
          | y :: rest ->
            Result.bind (order x y) (fun n ->
              if n <= 0
              then Ok (x :: y :: rest)
              else Result.map (fun t -> y :: t) (insert x rest))
        in
        let rec sort = function
          | [] -> Ok []
          | x :: rest -> Result.bind (sort rest) (fun sorted -> insert x sorted)
        in
        Result.map value_of_list (sort items))
    ; prim "Hashtbl.create" 1 (fun _apply args ->
        one "Hashtbl.create" args
        >>= as_int "Hashtbl.create"
        |> Result.map (fun _ -> Value.table ()))
    ; prim "Hashtbl.find_opt" 2 (fun _apply args ->
        two "Hashtbl.find_opt" args
        >>= fun (tbl, key) ->
        let* tbl = as_table "Hashtbl.find_opt" tbl in
        Ok
          (match Value.table_find tbl key with
           | None -> Value.construct "None" []
           | Some v -> Value.construct "Some" [ v ]))
    ; prim "Hashtbl.replace" 3 (fun _apply args ->
        three "Hashtbl.replace" args
        >>= fun (tbl, key, value) ->
        let* tbl = as_table "Hashtbl.replace" tbl in
        Value.table_replace tbl key value;
        Ok Value.unit)
    ; prim "Hashtbl.remove" 2 (fun _apply args ->
        two "Hashtbl.remove" args
        >>= fun (tbl, key) ->
        let* tbl = as_table "Hashtbl.remove" tbl in
        Value.table_remove tbl key;
        Ok Value.unit)
    ; prim "Hashtbl.length" 1 (fun _apply args ->
        one "Hashtbl.length" args
        >>= as_table "Hashtbl.length"
        |> Result.map (fun tbl -> Value.int (Value.table_length tbl)))
    ]
  in
  Env.extend bindings Env.empty
;;
