use std::{cell::RefCell, rc::Rc};

use crate::event::Event;
use crate::event_parser::{CompiledTimeExpression, ExprContextAll};

#[derive(Clone, Debug)]
pub struct Command {
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug)]
pub struct Transition {
    pub from: Rc<RefCell<StateAspect>>,
    pub to: Rc<RefCell<StateAspect>>,
    pub condition_str: String, // Original string for serialization/debugging
    pub condition: Rc<ExprContextAll<'static>>, // Parsed expression for evaluation
    pub action: Option<Command>,
}

#[derive(Debug)]
pub struct StateAspect {
    state_name: String,
    description: String,
    activation_conditions: Vec<String>, // Store condition strings instead of compiled expressions
    parent_state: Option<String>,
    associated_transitions: Vec<Transition>,
}

impl StateAspect {
    pub fn new(
        state_name: String,
        description: String,
        activation_conditions: Vec<String>,
        parent_state: Option<String>,
        associated_transitions: Vec<Transition>,
    ) -> Self {
        Self {
            state_name,
            description,
            activation_conditions,
            parent_state,
            associated_transitions,
        }
    }

    pub fn state_name(&self) -> &str {
        &self.state_name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn activation_conditions(&self) -> &Vec<String> {
        &self.activation_conditions
    }
    pub fn parent_state(&self) -> Option<&String> {
        self.parent_state.as_ref()
    }
    pub fn associated_transitions(&self) -> &Vec<Transition> {
        &self.associated_transitions
    }
    pub fn add_transition(&mut self, transition: Transition) {
        self.associated_transitions.push(transition);
    }
}

#[derive(Debug)]
pub struct StateMachine {
    pub current_states: Rc<RefCell<Vec<StateAspect>>>, // list of current states
    pub all_states: Rc<RefCell<Vec<Rc<RefCell<StateAspect>>>>>,
    pub all_transitions: Rc<RefCell<Vec<Transition>>>,
}

// Implementation for Transition with safe parsing
impl Transition {
    /// Creates a new Transition with parsed condition
    ///
    /// # Arguments
    /// * `from` - Source state
    /// * `to` - Target state
    /// * `condition_str` - The condition string to parse
    /// * `action` - Optional command to execute
    ///
    /// # Returns
    /// * `Ok(Transition)` - Successfully created transition
    /// * `Err(String)` - Parsing error message
    pub fn new(
        from: Rc<RefCell<StateAspect>>,
        to: Rc<RefCell<StateAspect>>,
        condition_str: String,
        action: Option<Command>,
    ) -> Result<Self, String> {
        // Parse the condition string once at construction
        let parsed = crate::event_parser::parse_expression(&condition_str)
            .map_err(|e| format!("Failed to parse condition '{}': {}", condition_str, e))?;

        // Convert to 'static lifetime - this is safe because we own the string
        // and the parsed expression references the string data
        let parsed_static = unsafe {
            std::mem::transmute::<
                Rc<crate::event_parser::ExprContextAll<'_>>,
                Rc<crate::event_parser::ExprContextAll<'static>>,
            >(parsed)
        };

        Ok(Self {
            from,
            to,
            condition_str,
            condition: parsed_static,
            action,
        })
    }

    /// Creates a Transition from a string condition
    pub fn from_string(
        from: Rc<RefCell<StateAspect>>,
        to: Rc<RefCell<StateAspect>>,
        condition_str: &str,
        action: Option<Command>,
    ) -> Result<Self, String> {
        Self::new(from, to, condition_str.to_string(), action)
    }

    /// Evaluates the condition using the pre-parsed expression
    pub fn evaluate_condition(
        &self,
        context: crate::event_parser::FilterContext,
        current_event: &Event,
    ) -> Result<bool, String> {
        let evaluator = crate::event_parser::FilterEvaluator::new(context);
        evaluator.evaluate(&self.condition, current_event)
    }

    /// Gets the original condition string
    pub fn get_condition_string(&self) -> &str {
        &self.condition_str
    }

    /// Gets the parsed condition expression
    pub fn get_condition_expression(&self) -> &Rc<crate::event_parser::ExprContextAll<'static>> {
        &self.condition
    }
}

