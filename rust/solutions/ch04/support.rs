// SPDX-License-Identifier: GPL-3.0-only
//
// Shared typed support for the chapter-4 reference solutions. Checked
// Rust programs go through the frozen admission API. The environment
// and constructor helpers below are small domain values for the
// evaluator and query lessons; they do not duplicate an engine.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use ch04::sec_4_1::{admit, run, run_analyzed};
use ch04::sec_4_4::{Query, Rule, Substitution, Term};
use sicp_runtime::SicpError;
use sicp_runtime::host::check::CheckedProgram;
use sicp_runtime::host::diag::Diag;
use sicp_runtime::host::ops::RunOutcome;

/// Admits one checked Rust program through the frozen source front end.
///
/// # Errors
/// The admission [`Diag`] when the source is outside the subset.
pub fn checked(source: &str) -> Result<CheckedProgram, Diag> {
    admit(source)
}

/// Runs one checked Rust source text on the direct evaluator.
///
/// # Errors
/// The admission [`Diag`] when the source is rejected before effects.
pub fn direct(source: &str) -> Result<RunOutcome, Diag> {
    checked(source).map(|program| run(&program))
}

/// Runs one checked Rust source text on the analyzer.
///
/// # Errors
/// The admission [`Diag`] when the source is rejected before effects.
pub fn analyzed(source: &str) -> Result<RunOutcome, Diag> {
    checked(source).map(|program| run_analyzed(&program))
}

/// Returns the direct and analyzed stdout transcripts for one source.
///
/// # Errors
/// The admission [`Diag`] when the source is rejected before effects.
pub fn both(source: &str) -> Result<(String, String), Diag> {
    let program = checked(source)?;
    Ok((run(&program).stdout, run_analyzed(&program).stdout))
}

/// Returns the direct stdout transcript for one source.
///
/// # Errors
/// The admission [`Diag`] when the source is rejected before effects.
pub fn stdout(source: &str) -> Result<String, Diag> {
    direct(source).map(|outcome| outcome.stdout)
}

/// The small value domain the environment lessons bind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingValue {
    /// An integer value.
    Int(i64),
    /// A Boolean value.
    Bool(bool),
    /// A text value.
    Text(String),
    /// A symbolic value.
    Symbol(String),
    /// The scan-out placeholder read before initialization.
    Unassigned,
}

impl fmt::Display for BindingValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(value) => write!(f, "{value}"),
            Self::Bool(value) => write!(f, "{}", if *value { "true" } else { "false" }),
            Self::Text(value) => write!(f, "\"{value}\""),
            Self::Symbol(value) => f.write_str(value),
            Self::Unassigned => f.write_str("*unassigned*"),
        }
    }
}

/// The environment's typed error, using the edition's published error.
pub type EnvError = SicpError;

/// A lexical environment frame chain.
#[derive(Debug, Clone)]
pub struct Env(Rc<RefCell<Frame>>);

#[derive(Debug)]
struct Frame {
    bindings: HashMap<String, BindingValue>,
    parent: Option<Env>,
}

impl Default for Env {
    fn default() -> Self {
        Self::root()
    }
}

impl Env {
    /// Builds an empty root environment.
    #[must_use]
    pub fn root() -> Self {
        Self(Rc::new(RefCell::new(Frame {
            bindings: HashMap::new(),
            parent: None,
        })))
    }

    /// Builds a child frame whose parent is this environment.
    #[must_use]
    pub fn child(&self) -> Self {
        Self(Rc::new(RefCell::new(Frame {
            bindings: HashMap::new(),
            parent: Some(self.clone()),
        })))
    }

    /// Defines a name in the current frame.
    pub fn define(&self, name: impl Into<String>, value: BindingValue) {
        self.0.borrow_mut().bindings.insert(name.into(), value);
    }

    /// Looks up a name by walking outward.
    #[must_use]
    pub fn lookup(&self, name: &str) -> Option<BindingValue> {
        let mut current = self.clone();
        loop {
            let found = current.0.borrow().bindings.get(name).cloned();
            if found.is_some() {
                return found;
            }
            let parent = current.0.borrow().parent.clone()?;
            current = parent;
        }
    }

    /// Assigns an existing name, preserving its owning frame.
    ///
    /// # Errors
    /// [`SicpError::UnboundVariable`] when no frame binds the name.
    pub fn assign(&self, name: &str, value: BindingValue) -> Result<(), EnvError> {
        let mut current = self.clone();
        loop {
            let mut frame = current.0.borrow_mut();
            if frame.bindings.contains_key(name) {
                frame.bindings.insert(name.to_owned(), value);
                return Ok(());
            }
            drop(frame);
            let Some(parent) = current.0.borrow().parent.clone() else {
                return Err(SicpError::UnboundVariable(name.to_owned()));
            };
            current = parent;
        }
    }

