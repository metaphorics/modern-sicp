// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.78: the query language as a
//! nondeterministic program. The evaluator is the 4.3 mechanism
//! re-skinned for frames: a simple query pushes one choice frame whose
//! alternatives are the data-base matches and the rule applications,
//! `and` threads continuations, `or` pushes its branches as
//! alternatives, and a dead end returns the typed backtrack signal to
//! the driver, which hands answers over one `try-again` at a time.
//! Much of the stream mechanism is indeed subsumed -- the matcher, the
//! unifier, and the data base are this section's own -- and the
//! behavioral difference the exercise asks for is pinned: the amb `or`
//! is depth-first where the stream engine interleaves.

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;

use ch04::sec_4_1::is_true;
use ch04::sec_4_4::{
    Engine, Frame, conclusion, contract_question_mark, instantiate, microshaft, pattern_match,
    query_syntax_process, rename_variables_in, rule_body, split_list, unify_match,
};
use sicp_runtime::{SchemeError, Value, print_value};

mod ex_4_78 {
    //! Exercise 4.78: the query language on an explicit backtracking
    //! engine.

    use super::*;

    /// The rest of one query's evaluation: called with the extended
    /// frame; `Err(Backtrack)` is this branch's failure.
    pub type K = Rc<dyn Fn(&AmbQuery, Frame) -> Result<(), SchemeError>>;

    /// One runnable alternative.
    type Alt = Box<dyn FnOnce(&AmbQuery) -> Result<(), SchemeError>>;

    /// One choice frame: the alternatives not yet tried.
    struct Level {
        alts: VecDeque<Alt>,
    }

    /// The engine: the data base, the choice stack, the answers recorded
    /// so far, and the rule-application counter.
    pub struct AmbQuery {
        db: Engine,
        stack: RefCell<Vec<Level>>,
        answers: RefCell<Vec<String>>,
        counter: Cell<u64>,
    }

    impl AmbQuery {
        /// One engine over `db`'s data base.
        #[must_use]
        pub fn new(db: &Engine) -> Self {
            Self {
                db: Rc::clone(db),
                stack: RefCell::new(Vec::new()),
                answers: RefCell::new(Vec::new()),
                counter: Cell::new(0),
            }
        }

        /// Schedules `query` from the empty frame and drives the search
        /// `n` answers deep, answering the instantiated patterns.
        pub fn answers_upto(&self, query: &str, n: usize) -> Vec<String> {
            let processed = query_syntax_process(&sicp_runtime::read(query).expect("parses"));
            let sink_pattern = processed.clone();
            let sink: K = Rc::new(move |amb, frame| {
                let printed =
                    print_value(&instantiate(&sink_pattern, &frame, &contract_question_mark));
                amb.answers.borrow_mut().push(printed);
                Ok(())
            });
            self.schedule(classify(&processed), Frame::new(), sink)
                .expect("the top-level query schedules");
            (0..n).filter_map(|_| self.drive()).collect()
        }

        /// The driver: run the deepest choice frame's next alternative;
        /// a recorded answer returns with the frame's remaining
        /// alternatives suspended on the stack, a dead end falls through
        /// to the frame below.
        fn drive(&self) -> Option<String> {
            loop {
                let alt = {
                    let mut stack = self.stack.borrow_mut();
                    stack.last_mut()?.alts.pop_front()
                };
                match alt {
                    None => {
                        self.stack.borrow_mut().pop();
                    }
                    Some(alt) => {
                        let before = self.answers.borrow().len();
                        match alt(self) {
                            // Only a level run that RECORDED an answer
                            // yields; a bare `schedule` (a pushed
                            // choice frame) keeps the search going.
                            Ok(()) => {
                                let answers = self.answers.borrow();
                                if answers.len() > before {
                                    return answers.last().cloned();
                                }
                            }
                            Err(SchemeError::Backtrack) => {}
                            Err(error) => panic!("the search raised: {error}"),
                        }
                    }
                }
            }
        }