// Implementation for StateMachine
impl StateMachine {
    /// Creates a new empty state machine
    pub fn new() -> Self {
        Self {
            current_states: Rc::new(RefCell::new(Vec::new())),
            all_states: Rc::new(RefCell::new(Vec::new())),
            all_transitions: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Adds a transition to the state machine
    ///
    /// # Arguments
    /// * `from_state_name` - Name of the source state
    /// * `to_state_name` - Name of the target state
    /// * `condition_str` - Condition string to parse
    /// * `action` - Optional command to execute
    ///
    /// # Returns
    /// * `Ok(())` - Successfully added transition
    /// * `Err(String)` - Error message (parsing error or state not found)
    pub fn add_transition(
        &self,
        from_state_name: &str,
        to_state_name: &str,
        condition_str: &str,
        action: Option<Command>,
    ) -> Result<(), String> {
        // Find the source and target states
        let all_states = self.all_states.borrow();

        let from_state = all_states
            .iter()
            .find(|state| state.borrow().state_name() == from_state_name)
            .ok_or_else(|| format!("Source state '{}' not found", from_state_name))?;

        let to_state = all_states
            .iter()
            .find(|state| state.borrow().state_name() == to_state_name)
            .ok_or_else(|| format!("Target state '{}' not found", to_state_name))?;

        // Create the transition with parsed condition
        let transition = Transition::from_string(
            Rc::clone(from_state),
            Rc::clone(to_state),
            condition_str,
            action,
        )?;

        // Add to transitions list
        self.all_transitions.borrow_mut().push(transition);

        Ok(())
    }

    /// Evaluates all transitions for a given event
    ///
    /// # Arguments
    /// * `context` - Filter context with events
    /// * `current_event` - Current event to evaluate against
    ///
    /// # Returns
    /// * `Vec<Transition>` - List of transitions whose conditions are true
    pub fn evaluate_transitions(
        &self,
        context: crate::event_parser::FilterContext,
        current_event: &Event,
    ) -> Vec<Transition> {
        let transitions = self.all_transitions.borrow();
        transitions
            .iter()
            .filter(|transition| {
                transition
                    .evaluate_condition(context.clone(), current_event)
                    .unwrap_or(false)
            })
            .cloned()
            .collect()
    }

    /// Gets all transitions
    pub fn get_transitions(&self) -> Vec<Transition> {
        self.all_transitions.borrow().iter().cloned().collect()
    }

    /// Gets transitions from a specific state
    pub fn get_transitions_from(&self, state_name: &str) -> Vec<Transition> {
        self.all_transitions
            .borrow()
            .iter()
            .filter(|transition| transition.from.borrow().state_name() == state_name)
            .cloned()
            .collect()
    }

    pub fn get_all_compiled_conditions_for_states(
        &self,
    ) -> Vec<(String, Rc<CompiledTimeExpression>)> {
        let mut conditions = Vec::new();
        let all_states = self.all_states.borrow();

        for state in all_states.iter() {
            log::debug!("State: {:?}", state.borrow().state_name());
            let state_borrow = state.borrow();
            let state_name = state_borrow.state_name().to_string();

            // Parse condition strings on demand
            for condition_str in state_borrow.activation_conditions().iter() {
                log::debug!("Condition: {:?}", condition_str);

                // Parse the condition string
                match crate::event_parser::parse_expression(condition_str) {
                    Ok(_parsed_expr) => {
                        // Convert to 'static lifetime safely by cloning the string data
                        let condition_str_static = condition_str.clone();
                        let condition_str_static = Box::leak(condition_str_static.into_boxed_str());

                        // Re-parse with static string
                        match crate::event_parser::parse_expression(condition_str_static) {
                            Ok(static_expr) => {
                                let compiled_condition = CompiledTimeExpression {
                                    expression: static_expr,
                                };
                                conditions.push((state_name.clone(), Rc::new(compiled_condition)));
                            }
                            Err(e) => {
                                log::error!("Failed to parse condition '{}': {}", condition_str, e);
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to parse condition '{}': {}", condition_str, e);
                    }
                }
            }
        }
        conditions
    }

    /// Collect all event names and wildcard usage from all transition conditions
    pub fn get_all_needed_events(&self) -> (std::collections::HashSet<String>, bool) {
        use crate::event_parser::collect_events_from_expr;
        let mut all_events = std::collections::HashSet::new();
        let mut has_wildcard = false;
        for transition in self.all_transitions.borrow().iter() {
            let (events, wildcard) = collect_events_from_expr(&transition.condition);
            all_events.extend(events);
            if wildcard {
                has_wildcard = true;
            }
        }
        (all_events, has_wildcard)
    }

    pub fn get_state_by_name(&self, state_name: &str) -> Option<Rc<RefCell<StateAspect>>> {
        let all_states = self.all_states.borrow();
        all_states
            .iter()
            .find(|state| state.borrow().state_name() == state_name)
            .cloned()
    }
}
