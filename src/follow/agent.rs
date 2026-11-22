use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    state_description::StateDescription,
    state_machine::StateMachine,
    event::{Event, EventValue},
    event_filter_executor::Events,
};

pub struct StateAgent {
    pub id: String,
    pub events: Rc<RefCell<Events>>,
    pub state_machine: StateMachine,
    current_states: Vec<String>,
}

impl StateAgent {
    /// Follow one subject. `id` is the caller's identifier for that subject.
    pub fn new(
        id: String,
        description: StateDescription,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let state_machine = description.to_state_machine()?;
        let events = Rc::new(RefCell::new(Events::new()));

        Ok(Self {
            id,
            events,
            state_machine,
            current_states: Vec::new(),
        })
    }

    pub fn get_current_states(&self) -> HashSet<String> {
        self.current_states.iter().cloned().collect()
    }

    // return new state, and state changes events
    pub fn event_arrived_so_evaluate_states(
        &mut self,
        event: &Event,
    ) -> Result<(Vec<String>, Vec<Event>), Box<dyn std::error::Error>> {
        log::debug!("Event arrived: {:?}", event);
        let mut events = self.events.borrow_mut();
        events.push_event(event.clone());
        drop(events);

        let old_current_states = self.current_states.clone();
        self.current_states.clear();

        // evaluate the states, and retain current ones
        // list all the conditions that are true on the existing states
        // in the state machine
        let all_compiled_conditions = self
            .state_machine
            .get_all_compiled_conditions_for_states();
        for condition in all_compiled_conditions {
            log::debug!("Evaluating condition: {:?}", condition.1.expression);
            let evaluated_state_name = condition.0;
            let compiled_condition = condition.1;

            let result = compiled_condition.evaluate(&self.events, event);
            log::debug!("Result: {:?}", result);

            if result.is_ok() && result.clone().unwrap() {
                log::debug!("Adding state: {:?}", evaluated_state_name);
                self.current_states.push(evaluated_state_name.clone());
            } else {
                log::debug!("Not adding state: {:?}", evaluated_state_name);
            }
        }

        // making difference between old and new states, to be able to generate new events on states that are new, or deactivated
        let new_states = self.current_states.clone();

        let mut states_changes_events = vec![];
        for state in &new_states {
            if !old_current_states.contains(state) {
                states_changes_events.push(Event {
                    name: "state_activated".into(),
                    is_state: false,
                    attributes: {
                        let mut attributes = HashMap::new();
                        attributes.insert("state".to_string(), EventValue::String(state.clone()));
                        attributes
                    },
                    timestamp: event.timestamp,
                });
            }
        }
        // unactivates state
        for state in &old_current_states {
            if !new_states.contains(state) {
                states_changes_events.push(Event {
                    name: "state_deactivated".into(),
                    is_state: false,
                    attributes: {
                        let mut attributes = HashMap::new();
                        attributes.insert("state".to_string(), EventValue::String(state.clone()));
                        attributes
                    },
                    timestamp: event.timestamp,
                });
            }
        }

        Ok((new_states, states_changes_events))
    }
}
