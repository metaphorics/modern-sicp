(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs of SICP section 2.3 *)

(** The named definitions behind the listings of section 2.3, grouped by
    subsection. Each value a listing displays is asserted by
    [run_sec_2_3] through [Replay], so the book's result comments are
    true by construction.

    The section's hard spot is the line between syntax and values.
    Scheme draws the line at run time with quotation; OCaml draws it at
    compile time with types, so there is no quotation to explain.
    Symbols become explicit constructors, the algebraic expressions and
    the Huffman trees become variants, and the set representations stay
    separate modules behind one signature. *)

(** Quotation, subsection 2.3.1. A symbol is the [Sym] constructor
    wrapped around its name, so writing a symbol as data is an ordinary
    expression and no evaluation can touch it. [memq] is the book's
    symbol search over any list whose items compare with [=]; it
    answers [None] where the book's procedure answers false and
    [Some sublist] where it answers the sublist. *)
module Symbols = struct
  type symbol = Sym of string

  let rec memq item = function
    | [] -> None
    | first :: rest -> if first = item then Some (first :: rest) else memq item rest
  ;;
end

(** Symbolic differentiation, subsection 2.3.2, first stage. The
    algebraic expression is a [variant] whose constructors answer the
    book's predicates and whose pattern positions answer its selectors;
    [make_sum] and [make_product] build sums and products without
    simplifying them. *)
module Deriv_naive = struct
  type expr =
    | Const of int
    | Var of string
    | Sum of expr * expr
    | Prod of expr * expr

  let is_variable = function
    | Var _ -> true
    | _ -> false
  ;;

  let same_variable v1 v2 = is_variable v1 && is_variable v2 && v1 = v2

  let is_sum = function
    | Sum _ -> true
    | _ -> false
  ;;

  let is_product = function
    | Prod _ -> true
    | _ -> false
  ;;

  let make_sum a1 a2 = Sum (a1, a2)
  let make_product m1 m2 = Prod (m1, m2)

  let addend = function
    | Sum (a, _) -> a
    | _ -> invalid_arg "addend: not a sum"
  ;;

  let augend = function
    | Sum (_, a) -> a
    | _ -> invalid_arg "augend: not a sum"
  ;;

  let multiplier = function
    | Prod (m, _) -> m
    | _ -> invalid_arg "multiplier: not a product"
  ;;

  let multiplicand = function
    | Prod (_, m) -> m
    | _ -> invalid_arg "multiplicand: not a product"
  ;;

  let rec deriv exp var =
    match exp with
    | Const _ -> Const 0
    | Var _ -> if same_variable exp (Var var) then Const 1 else Const 0
    | Sum (a1, a2) -> make_sum (deriv a1 var) (deriv a2 var)
    | Prod (m1, m2) ->
      make_sum (make_product m1 (deriv m2 var)) (make_product (deriv m1 var) m2)
  ;;
end

(** Symbolic differentiation, second stage. The [deriv] program is
    unchanged; only the constructors learned the reduction rules that
    [x * 0 = 0], [1 * y = y], and [0 + y = y], together with
    [is_number], the test the rules need. OCaml cannot rebind
    [Deriv_naive.make_sum], so the revision lives here under the
    plainer name and the first stage keeps its own module. *)
module Deriv = struct
  type expr = Deriv_naive.expr =
    | Const of int
    | Var of string
    | Sum of expr * expr
    | Prod of expr * expr

  let is_variable = Deriv_naive.is_variable
  let same_variable = Deriv_naive.same_variable
  let is_sum = Deriv_naive.is_sum
  let is_product = Deriv_naive.is_product
  let addend = Deriv_naive.addend
  let augend = Deriv_naive.augend
  let multiplier = Deriv_naive.multiplier
  let multiplicand = Deriv_naive.multiplicand

  (** [is_number exp n] holds when [exp] is the constant [n]. *)
  let is_number exp n =
    match exp with
    | Const m -> m = n
    | _ -> false
  ;;

  let make_sum a1 a2 =
    if is_number a1 0
    then a2
    else if is_number a2 0
    then a1
    else (
      match a1, a2 with
      | Const m, Const n -> Const (m + n)
      | _ -> Sum (a1, a2))
  ;;

  let make_product m1 m2 =
    if is_number m1 0 || is_number m2 0
    then Const 0
    else if is_number m1 1
    then m2
    else if is_number m2 1
    then m1
    else (
      match m1, m2 with
      | Const m, Const n -> Const (m * n)
      | _ -> Prod (m1, m2))
  ;;

  let rec deriv exp var =
    match exp with
    | Const _ -> Const 0
    | Var _ -> if same_variable exp (Var var) then Const 1 else Const 0
    | Sum (a1, a2) -> make_sum (deriv a1 var) (deriv a2 var)
    | Prod (m1, m2) ->
      make_sum (make_product m1 (deriv m2 var)) (make_product (deriv m1 var) m2)
  ;;
end

(** Sets, subsection 2.3.3. The signature names the four operations
    that define a set; each representation below supplies the
    operations the running text has developed so far, and a module
    earns the signature when Exercises 2.59, 2.61, 2.62, and 2.65
    supply the operations still missing. *)
module type Set = sig
  (** The representation type: a set is whatever this module says it is. *)
  type t

  val element_of_set : int -> t -> bool
  val adjoin_set : int -> t -> t
  val union_set : t -> t -> t
  val intersection_set : t -> t -> t
end

