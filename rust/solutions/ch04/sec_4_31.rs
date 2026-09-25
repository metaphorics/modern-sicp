// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.31: `lazy` and `lazy-memo`
//! parameter declarations as object-language syntax. Parameter
//! specifications `(name lazy)` and `(name lazy-memo)` extend the
//! grammar of procedure declarations; the strictness of each parameter
//! lives beside the evaluator, keyed by the procedure's identity (the
//! 4.1.7 bodies-table shape, since the shared `Closure` has no slot for
//! it), and the application clause binds per parameter: strict
//! operands evaluate now, `lazy` operands delay without memoization,
//! `lazy-memo` operands delay and memoize.

use ch04::eval_support::*;
use sicp_runtime::ThunkState;

mod ex_4_31 {
    use super::*;

    /// The strictness a declared parameter asks for.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ParamMode {
        /// Evaluated when the procedure is called (the default).
        Strict,
        /// Delayed; every forcing re-evaluates.
        Lazy,
        /// Delayed and memoized.
        LazyMemo,
    }

    /// The upward-compatible evaluator: undeclared procedures stay
    /// strict, declared parameters follow their modes.
    #[derive(Debug, Default)]
    pub struct LazyDeclared {
        modes: RefCell<HashMap<usize, Rc<[ParamMode]>>>,
    }

    impl LazyDeclared {
        /// A fresh evaluator with an empty mode table.
        #[must_use]
        pub fn new() -> Self {
            Self::default()
        }

        /// Builds the procedure value of a declaration with declared
        /// parameter specifications, registering the modes under the
        /// closure's identity.
        ///
        /// # Errors
        /// [`SchemeError::TypeMismatch`] on a malformed specification.
        pub fn declared_closure(
            &self,
            name: Option<Symbol>,
            specs: &[Value],
            body: &[Value],
            env: &Rc<Env>,
        ) -> EvalResult {
            let mut params = Vec::with_capacity(specs.len());
            let mut modes = Vec::with_capacity(specs.len());
            for spec in specs {
                let (param, mode) = parse_spec(spec)?;
                params.push(param);
                modes.push(mode);
            }
            let closure = Rc::new(Closure {
                name,
                params,
                rest: None,
                body: body.to_vec(),
                env: Rc::clone(env),
            });
            self.modes
                .borrow_mut()
                .insert(Rc::as_ptr(&closure) as usize, modes.into());
            Ok(Value::Closure(closure))
        }
    }

    /// Parses one parameter specification: a bare name is strict, and
    /// `(name lazy)` or `(name lazy-memo)` declares its mode.
    ///
    /// # Errors
    /// [`SchemeError::TypeMismatch`] on an unknown declaration.
    pub fn parse_spec(spec: &Value) -> Result<(Symbol, ParamMode), SchemeError> {
        let Value::Sym(name) = spec else {
            let items = spec.list_items()?;
            let Some(Value::Sym(name)) = items.first() else {
                return Err(SchemeError::TypeMismatch(format!(
                    "not a declared parameter: {spec}"
                )));
            };
            let mode = match items.get(1) {
                None => ParamMode::Strict,
                Some(Value::Sym(tag)) if &**tag == "lazy" => ParamMode::Lazy,
                Some(Value::Sym(tag)) if &**tag == "lazy-memo" => ParamMode::LazyMemo,
                Some(other) => {
                    return Err(SchemeError::TypeMismatch(format!(
                        "unknown parameter declaration: {other}"
                    )));
                }
            };
            return Ok((name.clone(), mode));
        };
        Ok((name.clone(), ParamMode::Strict))
    }

    impl LazyEval for LazyDeclared {
        fn force_value(&self, value: Value) -> EvalResult {
            let mut current = value;
            while let Some(cell) = Self::pending_cell(&current) {
                current = self.force_cell(&current, cell)?;
            }
            Ok(current)
        }

        fn delay_operand(&self, proc: &Rc<Closure>, position: usize) -> Option<bool> {
            let modes = self
                .modes
                .borrow()
                .get(&(Rc::as_ptr(proc) as usize))
                .cloned();
            match modes {
                None => None,
                Some(modes) => match modes.get(position).copied().unwrap_or(ParamMode::Strict) {
                    ParamMode::Strict => None,
                    ParamMode::Lazy => Some(false),
                    ParamMode::LazyMemo => Some(true),
                },
            }
        }
    }

    impl LazyDeclared {
        /// The thunk cell a declared forcing evaluates next: the cell
        /// inside a `lazy` wrapper recomputes, a `lazy-memo` thunk
        /// memoizes, and anything else is already in hand.
        fn pending_cell(value: &Value) -> Option<Rc<RefCell<ThunkState>>> {
            if is_recomputing_thunk(value) {
                if let Value::Thunk(cell) = wrapper_payload(value) {
                    return Some(Rc::clone(cell));
                }
                return None;
            }
            if let Value::Thunk(cell) = value {
                return Some(Rc::clone(cell));
            }
            None
        }

        /// Forces one pending cell: a `lazy` wrapper's cell recomputes
        /// in place, a plain thunk memoizes through the runtime cell.
        ///
        /// # Errors
        /// Whatever the thunk's expression raises.
        fn force_cell(&self, value: &Value, cell: Rc<RefCell<ThunkState>>) -> EvalResult {
            if is_recomputing_thunk(value) {
                return recompute_cell(self, &cell);
            }
            force_memo(self, Value::Thunk(cell))
        }
    }

    impl Evaluator for LazyDeclared {
        fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
            if is_lambda(exp) {
                let items = exp.list_items()?;
                let Some(specs) = items.get(1).cloned() else {
                    return Err(SchemeError::TypeMismatch(format!(
                        "malformed lambda: {exp}"
                    )));
                };
                let specs = specs.list_items()?;
                let body: Vec<Value> = items.iter().skip(2).cloned().collect();
                return self
                    .declared_closure(None, &specs, &body, env)
                    .map(Step::Done);
            }
            if is_definition(exp) {
                return Ok(Step::Done(self.declared_definition(exp, env)?));
            }
            lazy_step(self, exp, env)
        }
    }

    impl LazyDeclared {
        /// The book's `eval-definition` with declared specifications:
        /// a variable define evaluates as usual, a procedure define
        /// parses its parameter modes before building the closure.
        ///
        /// # Errors
        /// [`SchemeError::TypeMismatch`] on a malformed define; whatever
        /// the value expression raises.
        pub fn declared_definition(&self, exp: &Value, env: &Rc<Env>) -> EvalResult {
            let name = definition_variable(exp)?;
            let Value::Sym(name) = name else {
                return Err(SchemeError::TypeMismatch(format!(
                    "not a definition target: {name}"
                )));
            };
            let items = exp.list_items()?;
            let second = items.get(1).cloned().unwrap_or(Value::Nil);
            if is_variable(&second) {
                let value = self.eval(&items.get(2).cloned().unwrap_or(Value::Nil), env)?;
                return define_variable_(&name, value, env);
            }
            let head = rest_of(&second)?;
            let specs = head.list_items()?;
            let body: Vec<Value> = items.into_iter().skip(2).collect();
            let value = self.declared_closure(Some(name.clone()), &specs, &body, env)?;
            define_variable_(&name, value, env)
        }
    }

    /// The book's session, with the declared procedures of parts (a)
    /// and the probes the map's idea cell describes.
    const SESSION: &[&str] = &[
        "(define count 0)",
        "(define (id n) (set! count (+ count 1)) n)",
        "(define (pick (x lazy)) 'taken)",
        "(pick (car '()))",
        "(define (f a (b lazy) c (d lazy-memo)) (list a b b c d d))",
        "(f (id 1) (id (+ 2 3)) (id 4) (id (* 5 6)))",
        "count",
        "(define (g (x lazy)) (list x x))",
        "(g (id 10))",
        "count",
        "(define (h (x lazy-memo)) (list x x))",
        "(h (id 10))",
        "count",
    ];

    /// The printed answers of the session, in order.
    pub fn answers() -> Vec<String> {
        let transcript = lazy_driver_transcript(&LazyDeclared::new(), SESSION);
        transcript
            .lines()
            .filter_map(|line| line.strip_prefix(";;; L-Eval value: "))
            .map(str::to_owned)
            .collect()
    }
}

#[test]
fn ex_4_31() {
    let lines = ex_4_31::answers();
    // The lazy parameter is never demanded: `(car '())` stays a thunk
    // and `taken` comes back where the strict reading would raise.
    assert_eq!(&lines[3..4], &["taken"]);
    // Strict `a` and `c` evaluate at the call (two id runs), `lazy`
    // `b` recomputes at each of its two demands (two runs), and
    // `lazy-memo` `d` computes at its first of two demands (one run):
    // five runs, values (1 5 5 4 30 30).
    assert_eq!(&lines[5..7], &["(1 5 5 4 30 30)", "5"]);
    // `g`'s `lazy` parameter demanded twice: two runs (count 7 - 5).
    assert_eq!(&lines[8..10], &["(10 10)", "7"]);
    // `h`'s `lazy-memo` parameter demanded twice: one run (count 8),
    // the memo serves the second demand.
    assert_eq!(&lines[11..], &["(10 10)", "8"]);
}
