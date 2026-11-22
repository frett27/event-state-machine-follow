use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::state_machine::{StateMachine, StateAspect, Transition, Command};

use crate::event_parser::parse_expression;
use std::rc::Rc;
use std::cell::RefCell;

/// Source location information for JSON elements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

/// Decorated version of ActivationRule with source location
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivationRuleWithLocation {
    pub condition: String,
    pub source_location: SourceLocation,
}

/// Decorated version of TransitionDef with source location
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransitionDefWithLocation {
    pub name: String,
    pub from: String,
    pub to: String,
    pub condition: String,
    pub action: ActionDef,
    pub source_location: SourceLocation,
}

/// Decorated version of State with source location
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateWithLocation {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(rename = "facts")]
    pub activation: Option<ActivationRuleWithLocation>,
    #[serde(default)]
    pub states: Option<HashMap<String, StateWithLocation>>,
    #[serde(default)]
    pub transitions: Option<Vec<TransitionDefWithLocation>>,
    pub source_location: SourceLocation,
}

/// Decorated version of StateDescription with source location
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateDescriptionWithLocation {
    pub states: HashMap<String, StateWithLocation>,
    pub source_location: SourceLocation,
}

/// Memoized parser that tracks source locations
#[derive(Clone)]
pub struct StateParser {
    source_text: String,
    memoized_locations: HashMap<String, SourceLocation>,
}

impl StateParser {
    /// Create a new parser with source text
    pub fn new(source_text: String) -> Self {
        Self {
            source_text,
            memoized_locations: HashMap::new(),
        }
    }

    /// Parse JSON with source location tracking
    pub fn parse_with_locations(&mut self) -> Result<StateDescriptionWithLocation, serde_json::Error> {
        // First parse as regular JSON to validate structure
        let regular: StateDescription = serde_json::from_str(&self.source_text)?;
        
        // Then build the decorated version with estimated locations
        let decorated = self.build_decorated_with_locations(&regular)?;
        
        // Memoize locations for quick lookup
        self.memoize_locations(&decorated);
        
        Ok(decorated)
    }

    /// Parse JSON and convert to regular StateDescription
    pub fn parse(&mut self) -> Result<StateDescription, serde_json::Error> {
        serde_json::from_str(&self.source_text)
    }

    /// Get source location for a specific element
    pub fn get_location(&self, element_path: &str) -> Option<&SourceLocation> {
        self.memoized_locations.get(element_path)
    }

    /// Get the source text
    pub fn get_source_text(&self) -> &str {
        &self.source_text
    }

    /// Build decorated structure with estimated locations
    fn build_decorated_with_locations(&self, regular: &StateDescription) -> Result<StateDescriptionWithLocation, serde_json::Error> {
        let mut decorated_states = HashMap::new();
        
        for (state_name, state) in &regular.states {
            let decorated_state = self.build_state_with_location(state_name, state, 0)?;
            decorated_states.insert(state_name.clone(), decorated_state);
        }
        
        Ok(StateDescriptionWithLocation {
            states: decorated_states,
            source_location: SourceLocation { line: 1, column: 1, length: self.source_text.len() },
        })
    }

    /// Build state with location information
    fn build_state_with_location(&self, state_name: &str, state: &State, depth: usize) -> Result<StateWithLocation, serde_json::Error> {
        // Estimate location based on state name position in source
        let location = self.find_state_location(state_name, depth);
        
        let mut decorated_child_states = None;
        if let Some(ref child_states) = state.states {
            let mut child_map = HashMap::new();
            for (child_name, child_state) in child_states {
                let decorated_child = self.build_state_with_location(child_name, child_state, depth + 1)?;
                child_map.insert(child_name.clone(), decorated_child);
            }
            decorated_child_states = Some(child_map);
        }
        
        let decorated_activation = if let Some(ref activation) = state.activation {
            Some(ActivationRuleWithLocation {
                condition: activation.condition.clone(),
                source_location: self.find_facts_location(state_name),
            })
        } else {
            None
        };
        
        let decorated_transitions = if let Some(ref transitions) = state.transitions {
            let mut decorated_trans = Vec::new();
            for (i, transition) in transitions.iter().enumerate() {
                decorated_trans.push(TransitionDefWithLocation {
                    name: transition.name.clone(),
                    from: transition.from.clone(),
                    to: transition.to.clone(),
                    condition: transition.condition.clone(),
                    action: transition.action.clone(),
                    source_location: self.find_transition_location(state_name, i),
                });
            }
            Some(decorated_trans)
        } else {
            None
        };
        
        Ok(StateWithLocation {
            description: state.description.clone(),
            activation: decorated_activation,
            states: decorated_child_states,
            transitions: decorated_transitions,
            source_location: location,
        })
    }

