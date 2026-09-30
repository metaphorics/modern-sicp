(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs in SICP section 5.3 *)

let ( let* ) = Result.bind

module Eval_error = Sicp_common.Eval_error
module M = Sec_5_1

type error = Sec_5_1.error

type word =
  | Pair of int
  | Num of int
  | Atom of string
  | Empty
  | Broken_heart
  | Bool of bool
  | Vec of int * bool
  | Lab of string

let word_to_string = function
  | Pair i -> "p" ^ string_of_int i
  | Num n -> "n" ^ string_of_int n
  | Atom s -> s
  | Empty -> "e0"
  | Broken_heart -> "broken-heart"
  | Bool b -> string_of_bool b
  | Vec (k, cars) -> (if cars then "cars-" else "cdrs-") ^ string_of_int k
  | Lab l -> l
;;

let equal_word a b =
  match a, b with
  | Pair x, Pair y | Num x, Num y -> x = y
  | Atom x, Atom y | Lab x, Lab y -> String.equal x y
  | Empty, Empty | Broken_heart, Broken_heart -> true
  | Bool x, Bool y -> Bool.equal x y
  | Vec (k, c), Vec (l, d) -> k = l && Bool.equal c d
  | _ -> false
;;

let failed detail = Error (Eval_error.Bounds_error detail)
let exhausted = "the memory is exhausted"

(* {1 The memory} *)

type memory =
  { size : int
  ; root_capacity : int
  ; spaces : (word array * word array) array
  ; mutable free : int
  ; mutable working : int
  ; mutable collections : int
  ; mutable trace : string list
  }

let make_memory ~size ~root_capacity ~free =
  let blank () = Array.make size Empty in
  { size
  ; root_capacity
  ; spaces = [| blank (), blank (); blank (), blank () |]
  ; free
  ; working = 0
  ; collections = 0
  ; trace = []
  }
;;

let data_limit mem = mem.size - mem.root_capacity
let cars mem k = fst mem.spaces.(k)
let cdrs mem k = snd mem.spaces.(k)
let free_word mem = Pair mem.free
let collections mem = mem.collections
let working mem = mem.working
let allocation_trace mem = List.rev mem.trace

let read vector i =
  if i >= 0 && i < Array.length vector
  then Ok vector.(i)
  else failed "memory index out of range"
;;

let store vector i v =
  if i >= 0 && i < Array.length vector
  then (
    vector.(i) <- v;
    Ok ())
  else failed "memory index out of range"
;;

let cons mem a d =
  if mem.free >= data_limit mem
  then failed exhausted
  else (
    let i = mem.free in
    let* () = store (cars mem mem.working) i a in
    let* () = store (cdrs mem mem.working) i d in
    mem.free <- i + 1;
    mem.trace
    <- Printf.sprintf
         "cons -> p%d = (%s, %s); free p%d -> p%d"
         i
         (word_to_string a)
         (word_to_string d)
         i
         mem.free
       :: mem.trace;
    Ok (Pair i))
;;

let not_pair what w =
  Error (Eval_error.Type_error (what ^ ": not a pair: " ^ word_to_string w))
;;

let car mem = function
  | Pair i -> read (cars mem mem.working) i
  | w -> not_pair "car" w
;;

let cdr mem = function
  | Pair i -> read (cdrs mem mem.working) i
  | w -> not_pair "cdr" w
;;

let set_car mem w v =
  match w with
  | Pair i -> store (cars mem mem.working) i v
  | _ -> not_pair "set-car!" w
;;

let set_cdr mem w v =
  match w with
  | Pair i -> store (cdrs mem mem.working) i v
  | _ -> not_pair "set-cdr!" w
;;

let is_pair = function
  | Pair _ -> true
  | _ -> false
;;

let is_null = function
  | Empty -> true
  | _ -> false
;;

let is_atom = function
  | Atom _ -> true
  | _ -> false
;;

let is_number = function
  | Num _ -> true
  | _ -> false
;;

let is_broken_heart = function
  | Broken_heart -> true
  | _ -> false
;;

let dump mem =
  let cs, ds = mem.spaces.(mem.working) in
  let width i =
    2
    + max
        (String.length (string_of_int i))
        (max
           (String.length (word_to_string cs.(i)))
           (String.length (word_to_string ds.(i))))
  in
  let widths = Array.init (Array.length cs) width in
  let pad w s = s ^ String.make (max 1 (w - String.length s)) ' ' in
  let row name cell =
    String.trim
      (name
       ^ String.concat "" (Array.to_list (Array.mapi (fun i w -> pad w (cell i)) widths))
      )
  in
  String.concat
    "\n"
    [ row "index    " string_of_int
    ; row "the-cars " (fun i -> word_to_string cs.(i))
    ; row "the-cdrs " (fun i -> word_to_string ds.(i))
    ]
