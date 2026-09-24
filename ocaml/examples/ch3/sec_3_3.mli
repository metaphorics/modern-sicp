(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The named definitions behind the listings of section 3.3, grouped by
    subsection. The mutable pair record is the section's dedicated mutable
    cell; the queue, the tables, the digital-circuit simulator, and the
    constraint network are all built on it. Every value a listing displays
    is asserted by [run_sec_3_3] through [Replay], so the book's result
    comments are true by construction. *)

(** 3.3.1: the mutable pair record and the mutators. [mobj] is the
    edition's typed stand-in for the book's pointers: a mutable-list
    object is a number, a symbol (an immutable string), a pointer to a
    pair, the empty list, or -- only inside the agenda -- a scheduled
    procedure. *)
module Mpairs : sig
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

  val mnil : mobj
  val mint : int -> mobj
  val msym : string -> mobj
  val pair_of : mobj -> mpair
  val is_pair : mobj -> bool
  val car : mobj -> mobj
  val cdr : mobj -> mobj
  val set_car : mobj -> mobj -> unit
  val set_cdr : mobj -> mobj -> unit
  val mcons : mobj -> mobj -> mobj
  val from_symbols : string list -> mobj
  val from_objs : mobj list -> mobj
  val last_pair : mobj -> mobj
  val append : mobj -> mobj -> mobj
  val cons_from_mutators : mobj -> mobj -> mobj
  val set_to_wow : mobj -> mobj

  (** [show o] is the book's printed notation for [o]; it refuses
      structures larger than [fuel] nodes, so a cyclic structure raises
      instead of diverging (exercise 3.13a builds the cycle-safe
      printer). *)
  val show : ?fuel:int -> mobj -> string
end

(** 3.3.2: a queue is the cons of a front pointer and a rear pointer
    into one ordinary pair-list. *)
module Queue : sig
  type t = Mpairs.mpair

  val make_queue : unit -> t
  val front_ptr : t -> Mpairs.mobj
  val rear_ptr : t -> Mpairs.mobj
  val set_front_ptr : t -> Mpairs.mobj -> unit
  val set_rear_ptr : t -> Mpairs.mobj -> unit
  val empty_queue : t -> bool
  val front_queue : t -> Mpairs.mobj
  val insert_queue : t -> Mpairs.mobj -> t
  val delete_queue : t -> t

  (** The sequence of items a [print-queue] would show. *)
  val items : t -> Mpairs.mobj list
end

(** 3.3.3: tables as headed lists of records. Keys are symbols or
    numbers and are compared structurally, as the book compares them
    with [equal?]. *)
module Table : sig
  val assoc : Mpairs.mobj -> Mpairs.mobj -> Mpairs.mobj option
  val make_table : unit -> Mpairs.mobj
  val lookup : Mpairs.mobj -> Mpairs.mobj -> Mpairs.mobj option
  val insert : Mpairs.mobj -> Mpairs.mobj -> Mpairs.mobj -> unit
  val lookup2 : Mpairs.mobj -> Mpairs.mobj -> Mpairs.mobj -> Mpairs.mobj option
  val insert2 : Mpairs.mobj -> Mpairs.mobj -> Mpairs.mobj -> Mpairs.mobj -> unit

  (** A table as an object with local state: a record of closures over
      one internal table. *)
  type table_object =
    { lookup_proc : Mpairs.mobj -> Mpairs.mobj -> Mpairs.mobj option
    ; insert_proc : Mpairs.mobj -> Mpairs.mobj -> Mpairs.mobj -> unit
    }

  val make_table_object : unit -> table_object
end

(** 3.3.4: the digital-circuit simulator. The agenda and the delays
    travel together in one [sim] record; the hand-built agenda of time
    segments reuses the pair and queue machinery above. *)
module Circuit : sig
  type wire =
    { get_signal : unit -> int
    ; set_signal : int -> unit
    ; add_action : (unit -> unit) -> unit
    }

  type sim = private
    { mutable current_time : int
    ; mutable segments : Mpairs.mobj
    ; inverter_delay : int
    ; and_gate_delay : int
    ; or_gate_delay : int
    ; trace : string list ref
    }

  val make_wire : unit -> wire

  val make_sim
    :  inverter_delay:int
    -> and_gate_delay:int
    -> or_gate_delay:int
    -> unit
    -> sim

  val empty_agenda : sim -> bool
  val first_segment : sim -> Mpairs.mobj
  val rest_segments : sim -> Mpairs.mobj
  val add_to_agenda : sim -> int -> (unit -> unit) -> unit
  val remove_first_agenda_item : sim -> unit
  val first_agenda_item : sim -> unit -> unit
  val after_delay : sim -> int -> (unit -> unit) -> unit
  val propagate : sim -> unit

  (** The probe records one transcript line per signal change; the
      runner prints the lines the book shows as output. *)
  val probe : sim -> string -> wire -> unit

  val logical_not : int -> int
  val inverter : sim -> wire -> wire -> unit
  val logical_and : int -> int -> int
  val and_gate : sim -> wire -> wire -> wire -> unit
  val logical_or : int -> int -> int
  val or_gate : sim -> wire -> wire -> wire -> unit
  val half_adder : sim -> wire -> wire -> wire -> wire -> unit
  val full_adder : sim -> wire -> wire -> wire -> wire -> wire -> unit
end

(** 3.3.5: the constraint network. A constraint is the book's [me]
    dispatch as one function; a connector is a record of closures over
    its value, informant, and constraint list. *)
module Constraints : sig
  type request =
    | I_have_a_value
    | I_lost_my_value

  type constraint_ = { me : request -> unit }

  type informant =
    | User
    | Of_constraint of constraint_

  type connector = private
    { has_value : unit -> bool
    ; get_value : unit -> int
    ; set_value : int -> informant -> unit
    ; forget_value : informant -> unit
    ; connect : constraint_ -> unit
    }

  (* The book's [for-each-except]: the exception is an informant (the
     setter, which may be the user), compared physically against the
     constraints in the list. *)
  val for_each_except : informant -> (constraint_ -> unit) -> constraint_ list -> unit

  (* Whether two informants designate the same setter: identity for
     constraints, equality for the user. *)
  val same_informant : informant -> informant -> bool
  val make_connector : unit -> connector
  val adder : connector -> connector -> connector -> constraint_
  val multiplier : connector -> connector -> connector -> constraint_
  val constant : int -> connector -> constraint_

  (** The probe is itself a constraint; it records one line per value
      event for the runner to print. *)
  val probe : string list ref -> string -> connector -> unit

  val celsius_fahrenheit_converter : connector -> connector -> unit
end