    /// Find state location in source text
    fn find_state_location(&self, state_name: &str, depth: usize) -> SourceLocation {
        // Simple heuristic: find the state name in the source text
        if let Some(pos) = self.source_text.find(&format!("\"{}\"", state_name)) {
            let before_text = &self.source_text[..pos];
            let line = before_text.lines().count() + 1;
            let column = before_text.lines().last().map(|l| l.len()).unwrap_or(0) + 1;
            SourceLocation {
                line,
                column,
                length: state_name.len() + 2, // +2 for quotes
            }
        } else {
            // Fallback location
            SourceLocation {
                line: 1 + depth,
                column: 1,
                length: state_name.len(),
            }
        }
    }

    /// Find facts location in source text
    fn find_facts_location(&self, state_name: &str) -> SourceLocation {
        // Find "facts" keyword after the state name
        if let Some(state_pos) = self.source_text.find(&format!("\"{}\"", state_name)) {
            if let Some(facts_pos) = self.source_text[state_pos..].find("\"facts\"") {
                let pos = state_pos + facts_pos;
                let before_text = &self.source_text[..pos];
                let line = before_text.lines().count() + 1;
                let column = before_text.lines().last().map(|l| l.len()).unwrap_or(0) + 1;
                SourceLocation {
                    line,
                    column,
                    length: 7, // "facts".len()
                }
            } else {
                SourceLocation { line: 1, column: 1, length: 7 }
            }
        } else {
            SourceLocation { line: 1, column: 1, length: 7 }
        }
    }

    /// Find transition location in source text
    fn find_transition_location(&self, state_name: &str, transition_index: usize) -> SourceLocation {
        // Find "transitions" keyword after the state name
        if let Some(state_pos) = self.source_text.find(&format!("\"{}\"", state_name)) {
            if let Some(transitions_pos) = self.source_text[state_pos..].find("\"transitions\"") {
                let pos = state_pos + transitions_pos;
                let before_text = &self.source_text[..pos];
                let line = before_text.lines().count() + 1;
                let column = before_text.lines().last().map(|l| l.len()).unwrap_or(0) + 1;
                SourceLocation {
                    line,
                    column,
                    length: 12, // "transitions".len()
                }
            } else {
                SourceLocation { line: 1, column: 1, length: 12 }
            }
        } else {
            SourceLocation { line: 1, column: 1, length: 12 }
        }
    }

    /// Memoize locations for quick lookup
    fn memoize_locations(&mut self, decorated: &StateDescriptionWithLocation) {
        self.memoized_locations.clear();
        
        // Memoize root location
        self.memoized_locations.insert("root".to_string(), decorated.source_location.clone());
        
        // Recursively memoize all state locations
        self.memoize_state_locations("", &decorated.states);
    }

    /// Recursively memoize state locations
    fn memoize_state_locations(&mut self, prefix: &str, states: &HashMap<String, StateWithLocation>) {
        for (state_name, state) in states {
            let full_path = if prefix.is_empty() {
                state_name.clone()
            } else {
                format!("{}.{}", prefix, state_name)
            };
            
            // Memoize state location
            self.memoized_locations.insert(full_path.clone(), state.source_location.clone());
            
            // Memoize activation (JSON "facts") location if present
            if let Some(ref activation) = state.activation {
                let facts_path = format!("{}.facts", full_path);
                self.memoized_locations.insert(facts_path, activation.source_location.clone());
            }
            
            // Memoize transitions if present
            if let Some(ref transitions) = state.transitions {
                for (i, transition) in transitions.iter().enumerate() {
                    let transition_path = format!("{}.transitions[{}]", full_path, i);
                    self.memoized_locations.insert(transition_path, transition.source_location.clone());
                }
            }
            
            // Recursively process child states
            if let Some(ref child_states) = state.states {
                self.memoize_state_locations(&full_path, child_states);
            }
        }
    }
}

