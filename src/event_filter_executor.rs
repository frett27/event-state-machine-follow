use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

use crate::{event::*, event_parser::ExprContextAll};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EventKey {
    pub timestamp: u64,
    pub name: String,
}

impl PartialOrd for EventKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EventKey {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.timestamp == other.timestamp {
            self.name.cmp(&other.name)
        } else {
            other.timestamp.cmp(&self.timestamp)
        }
    }
}

// collection of events, with internal structures to speed up lookups
// in time, used for the event filter executor, and an initial implementation for events store
pub struct Events {
    pub ordered_events: BTreeMap<EventKey, Event>,
    // Fast index for event name lookups - stores Vec<Event> sorted by timestamp descending
    pub events_by_name: HashMap<String, Vec<Event>>,
}

impl Events {
    
    pub fn new() -> Self {
        Self {
            ordered_events: BTreeMap::new(),
            events_by_name: HashMap::new(),
        }
    }

    pub fn push_event(&mut self, event: Event) {
        let event_key = EventKey {
            timestamp: event.timestamp,
            name: event.name.clone(),
        };
        // Insert into ordered events
        self.ordered_events.insert(event_key, event.clone());
        // Insert into events_by_name, keeping the Vec sorted by timestamp descending
        let name_events = self.events_by_name.entry(event.name.clone()).or_insert_with(Vec::new);
        // Insert in sorted order (most recent first)
        let insert_pos = name_events.binary_search_by(|e| e.timestamp.cmp(&event.timestamp).reverse())
            .unwrap_or_else(|pos| pos);
        name_events.insert(insert_pos, event);
    }

    pub fn get_events(&self) -> Vec<&Event> {
        self.ordered_events.values().collect()
    }

    /// Fast lookup of events by name - O(1) average case
    pub fn get_events_by_name(&self, name: &str) -> Vec<&Event> {
        if let Some(events) = self.events_by_name.get(name) {
            events.iter().collect()
        } else {
            Vec::new()
        }
    }

    /// Get the most recent event of a specific name - O(1)
    pub fn get_latest_event_by_name(&self, name: &str) -> Option<&Event> {
        self.events_by_name.get(name).and_then(|events| events.first())
    }

