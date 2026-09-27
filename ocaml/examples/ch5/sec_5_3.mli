(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.3 *)

(** The list-structure memory of section 5.3: the typed word vector,
    the free pointer and the allocation path, and the stop-and-copy
    collector over two semispaces.

    A machine word is the book's letter-prefixed pointer: [p3] names
    the pair cell at index 3, [n4] the number 4, [e0] the empty list,
    and the cars of a moved cell hold the [broken-heart] tag. Registers
    and the save/restore stack hold words, so the collector's roots are
    exactly the book's -- every machine register, the stack register
    among them. Controllers are still data in the book's notation read
    by the shared reader; the machine type is the 5.2 surface with
    vector memory behind it. The stop-and-copy collector is the book's
    controller listing, run verbatim. Nothing raises. *)

type word =
  | Pair of int (** [p<i>], a pair pointer: the index part names a cell. *)
  | Num of int (** [n<i>], a number. *)
  | Sym of string (** a symbol pointer. *)
  | Empty (** [e0], the empty list. *)
  | Broken_heart (** the moved-object tag a collector leaves behind. *)
  | Bool of bool (** a test's answer; never a memory cell. *)
  | Vec of int * bool
  (** a semispace vector base, as the [the-cars]/[the-cdrs] family of
          registers holds it: space [k], [true] for the cars, [false]
          for the cdrs. *)
  | Lab of string (** a label value, as the [relocate-continue] register holds it. *)

(** [word_to_string w] renders [w] in the book's pointer notation. *)
val word_to_string : word -> string

(** [equal_word a b] tests the equality of all fields, the book's
    [eq?] over typed pointers. *)
val equal_word : word -> word -> bool

type error = Sec_5_1.error =
  | Parse of string
  | Unknown_register of string
  | Unknown_operation of string
  | Unknown_label of string
  | Bad_instruction of string
  | Arity of string
  | Op_failed of string
  | Stack_underflow of string
  | Branch_without_test

(** [error_to_string e] renders [e] for a transcript or a test. *)
val error_to_string : error -> string

(** One list-structure memory: two semispaces of [size] data cells
    each. [root_capacity] is the room the collector's pre-allocated
    root list needs -- the allocation path collects before the free
    pointer eats into this headroom, which is also what guarantees a
    collection's copies fit the new semispace. [free] is the next free
    index of the working semispace. *)
type memory

(** [make_memory ~size ~root_capacity ~free] is a fresh memory whose
    working semispace allocates from [free] -- Exercise 5.20 starts its
    drawing with the free pointer at [p1], so the start is the caller's. *)
val make_memory : size:int -> root_capacity:int -> free:int -> memory

(** [free_word mem] is the free pointer as the book's register holds
    it, a pair pointer such as [p4]. *)
val free_word : memory -> word

(** [cons mem a d] allocates the cell [(a d)] at the free pointer,
    records the allocation on the memory's trace, and answers the new
    pair pointer. A full memory is the typed exhausted-memory failure;
    the collector hook lives on machines, not on the raw vectors. *)
val cons : memory -> word -> word -> (word, error) result

(** [car mem w] is the car field of the pair [w] in the working
    semispace; [cdr] the cdr field. *)
val car : memory -> word -> (word, error) result

val cdr : memory -> word -> (word, error) result

(** [set_car mem w v] stores [v] in the car field of the pair [w];
    [set_cdr] the cdr field. *)
val set_car : memory -> word -> word -> (unit, error) result

val set_cdr : memory -> word -> word -> (unit, error) result

(** The type-field predicates of 5.3.1: each checks only the word's
    kind. [pointer_to_pair] is the collector's low-level
    [pointer-to-pair?]; [is_broken_heart] recognizes the moved tag. *)
val is_pair : word -> bool

val is_null : word -> bool
val is_symbol : word -> bool
val is_number : word -> bool
val pointer_to_pair : word -> bool
val is_broken_heart : word -> bool

(** [allocation_trace mem] is every allocation the memory recorded,
    oldest first -- the instrument the free-pointer exercises read. *)
val allocation_trace : memory -> string list

(** [dump mem] is the memory-vector drawing of the working semispace,
    the [the-cars]/[the-cdrs]/index table of Figure 5.14 with one
    column per cell. *)
val dump : memory -> string

(** [write mem w] renders the list structure at [w] the way the book
    prints lists, reading the cells through the working semispace. *)
val write : memory -> word -> string

(** One operation of a word machine: a [Value_op] computes a word for
    an [assign] or a [test]; an [Action_op] is an action under
    [perform]. *)
type op =
  | Value_op of (word list -> (word, error) result)
  | Action_op of (word list -> (unit, error) result)

(** One machine over the word vector. Its registers always include
    [the-stack], the save/restore stack lives in the memory as the
    5.3.1 expansion pushes it, and the machine's registers -- the stack
    register among them -- are the collector's roots. *)
type machine

(** [make_machine ~registers ~operations ~controller ~memory] assembles
    the controller over the word vector; an unknown register, label, or
    operation fails before the machine can start. The operations named
    [cons], [car], [cdr], [set-car!], [set-cdr!], and the predicates
    are installed by default over the memory; [operations] extends
    them and, where a name collides, overrides the default. *)
val make_machine
  :  registers:string list
  -> operations:(string * op) list
  -> controller:string
  -> memory:memory
  -> (machine, error) result

(** [set_register m r w] loads a register before [start]; [get_register]
    reads one after the machine stops. *)
val set_register : machine -> string -> word -> (unit, error) result

val get_register : machine -> string -> (word, error) result

(** [attach_collector m] builds the section's stop-and-copy collector
    for [m]'s memory and arms the allocation path: a [cons] that finds
    the working semispace full collects garbage first and retries once. *)
val attach_collector : machine -> (unit, error) result

(** [collect_garbage m] runs the section's stop-and-copy collector now:
    the same driver the allocation path arms, exposed so the
    reconfiguration of Figure 5.15 can be shown around a collection. *)
val collect_garbage : machine -> (unit, error) result

(** [start m] runs the machine from the first instruction until the
    sequence ends or an instruction fails. *)
val start : machine -> (unit, error) result

(** [print_stack_statistics m] renders the monitored stack's counters,
    [total-pushes] and [maximum-depth]. *)
val print_stack_statistics : machine -> string

(** [transcript m] is what the machine's print actions have printed, in
    order. *)
val transcript : machine -> string list

(** [collections mem] is how many collections the memory has run. *)
val collections : memory -> int

(** [working mem] is which semispace is working, 0 or 1; a collection
    swaps it -- the pointer-swap the book proves by hand. *)
val working : memory -> int