// Conversion from decorated to regular structures
impl From<StateDescriptionWithLocation> for StateDescription {
    fn from(decorated: StateDescriptionWithLocation) -> Self {
        Self {
            states: decorated.states.into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl From<StateWithLocation> for State {
    fn from(decorated: StateWithLocation) -> Self {
        Self {
            description: decorated.description,
            activation: decorated.activation.map(|f| ActivationRule { condition: f.condition }),
            states: decorated.states.map(|s| s.into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect()),
            transitions: decorated.transitions.map(|t| t.into_iter()
                .map(|t| TransitionDef {
                    name: t.name,
                    from: t.from,
                    to: t.to,
                    condition: t.condition,
                    action: t.action,
                })
                .collect()),
        }
    }
}

// Original structures (kept clean without source location)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateDescription {
    pub states: HashMap<String, State>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    #[serde(default)]
    pub description: Option<String>,
    /// Activation rule for this state. Wire format keeps the historical JSON key `"facts"`.
    #[serde(default, rename = "facts")]
    pub activation: Option<ActivationRule>,
    #[serde(default)]
    pub states: Option<HashMap<String, State>>,
    #[serde(default)]
    pub transitions: Option<Vec<TransitionDef>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivationRule {
    pub condition: String,
}

/// Backward-compatible alias for the historical JSON `"facts"` activation rule type.
pub type Facts = ActivationRule;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransitionDef {
    pub name: String,
    pub from: String,
    pub to: String,
    pub condition: String,
    pub action: ActionDef,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionDef {
    pub name: String,
    pub description: String,
}

impl StateDescription {
    /// Parse a JSON string into a StateDescription
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Convert the StateParser back to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Generate a StateMachine from the parsed JSON
    /// 
    /// This method converts the parsed JSON structure into a runtime StateMachine
    /// that can be used for state evaluation and transitions.
    /// 
    /// # Returns
    /// * `Ok(StateMachine)` - Successfully created state machine
    /// * `Err(String)` - Error message (parsing error or invalid structure)
    pub fn to_state_machine(&self) -> Result<StateMachine, String> {
        let mut all_states = Vec::new();
        let mut all_transitions = Vec::new();
        let mut state_map = HashMap::new(); // Map state names to Rc<RefCell<StateAspect>>

        // Helper function to recursively process states and create StateAspects
        fn process_states(
            states: &HashMap<String, State>,
            parent_name: Option<String>,
            all_states: &mut Vec<Rc<RefCell<StateAspect>>>,
            state_map: &mut HashMap<String, Rc<RefCell<StateAspect>>>,
        ) -> Result<(), String> {
            for (state_name, state) in states {
                let full_state_name = if let Some(ref parent) = parent_name {
                    format!("{}.{}", parent, state_name)
                } else {
                    state_name.clone()
                };

                // Parse activation conditions if they exist
                let mut activation_conditions = Vec::new();
                if let Some(ref activation) = state.activation {
                    // Store the condition string directly instead of compiling it
                    activation_conditions.push(activation.condition.clone());
                }

                // Create StateAspect
                let aspect = Rc::new(RefCell::new(StateAspect::new(
                    full_state_name.clone(),
                    state.description.clone().unwrap_or_default(),
                    activation_conditions,
                    parent_name.clone(),
                    Vec::new(),
                )));

                // Store in map for later reference
                state_map.insert(full_state_name.clone(), aspect.clone());
                all_states.push(aspect.clone());

                // Recursively process child states
                if let Some(ref child_states) = state.states {
                    process_states(child_states, Some(full_state_name), all_states, state_map)?;
                }
            }
            Ok(())
        }

        // Process all states to create StateAspects
        process_states(&self.states, None, &mut all_states, &mut state_map)?;

        // Helper function to create transitions from TransitionDef
        fn create_transitions_from_defs(
            transitions: &[TransitionDef],
            state_map: &HashMap<String, Rc<RefCell<StateAspect>>>,
            all_transitions: &mut Vec<Transition>,
        ) -> Result<(), String> {
            for transition_def in transitions {
                // Find the source and target states
                let from_state = state_map.get(&transition_def.from)
                    .ok_or_else(|| format!("Source state '{}' not found", transition_def.from))?;
                
                let to_state = state_map.get(&transition_def.to)
                    .ok_or_else(|| format!("Target state '{}' not found", transition_def.to))?;

                // Parse the condition expression
                let parsed_condition = parse_expression(&transition_def.condition)
                    .map_err(|e| format!("Failed to parse condition for transition '{}': {}", transition_def.name, e))?;

                // Convert to 'static lifetime
                let parsed_static = unsafe {
                    std::mem::transmute::<Rc<crate::event_parser::ExprContextAll<'_>>, Rc<crate::event_parser::ExprContextAll<'static>>>(parsed_condition)
                };

                // Create Command from ActionDef
                let command = Command {
                    name: transition_def.action.name.clone(),
                    description: transition_def.action.description.clone(),
                };

                // Create transition
                let transition = Transition {
                    from: from_state.clone(),
                    to: to_state.clone(),
                    condition_str: transition_def.condition.clone(),
                    condition: parsed_static,
                    action: Some(command),
                };

                all_transitions.push(transition);
            }
            Ok(())
        }

        // Process transitions from all state groups
        fn process_transitions_recursive(
            states: &HashMap<String, State>,
            state_map: &HashMap<String, Rc<RefCell<StateAspect>>>,
            all_transitions: &mut Vec<Transition>,
        ) -> Result<(), String> {
            for (_, state) in states {
                // Process transitions at this level
                if let Some(ref transitions) = state.transitions {
                    create_transitions_from_defs(transitions, state_map, all_transitions)?;
                }

                // Recursively process child states
                if let Some(ref child_states) = state.states {
                    process_transitions_recursive(child_states, state_map, all_transitions)?;
                }
            }
            Ok(())
        }

        // Process all transitions
        process_transitions_recursive(&self.states, &state_map, &mut all_transitions)?;

        Ok(StateMachine {
            current_states: Rc::new(RefCell::new(Vec::new())),
            all_states: Rc::new(RefCell::new(all_states.into_iter().map(|rc| Rc::clone(&rc)).collect())),
            all_transitions: Rc::new(RefCell::new(all_transitions)),
        })
    }

    /// Get all state names from the state machine (including nested states)
    pub fn get_all_state_names(&self) -> Vec<String> {
        let mut state_names = Vec::new();
        self.collect_state_names("", &self.states, &mut state_names);
        state_names
    }

    /// Recursively collect state names with their full path
    fn collect_state_names(&self, prefix: &str, states: &HashMap<String, State>, result: &mut Vec<String>) {
        for (state_name, state) in states {
            let full_name = if prefix.is_empty() {
                state_name.clone()
            } else {
                format!("{}.{}", prefix, state_name)
            };
            
            // Add this state if it has an activation rule (is a leaf state)
            if state.activation.is_some() {
                result.push(full_name.clone());
            }
            
            // Recursively process child states
            if let Some(ref child_states) = state.states {
                self.collect_state_names(&full_name, child_states, result);
            }
        }
    }

    /// Get a specific state by its full path (e.g., "not_operational.unconnected")
    pub fn get_state(&self, state_path: &str) -> Option<&State> {
        let parts: Vec<&str> = state_path.split('.').collect();
        if parts.is_empty() {
            return None;
        }
        
        let mut current_states = &self.states;
        let mut current_state: Option<&State> = None;
        
        for (i, part) in parts.iter().enumerate() {
            if let Some(state) = current_states.get(*part) {
                if i == parts.len() - 1 {
                    // This is the final state we're looking for
                    current_state = Some(state);
                } else {
                    // Continue to child states
                    if let Some(ref child_states) = state.states {
                        current_states = child_states;
                    } else {
                        return None; // No child states available
                    }
                }
            } else {
                return None; // State not found
            }
        }
        
        current_state
    }

    /// Get the condition for a specific state
    pub fn get_state_condition(&self, state_path: &str) -> Option<&str> {
        self.get_state(state_path)
            .and_then(|state| state.activation.as_ref())
            .map(|activation| activation.condition.as_str())
    }

    /// Get all leaf states (states that have activation conditions)
    pub fn get_leaf_states(&self) -> Vec<(String, &State)> {
        let mut leaf_states = Vec::new();
        self.collect_leaf_states("", &self.states, &mut leaf_states);
        leaf_states
    }

    /// Recursively collect leaf states (states with activation rules)
    fn collect_leaf_states<'a>(&'a self, prefix: &str, states: &'a HashMap<String, State>, result: &mut Vec<(String, &'a State)>) {
        for (state_name, state) in states {
            let full_name = if prefix.is_empty() {
                state_name.clone()
            } else {
                format!("{}.{}", prefix, state_name)
            };
            
            // If this state has an activation rule, it's a leaf state
            if state.activation.is_some() {
                result.push((full_name.clone(), state));
            }
            
            // Recursively process child states
            if let Some(ref child_states) = state.states {
                self.collect_leaf_states(&full_name, child_states, result);
            }
        }
    }

    /// Get all state groups (non-leaf states that contain other states)
    pub fn get_state_groups(&self) -> Vec<(String, &State)> {
        let mut state_groups = Vec::new();
        self.collect_state_groups("", &self.states, &mut state_groups);
        state_groups
    }

    /// Recursively collect state groups (states that have child states)
    fn collect_state_groups<'a>(&'a self, prefix: &str, states: &'a HashMap<String, State>, result: &mut Vec<(String, &'a State)>) {
        for (state_name, state) in states {
            let full_name = if prefix.is_empty() {
                state_name.clone()
            } else {
                format!("{}.{}", prefix, state_name)
            };
            
            // If this state has child states, it's a state group
            if state.states.is_some() {
                result.push((full_name.clone(), state));
            }
            
            // Recursively process child states
            if let Some(ref child_states) = state.states {
                self.collect_state_groups(&full_name, child_states, result);
            }
        }
    }