    /// Removes a binding from the current frame only.
    ///
    /// # Errors
    /// [`SicpError::TypeMismatch`] when the current frame lacks the name.
    pub fn unbind(&self, name: &str) -> Result<(), EnvError> {
        self.0.borrow_mut().bindings.remove(name).map_or_else(
            || {
                Err(SicpError::TypeMismatch(format!(
                    "not bound in the current frame: {name}"
                )))
            },
            |_| Ok(()),
        )
    }

    /// Returns the current frame's bindings sorted for stable tests.
    #[must_use]
    pub fn frame_bindings(&self) -> Vec<(String, BindingValue)> {
        let mut entries: Vec<_> = self
            .0
            .borrow()
            .bindings
            .iter()
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect();
        entries.sort_by(|left, right| left.0.cmp(&right.0));
        entries
    }

    /// Returns the number of frames from this environment to the root.
    #[must_use]
    pub fn depth(&self) -> usize {
        let mut depth = 1;
        let mut current = self.clone();
        loop {
            let parent = current.0.borrow().parent.clone();
            let Some(parent) = parent else {
                break;
            };
            depth += 1;
            current = parent;
        }
        depth
    }
}

/// Builds a query variable term.
#[must_use]
pub fn var(name: &str) -> Term {
    Term::Variable(name.to_owned())
}

/// Builds an integer term.
#[must_use]
pub fn int(value: i64) -> Term {
    Term::Integer(value)
}

/// Builds an atom term.
#[must_use]
pub fn atom(name: &str) -> Term {
    Term::Atom(name.to_owned())
}

/// Builds a text term.
#[must_use]
pub fn text(value: &str) -> Term {
    Term::Text(value.to_owned())
}

/// Builds a pair term.
#[must_use]
pub fn pair(left: Term, right: Term) -> Term {
    Term::Pair(Box::new(left), Box::new(right))
}

/// Builds a right-associated list term ending in `Empty`.
#[must_use]
pub fn list(items: Vec<Term>) -> Term {
    items
        .into_iter()
        .rev()
        .fold(Term::Empty, |tail, item| pair(item, tail))
}

/// Builds a relation query.
#[must_use]
pub fn relation(name: &str, arguments: Vec<Term>) -> Query {
    Query::Relation {
        name: name.to_owned(),
        arguments,
    }
}

/// Builds an asserted relation fact: the head atom followed by its
/// arguments, matching the `Database` assertion convention the query
/// engine unifies `Query::Relation` against.
#[must_use]
pub fn fact(name: &str, arguments: Vec<Term>) -> Term {
    pair(atom(name), list(arguments))
}

/// Builds a conjunction query.
#[must_use]
pub fn and(queries: Vec<Query>) -> Query {
    Query::And(queries)
}

/// Builds a disjunction query.
#[must_use]
pub fn or(queries: Vec<Query>) -> Query {
    Query::Or(queries)
}

/// Builds a negation query.
#[must_use]
pub fn not(query: Query) -> Query {
    Query::Not(Box::new(query))
}

/// Builds a unique query.
#[must_use]
pub fn unique(query: Query) -> Query {
    Query::Unique(Box::new(query))
}

/// Builds a rule from its conclusion and ordered conditions.
#[must_use]
pub fn rule(conclusion: Term, conditions: Vec<Query>) -> Rule {
    Rule {
        conclusion,
        conditions,
    }
}

/// Returns one substitution binding as a stable rendering, following
/// variable-to-variable aliases (rule application renames apart, so a
/// query variable typically answers through its renamed counterpart).
#[must_use]
pub fn answer_text(substitution: &Substitution, name: &str) -> String {
    let mut seen = Vec::new();
    let mut current = substitution.get(name);
    while let Some(Term::Variable(alias)) = current {
        if seen.iter().any(|known| known == alias) {
            break;
        }
        seen.push(alias.clone());
        current = substitution.get(alias);
    }
    current.map_or_else(|| "<unbound>".to_owned(), term_text)
}

/// Renders one query term in the solutions' stable notation.
#[must_use]
pub fn term_text(term: &Term) -> String {
    match term {
        Term::Variable(name) => format!("?{name}"),
        Term::Integer(value) => value.to_string(),
        Term::Text(value) => format!("\"{value}\""),
        Term::Atom(value) => value.clone(),
        Term::Empty => "()".to_owned(),
        Term::Pair(left, right) => {
            let mut items = vec![term_text(left)];
            let mut tail = right.as_ref();
            while let Term::Pair(next_left, next_right) = tail {
                items.push(term_text(next_left));
                tail = next_right.as_ref();
            }
            if !matches!(tail, Term::Empty) {
                items.push(format!(". {}", term_text(tail)));
            }
            format!("({})", items.join(" "))
        }
    }
}
