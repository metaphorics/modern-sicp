(* SPDX-License-Identifier: GPL-3.0-only *)

(** The list-structure memory of section 5.3 (grammar section 13, row
    5.3): two semispaces, each a pair of parallel [word array]s for the
    cars and the cdrs; a free pointer; register machines of [Sec_5_1]
    whose words index that memory; and the stop-and-copy collector of
    5.3.2, itself a [Sec_5_1] machine running the section's controller.

    [Save] and [Restore] are the 5.3.1 expansions: a cons onto the
    [the-stack] register and a car/cdr off it, so the stack is ordinary
    list structure the collector reaches as a root.  When an allocation
    finds the memory exhausted, the machine stores its registers in a
    pre-allocated root list in the reserved strip at the top of the
    working semispace, runs the collector from that list's head, hands
    every register its forwarded word after the flip, and re-executes
    the instruction that asked for the cell. *)

type error = Sec_5_1.error

(** {1 Words} *)

type word =
  | Pair of int (** A pair pointer: the index of a cell. *)
  | Num of int (** A number. *)
  | Atom of string (** An atomic datum, compared by name. *)
  | Empty (** The empty list. *)
  | Broken_heart (** The tag a collector leaves in a moved cell. *)
  | Bool of bool (** A test's answer; never stored in a cell by the book's code. *)
  | Vec of int * bool (** Semispace [k]'s cars ([true]) or cdrs ([false]) vector. *)
  | Lab of string (** A label held as data. *)

val word_to_string : word -> string
val equal_word : word -> word -> bool

(** {1 The memory} *)

type memory

(** [make_memory ~size ~root_capacity ~free] is two semispaces of [size]
    cells each, the top [root_capacity] of them reserved for the
    collector's root list, with the free pointer at [free]. *)
val make_memory : size:int -> root_capacity:int -> free:int -> memory

val free_word : memory -> word

(** [cons mem a d] allocates the next free cell; the allocation is
    recorded in [allocation_trace]. *)
val cons : memory -> word -> word -> (word, error) result

val car : memory -> word -> (word, error) result
val cdr : memory -> word -> (word, error) result
val set_car : memory -> word -> word -> (unit, error) result
val set_cdr : memory -> word -> word -> (unit, error) result
val is_pair : word -> bool
val is_null : word -> bool
val is_atom : word -> bool
val is_number : word -> bool
val is_broken_heart : word -> bool
val allocation_trace : memory -> string list

(** [dump mem] draws the working semispace as the book's three rows:
    index, the cars, the cdrs. *)
val dump : memory -> string

(** [write mem w] renders the list structure at [w] in OCaml list
    notation: a proper list as [[a; b]], an improper tail as [a :: b]. *)
val write : memory -> word -> string

val collections : memory -> int
val working : memory -> int

(** {1 Machines over the memory} *)

type machine

(** [make_machine ~registers ~operations ~controller ~memory] assembles
    [controller] over [registers] plus [the-stack], with the list
    operations [cons], [car], [cdr], [set-car!], [set-cdr!], the tests
    [eq?], [=], [null?], [pair?], [atom?], [number?], [+], [-], and the
    stack monitors, then [operations]. *)
val make_machine
  :  registers:string list
  -> operations:(string * word Sec_5_1.op) list
  -> controller:word Sec_5_1.instruction list
  -> memory:memory
  -> (machine, error) result

val set_register : machine -> string -> word -> (unit, error) result
val get_register : machine -> string -> (word, error) result

(** [memory m] is the machine's memory. *)
val memory : machine -> memory

(** [gc_controller] is the collector of 5.3.2 as typed instructions. *)
val gc_controller : word Sec_5_1.instruction list

(** [attach_collector m] arms the allocation path of [m] with a
    collector machine running [gc_controller]. *)
val attach_collector : machine -> (unit, error) result

(** [collect_garbage m] runs the attached collector now. *)
val collect_garbage : machine -> (unit, error) result

(** [start m] runs [m] from its first instruction.  An exhausted
    allocation is retried once after a collection when a collector is
    attached. *)
val start : machine -> (unit, error) result

val print_stack_statistics : machine -> string

(** [transcript m] is the lines the machine's monitors wrote. *)
val transcript : machine -> string list
