(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The named definitions behind the listings of section 3.3, grouped by
    subsection. The mutable pair record is the section's dedicated mutable
    cell; the queue, the tables, the digital-circuit simulator, and the
    constraint network are all built on it. Every value a listing displays
    is asserted by [run_sec_3_3] through [Replay], so the book's result
    comments are true by construction. *)

module Mpairs = struct
  (** 3.3.1: the mutable pair record and the operations the section
      defines on it. [mobj] is the edition's typed stand-in for the
      book's pointers: a mutable-list object is either a number, a
      symbol (an immutable string), a pointer to a pair, the empty
      list, or -- only inside the agenda -- a scheduled procedure. *)

  type mobj =
    | Int of int
    | Sym of string
    | Pair of mpair
    | Nil
    | Proc of (unit -> unit)

  and mpair =
    { mutable car : mobj
    ; mutable cdr : mobj
    }

  let mnil = Nil
  let mint n = Int n
  let msym s = Sym s

  let pair_of = function
    | Pair p -> p
    | _ -> invalid_arg "expected a pair"
  ;;

  let is_pair = function
    | Pair _ -> true
    | _ -> false
  ;;

  let car o = (pair_of o).car
  let cdr o = (pair_of o).cdr
  let set_car o v = (pair_of o).car <- v
  let set_cdr o v = (pair_of o).cdr <- v
  let mcons a d = Pair { car = a; cdr = d }

  let rec from_symbols = function
    | [] -> mnil
    | s :: rest -> mcons (Sym s) (from_symbols rest)
  ;;

  let rec from_objs = function
    | [] -> mnil
    | o :: rest -> mcons o (from_objs rest)
  ;;

  let rec last_pair x = if is_pair (cdr x) then last_pair (cdr x) else x

  (* The book's append of @ref{2.2.1}, building a fresh chain of new
     pairs whose cars are the elements of [x] and whose final cdr is
     [y]. *)
  let rec append x y =
    match x with
    | Nil -> y
    | Pair _ -> mcons (car x) (append (cdr x) y)
    | _ -> invalid_arg "append: not a list"
  ;;

  (* The book's [cons] built from the two mutators, as the section
     shows it: take a fresh pair, set its two pointers, return it. *)
  let cons_from_mutators x y =
    let new_pair = { car = mnil; cdr = mnil } in
    set_car (Pair new_pair) x;
    set_cdr (Pair new_pair) y;
    Pair new_pair
  ;;

  (* The book's [set-to-wow!]. *)
  let set_to_wow x =
    set_car (car x) (Sym "wow");
    x
  ;;

  (* The book's printed notation. The walk refuses structures too
     large or cyclic: exercise 3.13a builds the cycle-safe printer. *)
  let show ?(fuel = 100_000) o =
    let budget = ref fuel in
    let tick () =
      decr budget;
      if !budget < 0
      then
        invalid_arg
          "show: structure too large or cyclic (see exercise 3.13a for a cycle-safe \
           printer)"
    in
    let rec go = function
      | Int n -> string_of_int n
      | Sym s -> s
      | Nil -> "()"
      | Proc _ -> "#proc"
      | Pair p ->
        tick ();
        let rec spine tail items =
          match tail with
          | Pair _ ->
            tick ();
            spine (cdr tail) (car tail :: items)
          | Nil -> List.rev items, None
          | other -> List.rev items, Some other
        in
        let items, dotted = spine p.cdr [ p.car ] in
        let parts = List.map go items in
        let parts =
          match dotted with
          | None -> parts
          | Some tail -> parts @ [ "."; go tail ]
        in
        "(" ^ String.concat " " parts ^ ")"
    in
    go o
  ;;
end

module Queue = struct
  (** 3.3.2: a queue is the cons of a front pointer and a rear pointer
      into one ordinary pair-list. *)

  type t = Mpairs.mpair

  let make_queue () : t = { Mpairs.car = Mpairs.Nil; cdr = Mpairs.Nil }
  let front_ptr (q : t) = q.Mpairs.car
  let rear_ptr (q : t) = q.Mpairs.cdr
  let set_front_ptr (q : t) item = q.Mpairs.car <- item
  let set_rear_ptr (q : t) item = q.Mpairs.cdr <- item
  let empty_queue (q : t) = Mpairs.Nil = front_ptr q

  let front_queue q =
    if empty_queue q
    then invalid_arg "FRONT called with an empty queue"
    else Mpairs.car (front_ptr q)
  ;;

  let insert_queue q item =
    let new_pair = { Mpairs.car = item; cdr = Mpairs.Nil } in
    if empty_queue q
    then (
      set_front_ptr q (Mpairs.Pair new_pair);
      set_rear_ptr q (Mpairs.Pair new_pair))
    else (
      Mpairs.set_cdr (rear_ptr q) (Mpairs.Pair new_pair);
      set_rear_ptr q (Mpairs.Pair new_pair));
    q
  ;;

  let delete_queue q =
    if empty_queue q
    then invalid_arg "DELETE called with an empty queue"
    else (
      set_front_ptr q (Mpairs.cdr (front_ptr q));
      q)
  ;;

  (* The sequence of items a [print-queue] would show. *)
  let items q =
    let rec go = function
      | Mpairs.Nil -> []
      | o -> Mpairs.car o :: go (Mpairs.cdr o)
    in
    if empty_queue q then [] else go (front_ptr q)
  ;;
end

module Table = struct
  (** 3.3.3: headed lists of records. Keys are symbols or numbers and
      are compared structurally, as the book compares them with
      [equal?]. *)

  open Mpairs

  (* The book's [assoc] over a record chain, answering [None] where
     the book answers false. *)
  let rec assoc key = function
    | Pair p when key_equal (car p.car) key -> Some p.car
    | Pair p -> assoc key p.cdr
    | _ -> None

  and key_equal a b =
    match a, b with
    | Int x, Int y -> Int.compare x y = 0
    | Sym x, Sym y -> String.equal x y
    | _ -> false
  ;;

  let make_table () = mcons (Sym "*table*") mnil

  let lookup key table =
    match assoc key (cdr table) with
    | Some record -> Some (cdr record)
    | None -> None
  ;;

  let insert key value table =
    match assoc key (cdr table) with
    | Some record -> set_cdr record value
    | None -> set_cdr table (mcons (mcons key value) (cdr table))
  ;;

  let lookup2 key1 key2 table =
    match assoc key1 (cdr table) with
    | None -> None
    | Some subtable ->
      (match assoc key2 (cdr subtable) with
       | Some record -> Some (cdr record)
       | None -> None)
  ;;

  let insert2 key1 key2 value table =
    match assoc key1 (cdr table) with
    | Some subtable ->
      (match assoc key2 (cdr subtable) with
       | Some record -> set_cdr record value
       | None -> set_cdr subtable (mcons (mcons key2 value) (cdr subtable)))
    | None ->
      set_cdr table (mcons (mcons key1 (mcons (mcons key2 value) mnil)) (cdr table))
  ;;

  (* A table as an object with local state: a record of closures over
     one internal table. *)
  type table_object =
    { lookup_proc : mobj -> mobj -> mobj option
    ; insert_proc : mobj -> mobj -> mobj -> unit
    }

  let make_table_object () =
    let local_table = make_table () in
    { lookup_proc = (fun k1 k2 -> lookup2 local_table k1 k2)
    ; insert_proc = (fun k1 k2 v -> insert2 local_table k1 k2 v)
    }
  ;;
end

module Circuit = struct
  (** 3.3.4: the digital-circuit simulator. The agenda and the delays
      travel together in one [sim] record, the way section 3.1 passed
      random states explicitly; the section's hand-built agenda of
      time segments reuses the pair and queue machinery above. *)

  open Mpairs

  type wire =
    { get_signal : unit -> int
    ; set_signal : int -> unit
    ; add_action : (unit -> unit) -> unit
    }

  type sim =
    { mutable current_time : int
    ; mutable segments : mobj
    ; inverter_delay : int
    ; and_gate_delay : int
    ; or_gate_delay : int
    ; trace : string list ref
    }

  let make_wire () =
    let signal_value = ref 0 in
    let action_procedures = ref [] in
    let set_my_signal new_value =
      if !signal_value <> new_value
      then (
        signal_value := new_value;
        List.iter (fun p -> p ()) !action_procedures)
    in
    let accept_action_procedure proc =
      action_procedures := proc :: !action_procedures;
      proc ()
    in
    { get_signal = (fun () -> !signal_value)
    ; set_signal = set_my_signal
    ; add_action = accept_action_procedure
    }
  ;;

  let make_sim ~inverter_delay ~and_gate_delay ~or_gate_delay () =
    { current_time = 0
    ; segments = mnil
    ; inverter_delay
    ; and_gate_delay
    ; or_gate_delay
    ; trace = ref []
    }
  ;;

  (* Time segments: a pair of a time and a queue of scheduled
     procedures, chained as an ordered list. *)
  let make_time_segment time queue = mcons (Int time) (Pair queue)
  let segment_time s = car s
  let segment_queue s = pair_of (cdr s)
  let empty_agenda sim = Nil = sim.segments
  let first_segment sim = car sim.segments
  let rest_segments sim = cdr sim.segments

  let add_to_agenda sim time action =
    let cell_time segments =
      match segment_time (car segments) with
      | Int t -> t
      | _ -> invalid_arg "corrupt agenda segment"
    in
    let belongs_before segments =
      match segments with
      | Nil -> true
      | Pair _ -> time < cell_time segments
      | _ -> invalid_arg "corrupt agenda"
    in
    let make_new_time_segment time action =
      let q = Queue.make_queue () in
      ignore (Queue.insert_queue q (Proc action));
      make_time_segment time q
    in
    let rec add_to_segments segments =
      if cell_time segments = time
      then ignore (Queue.insert_queue (segment_queue (car segments)) (Proc action))
      else (
        let rest = cdr segments in
        if belongs_before rest
        then set_cdr segments (mcons (make_new_time_segment time action) rest)
        else add_to_segments rest)
    in
    if belongs_before sim.segments
    then sim.segments <- mcons (make_new_time_segment time action) sim.segments
    else add_to_segments sim.segments
  ;;

  let remove_first_agenda_item sim =
    let q = segment_queue (first_segment sim) in
    ignore (Queue.delete_queue q);
    if Queue.empty_queue q then sim.segments <- cdr sim.segments
  ;;

  let first_agenda_item sim =
    if empty_agenda sim
    then invalid_arg "Agenda is empty: FIRST-AGENDA-ITEM"
    else (
      let first_seg = first_segment sim in
      sim.current_time
      <- (match segment_time first_seg with
          | Int t -> t
          | _ -> assert false);
      match Queue.front_queue (segment_queue first_seg) with
      | Proc action -> action
      | _ -> invalid_arg "agenda item is not a procedure")
  ;;

  let after_delay sim delay action = add_to_agenda sim (delay + sim.current_time) action

  let rec propagate sim =
    if empty_agenda sim
    then ()
    else (
      let action = first_agenda_item sim in
      action ();
      remove_first_agenda_item sim;
      propagate sim)
  ;;

  (* The probe records one transcript line per signal change; the
     test runner prints the lines the book shows as output. *)
  let probe sim name wire =
    wire.add_action (fun () ->
      sim.trace
      := Printf.sprintf "%s %d  New-value = %d" name sim.current_time (wire.get_signal ())
         :: !(sim.trace))
  ;;

  let logical_not = function
    | 0 -> 1
    | 1 -> 0
    | s -> invalid_arg (Printf.sprintf "Invalid signal %d" s)
  ;;

  let inverter sim input output =
    let invert_input () =
      let new_value = logical_not (input.get_signal ()) in
      after_delay sim sim.inverter_delay (fun () -> output.set_signal new_value)
    in
    input.add_action invert_input
  ;;

  let logical_and a b = if a = 1 && b = 1 then 1 else 0

  let and_gate sim a1 a2 output =
    let and_action_procedure () =
      let new_value = logical_and (a1.get_signal ()) (a2.get_signal ()) in
      after_delay sim sim.and_gate_delay (fun () -> output.set_signal new_value)
    in
    a1.add_action and_action_procedure;
    a2.add_action and_action_procedure
  ;;

  let logical_or a b = if a = 1 || b = 1 then 1 else 0

  let or_gate sim a1 a2 output =
    let or_action_procedure () =
      let new_value = logical_or (a1.get_signal ()) (a2.get_signal ()) in
      after_delay sim sim.or_gate_delay (fun () -> output.set_signal new_value)
    in
    a1.add_action or_action_procedure;
    a2.add_action or_action_procedure
  ;;

  let half_adder sim a b s c =
    let d = make_wire () in
    let e = make_wire () in
    or_gate sim a b d;
    and_gate sim a b c;
    inverter sim c e;
    and_gate sim d e s
  ;;

  let full_adder sim a b c_in sum c_out =
    let s = make_wire () in
    let c1 = make_wire () in
    let c2 = make_wire () in
    half_adder sim b c_in s c1;
    half_adder sim a s sum c2;
    or_gate sim c1 c2 c_out
  ;;
end

module Constraints = struct
  (** 3.3.5: the constraint network. A constraint is the book's [me]
      dispatch as one function; a connector is a record of closures
      over its value, informant, and constraint list. *)

  type request =
    | I_have_a_value
    | I_lost_my_value

  type constraint_ = { me : request -> unit }

  type informant =
    | User
    | Of_constraint of constraint_

  type connector =
    { has_value : unit -> bool
    ; get_value : unit -> int
    ; set_value : int -> informant -> unit
    ; forget_value : informant -> unit
    ; connect : constraint_ -> unit
    }

  (* The book's [for-each-except]: the exception is an informant (the
     setter, which may be the user), compared physically against the
     constraints in the list. Identity is asked of the constraint
     record inside the wrapper, never of the wrapper itself. *)
  let for_each_except exception_ inform constraints =
    match exception_ with
    | User -> List.iter inform constraints
    | Of_constraint me ->
      List.iter (fun item -> if not (item == me) then inform item) constraints
  ;;

  let same_informant a b =
    match a, b with
    | User, User -> true
    | Of_constraint x, Of_constraint y -> x == y
    | _ -> false
  ;;

  let rec make_connector () =
    let value = ref 0 in
    let informant = ref None in
    let constraints = ref [] in
    let has_value () =
      match !informant with
      | Some _ -> true
      | None -> false
    in
    let set_my_value newval setter =
      match !informant with
      | None ->
        value := newval;
        informant := Some setter;
        for_each_except setter inform_about_value !constraints
      | Some _ when !value <> newval ->
        invalid_arg (Printf.sprintf "Contradiction (%d %d)" !value newval)
      | _ -> ()
    in
    let forget_my_value retractor =
      match !informant with
      | Some who when same_informant who retractor ->
        informant := None;
        for_each_except retractor inform_about_no_value !constraints
      | _ -> ()
    in
    let connect new_constraint =
      if not (List.exists (fun c -> c == new_constraint) !constraints)
      then constraints := new_constraint :: !constraints;
      if has_value () then new_constraint.me I_have_a_value
    in
    { has_value
    ; get_value = (fun () -> !value)
    ; set_value = set_my_value
    ; forget_value = forget_my_value
    ; connect
    }

  and inform_about_value constraint_ = constraint_.me I_have_a_value
  and inform_about_no_value constraint_ = constraint_.me I_lost_my_value

  let adder a1 a2 sum =
    let rec me =
      { me =
          (function
            | I_have_a_value -> process_new_value ()
            | I_lost_my_value -> process_forget_value ())
      }
    and process_new_value () =
      if a1.has_value () && a2.has_value ()
      then sum.set_value (a1.get_value () + a2.get_value ()) (Of_constraint me)
      else if a1.has_value () && sum.has_value ()
      then a2.set_value (sum.get_value () - a1.get_value ()) (Of_constraint me)
      else if a2.has_value () && sum.has_value ()
      then a1.set_value (sum.get_value () - a2.get_value ()) (Of_constraint me)
    and process_forget_value () =
      sum.forget_value (Of_constraint me);
      a1.forget_value (Of_constraint me);
      a2.forget_value (Of_constraint me);
      process_new_value ()
    in
    a1.connect me;
    a2.connect me;
    sum.connect me;
    me
  ;;

  let multiplier m1 m2 product =
    let rec me =
      { me =
          (function
            | I_have_a_value -> process_new_value ()
            | I_lost_my_value -> process_forget_value ())
      }
    and process_new_value () =
      if
        (m1.has_value () && m1.get_value () = 0)
        || (m2.has_value () && m2.get_value () = 0)
      then product.set_value 0 (Of_constraint me)
      else if m1.has_value () && m2.has_value ()
      then product.set_value (m1.get_value () * m2.get_value ()) (Of_constraint me)
      else if product.has_value () && m1.has_value ()
      then m2.set_value (product.get_value () / m1.get_value ()) (Of_constraint me)
      else if product.has_value () && m2.has_value ()
      then m1.set_value (product.get_value () / m2.get_value ()) (Of_constraint me)
    and process_forget_value () =
      product.forget_value (Of_constraint me);
      m1.forget_value (Of_constraint me);
      m2.forget_value (Of_constraint me);
      process_new_value ()
    in
    m1.connect me;
    m2.connect me;
    product.connect me;
    me
  ;;

  let constant value connector =
    let handle = { me = (fun _ -> invalid_arg "Unknown request: CONSTANT") } in
    connector.connect handle;
    connector.set_value value (Of_constraint handle);
    handle
  ;;

  (* The probe is itself a constraint; it records one line per value
     event for the runner to print. *)
  let probe log name connector =
    connector.connect
      { me =
          (function
            | I_have_a_value ->
              log := Printf.sprintf "Probe: %s = %d" name (connector.get_value ()) :: !log
            | I_lost_my_value -> log := Printf.sprintf "Probe: %s = ?" name :: !log)
      }
  ;;

  let celsius_fahrenheit_converter c f =
    let u = make_connector () in
    let v = make_connector () in
    let w = make_connector () in
    let x = make_connector () in
    let y = make_connector () in
    ignore (multiplier c w u);
    ignore (multiplier v x u);
    ignore (adder v y f);
    ignore (constant 9 w);
    ignore (constant 5 x);
    ignore (constant 32 y)
  ;;
end
