(* SPDX-License-Identifier: GPL-3.0-only *)

type env = (string * t option ref) list

and thunk_state =
  | Delayed of Ast.expr * env
  | Forced of t

and apply_fun = t -> t list -> (t, Eval_error.t) result

and t =
  | VInt of int
  | VFloat of float
  | VBool of bool
  | VString of string
  | VUnit
  | VTuple of t list
  | VConstructor of string * t list
  | VNil
  | VCons of t * t
  | VRecord of (string * t) list
  | VClosure of closure
  | VPrimitive of primitive
  | VPartial of primitive * t list
  | VCompiled of compiled
  | VRef of t ref
  | VArray of t array
  | VTable of table
  | VThunk of thunk_state ref
  | VIndirect of t option ref
  (** A member of a recursive group read while its right-hand side
      is still running: the value the cell will hold. *)

and closure =
  { name : string option
  ; parameters : string list
  ; body : Ast.expr
  ; env : env
  }

and compiled =
  { entry : string
  ; entry_parameters : string list
  ; entry_env : env
  }

and primitive =
  { prim_name : string
  ; prim_arity : int
  ; prim_apply : apply_fun -> t list -> (t, Eval_error.t) result
  }

and table = (t * t) list ref

type closure_view =
  { name : string option
  ; parameters : string list
  ; body : Ast.expr
  ; env : env
  }

type view =
  | Int of int
  | Float of float
  | Bool of bool
  | String of string
  | Unit
  | Tuple of t list
  | Constructor of string * t list
  | Nil
  | Cons of t * t
  | Record of (string * t) list
  | Closure of closure_view
  | Primitive of primitive
  | Partial of primitive * t list
  | Compiled of compiled
  | Ref of t ref
  | Array of t array
  | Table of table
  | Thunk of thunk_state ref

let rec resolve = function
  | VIndirect cell ->
    (match !cell with
     | Some v -> resolve v
     | None -> invalid_arg "Value: a recursive binding is used before its value exists")
  | v -> v
;;

let view v =
  match resolve v with
  | VInt n -> Int n
  | VFloat f -> Float f
  | VBool b -> Bool b
  | VString s -> String s
  | VUnit -> Unit
  | VTuple parts -> Tuple parts
  | VConstructor (name, fields) -> Constructor (name, fields)
  | VNil -> Nil
  | VCons (head, tail) -> Cons (head, tail)
  | VRecord fields -> Record fields
  | VClosure c ->
    Closure { name = c.name; parameters = c.parameters; body = c.body; env = c.env }
  | VPrimitive p -> Primitive p
  | VPartial (p, args) -> Partial (p, args)
  | VCompiled c -> Compiled c
  | VRef cell -> Ref cell
  | VArray a -> Array a
  | VTable tbl -> Table tbl
  | VThunk cell -> Thunk cell
  | VIndirect _ -> invalid_arg "Value.view: unresolved indirection"
;;

let int n = VInt n
let float f = VFloat f
let bool b = VBool b
let string s = VString s
let unit = VUnit
let tuple parts = VTuple parts
let construct name fields = VConstructor (name, fields)
let nil = VNil
let cons head tail = VCons (head, tail)
let record fields = VRecord fields
let closure ~name ~parameters ~body ~env = VClosure { name; parameters; body; env }

let primitive ~name ~arity apply =
  VPrimitive { prim_name = name; prim_arity = arity; prim_apply = apply }
;;

let partial p args = VPartial (p, args)

let compiled ~entry ~parameters ~env =
  VCompiled { entry; entry_parameters = parameters; entry_env = env }
;;

let ref_value v = VRef (ref v)
let array a = VArray a
let table () = VTable (ref [])
let thunk ~expr ~env = VThunk (ref (Delayed (expr, env)))
let forced v = VThunk (ref (Forced v))

let thunk_state_of v =
  match resolve v with
  | VThunk cell -> Some cell
  | _ -> None
;;

let set_thunk_state cell state = cell := state

let rec key_equal a b =
  match resolve a, resolve b with
  | VInt x, VInt y -> Int.equal x y
  | VFloat x, VFloat y -> Float.equal x y
  | VBool x, VBool y -> Bool.equal x y
  | VString x, VString y -> String.equal x y
  | VUnit, VUnit -> true
  | VTuple xs, VTuple ys -> key_equal_list xs ys
  | VConstructor (nx, xs), VConstructor (ny, ys) ->
    String.equal nx ny && key_equal_list xs ys
  | VNil, VNil -> true
  | VCons (hx, tx), VCons (hy, ty) -> key_equal hx hy && key_equal tx ty
  | VRecord xs, VRecord ys ->
    List.length xs = List.length ys
    && List.for_all2
         (fun (fx, vx) (fy, vy) -> String.equal fx fy && key_equal vx vy)
         xs
         ys
  | _ -> false

and key_equal_list xs ys =
  List.length xs = List.length ys && List.for_all2 key_equal xs ys
;;