(** Sets as unordered lists. *)
module Unordered_list_set = struct
  type t = int list

  let rec element_of_set x = function
    | [] -> false
    | first :: rest -> first = x || element_of_set x rest
  ;;

  let adjoin_set x set = if element_of_set x set then set else x :: set

  let rec intersection_set set1 set2 =
    match set1 with
    | [] -> []
    | first :: rest ->
      if element_of_set first set2
      then first :: intersection_set rest set2
      else intersection_set rest set2
  ;;
end

(** Sets as ordered lists. The elements are numbers, so [<] settles
    each comparison; [element_of_set] stops at the first larger
    element, and [intersection_set] advances whichever set carries the
    smaller first element. *)
module Ordered_list_set = struct
  type t = int list

  let rec element_of_set x = function
    | [] -> false
    | first :: _ when x = first -> true
    | first :: _ when x < first -> false
    | _ :: rest -> element_of_set x rest
  ;;

  let rec intersection_set set1 set2 =
    match set1, set2 with
    | [], _ | _, [] -> []
    | first1 :: rest1, first2 :: rest2 ->
      if first1 = first2
      then first1 :: intersection_set rest1 rest2
      else if first1 < first2
      then intersection_set rest1 set2
      else intersection_set set1 rest2
  ;;
end

(** Sets as binary trees. The book builds trees from lists whose empty
    list plays the empty subtree; here the variant's [Empty]
    constructor plays that part, so a tree is never mistaken for an
    element list and no selector can run off the end. [union_set] and
    [intersection_set] wait for Exercises 2.65's list-round-trip
    implementations. *)
module Tree_set = struct
  type tree =
    | Empty
    | Node of tree * int * tree

  type t = tree

  let entry = function
    | Node (_, e, _) -> e
    | Empty -> invalid_arg "entry: empty tree"
  ;;

  let left_branch = function
    | Node (l, _, _) -> l
    | Empty -> invalid_arg "left_branch: empty tree"
  ;;

  let right_branch = function
    | Node (_, _, r) -> r
    | Empty -> invalid_arg "right_branch: empty tree"
  ;;

  let make_tree entry left right = Node (left, entry, right)

  let rec element_of_set x = function
    | Empty -> false
    | Node (left, e, right) ->
      if x = e
      then true
      else if x < e
      then element_of_set x left
      else element_of_set x right
  ;;

  let rec adjoin_set x = function
    | Empty -> Node (Empty, x, Empty)
    | Node (left, e, right) as set ->
      if x = e
      then set
      else if x < e
      then Node (adjoin_set x left, e, right)
      else Node (left, e, adjoin_set x right)
  ;;
end

(** Sets and information retrieval. A record carries its key next to
    whatever else the record holds; [lookup] scans an unordered
    record list for the one whose key matches. *)
module Record_db = struct
  type record =
    { key : int
    ; name : string
    }

  let rec lookup given_key = function
    | [] -> None
    | first :: rest -> if given_key = first.key then Some first else lookup given_key rest
  ;;
end

(** Huffman encoding trees, subsection 2.3.4. A bit is the [Zero] or
    [One] constructor rather than a number, so the book's complaint
    about a bit that is neither has no case left to match. A tree is a
    [Leaf] holding a symbol and its weight or a [Node] holding two
    branches, the symbols below it, and their total weight. The
    dispatching selectors [symbols] and [weight] are the book's
    @newterm{generic procedures}: they do one of two things depending
    on the kind of node they meet. *)
module Huffman = struct
  type bit =
    | Zero
    | One

  type tree =
    | Leaf of string * int
    | Node of tree * tree * string list * int

  let make_leaf symbol weight = Leaf (symbol, weight)

  let symbols = function
    | Leaf (s, _) -> [ s ]
    | Node (_, _, ss, _) -> ss
  ;;

  let weight = function
    | Leaf (_, w) -> w
    | Node (_, _, _, w) -> w
  ;;

  let make_code_tree left right =
    Node (left, right, symbols left @ symbols right, weight left + weight right)
  ;;

  let left_branch = function
    | Node (l, _, _, _) -> l
    | Leaf _ -> invalid_arg "left_branch: a leaf has no branches"
  ;;

  let right_branch = function
    | Node (_, r, _, _) -> r
    | Leaf _ -> invalid_arg "right_branch: a leaf has no branches"
  ;;

  let choose_branch bit branch =
    match bit with
    | Zero -> left_branch branch
    | One -> right_branch branch
  ;;

  let decode bits tree =
    let rec decode_1 bits current_branch =
      match bits with
      | [] -> []
      | bit :: rest ->
        (match choose_branch bit current_branch with
         | Leaf (symbol, _) -> symbol :: decode_1 rest tree
         | next_branch -> decode_1 rest next_branch)
    in
    decode_1 bits tree
  ;;

  let rec adjoin_set x = function
    | [] -> [ x ]
    | first :: rest as set ->
      if weight x < weight first then x :: set else first :: adjoin_set x rest
  ;;

  let rec make_leaf_set pairs =
    match pairs with
    | [] -> []
    | (symbol, frequency) :: rest ->
      adjoin_set (make_leaf symbol frequency) (make_leaf_set rest)
  ;;

  (** The tree of @ref{Figure 2.18}: designed for messages where A has
      relative frequency 8, B has 3, and every other letter has 1. *)
  let sample_tree =
    make_code_tree
      (make_leaf "A" 8)
      (make_code_tree
         (make_code_tree
            (make_leaf "B" 3)
            (make_code_tree (make_leaf "C" 1) (make_leaf "D" 1)))
         (make_code_tree
            (make_code_tree (make_leaf "E" 1) (make_leaf "F" 1))
            (make_code_tree (make_leaf "G" 1) (make_leaf "H" 1))))
  ;;
end
