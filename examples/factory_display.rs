//! Follow one physical display through the traces it leaves.
//!
//! ```text
//! cargo run -p event_statemachine_follow --example factory_display
//! ```
//!
//! Timestamps are seconds. `since N hours from now` uses the arriving event
//! as "now", so a later clock reading can age a trace out of a state.

use std::collections::HashMap;
use std::rc::Rc;

use event_statemachine_follow::follow::StateAgent;
use event_statemachine_follow::state_description::StateDescription;
use event_statemachine_follow::state_machine::StateMachine;
use event_statemachine_follow::event::{Event, EventValue};
use event_statemachine_follow::event_parser::FilterContext;

const HOUR: u64 = 3_600;
const T0: u64 = 200_000;

const DESCRIPTION: &str = r##"{
  "states": {
    "not_operational": {
      "description": "The display is not in service",
      "states": {
        "unconnected": {
          "description": "No enrollment or cloud connection in the last 2 hours",
          "facts": {
            "condition": "NOT (enrollment since 2 hours from now OR connection_to_cloud since 2 hours from now)"
          }
        },
        "un_enrolled": {
          "description": "No enrollment id has been created",
          "facts": {
            "condition": "NOT enrollment_id_created"
          }
        }
      },
      "transitions": [
        {
          "name": "enroll_device_transition",
          "from": "not_operational.unconnected",
          "to": "not_operational.un_enrolled",
          "condition": "user_validation_on_this_device_status since 1 minute from now",
          "action": {
            "name": "enroll_device",
            "description": "Enroll the display"
          }
        }
      ]
    },
    "operational": {
      "description": "The display is in service",
      "states": {
        "operating": {
          "description": "A successful connection trace in the last 2 hours",
          "facts": {
            "condition": "ConnectionTraceFromDevice#[wifi == \"factory_display_001\" AND status == true] since 2 hours from now"
          }
        },
        "hot": {
          "description": "A temperature reading above 30 in the last hour",
          "facts": {
            "condition": "temperature#[value > 30] since 1 hour from now"
          }
        }
      }
    }
  }
}"##;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let description = StateDescription::from_json(DESCRIPTION)?;
    println!("factory_display_001");
    println!("status sentences:");
    let mut leaves: Vec<_> = description
        .get_leaf_states()
        .into_iter()
        .map(|(name, _)| name)
        .collect();
    leaves.sort();
    for name in &leaves {
        if let Some(condition) = description.get_state_condition(name) {
            println!("  {name}");
            println!("    {condition}");
        }
    }
    println!();

    // Keep this description alive: transition conditions are parsed against its strings.
    let actions = description.to_state_machine()?;
    let mut agent = StateAgent::new("factory_display_001".into(), description.clone())?;

    let steps = [
        ("first look, no traces yet", clock(T0)),
        (
            "a technician validates the display",
            plain("user_validation_on_this_device_status", T0 + 10),
        ),
        (
            "enrollment id is created (the validation minute has passed)",
            plain("enrollment_id_created", T0 + 120),
        ),
        ("the display enrolls", plain("enrollment", T0 + 130)),
        (
            "two hours of silence",
            clock(T0 + 130 + 2 * HOUR + 1),
        ),
        (
            "the display reaches the cloud",
            plain("connection_to_cloud", T0 + 130 + 2 * HOUR + 11),
        ),
        (
            "connection trace for this display",
            attrs(
                "ConnectionTraceFromDevice",
                [
                    ("wifi", EventValue::String("factory_display_001".into())),
                    ("status", EventValue::Boolean(true)),
                ],
                T0 + 130 + 2 * HOUR + 31,
            ),
        ),
        (
            "temperature 40",
            attrs(
                "temperature",
                [("value", EventValue::Float(40.0))],
                T0 + 130 + 2 * HOUR + 51,
            ),
        ),
        (
            "one hour after the hot reading",
            clock(T0 + 130 + 2 * HOUR + 51 + HOUR + 1),
        ),
        (
            "two hours after the connection trace",
            clock(T0 + 130 + 2 * HOUR + 31 + 2 * HOUR + 1),
        ),
    ];

    for (label, event) in steps {
        follow(&mut agent, &actions, label, &event)?;
    }

    Ok(())
}

fn follow(
    agent: &mut StateAgent,
    actions: &StateMachine,
    label: &str,
    event: &Event,
) -> Result<(), Box<dyn std::error::Error>> {
    let (active, changes) = agent.event_arrived_so_evaluate_states(event)?;
    let mut active = active;
    active.sort();

    println!("{label}");
    println!("  event {} @ {}", event.name, event.timestamp);
    if active.is_empty() {
        println!("  now: (none)");
    } else {
        println!("  now: {}", active.join(", "));
    }
    for change in &changes {
        let mark = if change.name == "state_activated" {
            "+"
        } else {
            "-"
        };
        println!("  {mark} {}", state_name(change));
    }

    let ready = actions.evaluate_transitions(
        FilterContext::new(Rc::clone(&agent.events)),
        event,
    );
    for transition in ready {
        let action = transition
            .action
            .as_ref()
            .map(|command| command.name.as_str())
            .unwrap_or("none");
        println!(
            "  action ready: {action} ({} → {})",
            transition.from.borrow().state_name(),
            transition.to.borrow().state_name()
        );
    }
    println!();
    Ok(())
}

fn state_name(event: &Event) -> &str {
    match event.attributes.get("state") {
        Some(EventValue::String(name)) => name,
        _ => "?",
    }
}

fn clock(timestamp: u64) -> Event {
    plain("clock", timestamp)
}

fn plain(name: &str, timestamp: u64) -> Event {
    Event::new(name.into(), false, HashMap::new(), timestamp)
}

fn attrs<const N: usize>(name: &str, fields: [(&str, EventValue); N], timestamp: u64) -> Event {
    Event::new(
        name.into(),
        false,
        fields
            .into_iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect(),
        timestamp,
    )
}