let table_of_value v =
  match resolve v with
  | VTable tbl -> Some tbl
  | _ -> None
;;

let table_find tbl_value key =
  match table_of_value tbl_value with
  | None -> None
  | Some tbl -> List.find_opt (fun (k, _) -> key_equal k key) !tbl |> Option.map snd
;;

let table_replace tbl_value key value =
  match table_of_value tbl_value with
  | None -> ()
  | Some tbl ->
    let rest = List.filter (fun (k, _) -> not (key_equal k key)) !tbl in
    tbl := rest @ [ key, value ]
;;

let table_remove tbl_value key =
  match table_of_value tbl_value with
  | None -> ()
  | Some tbl -> tbl := List.filter (fun (k, _) -> not (key_equal k key)) !tbl
;;

let table_length tbl_value =
  match table_of_value tbl_value with
  | None -> 0
  | Some tbl -> List.length !tbl
;;

let float_to_string f =
  if Float.is_integer f && Float.abs f < 1e16
  then Printf.sprintf "%.0f" f
  else (
    let rec precision p =
      let s = Printf.sprintf "%.*g" p f in
      if p >= 17 || Float.equal (Float.of_string s) f then s else precision (p + 1)
    in
    precision 15)
;;

let escape s =
  let buf = Buffer.create (String.length s + 2) in
  String.iter
    (function
      | '"' -> Buffer.add_string buf "\\\""
      | '\\' -> Buffer.add_string buf "\\\\"
      | '\n' -> Buffer.add_string buf "\\n"
      | '\r' -> Buffer.add_string buf "\\r"
      | '\t' -> Buffer.add_string buf "\\t"
      | c -> Buffer.add_char buf c)
    s;
  Buffer.contents buf
;;

let rec to_string v =
  match resolve v with
  | VInt n -> string_of_int n
  | VFloat f -> float_to_string f
  | VBool true -> "true"
  | VBool false -> "false"
  | VString s -> "\"" ^ escape s ^ "\""
  | VUnit -> "()"
  | VTuple parts -> "(" ^ String.concat ", " (List.map to_string parts) ^ ")"
  | VConstructor (name, []) -> name
  | VConstructor (name, fields) ->
    name ^ " (" ^ String.concat ", " (List.map to_string fields) ^ ")"
  | VNil -> "[]"
  | VCons _ as v ->
    let rec elements acc v =
      match resolve v with
      | VCons (head, tail) -> elements (to_string head :: acc) tail
      | VNil -> List.rev acc
      | other -> List.rev ((to_string other ^ " improper") :: acc)
    in
    "[" ^ String.concat "; " (elements [] v) ^ "]"
  | VRecord fields ->
    "{"
    ^ String.concat "; " (List.map (fun (name, v) -> name ^ " = " ^ to_string v) fields)
    ^ "}"
  | VClosure c ->
    (match c.name with
     | Some name -> "closure " ^ name
     | None -> "closure")
  | VPrimitive p -> "primitive " ^ p.prim_name
  | VPartial (p, _) -> "partial " ^ p.prim_name
  | VCompiled c -> "compiled procedure " ^ c.entry
  | VRef _ -> "ref"
  | VArray _ -> "array"
  | VTable _ -> "table"
  | VThunk cell ->
    (match !cell with
     | Delayed _ -> "thunk delayed"
     | Forced v -> "thunk forced (" ^ to_string v ^ ")")
  | VIndirect _ -> "recursive binding"
;;

let compare_scalars a b =
  match resolve a, resolve b with
  | VInt x, VInt y -> Ok (Int.compare x y)
  | VFloat x, VFloat y -> Ok (Float.compare x y)
  | VString x, VString y -> Ok (String.compare x y)
  | _ ->
    Error
      (Eval_error.Type_error "ordered comparison operands must be int, float, or string")
;;

let equal_scalars a b =
  match resolve a, resolve b with
  | VUnit, VUnit -> Ok true
  | VInt x, VInt y -> Ok (Int.equal x y)
  | VFloat x, VFloat y -> Ok (x = y)
  | VBool x, VBool y -> Ok (Bool.equal x y)
  | VString x, VString y -> Ok (String.equal x y)
  | _ ->
    Error
      (Eval_error.Type_error "equality operands must be unit, int, float, bool, or string")
;;

let env_empty () = []

let env_extend bindings env =
  List.map (fun (name, v) -> name, ref (Some v)) bindings @ env
;;

let env_extend_recursive names env =
  let cells = List.map (fun name -> name, ref None) names in
  cells @ env, List.map snd cells
;;

let env_fill cell v = cell := Some v

let env_find env name =
  match List.assoc_opt name env with
  | None -> None
  | Some cell ->
    (match !cell with
     | Some v -> Some v
     | None -> Some (VIndirect cell))
;;

let env_find_at env n =
  match if n < 0 then None else List.nth_opt env n with
  | Some (name, { contents = Some v }) -> Some (name, v)
  | Some (_, { contents = None }) | None -> None
;;