    /// Recursively collect all event names and wildcard usage from facts and transitions at all levels
    pub fn collect_all_event_names_and_wildcards(&self) -> (std::collections::HashSet<String>, bool) {
        use crate::event_parser::{collect_events_from_expr, parse_expression};
        use std::collections::HashSet;
        let mut all_events = HashSet::new();
        let mut has_wildcard = false;

        fn walk_states(
            states: &std::collections::HashMap<String, State>,
            all_events: &mut HashSet<String>,
            has_wildcard: &mut bool,
        ) {
            for (_state_name, state) in states {
                // Collect from activation rules (state condition)
                if let Some(ref activation) = state.activation {
                    if let Ok(expr) = parse_expression(&activation.condition) {
                        let (events, wildcard) = collect_events_from_expr(&expr);
                        all_events.extend(events);
                        if wildcard {
                            *has_wildcard = true;
                        }
                    }
                }
                // Collect from transitions at this state
                if let Some(ref transitions) = state.transitions {
                    for transition in transitions {
                        if let Ok(expr) = parse_expression(&transition.condition) {
                            let (events, wildcard) = collect_events_from_expr(&expr);
                            all_events.extend(events);
                            if wildcard {
                                *has_wildcard = true;
                            }
                        }
                    }
                }
                // Recurse into substates
                if let Some(ref substates) = state.states {
                    walk_states(substates, all_events, has_wildcard);
                }
            }
        }

        walk_states(&self.states, &mut all_events, &mut has_wildcard);
        (all_events, has_wildcard)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_state_description() {
        let json = r#"{
            "states": {
                "not_operational": {
                    "states": {
                        "unconnected": {
                            "description": "The device is not connected to the network",
                            "facts": {
                                "condition": "NOT (enrollment since 72 hours from now OR connection_to_cloud since 72 hours from now)"
                            }
                        },
                        "un_enrolled": {
                            "description": "The device is not enrolled in the system",
                            "facts": {
                                "condition": "NOT enrollment_id_created"
                            }
                        },
                        "unconnected_enrolled": {
                            "facts": {
                                "condition": "enrollment_id_created AND NOT (connection_to_cloud since 24 hours from now)"
                            }
                        }
                    }
                },
                "operational": {
                    "states": {
                        "operating": {
                            "description": "The device is enrolled in the system",
                            "facts": {
                                "condition": "ConnectionTraceFromDevice#[wifi == \"factory_display_001\" AND status == true] AND DataCollectionSuccess#[wifi == \"factory_display_001\" AND status == true] since 72 hours from now"
                            }
                        },
                        "missed_schedule": {
                            "facts": {
                                "condition": "ScheduleMissed#[wifi == \"factory_display_001\" AND status == true]"
                            }
                        }
                    }
                }
            }
        }"#;

        let result = StateDescription::from_json(json);
        assert!(result.is_ok(), "Should parse JSON successfully");
        
        let parser = result.unwrap();
        
        // Test getting all state names
        let state_names = parser.get_all_state_names();
        assert_eq!(state_names.len(), 5);
        assert!(state_names.contains(&"not_operational.unconnected".to_string()));
        assert!(state_names.contains(&"not_operational.un_enrolled".to_string()));
        assert!(state_names.contains(&"not_operational.unconnected_enrolled".to_string()));
        assert!(state_names.contains(&"operational.operating".to_string()));
        assert!(state_names.contains(&"operational.missed_schedule".to_string()));
        
        // Test getting specific states
        let unconnected_state = parser.get_state("not_operational.unconnected");
        assert!(unconnected_state.is_some());
        assert_eq!(unconnected_state.unwrap().description, Some("The device is not connected to the network".to_string()));
        
        // Test getting conditions
        let condition = parser.get_state_condition("not_operational.unconnected");
        assert_eq!(condition, Some("NOT (enrollment since 72 hours from now OR connection_to_cloud since 72 hours from now)"));
        
        // Test getting leaf states
        let leaf_states = parser.get_leaf_states();
        assert_eq!(leaf_states.len(), 5);
        
        // Test getting state groups
        let state_groups = parser.get_state_groups();
        assert_eq!(state_groups.len(), 2); // not_operational and operational
        
        // Test serialization back to JSON
        let serialized = parser.to_json();
        assert!(serialized.is_ok(), "Should serialize back to JSON");
    }