;;

let rec write_word mem w =
  match w with
  | Pair _ ->
    let rec items acc w =
      match w with
      | Empty -> Ok ("[" ^ String.concat "; " (List.rev acc) ^ "]")
      | Pair _ ->
        let* a = car mem w in
        let* a = write_word mem a in
        let* d = cdr mem w in
        items (a :: acc) d
      | tail ->
        let* tail = write_word mem tail in
        Ok (String.concat " :: " (List.rev (tail :: acc)))
    in
    items [] w
  | Num n -> Ok (string_of_int n)
  | Empty -> Ok "[]"
  | Atom _ | Broken_heart | Bool _ | Vec _ | Lab _ -> Ok (word_to_string w)
;;

let write mem w =
  match write_word mem w with
  | Ok s -> s
  | Error e -> "unreadable: " ^ Eval_error.to_string e
;;

(* {1 Machines} *)

(* The monitors and the simulator a machine's own operations reach. *)
type state =
  { mem : memory
  ; lines : string list ref
  ; pushes : int ref
  ; depth : int ref
  ; max_depth : int ref
  ; sim : word M.machine option ref
  }

type machine =
  { machine : word M.machine
  ; state : state
  ; mutable collect : (unit -> (unit, error) result) option
  }

let words =
  { M.label = (fun l -> Lab l)
  ; to_label =
      (function
        | Lab l -> Some l
        | _ -> None)
  ; show = word_to_string
  ; unassigned = Empty
  }
;;

(* The 5.3.1 expansions of [save] and [restore] onto list structure. *)
let expand_stack controller =
  List.concat_map
    (function
      | M.Save r ->
        [ M.Assign_op ("the-stack", "stack-push", [ M.Reg r; M.Reg "the-stack" ]) ]
      | M.Restore r ->
        [ M.Assign_op (r, "stack-top", [ M.Reg "the-stack" ])
        ; M.Assign_op ("the-stack", "stack-pop", [ M.Reg "the-stack" ])
        ]
      | i -> [ i ])
    controller
;;

let arity expected ws =
  Error (Eval_error.Arity_mismatch { expected; given = List.length ws })
;;

let value1 name f =
  ( name
  , M.Value_op
      (function
        | [ w ] -> f w
        | ws -> arity 1 ws) )
;;

let value2 name f =
  ( name
  , M.Value_op
      (function
        | [ a; b ] -> f a b
        | ws -> arity 2 ws) )
;;

let test1 name p =
  ( name
  , M.Test_op
      (function
        | [ w ] -> Ok (p w)
        | ws -> arity 1 ws) )
;;

let numbers name f =
  value2 name (fun a b ->
    match a, b with
    | Num x, Num y -> Ok (Num (f x y))
    | _ -> Error (Eval_error.Type_error (name ^ " expects two numbers")))
;;

let list_operations st =
  let mem = st.mem in
  [ value2 "cons" (cons mem)
  ; value1 "car" (car mem)
  ; value1 "cdr" (cdr mem)
  ; ( "set-car!"
    , M.Action_op
        (function
          | [ p; v ] -> set_car mem p v
          | ws -> arity 2 ws) )
  ; ( "set-cdr!"
    , M.Action_op
        (function
          | [ p; v ] -> set_cdr mem p v
          | ws -> arity 2 ws) )
  ; ( "eq?"
    , M.Test_op
        (function
          | [ a; b ] -> Ok (equal_word a b)
          | ws -> arity 2 ws) )
  ; ( "="
    , M.Test_op
        (function
          | [ Num a; Num b ] -> Ok (a = b)
          | [ _; _ ] -> Error (Eval_error.Type_error "= expects two numbers")
          | ws -> arity 2 ws) )
  ; numbers "+" ( + )
  ; numbers "-" ( - )
  ; test1 "null?" is_null
  ; test1 "pair?" is_pair
  ; test1 "atom?" is_atom
  ; test1 "number?" is_number
  ; value2 "stack-push" (fun w stack ->
      let* cell = cons mem w stack in
      incr st.pushes;
      incr st.depth;
      if !(st.depth) > !(st.max_depth) then st.max_depth := !(st.depth);
      Ok cell)
  ; value1 "stack-top" (function
      | Pair _ as stack -> car mem stack
      | _ -> Error (Eval_error.Bad_instruction "restore from an empty stack"))
  ; value1 "stack-pop" (function
      | Pair _ as stack ->
        decr st.depth;
        cdr mem stack
      | _ -> Error (Eval_error.Bad_instruction "restore from an empty stack"))
  ; ( "initialize-stack"
    , M.Action_op
        (fun _ ->
          st.pushes := 0;
          st.depth := 0;
          st.max_depth := 0;
          match !(st.sim) with
          | Some sim -> M.set_register sim "the-stack" Empty
          | None -> Error (Eval_error.Invalid_form "the machine is not assembled")) )
  ; ( "print-stack-statistics"
    , M.Action_op
        (fun _ ->
          st.lines
          := Printf.sprintf
               "total-pushes = %d maximum-depth = %d"
               !(st.pushes)
               !(st.max_depth)
             :: !(st.lines);
          Ok ()) )
  ]
;;

let make_machine ~registers ~operations ~controller ~memory =
  let st =
    { mem = memory
    ; lines = ref []
    ; pushes = ref 0
    ; depth = ref 0
    ; max_depth = ref 0
    ; sim = ref None
    }
  in
  let* machine =
    M.make
      ~words
      ~registers:(registers @ [ "the-stack" ])
      ~operations:(operations @ list_operations st)
      (expand_stack controller)
  in
  let* () = M.set_register machine "the-stack" Empty in
  st.sim := Some machine;
  Ok { machine; state = st; collect = None }
;;

let set_register m = M.set_register m.machine
let get_register m = M.get_register m.machine
let memory m = m.state.mem

let print_stack_statistics m =
  Printf.sprintf
    "total-pushes = %d maximum-depth = %d"
    !(m.state.pushes)
    !(m.state.max_depth)
;;

let transcript m = List.rev !(m.state.lines)

(* {1 The stop-and-copy collector of 5.3.2} *)

let gc_controller =
  let r name = M.Reg name in
  [ M.Label "begin-garbage-collection"
  ; M.Assign ("free", M.Const (Pair 0))
  ; M.Assign ("scan", M.Const (Pair 0))
  ; M.Assign ("old", r "root")
  ; M.Assign ("relocate-continue", M.Label_ref "reassign-root")
  ; M.Goto "relocate-old-result-in-new"
  ; M.Label "reassign-root"
  ; M.Assign ("root", r "new")
  ; M.Goto "gc-loop"
  ; M.Label "gc-loop"
  ; M.Test ("=", [ r "scan"; r "free" ])
  ; M.Branch "gc-flip"
  ; M.Assign_op ("old", "vector-ref", [ r "new-cars"; r "scan" ])
  ; M.Assign ("relocate-continue", M.Label_ref "update-car")
  ; M.Goto "relocate-old-result-in-new"
  ; M.Label "update-car"
  ; M.Perform ("vector-set!", [ r "new-cars"; r "scan"; r "new" ])
  ; M.Assign_op ("old", "vector-ref", [ r "new-cdrs"; r "scan" ])
  ; M.Assign ("relocate-continue", M.Label_ref "update-cdr")
  ; M.Goto "relocate-old-result-in-new"
  ; M.Label "update-cdr"
  ; M.Perform ("vector-set!", [ r "new-cdrs"; r "scan"; r "new" ])
  ; M.Assign_op ("scan", "+", [ r "scan"; M.Const (Num 1) ])
  ; M.Goto "gc-loop"
  ; M.Label "relocate-old-result-in-new"
  ; M.Test ("pointer-to-pair?", [ r "old" ])
  ; M.Branch "pair"
  ; M.Assign ("new", r "old")
  ; M.Goto_reg "relocate-continue"
  ; M.Label "pair"
  ; M.Assign_op ("oldcr", "vector-ref", [ r "the-cars"; r "old" ])
  ; M.Test ("broken-heart?", [ r "oldcr" ])
  ; M.Branch "already-moved"
  ; M.Assign ("new", r "free")
  ; M.Assign_op ("free", "+", [ r "free"; M.Const (Num 1) ])
  ; M.Perform ("vector-set!", [ r "new-cars"; r "new"; r "oldcr" ])
  ; M.Assign_op ("oldcr", "vector-ref", [ r "the-cdrs"; r "old" ])
  ; M.Perform ("vector-set!", [ r "new-cdrs"; r "new"; r "oldcr" ])
  ; M.Perform ("vector-set!", [ r "the-cars"; r "old"; M.Const Broken_heart ])
  ; M.Perform ("vector-set!", [ r "the-cdrs"; r "old"; r "new" ])
  ; M.Goto_reg "relocate-continue"
  ; M.Label "already-moved"
  ; M.Assign_op ("new", "vector-ref", [ r "the-cdrs"; r "old" ])
  ; M.Goto_reg "relocate-continue"
  ; M.Label "gc-flip"
  ; M.Assign ("temp", r "the-cdrs")
  ; M.Assign ("the-cdrs", r "new-cdrs")
  ; M.Assign ("new-cdrs", r "temp")
  ; M.Assign ("temp", r "the-cars")
  ; M.Assign ("the-cars", r "new-cars")
  ; M.Assign ("new-cars", r "temp")
  ]
;;

let gc_registers =
  [ "root"
  ; "free"
  ; "scan"
  ; "old"
  ; "new"
  ; "oldcr"
  ; "relocate-continue"
  ; "the-cars"
  ; "the-cdrs"
  ; "new-cars"
  ; "new-cdrs"
  ; "temp"
  ]
;;

let index = function
  | Pair i | Num i -> Ok i
  | w -> Error (Eval_error.Type_error ("not a memory index: " ^ word_to_string w))
;;

let collector_operations mem =
  let vector = function
    | Vec (k, true) -> Ok (cars mem k)
    | Vec (k, false) -> Ok (cdrs mem k)
    | w -> Error (Eval_error.Type_error ("not a vector: " ^ word_to_string w))
  in
  [ ( "vector-ref"
    , M.Value_op
        (function
          | [ v; i ] ->
            let* v = vector v in
            let* i = index i in
            read v i
          | ws -> arity 2 ws) )
  ; ( "vector-set!"
    , M.Action_op
        (function
          | [ v; i; w ] ->
            let* v = vector v in
            let* i = index i in
            store v i w
          | ws -> arity 3 ws) )
  ; ( "="
    , M.Test_op
        (function
          | [ a; b ] ->
            let* a = index a in
            let* b = index b in
            Ok (a = b)
          | ws -> arity 2 ws) )
  ; ( "+"
    , M.Value_op
        (function
          | [ Pair a; Num b ] -> Ok (Pair (a + b))
          | [ Num a; Num b ] -> Ok (Num (a + b))
          | ws ->
            Error
              (Eval_error.Type_error
                 ("+ cannot take " ^ String.concat ", " (List.map word_to_string ws)))) )
  ; test1 "pointer-to-pair?" is_pair
  ; test1 "broken-heart?" is_broken_heart
  ]
;;

(* The book's arrangement around the collector: the registers go into a
   root list in the reserved strip, the collector relocates from its
   head, and after the flip each register takes its forwarded word. *)
let collect app gc =
  let mem = app.state.mem in
  let names = M.registers app.machine in
  let* roots =
    List.fold_right
      (fun name acc ->
         let* acc = acc in
         let* w = get_register app name in
         Ok (w :: acc))
      names
      (Ok [])
  in
  let n = List.length roots in
  if n > mem.root_capacity
  then failed "the root strip is smaller than the register set"
  else (
    let spine i = mem.size - mem.root_capacity + i in
    List.iteri
      (fun i w ->
         (cars mem mem.working).(spine i) <- w;
         (cdrs mem mem.working).(spine i)
         <- (if i + 1 < n then Pair (spine (i + 1)) else Empty))
      roots;
    let other = 1 - mem.working in
    M.restart gc;
    let* () = M.set_register gc "root" (Pair (spine 0)) in
    let* () = M.set_register gc "the-cars" (Vec (mem.working, true)) in
    let* () = M.set_register gc "the-cdrs" (Vec (mem.working, false)) in
    let* () = M.set_register gc "new-cars" (Vec (other, true)) in
    let* () = M.set_register gc "new-cdrs" (Vec (other, false)) in
    let* () = M.start gc in
    let* free = Result.bind (M.get_register gc "free") index in
    let* k =
      match M.get_register gc "the-cars" with
      | Ok (Vec (k, _)) -> Ok k
      | Ok w -> failed ("the collector left the-cars = " ^ word_to_string w)
      | Error e -> Error e
    in
    mem.free <- free;
    mem.working <- k;
    mem.collections <- mem.collections + 1;
    let* head = M.get_register gc "root" in
    let rec walk cell = function
      | [] -> Ok ()
      | name :: rest ->
        let* w = car mem cell in
        let* () = set_register app name w in
        let* next = cdr mem cell in
        walk next rest
    in
    if n = 0 then Ok () else walk head names)
;;

let attach_collector app =
  let* gc =
    M.make
      ~words
      ~registers:gc_registers
      ~operations:(collector_operations app.state.mem)
      gc_controller
  in
  app.collect <- Some (fun () -> collect app gc);
  Ok ()
;;

let collect_garbage m =
  match m.collect with
  | Some collect -> collect ()
  | None -> Error (Eval_error.Invalid_form "no collector is attached to the machine")
;;

(* An allocation that finds the memory exhausted fails before its
   instruction completes, so the program counter still names it: after
   one collection the same instruction runs again and re-reads its
   (forwarded) register operands. *)
let start m =
  M.restart m.machine;
  let* () = M.set_register m.machine "the-stack" Empty in
  let rec run retried =
    match M.step m.machine with
    | Ok true -> run false
    | Ok false -> Ok ()
    | Error (Eval_error.Bounds_error detail)
      when String.equal detail exhausted && not retried ->
      (match m.collect with
       | Some collect ->
         let* () = collect () in
         run true
       | None -> failed exhausted)
    | Error e -> Error e
  in
  run false
;;
