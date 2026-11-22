use std::collections::{HashMap, HashSet};

use crate::{state_description::StateDescription, event::{Event, EventValue}};

use super::*;

#[test]
fn test_state_agent() {
    let _ = env_logger::try_init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let description = StateDescription::from_json(
        r#"
        {
            "states": {
                "state1": {
                    "facts": {
                        "condition": "temperature#[value > 20]"
                    }
                },
                "state2": {
                    "facts": {
                        "condition": "temperature#[value < 20] OR temperature#[value > 30]"
                    }
                }
                
            }
        }"#,
    );

    let mut agent =
        StateAgent::new("subject-1".to_string(), description.unwrap()).unwrap();

    let state1 = agent
        .state_machine
        .get_state_by_name("state1")
        .unwrap();
    assert_eq!(state1.borrow().state_name(), "state1");
    assert_eq!(state1.borrow().activation_conditions().len(), 1);
    let state2 = agent
        .state_machine
        .get_state_by_name("state2")
        .unwrap();
    assert_eq!(state2.borrow().state_name(), "state2");
    assert_eq!(state2.borrow().activation_conditions().len(), 1);

    let event = Event {
        name: "temperature".into(),
        is_state: false,
        attributes: {
            let mut attributes = HashMap::new();
            attributes.insert("value".to_string(), EventValue::Float(40.0));
            attributes
        },
        timestamp: 1000,
    };

    let (_new_states, states_changes_events) = agent
        .event_arrived_so_evaluate_states(&event)
        .unwrap();

    assert_eq!(
        agent.get_current_states(),
        HashSet::from(["state1".to_string(), "state2".to_string()])
    );

    log::debug!("states_changes_events: {:?}", states_changes_events);

    // send event with lower temperature
    let event = Event {
        name: "temperature".into(),
        is_state: false,
        attributes: {
            let mut attributes = HashMap::new();
            attributes.insert("value".to_string(), EventValue::Float(10.0));
            attributes
        },
        timestamp: 1001,
    };

    let (_new_states, states_changes_events) = agent
        .event_arrived_so_evaluate_states(&event)
        .unwrap();

    // state1 stays active: earlier temperature@1000 still matches value > 20.
    // state2 is active from temperature@1001 matching value < 20.
    assert_eq!(
        agent.get_current_states(),
        HashSet::from(["state1".to_string(), "state2".to_string()])
    );

    log::debug!("states_changes_events: {:?}", states_changes_events);
}

#[test]
fn test_state_agent_nested() {
    let _ = env_logger::try_init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let description = StateDescription::from_json(
        r#"
        {
            "states": {
                "state1": {
                    "facts": {
                        "condition": "temperature#[value > 20]"
                    },
                    "states": {
                        "state11": {
                            "facts": {
                                "condition": "temperature#[value > 20]"
                            }
                        }
                    }
                },
                "state2": {
                    "facts": {
                        "condition": "temperature#[value < 20] OR temperature#[value > 30]"
                    }
                }
                
            }
        }"#,
    );

    let mut agent =
        StateAgent::new("subject-1".to_string(), description.unwrap()).unwrap();

    let state1 = agent
        .state_machine
        .get_state_by_name("state1")
        .unwrap();
    assert_eq!(state1.borrow().state_name(), "state1");
    assert_eq!(state1.borrow().activation_conditions().len(), 1);
    let state11 = agent
        .state_machine
        .get_state_by_name("state1.state11")
        .unwrap();
    assert_eq!(state11.borrow().state_name(), "state1.state11");
    assert_eq!(state11.borrow().activation_conditions().len(), 1);
    let state2 = agent
        .state_machine
        .get_state_by_name("state2")
        .unwrap();
    assert_eq!(state2.borrow().state_name(), "state2");
    assert_eq!(state2.borrow().activation_conditions().len(), 1);

    let event = Event {
        name: "temperature".into(),
        is_state: false,
        attributes: {
            let mut attributes = HashMap::new();
            attributes.insert("value".to_string(), EventValue::Float(40.0));
            attributes
        },
        timestamp: 1000,
    };

    agent
        .event_arrived_so_evaluate_states(&event)
        .unwrap();

    assert_eq!(
        agent.get_current_states(),
        HashSet::from([
            "state1".to_string(),
            "state1.state11".to_string(),
            "state2".to_string()
        ])
    );
}