        /// Schedules one query goal: pushes a choice frame for a simple
        /// query or an `or`, threads continuations for the rest.
        fn schedule(&self, goal: Goal, frame: Frame, k: K) -> Result<(), SchemeError> {
            match goal {
                Goal::AlwaysTrue => k(self, frame),
                Goal::Not(inner) => {
                    if self.probe(&inner, &frame) {
                        Err(SchemeError::Backtrack)
                    } else {
                        k(self, frame)
                    }
                }
                Goal::LispValue(call) => {
                    let instantiated = instantiate(&call, &frame, &contract_question_mark);
                    if self.db.execute(&instantiated).is_ok_and(|v| is_true(&v)) {
                        k(self, frame)
                    } else {
                        Err(SchemeError::Backtrack)
                    }
                }
                Goal::And { first, rest } => {
                    let k2: K = if rest.is_nil() {
                        k
                    } else {
                        let rest2 = rest.clone();
                        let k_outer = Rc::clone(&k);
                        Rc::new(move |amb, extended| {
                            let (next, remaining) = split_list(&rest2);
                            amb.schedule(
                                Goal::And {
                                    first: next,
                                    rest: remaining,
                                },
                                extended,
                                Rc::clone(&k_outer),
                            )
                        })
                    };
                    self.schedule(classify(&first), frame, k2)
                }
                Goal::Or { first, rest } => {
                    let mut alts: VecDeque<Alt> = VecDeque::new();
                    let first2 = first.clone();
                    let frame2 = frame.clone();
                    let k2 = Rc::clone(&k);
                    alts.push_back(Box::new(move |amb| {
                        amb.schedule(classify(&first2), frame2, Rc::clone(&k2))
                    }));
                    if !rest.is_nil() {
                        let rest2 = rest.clone();
                        let k3 = Rc::clone(&k);
                        let frame3 = frame.clone();
                        alts.push_back(Box::new(move |amb| {
                            let (next, remaining) = split_list(&rest2);
                            amb.schedule(
                                Goal::Or {
                                    first: next,
                                    rest: remaining,
                                },
                                frame3,
                                Rc::clone(&k3),
                            )
                        }));
                    }
                    self.stack.borrow_mut().push(Level { alts });
                    Ok(())
                }
                Goal::Simple(pattern) => {
                    let mut alts: VecDeque<Alt> = VecDeque::new();
                    let k2 = Rc::clone(&k);
                    for matched in self.assertion_frames(&pattern, &frame) {
                        let k3 = Rc::clone(&k2);
                        alts.push_back(Box::new(move |amb| k3(amb, matched)));
                    }
                    let rules: Vec<Value> = self.db.fetch_rules(&pattern).iter().collect();
                    for rule in rules {
                        let k3 = Rc::clone(&k2);
                        let pattern2 = pattern.clone();
                        let frame2 = frame.clone();
                        alts.push_back(Box::new(move |amb| {
                            let id = amb.counter.get() + 1;
                            amb.counter.set(id);
                            let clean = rename_variables_in(&rule, id);
                            let Some(extended) =
                                unify_match(&pattern2, &conclusion(&clean), &frame2)
                            else {
                                return Err(SchemeError::Backtrack);
                            };
                            amb.schedule(classify(&rule_body(&clean)), extended, k3)
                        }));
                    }
                    self.stack.borrow_mut().push(Level { alts });
                    Ok(())
                }
            }
        }

        /// The extended frames the assertions contribute.
        fn assertion_frames(&self, pattern: &Value, frame: &Frame) -> Vec<Frame> {
            let assertions = self.db.fetch_assertions(pattern);
            let mut out = Vec::new();
            for assertion in &assertions {
                if let Some(extended) = pattern_match(pattern, &assertion, frame) {
                    out.push(extended);
                }
            }
            out
        }

