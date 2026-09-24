(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.38 *)

(** Exercise 3.38: Peter deposits 10, Paul withdraws 20, and Mary
    withdraws half of a shared balance that starts at 100. The map
    class is [T]: the statement translates directly, and the answer is
    computed rather than hand traced, since the edition can enumerate
    every schedule instead of drawing timing diagrams for each one. *)

type process =
  { name : string
  ; read_write : int -> int
  }

let peter = { name = "Peter"; read_write = (fun v -> v + 10) }
let paul = { name = "Paul"; read_write = (fun v -> v - 20) }
let mary = { name = "Mary"; read_write = (fun v -> v / 2) }

(* Every permutation of [xs]. [process] carries a closure, so removing
   one occurrence of [x] must compare by physical identity: structural
   [=] or [compare] on a value holding a function raises. *)
let rec permutations = function
  | [] -> [ [] ]
  | xs ->
    List.concat_map
      (fun x ->
         let rest = List.filter (fun y -> y != x) xs in
         List.map (fun p -> x :: p) (permutations rest))
      xs
;;

let sequential_balances () =
  permutations [ peter; paul; mary ]
  |> List.map (fun order ->
    List.fold_left (fun balance p -> p.read_write balance) 100 order)
  |> List.sort_uniq compare
;;

(* One event of a process's read/write pair, in the order the process
   performs them. *)
type event =
  | Read of process
  | Write of process

let events p = [ Read p; Write p ]

let interleaved_balances () =
  Interleaving.interleavings [ events peter; events paul; events mary ]
  |> List.map (fun schedule ->
    let reads = Hashtbl.create 3 in
    let balance = ref 100 in
    List.iter
      (function
        | Read p -> Hashtbl.replace reads p.name !balance
        | Write p -> balance := p.read_write (Hashtbl.find reads p.name))
      schedule;
    !balance)
  |> List.sort_uniq compare
;;

let sample_concurrent_runs n =
  let one_run () =
    let balance = ref 100 in
    let run p () = balance := p.read_write !balance in
    let d1 = Domain.spawn (run peter) in
    let d2 = Domain.spawn (run paul) in
    run mary ();
    Domain.join d1;
    Domain.join d2;
    !balance
  in
  List.init n (fun _ -> one_run ()) |> List.sort_uniq compare
;;

let ex_3_38 () =
  sequential_balances (), interleaved_balances (), sample_concurrent_runs 200
;;
