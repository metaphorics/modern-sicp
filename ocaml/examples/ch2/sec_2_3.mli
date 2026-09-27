(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs of SICP section 2.3 *)

(** Quotation, subsection 2.3.1. *)
module Symbols : sig
  (** A symbol: the constructor wrapped around its printed name. *)
  type symbol = Sym of string

  (** [memq item items] is [Some] the sublist of [items] beginning with
      the first occurrence of [item], or [None] when no item equals
      it. The book's procedure answers false or the sublist; here the
      two answers are [None] and [Some sublist]. *)
  val memq : 'a -> 'a list -> 'a list option
end

(** Symbolic differentiation, subsection 2.3.2, first stage: the
    constructors build sums and products without simplifying. *)
module Deriv_naive : sig
  (** An algebraic expression over [+] and [*]: a constant, a variable,
      a two-term sum, or a two-term product. *)
  type expr =
    | Const of int
    | Var of string
    | Sum of expr * expr
    | Prod of expr * expr

  val is_variable : expr -> bool
  val same_variable : expr -> expr -> bool
  val is_sum : expr -> bool
  val is_product : expr -> bool
  val make_sum : expr -> expr -> expr
  val make_product : expr -> expr -> expr

  (** [addend], [augend], [multiplier], and [multiplicand] extract the
      named part of a sum or product; [invalid_arg] when the expression
      is of the other kind. *)
  val addend : expr -> expr

  val augend : expr -> expr
  val multiplier : expr -> expr
  val multiplicand : expr -> expr

  (** [deriv exp var] is the derivative of [exp] with respect to the
      variable named [var], with no simplification. *)
  val deriv : expr -> string -> expr
end

(** Symbolic differentiation, second stage: the constructors build in
    the rules [x * 0 = 0], [1 * y = y], and [0 + y = y]; [deriv] is
    unchanged. *)
module Deriv : sig
  type expr = Deriv_naive.expr =
    | Const of int
    | Var of string
    | Sum of expr * expr
    | Prod of expr * expr

  val is_variable : expr -> bool
  val same_variable : expr -> expr -> bool
  val is_sum : expr -> bool
  val is_product : expr -> bool
  val addend : expr -> expr
  val augend : expr -> expr
  val multiplier : expr -> expr
  val multiplicand : expr -> expr

  (** [is_number exp n] holds when [exp] is the constant [n]. *)
  val is_number : expr -> int -> bool

  val make_sum : expr -> expr -> expr
  val make_product : expr -> expr -> expr

  (** [deriv exp var] is the derivative of [exp] with respect to the
      variable named [var], simplified by the constructors. *)
  val deriv : expr -> string -> expr
end

(** Sets, subsection 2.3.3: the operations that define a set, gathered
    in one signature. A representation earns the signature when it
    supplies all four; [union_set] and the ordered [adjoin_set] wait
    for Exercises 2.59, 2.61, 2.62, and 2.65. *)
module type Set = sig
  (** The representation: whatever this module says a set is. *)
  type t

  val element_of_set : int -> t -> bool
  val adjoin_set : int -> t -> t
  val union_set : t -> t -> t
  val intersection_set : t -> t -> t
end

(** Sets as unordered lists. *)
module Unordered_list_set : sig
  type t = int list

  val element_of_set : int -> t -> bool
  val adjoin_set : int -> t -> t
  val intersection_set : t -> t -> t
end

(** Sets as ordered lists of numbers. *)
module Ordered_list_set : sig
  type t = int list

  val element_of_set : int -> t -> bool
  val intersection_set : t -> t -> t
end

(** Sets as binary trees. *)
module Tree_set : sig
  (** A binary tree: [Empty], or a node holding a left branch, an
      entry, and a right branch. *)
  type tree =
    | Empty
    | Node of tree * int * tree

  (** The representation of the set operations. *)
  type t = tree

  (** The selectors of the book's list-built representation; each
      signals [invalid_arg] on [Empty] where the book's [car] chain
      would tear a non-pair. *)
  val entry : tree -> int

  val left_branch : tree -> tree
  val right_branch : tree -> tree
  val make_tree : int -> tree -> tree -> tree
  val element_of_set : int -> t -> bool
  val adjoin_set : int -> t -> t
end

(** Sets and information retrieval. *)
module Record_db : sig
  type record =
    { key : int
    ; name : string
    }

  (** [lookup given_key records] is the first record whose key is
      [given_key], or [None] when no record has it. *)
  val lookup : int -> record list -> record option
end

(** Huffman encoding trees, subsection 2.3.4. *)
module Huffman : sig
  (** One bit of a message. *)
  type bit =
    | Zero
    | One

  (** An encoding tree: a leaf holds a symbol and its weight; a node
      holds its branches, the symbols below it, and their total
      weight. *)
  type tree =
    | Leaf of string * int
    | Node of tree * tree * string list * int

  val make_leaf : string -> int -> tree

  (** The generic selectors: the symbols and the total weight at any
      node, leaf or not. *)
  val symbols : tree -> string list

  val weight : tree -> int
  val make_code_tree : tree -> tree -> tree

  (** The branch selectors of a non-leaf node; [invalid_arg] on a leaf,
      which has no branches to choose. *)
  val left_branch : tree -> tree

  val right_branch : tree -> tree

  (** [choose_branch bit branch] is the branch [bit] names. The bit
      variant has no other case, so the book's bad-bit complaint has
      nothing left to complain about. *)
  val choose_branch : bit -> tree -> tree

  (** [decode bits tree] is the message the bit sequence names. *)
  val decode : bit list -> tree -> string list

  (** [adjoin_set x set] inserts [x] into the weight-ordered set. *)
  val adjoin_set : tree -> tree list -> tree list

  (** [make_leaf_set pairs] is the ordered set of leaves for the
      symbol-frequency pairs. *)
  val make_leaf_set : (string * int) list -> tree list

  val sample_tree : tree
end