        /// Whether the query holds in the frame, under a private stack.
        fn probe(&self, query: &Value, frame: &Frame) -> bool {
            let probe = Self::new(&self.db);
            let sink: K = Rc::new(|_amb, _frame| Ok(()));
            probe
                .schedule(classify(query), frame.clone(), sink)
                .expect("the probe schedules");
            probe.drive().is_some()
        }
    }

    /// The dispatch the stream engine's `qeval` performs.
    fn classify(query: &Value) -> Goal {
        let Value::Pair(cell) = query else {
            return Goal::Simple(query.clone());
        };
        let Value::Sym(tag) = &*cell.car.borrow() else {
            return Goal::Simple(query.clone());
        };
        let contents = cell.cdr.borrow().clone();
        match &**tag {
            "and" => {
                let (first, rest) = split_list(&contents);
                Goal::And { first, rest }
            }
            "or" => {
                let (first, rest) = split_list(&contents);
                Goal::Or { first, rest }
            }
            "not" => Goal::Not(first_of(&contents)),
            "lisp-value" => Goal::LispValue(first_of(&contents)),
            "always-true" => Goal::AlwaysTrue,
            _ => Goal::Simple(query.clone()),
        }
    }

    fn first_of(exps: &Value) -> Value {
        split_list(exps).0
    }

    /// One query goal, the shape `qeval` dispatches on.
    enum Goal {
        Simple(Value),
        And { first: Value, rest: Value },
        Or { first: Value, rest: Value },
        Not(Value),
        LispValue(Value),
        AlwaysTrue,
    }
}

#[test]
fn ex_4_78() {
    let engine = microshaft();
    let amb = ex_4_78::AmbQuery::new(&engine);

    // The book's `or` example. Depth-first: the first disjunct's whole
    // subtree before the second's -- Hacker, Fect, Tweakit, Reasoner --
    // where the stream engine interleaves: Hacker, Reasoner, Fect,
    // Tweakit.
    assert_eq!(
        amb.answers_upto(
            "(or (supervisor ?x (Bitdiddle Ben)) (supervisor ?x (Hacker Alyssa P)))",
            4
        ),
        [
            "(or (supervisor (Hacker Alyssa P) (Bitdiddle Ben)) (supervisor (Hacker Alyssa P) (Hacker Alyssa P)))",
            "(or (supervisor (Fect Cy D) (Bitdiddle Ben)) (supervisor (Fect Cy D) (Hacker Alyssa P)))",
            "(or (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (supervisor (Tweakit Lem E) (Hacker Alyssa P)))",
            "(or (supervisor (Reasoner Louis) (Bitdiddle Ben)) (supervisor (Reasoner Louis) (Hacker Alyssa P)))",
        ]
    );

    // The stream engine's interleaved order over the same query.
    let streamed = microshaft();
    assert_eq!(
        streamed.answers("(or (supervisor ?x (Bitdiddle Ben)) (supervisor ?x (Hacker Alyssa P)))"),
        [
            "(or (supervisor (Hacker Alyssa P) (Bitdiddle Ben)) (supervisor (Hacker Alyssa P) (Hacker Alyssa P)))",
            "(or (supervisor (Reasoner Louis) (Bitdiddle Ben)) (supervisor (Reasoner Louis) (Hacker Alyssa P)))",
            "(or (supervisor (Fect Cy D) (Bitdiddle Ben)) (supervisor (Fect Cy D) (Hacker Alyssa P)))",
            "(or (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (supervisor (Tweakit Lem E) (Hacker Alyssa P)))",
        ]
    );

    // Single answers with `try-again`: the married cycle under the amb
    // engine answers Minnie, and every further derivation route lands
    // on the same pair again.
    let married = microshaft();
    married.load(&[
        "(married Minnie Mickey)",
        "(rule (married ?x ?y) (married ?y ?x))",
    ]);
    let amb_married = ex_4_78::AmbQuery::new(&married);
    assert_eq!(
        amb_married.answers_upto("(married Mickey ?who)", 3),
        [
            "(married Mickey Minnie)",
            "(married Mickey Minnie)",
            "(married Mickey Minnie)",
        ]
    );
}