    #[test]
    fn test_invalid_state_name() {
        let json = r#"{
            "states": {
                "not_operational": {
                    "states": {
                        "unconnected": {
                            "facts": {
                                "condition": "test condition"
                            }
                        }
                    }
                },
                "operational": {
                    "states": {}
                }
            }
        }"#;

        let parser = StateDescription::from_json(json).unwrap();
        
        // Test invalid state names
        assert!(parser.get_state("invalid").is_none());
        assert!(parser.get_state("not_operational").is_some()); // This should work now as it's a state group
        assert!(parser.get_state("not_operational.unconnected.invalid").is_none());
    }

    #[test]
    fn test_deep_nested_states() {
        let json = r#"{
            "states": {
                "group1": {
                    "states": {
                        "subgroup1": {
                            "states": {
                                "leaf1": {
                                    "facts": {
                                        "condition": "condition1"
                                    }
                                },
                                "leaf2": {
                                    "facts": {
                                        "condition": "condition2"
                                    }
                                }
                            }
                        },
                        "leaf3": {
                            "facts": {
                                "condition": "condition3"
                            }
                        }
                    }
                }
            }
        }"#;

        let parser = StateDescription::from_json(json).unwrap();
        
        // Test deep nested state access
        let leaf1_state = parser.get_state("group1.subgroup1.leaf1");
        assert!(leaf1_state.is_some());
        assert_eq!(parser.get_state_condition("group1.subgroup1.leaf1"), Some("condition1"));
        
        // Test getting all state names
        let state_names = parser.get_all_state_names();
        assert_eq!(state_names.len(), 3);
        assert!(state_names.contains(&"group1.subgroup1.leaf1".to_string()));
        assert!(state_names.contains(&"group1.subgroup1.leaf2".to_string()));
        assert!(state_names.contains(&"group1.leaf3".to_string()));
        
        // Test getting leaf states
        let leaf_states = parser.get_leaf_states();
        assert_eq!(leaf_states.len(), 3);
        
        // Test getting state groups
        let state_groups = parser.get_state_groups();
        assert_eq!(state_groups.len(), 2); // group1 and subgroup1
    }

    #[test]
    fn test_to_state_machine() {
        let json = r#"{
            "states": {
                "not_operational": {
                    "states": {
                        "unconnected": {
                            "description": "The device is not connected to the network",
                            "facts": {
                                "condition": "NOT (enrollment since 72 hours from now OR connection_to_cloud since 72 hours from now)"
                            }
                        },
                        "un_enrolled": {
                            "description": "The device is not enrolled in the system",
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
                            "condition": "user_validation_on_this_device_status",
                            "action": {
                                "name": "enroll_device",
                                "description": "Enroll the device in the system"
                            }
                        }
                    ]
                },
                "operational": {
                    "states": {
                        "operating": {
                            "description": "The device is enrolled in the system",
                            "facts": {
                                "condition": "ConnectionTraceFromDevice#[wifi == \"factory_display_001\" AND status == true]"
                            }
                        }
                    },
                    "transitions": [
                        {
                            "name": "resume_operation_transition",
                            "from": "operational.operating",
                            "to": "operational.operating",
                            "condition": "ConnectionTraceFromDevice#[wifi == \"factory_display_001\" AND status == true]",
                            "action": {
                                "name": "resume_operation",
                                "description": "Resume normal operation"
                            }
                        }
                    ]
                }
            }
        }"#;

        let parser = StateDescription::from_json(json).unwrap();
        
        // Test conversion to StateMachine
        let state_machine = parser.to_state_machine();
        assert!(state_machine.is_ok(), "Should convert to StateMachine successfully");
        
        let state_machine = state_machine.unwrap();
        
        // Test that all states were created
        let all_states = state_machine.all_states.borrow();
        println!("Actual states: {:?}", all_states.iter().map(|s| s.borrow().state_name().to_string()).collect::<Vec<_>>());
        assert_eq!(all_states.len(), 5); // not_operational, not_operational.unconnected, not_operational.un_enrolled, operational, operational.operating
        
        // Test that transitions were created from the transition definitions
        let all_transitions = state_machine.all_transitions.borrow();
        assert_eq!(all_transitions.len(), 2); // Two transitions defined in the JSON
        
        // Verify specific states exist
        let state_names: Vec<String> = all_states.iter().map(|s| s.borrow().state_name().to_string()).collect();
        assert!(state_names.contains(&"not_operational".to_string()));
        assert!(state_names.contains(&"not_operational.unconnected".to_string()));
        assert!(state_names.contains(&"not_operational.un_enrolled".to_string()));
        assert!(state_names.contains(&"operational".to_string()));
        assert!(state_names.contains(&"operational.operating".to_string()));
        
        // Verify transitions have correct conditions and actions
        for transition in all_transitions.iter() {
            assert!(!transition.condition_str.is_empty(), "Transition should have a condition string");
            assert!(transition.action.is_some(), "Transition should have an action");
            if let Some(action) = &transition.action {
                assert!(!action.name.is_empty(), "Action should have a name");
                assert!(!action.description.is_empty(), "Action should have a description");
            }
        }
        
        println!("StateMachine conversion test passed!");
    }

    #[test]
    fn test_faulty_state_machine_from_json_file() {
        let faulty_json_content = 
            r#"
            {
                "states": {
                    "not_operational": {
                        "states": {
                            "unconnected": {
                                "description": "The device is not connected to the network",
                                "facts": {
                                    "condition": "NOT (enrollment since 72 hos from now OR connection_to_cloud since 72 hours from now)"
                                }
                            }
                        }
                    }
                }
            }
            "#;

        let description = StateDescription::from_json(&faulty_json_content).unwrap();

        let state_machine = description.to_state_machine();
        println!("Faulty JSON content: {:?}", state_machine);

           
    }


    #[test]
    fn test_state_machine_from_json_file() {
        use std::fs;
        use std::collections::HashSet;
        
        // Read the JSON file
        let json_content = fs::read_to_string("tests/state_description.json")
            .expect("Failed to read state_description.json");
        
        // Parse the JSON
        let description = StateDescription::from_json(&json_content)
            .expect("Failed to parse JSON");
        
        // Convert to StateMachine
        let state_machine = description.to_state_machine()
            .expect("Failed to convert to StateMachine");
        
        // Get all needed events (from facts and transitions at all levels)
        let (event_names, has_wildcard) = description.collect_all_event_names_and_wildcards();
        
        // Print all used events
        println!("\n=== Device State Machine Event Analysis ===");
        println!("Total unique events used: {}", event_names.len());
        println!("Events: {:?}", event_names);
        println!("Has wildcard (*): {}", has_wildcard);
        
        // Print transition details
        let all_transitions = state_machine.all_transitions.borrow();
        println!("\n=== Transition Details ===");
        println!("Total transitions: {}", all_transitions.len());
        for (i, transition) in all_transitions.iter().enumerate() {
            println!("Transition {}: {} -> {}", 
                i + 1, 
                transition.from.borrow().state_name(), 
                transition.to.borrow().state_name());
            println!("  Condition: {}", transition.condition_str);
            if let Some(action) = &transition.action {
                println!("  Action: {} - {}", action.name, action.description);
            }
        }
        
        // Measure memory usage
        let all_states = state_machine.all_states.borrow();
        println!("\n=== Memory Usage Analysis ===");
        println!("Total states: {}", all_states.len());
        println!("Total transitions: {}", all_transitions.len());
        
        // Estimate memory usage (rough calculation)
        let state_memory = all_states.len() * std::mem::size_of::<StateAspect>();
        let transition_memory = all_transitions.len() * std::mem::size_of::<Transition>();
        let total_estimated_memory = state_memory + transition_memory;
        
        println!("Estimated memory usage:");
        println!("  States: {} bytes", state_memory);
        println!("  Transitions: {} bytes", transition_memory);
        println!("  Total: {} bytes ({:.2} KB)", total_estimated_memory, total_estimated_memory as f64 / 1024.0);
        
        // Verify expected events are present
        let expected_events: HashSet<String> = [
            "enrollment".to_string(),
            "connection_to_cloud".to_string(),
            "enrollment_id_created".to_string(),
            "ConnectionTraceFromDevice".to_string(),
            "DataCollectionSuccess".to_string(),
            "ScheduleMissed".to_string(),
            "user_validation_on_this_device_status".to_string(),
        ].iter().cloned().collect();
        
        assert!(event_names.is_superset(&expected_events), 
            "Expected events {:?} not found in actual events {:?}", expected_events, event_names);
        
        println!("\n=== Test Results ===");
        println!("✓ Successfully parsed JSON file");
        println!("✓ Created StateMachine with {} states and {} transitions", 
            all_states.len(), all_transitions.len());
        println!("✓ Found {} unique event names", event_names.len());
        println!("✓ Memory usage: {:.2} KB", total_estimated_memory as f64 / 1024.0);
    }

    #[test]
    fn test_parser_with_source_locations() {
        let json = r#"{
            "states": {
                "not_operational": {
                    "states": {
                        "unconnected": {
                            "description": "The device is not connected to the network",
                            "facts": {
                                "condition": "NOT (enrollment since 72 hours from now OR connection_to_cloud since 72 hours from now)"
                            }
                        }
                    }
                }
            }
        }"#;

        let mut parser = StateParser::new(json.to_string());
        let result = parser.parse_with_locations();
        assert!(result.is_ok(), "Should parse JSON with locations successfully");
        
        let desc_with_locations = result.unwrap();
        
        // Test that we can get locations for elements
        let unconnected_location = parser.get_location("not_operational.unconnected");
        assert!(unconnected_location.is_some(), "Should find location for unconnected state");
        
        let facts_location = parser.get_location("not_operational.unconnected.facts");
        assert!(facts_location.is_some(), "Should find location for facts");
        
        // Test conversion to regular description
        let regular_desc: StateDescription = desc_with_locations.into();
        assert_eq!(regular_desc.states.len(), 1);
        assert!(regular_desc.states.contains_key("not_operational"));
    }

    #[test]
    fn test_source_location_accuracy() {
        let json = r#"{
            "states": {
                "test_state": {
                    "facts": {
                        "condition": "invalid_condition"
                    }
                }
            }
        }"#;

        let mut parser = StateParser::new(json.to_string());
        let result = parser.parse_with_locations();
        assert!(result.is_ok());
        
        // Test that locations are reasonable
        let test_state_location = parser.get_location("test_state");
        assert!(test_state_location.is_some());
        
        let location = test_state_location.unwrap();
        assert!(location.line > 0, "Line should be positive");
        assert!(location.column > 0, "Column should be positive");
        assert!(location.length > 0, "Length should be positive");
    }

    #[test]
    fn test_memoized_locations() {
        let json = r#"{
            "states": {
                "parent": {
                    "states": {
                        "child": {
                            "facts": {
                                "condition": "test_condition"
                            },
                            "transitions": [
                                {
                                    "name": "test_transition",
                                    "from": "parent.child",
                                    "to": "parent.child",
                                    "condition": "test_condition",
                                    "action": {
                                        "name": "test_action",
                                        "description": "Test action"
                                    }
                                }
                            ]
                        }
                    }
                }
            }
        }"#;

        let mut parser = StateParser::new(json.to_string());
        let result = parser.parse_with_locations();
        assert!(result.is_ok());
        
        // Test that all expected locations are memoized
        let expected_paths = vec![
            "parent",
            "parent.child",
            "parent.child.facts",
            "parent.child.transitions[0]",
        ];
        
        for path in expected_paths {
            let location = parser.get_location(path);
            assert!(location.is_some(), "Should find location for path: {}", path);
        }
    }
}