    /// Get events by name within a time range
    pub fn get_events_by_name_in_range(&self, name: &str, start_time: u64, end_time: u64) -> Vec<&Event> {
        if let Some(events) = self.events_by_name.get(name) {
            events.iter().filter(|event| event.timestamp >= start_time && event.timestamp <= end_time).collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_latest_events_by_name_and_timelimit(&self, name: &str, timelimit: u64) -> Vec<&Event> {
        if let Some(events) = self.events_by_name.get(name) {
            events.iter().filter(|event| event.timestamp >= timelimit).collect()
        } else {
            Vec::new()
        }
    }

    /// Get count of events by name
    #[cfg(test)]
    pub fn count_events_by_name(&self, name: &str) -> usize {
        self.events_by_name.get(name).map_or(0, |events| events.len())
    }

    /// Get all unique event names
    pub fn get_event_names(&self) -> Vec<&String> {
        self.events_by_name.keys().collect()
    }

    pub fn find_previous_events_matching_name(
        &self,
        name: &str,
        strict_timelimit: u64,
    ) -> Vec<&Event> {
        log::debug!("DEBUG: find_previous_events_matching_name: name = {}, strict_timelimit = {}", name, strict_timelimit);
        // Use the fast name index if available
        if name != "*" {
            log::debug!("DEBUG: find_previous_events_matching_name: name = {}, strict_timelimit = {}", name, strict_timelimit);
            return self.get_latest_events_by_name_and_timelimit(name, strict_timelimit);
        }
        // Fallback to original implementation for wildcard
        log::debug!("DEBUG: strict_timelimit = {}", strict_timelimit);
        log::debug!("DEBUG: ordered_events count = {}", self.ordered_events.len());
        
        let range_key = EventKey {
            timestamp: strict_timelimit,
            name: "".to_string(),
        };
        log::debug!("DEBUG: range_key = EventKey{{timestamp: {}, name: \"\"}}", range_key.timestamp);
        
        let events_in_range: Vec<&Event> = self.ordered_events
            .range(
                ..range_key,
            )
            .map(|(_, event)| event)
            .collect();
        
        log::debug!("DEBUG: events_in_range count = {}", events_in_range.len());
        for event in &events_in_range {
            log::debug!("DEBUG: event in range: {:?}", event);
        }
        
        events_in_range
    }
}

#[test]
fn test_find_previous_events_matching_name() {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).is_test(true).try_init();

    let mut events = Events::new();
    events.push_event(Event::new("event1".to_string(), false, HashMap::new(), 100));
    events.push_event(Event::new("event2".to_string(), false, HashMap::new(), 200));
    events.push_event(Event::new("event3".to_string(), false, HashMap::new(), 300));
    events.push_event(Event::new("event4".to_string(), false, HashMap::new(), 400));
    events.push_event(Event::new("event5".to_string(), false, HashMap::new(), 500));
    events.push_event(Event::new("event6".to_string(), false, HashMap::new(), 600));
    events.push_event(Event::new("event7".to_string(), false, HashMap::new(), 700));
    events.push_event(Event::new("event8".to_string(), false, HashMap::new(), 800));
    events.push_event(Event::new("event9".to_string(), false, HashMap::new(), 900));

    log::debug!(
        "first event of the collection: {:?}",
        events.ordered_events.iter().next().unwrap().1
    );
    log::debug!(
        "last event of the collection: {:?}",
        events.ordered_events.iter().next_back().unwrap().1
    );

    // ---- event_filter_executor::test_find_previous_events_matching_name stdout ----
    // first event of the collection: Event { name: "event9", is_state: false, attributes: {}, timestamp: 900 }
    // last event of the collection: Event { name: "event1", is_state: false, attributes: {}, timestamp: 100 }

    let previous_events = events
        .find_previous_events_matching_name("*", 900 - 200)
        .iter()
        .map(|event| event.name.clone())
        .collect::<Vec<String>>();
    assert_eq!(previous_events.len(), 2);
    assert_eq!(previous_events[0], "event9"); // plus recent
    assert_eq!(previous_events[1], "event8"); // plus ancien

    let previous_events = events
        .find_previous_events_matching_name("event1", 0)
        .iter()
        .map(|event| event.name.clone())
        .collect::<Vec<String>>();
    assert_eq!(previous_events.len(), 1);
    assert_eq!(previous_events[0], "event1");
}

#[test]
fn test_fast_event_name_lookup() {
    let mut events = Events::new();
    
    // Add multiple events with the same name
    events.push_event(Event::new("event1".to_string(), false, HashMap::new(), 100));
    events.push_event(Event::new("event1".to_string(), false, HashMap::new(), 200));
    events.push_event(Event::new("event1".to_string(), false, HashMap::new(), 300));
    events.push_event(Event::new("event2".to_string(), false, HashMap::new(), 150));
    events.push_event(Event::new("event2".to_string(), false, HashMap::new(), 250));
    events.push_event(Event::new("event3".to_string(), false, HashMap::new(), 400));
    
    // Test get_events_by_name
    let event1_events = events.get_events_by_name("event1");
    assert_eq!(event1_events.len(), 3);
    assert!(event1_events.iter().all(|e| e.name == "event1"));
    
    let event2_events = events.get_events_by_name("event2");
    assert_eq!(event2_events.len(), 2);
    assert!(event2_events.iter().all(|e| e.name == "event2"));
    
    let event3_events = events.get_events_by_name("event3");
    assert_eq!(event3_events.len(), 1);
    assert!(event3_events.iter().all(|e| e.name == "event3"));
    
    let nonexistent_events = events.get_events_by_name("nonexistent");
    assert_eq!(nonexistent_events.len(), 0);
    
    // Test get_latest_event_by_name
    let latest_event1 = events.get_latest_event_by_name("event1");
    assert!(latest_event1.is_some());
    assert_eq!(latest_event1.unwrap().timestamp, 300);
    
    let latest_event2 = events.get_latest_event_by_name("event2");
    assert!(latest_event2.is_some());
    assert_eq!(latest_event2.unwrap().timestamp, 250);
    
    let latest_nonexistent = events.get_latest_event_by_name("nonexistent");
    assert!(latest_nonexistent.is_none());
    
    // Test get_events_by_name_in_range
    let event1_in_range = events.get_events_by_name_in_range("event1", 150, 250);
    assert_eq!(event1_in_range.len(), 1);
    assert_eq!(event1_in_range[0].timestamp, 200);
    
    let event2_in_range = events.get_events_by_name_in_range("event2", 100, 300);
    assert_eq!(event2_in_range.len(), 2);
    
    // Test count_events_by_name
    assert_eq!(events.count_events_by_name("event1"), 3);
    assert_eq!(events.count_events_by_name("event2"), 2);
    assert_eq!(events.count_events_by_name("event3"), 1);
    assert_eq!(events.count_events_by_name("nonexistent"), 0);
    
    // Test get_event_names
    let event_names = events.get_event_names();
    assert_eq!(event_names.len(), 3);
    assert!(event_names.contains(&&"event1".to_string()));
    assert!(event_names.contains(&&"event2".to_string()));
    assert!(event_names.contains(&&"event3".to_string()));
}

#[test]
fn test_benchmark_name_lookup_performance() {

    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).is_test(true).try_init();

