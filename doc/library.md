# Library modules and functions

Crate root (`src/lib.rs`) exports six modules. There is no binary; consumers link the library and call these APIs directly.

## Module map

| Module | Path | Responsibility |
|--------|------|----------------|
| `event` | `src/event.rs` | Core `Event` / `EventValue` value types |
| `event_filter_executor` | `src/event_filter_executor.rs` | Ordered event store + filter executor shell |
| `event_parser` | `src/event_parser/` | ANTLR grammar + expression evaluation |
| `state_machine` | `src/state_machine.rs` | Runtime state machine graph |
| `state_description` | `src/state_description.rs` | JSON DDL → runtime graph |
| `follow` | `src/follow/` | Follow agent (`StateAgent`) |

Dependency direction (no cycles with fleetee):

```text
event
  ↑
event_filter_executor ←→ event_parser
  ↑                         ↑
state_machine ←─────────────┘
  ↑
state_description
  ↑
follow
```

## `event`

| Type | Purpose |
|------|---------|
| `Event` | `name`, `is_state`, `attributes: HashMap<String, EventValue>`, `timestamp: u64` |
| `EventValue` | `String` / `Integer` / `Boolean` / `Float` / `Null` |
| `Event::new(...)` | Constructor |

Serde-serializable; used everywhere as the unit of observation.

## `event_filter_executor`

| Type / method | Purpose |
|---------------|---------|
| `EventKey` | `(timestamp, name)` ordering key (newest-first for ranges) |
| `Events` | Dual index: `BTreeMap` by key + `HashMap` by name |
| `Events::push_event` | Insert into both indexes |
| `get_events_by_name` / `get_latest_event_by_name` | O(1)-ish name lookups |
| `get_events_by_name_in_range` / `get_latest_events_by_name_and_timelimit` | Time-bounded queries |
| `find_previous_events_matching_name` | History scan; `"*"` uses ordered map |
| `EventFilterExecutor` | Holds `Rc<RefCell<Events>>` + parsed filter (stub `execute_on` today) |

## `event_parser`

See [parsing.md](parsing.md) for grammar details. Library surface:

| Item | Purpose |
|------|---------|
| `parse_expression` | String → parse tree |
| `evaluate_filter_expression` | Convenience parse + bool |
| `evaluate_filter_event_expr` | Convenience parse + matching events |
| `collect_events_from_expr` | Static event-name extraction |
| `FilterContext` | Evaluation environment |
| `FilterEvaluator` | Tree visitor |
| `CompiledTimeExpression` | Reusable compiled condition (`evaluate` on an `Events` store) |
| `TimeExpressionParser` | Parser object with error listener |
| `ExprContextAll` | Re-exported parse-tree root type |
| `ParseError` | Parse failure |

## `state_machine` (runtime graph)

| Type | Purpose |
|------|---------|
| `StateAspect` | One state: name, description, `activation_conditions`, parent, transitions |
| `Transition` | `from` / `to` aspects, condition string + parsed expr, optional `Command` |
| `Command` | `name` + `description` (action metadata only; not a fact) |
| `StateMachine` | `all_states`, `all_transitions`, `current_states` |

Notable methods on `StateMachine`:

| Method | Purpose |
|--------|---------|
| `new` / `add_transition` | Build graph |
| `get_state_by_name` | Lookup aspect |
| `get_all_compiled_conditions_for_states` | Compile activation strings → `CompiledTimeExpression` |
| `evaluate_transitions` | Transitions whose conditions hold for an event |
| `get_all_needed_events` | Events referenced by transition conditions |

**Not linked to fleetee `Fact`:** transitions no longer carry `sent_facts`.

## `state_description` (DDL)

| Type | Purpose |
|------|---------|
| `StateDescription` / `State` / `ActivationRule` | Clean serde model |
| `TransitionDef` / `ActionDef` | Transition DDL |
| `StateParser` + `*WithLocation` | Source-location decoration for tooling |
| `Facts` | Type alias = `ActivationRule` (compat) |

Key methods: `from_json`, `to_json`, `to_state_machine`, leaf/group/condition helpers, `collect_all_event_names_and_wildcards`.

## `follow` (agent)

| Type / method | Purpose |
|---------------|---------|
| `StateAgent` | Follow runtime for one subject |
| `new(id, description)` | Build machine + empty `Events` |
| `event_arrived_so_evaluate_states` | Ingest event → active states + change events |
| `get_current_states` | Snapshot of active state names |

Details in [service.md](service.md).

## Dependencies

Minimal: `serde`, `serde_json`, `log`, `antlr-rust` (workspace path). No actix, MQTT, or facts crates.