    use std::time::Instant;
    
    let mut events = Events::new();
    let num_events = 100_000;
    
    // Create events with different names
    for i in 0..num_events {
        let event_name = if i % 10 == 0 {
            "event1".to_string()
        } else if i % 10 == 1 {
            "event2".to_string()
        } else {
            format!("event{}", i % 100)
        };
        
        events.push_event(Event::new(event_name, false, HashMap::new(), i as u64));
    }
    
    log::debug!("Created {} events with {} unique names", num_events, events.get_event_names().len());
    
    // Benchmark fast name lookup
    let start = Instant::now();
    let event1_count = events.count_events_by_name("event1");
    let fast_lookup_duration = start.elapsed();
    
    log::debug!("Fast name lookup for 'event1': {} events in {:?}", event1_count, fast_lookup_duration);
    
    // Benchmark getting events by name
    let start = Instant::now();
    let event1_events = events.get_events_by_name("event1");
    let get_events_duration = start.elapsed();
    
    log::debug!("Get events by name 'event1': {} events in {:?}", event1_events.len(), get_events_duration);
    
    // Benchmark latest event lookup
    let start = Instant::now();
    let latest_event1 = events.get_latest_event_by_name("event1");
    let latest_lookup_duration = start.elapsed();
    
    log::debug!("Latest event lookup for 'event1': {:?} in {:?}", 
             latest_event1.map(|e| e.timestamp), latest_lookup_duration);
    
    assert!(fast_lookup_duration.as_micros() < 1000, "Fast lookup should be very fast (< 1ms)");
    assert!(get_events_duration.as_micros() < 1000, "Get events should be very fast (< 1ms)");
    assert!(latest_lookup_duration.as_micros() < 1000, "Latest lookup should be very fast (< 1ms)");
}


//////////////////////////////////////////////////////////////////////////////////////
/// EventFilterExecutor

pub struct EventFilterExecutor<'input> {
    pub events: Rc<RefCell<Events>>,
    pub filter: Rc<RefCell<ExprContextAll<'input>>>,
}

impl<'input> EventFilterExecutor<'input> {
    pub fn new(filter: Rc<RefCell<ExprContextAll<'input>>>) -> Self {
        Self {
            events: Rc::new(RefCell::new(Events::new())),
            filter: Rc::clone(&filter),
        }
    }

    pub fn add_event(&self, event: Event) {
        self.events.borrow_mut().push_event(event);
    }

    pub fn execute_on(&self, event: &Event) -> Vec<Event> {
        let events = self.events.borrow().get_events();
        let mut result = Vec::new();

        result
    }
}
