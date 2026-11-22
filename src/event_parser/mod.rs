// Event Parser Module
// Using ANTLR4 for parsing time expressions

mod timeexprgrammarlexer;
mod timeexprgrammarparser;
mod timeexprgrammarlistener;
mod timeexprgrammarvisitor; // Commented out due to trait bound issues

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::recognizer::Recognizer;
use antlr_rust::token::Token;
use antlr_rust::Parser;
use antlr_rust::tree::ParseTree;

use timeexprgrammarlexer::TimeExprGrammarLexer;
use timeexprgrammarparser::TimeExprGrammarParser;
use timeexprgrammarparser::{
    ExprContextAttrs, AndExprContextAttrs, OrExprContextAttrs, NotExprContextAttrs,
    ParenExprContextAttrs, EventExprAsBoolContextAttrs, FilteredEventWithDataAttributesContextAttrs,
    AttributeNameContextAttrs,
    ValueContextAttrs, AttributeExistsContextAttrs, AttributeConditionsContextAttrs,
    EventExprWithTimeResolutionContextAttrs, EventNameContextAttrs,
    SingleEventNameContextAttrs, EventStateNameContextAttrs, TimeReferenceWithListOfEventsContextAttrs,
    EventListExpressionContextAttrs, SimpleAttributeFilterExpressionContextAttrs,
    FilterAttributeANDContextAttrs, FilterAttributeORContextAttrs, ParentAttributeContextAttrs
};

pub use timeexprgrammarparser::{
    ExprContextAll
};

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::event::Event;
use crate::event_filter_executor::EventKey;
use crate::event::EventValue;
use crate::event_filter_executor::Events;
use crate::event_parser::timeexprgrammarparser::AnyExistingEventContextAttrs;
use crate::event_parser::timeexprgrammarparser::SinceExprContextAttrs;

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    fn new(message: String) -> Self {
        ParseError { message }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.message)
    }
}

impl Error for ParseError {}

// Custom error listener to capture parsing errors
struct CustomErrorListener {
    errors: Rc<RefCell<Vec<String>>>,
}

impl CustomErrorListener {
    fn new(errors: Rc<RefCell<Vec<String>>>) -> Self {
        CustomErrorListener {
            errors,
        }
    }

    fn has_errors(&self) -> bool {
        !self.errors.borrow().is_empty()
    }

    fn get_errors(&self) -> Vec<String> {
        self.errors.borrow().clone()
    }

    fn clear_errors(&self) {
        self.errors.borrow_mut().clear();
    }
}

impl<'a, T: Recognizer<'a>> ErrorListener<'a, T> for CustomErrorListener {
    fn syntax_error(
        &self,
        _recognizer: &T,
        offending_symbol: Option<&<T::TF as antlr_rust::token_factory::TokenFactory<'a>>::Inner>,
        line: isize,
        column: isize,
        msg: &str,
        _e: Option<&antlr_rust::errors::ANTLRError>,
    ) {
        let token_text = if let Some(token) = offending_symbol {
            format!("{:?}", token.get_text())
        } else {
            "unknown".to_string()
        };
        
        let error_msg = format!(
            "line {}:{} mismatched input '{}' expecting {}",
            line, column, token_text, msg
        );
        println!("DEBUG: error_msg = {:?}", error_msg);
        
        self.errors.borrow_mut().push(error_msg);
    }
}



#[derive(Clone)]
pub struct FilterContext {
    events: Rc<RefCell<Events>>,
    pub current_event: Option<Event>, // used for time expressions
    pub attributes: HashMap<String, EventValue>, // used for data filters on some events
    pub time_ref: Option<u64>, // used for time expressions
}

impl FilterContext {
    pub fn new(events: Rc<RefCell<Events>>) -> Self {
        FilterContext {
            events: Rc::clone(&events),
            current_event: None,
            attributes: HashMap::new(),
            time_ref: None,
        }
    }

    pub fn add_attribute(&mut self, name: String, value: EventValue) {
        self.attributes.insert(name, value);
    }

    pub fn get_attribute(&self, name: &str) -> Option<&EventValue> {
        self.attributes.get(name)
    }
}

// Visitor trait for evaluating expressions
pub trait ExpressionVisitor {
    type ReturnType;
    
    fn visit_expr(&self, ctx: &ExprContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<bool, String>;
    fn visit_expr_for_events(&self, ctx: &ExprContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<Vec<Event>, String>;
    fn visit_boolean_expr(&self, ctx: &crate::event_parser::timeexprgrammarparser::BooleanExprContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<bool, String>;
    fn visit_boolean_expr_for_events(&self, ctx: &crate::event_parser::timeexprgrammarparser::BooleanExprContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<Vec<Event>, String>;
    fn visit_event_expr(&self, ctx: &crate::event_parser::timeexprgrammarparser::EventExprContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<Vec<Event>, String>;
    fn visit_event_filter_expr(&self, ctx: &crate::event_parser::timeexprgrammarparser::EventFilterExprContextAll, reference_event_for_timing: &Event) -> Result<Vec<Event>, String>;
    fn visit_attribute_data_filter_exp(&self, ctx: &crate::event_parser::timeexprgrammarparser::AttributeDataFilterExpContextAll, list_of_events: &Vec<Event>, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<Vec<Event>, String>;
    fn visit_event_attributes_data_filter(&self, ctx: &crate::event_parser::timeexprgrammarparser::EventAttributesDataFilterContextAll, list_of_events: &Vec<Event>, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<Vec<Event>, String>;
    fn visit_time_expr(&self, ctx: &crate::event_parser::timeexprgrammarparser::TimeExprContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<Option<u64>, String>;
    fn visit_any_event_expr(&self, ctx: &crate::event_parser::timeexprgrammarparser::AnyEventExpressionContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<bool, String>;
    fn visit_any_event_expr_for_events(&self, ctx: &crate::event_parser::timeexprgrammarparser::AnyEventExpressionContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<Vec<Event>, String>;
    fn visit_time_ref(&self, ctx: &crate::event_parser::timeexprgrammarparser::TimeRefContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<Option<u64>, String>;
    fn visit_event_list_expr(&self, ctx: &crate::event_parser::timeexprgrammarparser::EventListExprContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<BTreeMap<EventKey, Event>, String>;
}

// Filter evaluation visitor
pub struct FilterEvaluator {
    context: FilterContext,
}

impl FilterEvaluator {

    pub fn new(context: FilterContext) -> Self {
        FilterEvaluator { context }
    }

    /// evaluate the timed event expression
    pub fn evaluate(&self, expr: &ExprContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<bool, String> {
        self.visit_expr(expr, current_arrived_event_or_timer_event_for_reference)
    }

    pub fn evaluate_events(&self, expr: &ExprContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<Vec<Event>, String> {
        self.visit_expr_for_events(expr, current_arrived_event_or_timer_event_for_reference)
    }

    fn evaluate_comparison(&self, left: &EventValue, op: &str, right: &EventValue) -> Result<bool, String> {
        match (left, right) {
            (EventValue::Integer(l), EventValue::Integer(r)) => {
                match op {
                    ">" => Ok(l > r),
                    "<" => Ok(l < r),
                    ">=" => Ok(l >= r),
                    "<=" => Ok(l <= r),
                    "==" => Ok(l == r),
                    "!=" => Ok(l != r),
                    _ => Err(format!("Unknown comparison operator: {}", op)),
                }
            }
            (EventValue::Float(l), EventValue::Float(r)) => {
                match op {
                    ">" => Ok(l > r),
                    "<" => Ok(l < r),
                    ">=" => Ok(l >= r),
                    "<=" => Ok(l <= r),
                    "==" => Ok((l - r).abs() < f64::EPSILON),
                    "!=" => Ok((l - r).abs() >= f64::EPSILON),
                    _ => Err(format!("Unknown comparison operator: {}", op)),
                }
            }
            // Handle mixed Float/Integer comparisons
            (EventValue::Float(l), EventValue::Integer(r)) => {
                let r_float = *r as f64;
                match op {
                    ">" => Ok(l > &r_float),
                    "<" => Ok(l < &r_float),
                    ">=" => Ok(l >= &r_float),
                    "<=" => Ok(l <= &r_float),
                    "==" => Ok((l - r_float).abs() < f64::EPSILON),
                    "!=" => Ok((l - r_float).abs() >= f64::EPSILON),
                    _ => Err(format!("Unknown comparison operator: {}", op)),
                }
            }
            (EventValue::Integer(l), EventValue::Float(r)) => {
                let l_float = *l as f64;
                match op {
                    ">" => Ok(l_float > *r),
                    "<" => Ok(l_float < *r),
                    ">=" => Ok(l_float >= *r),
                    "<=" => Ok(l_float <= *r),
                    "==" => Ok((l_float - r).abs() < f64::EPSILON),
                    "!=" => Ok((l_float - r).abs() >= f64::EPSILON),
                    _ => Err(format!("Unknown comparison operator: {}", op)),
                }
            }
            (EventValue::String(l), EventValue::String(r)) => {
                match op {
                    "==" => Ok(l == r),
                    "!=" => Ok(l != r),
                    _ => Err(format!("String comparison only supports == and !=, got: {}", op)),
                }
            }
            (EventValue::Boolean(l), EventValue::Boolean(r)) => {
                match op {
                    "==" => Ok(l == r),
                    "!=" => Ok(l != r),
                    _ => Err(format!("Boolean comparison only supports == and !=, got: {}", op)),
                }
            }
            _ => Err(format!("Cannot compare {:?} and {:?}", left, right)),
        }
    }
}

impl ExpressionVisitor for FilterEvaluator {
    type ReturnType = Vec<Event>;

    fn visit_expr(&self, ctx: &ExprContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<bool, String> {
        if let Some(boolean_expr) = ctx.booleanExpr() {
            self.visit_boolean_expr(&boolean_expr, current_arrived_event_or_timer_event_for_reference)
        } else {
            Err("Expression must contain a boolean expression".to_string())
        }
    }

    fn visit_expr_for_events(&self, ctx: &ExprContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<Vec<Event>, String> {
        if let Some(boolean_expr) = ctx.booleanExpr() {
            self.visit_boolean_expr_for_events(&boolean_expr, current_arrived_event_or_timer_event_for_reference)
        } else {
            Err("Expression must contain a boolean expression".to_string())
        }
    }

    fn visit_boolean_expr(&self, ctx: &crate::event_parser::timeexprgrammarparser::BooleanExprContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<bool, String> {
        use crate::event_parser::timeexprgrammarparser::BooleanExprContextAll::*;
        match ctx {
            AndExprContext(and_ctx) => {
                let left = self.visit_boolean_expr(&and_ctx.booleanExpr(0).unwrap(), current_arrived_event_or_timer_event_for_reference)?;
                let right = self.visit_boolean_expr(&and_ctx.booleanExpr(1).unwrap(), current_arrived_event_or_timer_event_for_reference)?;
                Ok(left && right)
            }
            OrExprContext(or_ctx) => {
                let left = self.visit_boolean_expr(&or_ctx.booleanExpr(0).unwrap(), current_arrived_event_or_timer_event_for_reference)?;
                let right = self.visit_boolean_expr(&or_ctx.booleanExpr(1).unwrap(), current_arrived_event_or_timer_event_for_reference)?;
                Ok(left || right)
            }
            NotExprContext(not_ctx) => {
                let expr = self.visit_boolean_expr(&not_ctx.booleanExpr().unwrap(), current_arrived_event_or_timer_event_for_reference)?;
                Ok(!expr)
            }
            ParenExprContext(paren_ctx) => {
                self.visit_boolean_expr(&paren_ctx.booleanExpr().unwrap(), current_arrived_event_or_timer_event_for_reference)
            }
            EventExprAsBoolContext(event_ctx) => {
                if let Some(any_event_expr) = event_ctx.anyEventExpression() {
                    self.visit_any_event_expr(&any_event_expr, current_arrived_event_or_timer_event_for_reference)
                } else {
                    Err("Event expression not found".to_string())
                }
            }
            Error(_) => Err("Invalid boolean expression".to_string()),
        }
    }

    fn visit_boolean_expr_for_events(&self, ctx: &crate::event_parser::timeexprgrammarparser::BooleanExprContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<Vec<Event>, String> {
        use crate::event_parser::timeexprgrammarparser::BooleanExprContextAll::*;
        match ctx {
            AndExprContext(and_ctx) => {
                let left_events = self.visit_boolean_expr_for_events(&and_ctx.booleanExpr(0).unwrap(), current_arrived_event_or_timer_event_for_reference)?;
                let right_events = self.visit_boolean_expr_for_events(&and_ctx.booleanExpr(1).unwrap(), current_arrived_event_or_timer_event_for_reference)?;
                
                // For AND, we need events that appear in both left and right results
                if left_events.len() > 0 {
                    if right_events.len() > 0 {
                        let mut result = left_events;
                        for right_event in right_events {
                            if !result.iter().any(|event| event.name == right_event.name  && event.timestamp == right_event.timestamp ) {
                                result.push(right_event);
                            }
                        }
                        Ok(result)
                    } else {
                        Ok(Vec::new())
                    }
                } else {
                    // no events returned
                    Ok(Vec::new())
                }
            }
            OrExprContext(or_ctx) => {
                // there are events on the left
                let left_events = self.visit_boolean_expr_for_events(&or_ctx.booleanExpr(0).unwrap(), current_arrived_event_or_timer_event_for_reference)?;
                // there are events on the right
                let right_events = self.visit_boolean_expr_for_events(&or_ctx.booleanExpr(1).unwrap(), current_arrived_event_or_timer_event_for_reference)?;
                if left_events.len() == 0 {
                    Ok(right_events)
                } else {

                // For OR, combine all events, removing duplicates
                let mut result = left_events;
                for right_event in right_events {
                    // if the event is not in the left, add it to the result
                    if !result.iter().any(|event| event.name == right_event.name  && event.timestamp == right_event.timestamp ) {
                        result.push(right_event);
                    }
                }
                Ok(result)
            }
            }
            NotExprContext(not_ctx) => {
                // For NOT, we return empty since we can't easily determine which events don't match
                // This is a limitation of the current approach

                // events in the context have all the events
                let events_borrow = self.context.events.borrow();
                let all_events = events_borrow.get_events();

                // remove the event on the operator
                if let Some(event_on_operator) = not_ctx.booleanExpr() {
                    let event_on_operator_events = self.visit_boolean_expr_for_events(&event_on_operator, current_arrived_event_or_timer_event_for_reference)?;
                    let mut result: Vec<Event> = all_events.into_iter().cloned().collect();
                    for event in event_on_operator_events {
                        result.retain(|e| e.name != event.name || e.timestamp != event.timestamp);
                    }
                    Ok(result)
                } else {
                    Err("Invalid boolean expression".to_string())
                }
            }
            ParenExprContext(paren_ctx) => {
                self.visit_boolean_expr_for_events(&paren_ctx.booleanExpr().unwrap(), current_arrived_event_or_timer_event_for_reference)
            }
            EventExprAsBoolContext(event_ctx) => {
                if let Some(any_event_expr) = event_ctx.anyEventExpression() {
                    self.visit_any_event_expr_for_events(&any_event_expr, current_arrived_event_or_timer_event_for_reference)
                } else {
                    Err("Event expression not found".to_string())
                }
            }
            Error(_) => Err("Invalid boolean expression".to_string()),
        }
    }

    fn visit_any_event_expr(&self, ctx: &crate::event_parser::timeexprgrammarparser::AnyEventExpressionContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<bool, String> {
        use crate::event_parser::timeexprgrammarparser::AnyEventExpressionContextAll::*;
        match ctx {
            AnyExistingEventContext(any_event_ctx) => {
                let resulting_events = self.visit_event_expr(&any_event_ctx.eventExpr().unwrap(), current_arrived_event_or_timer_event_for_reference)?;
                Ok(resulting_events.len() > 0)
            }
            Error(_) => Err("Invalid any event expression".to_string()),
        }
    }

    fn visit_any_event_expr_for_events(&self, ctx: &crate::event_parser::timeexprgrammarparser::AnyEventExpressionContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<Vec<Event>, String> {
        use crate::event_parser::timeexprgrammarparser::AnyEventExpressionContextAll::*;
        match ctx {
            AnyExistingEventContext(any_event_ctx) => {
                let resulting_events = self.visit_event_expr(&any_event_ctx.eventExpr().unwrap(), current_arrived_event_or_timer_event_for_reference)?;
                Ok(resulting_events)
            }
            Error(_) => Err("Invalid any event expression".to_string()),
        }
    }

    fn visit_event_expr(&self, ctx: &crate::event_parser::timeexprgrammarparser::EventExprContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<Vec<Event>, String> {
        match ctx {
            crate::event_parser::timeexprgrammarparser::EventExprContextAll::EventExprWithTimeResolutionContext(event_ctx) => {
                if let Some(event_filter_expr) = event_ctx.eventFilterExpr() {
                    let resulting_events = self.visit_event_filter_expr(&event_filter_expr, current_arrived_event_or_timer_event_for_reference)?;
                    
                    if let Some(time_expr) = event_ctx.timeExpr() {
                        let time_value = self.visit_time_expr(&time_expr, current_arrived_event_or_timer_event_for_reference)?;
                        if let Some(time_value) = time_value {
                            let resulting_events = resulting_events.into_iter().filter(|event| event.timestamp >=time_value).collect();
                            Ok(resulting_events)
                        } else {
                            Ok(resulting_events)
                        }
                    } else {
                        // to time filtering is not supported yet
                        Ok(resulting_events)
                    }
                } else {
                    // If no eventFilterExpr, we need to get the event name from the context
                    // This case should not happen in normal parsing, but we handle it gracefully
                    let events_borrow = self.context.events.borrow();
                    let all_events = events_borrow.get_events();
                    Ok(all_events.into_iter().cloned().collect())
                }
            }
            crate::event_parser::timeexprgrammarparser::EventExprContextAll::Error(_) => {
                Err("Invalid event expression".to_string())
            }
        }
    }

    fn visit_event_filter_expr(&self, ctx: &crate::event_parser::timeexprgrammarparser::EventFilterExprContextAll, reference_event_for_timing: &Event) -> Result<Vec<Event>, String> {
        match ctx {
            crate::event_parser::timeexprgrammarparser::EventFilterExprContextAll::FilteredEventWithDataAttributesContext(filtered_ctx) => {
                if let Some(event_name_ctx) = filtered_ctx.eventName() {

                    // there is an event name
                    let event_name = if let Some(single_event_name) = event_name_ctx.singleEventName() {
                        single_event_name.ID().unwrap().get_text()
                    } else if let Some(event_state_name) = event_name_ctx.eventStateName() {
                        event_state_name.ID().unwrap().get_text()
                    } else {
                        return Err("Invalid event name".to_string());
                    };

                    // Use fast name lookup instead of filtering all events
                    let events_borrow = self.context.events.borrow();
                    let mut matching_events: Vec<Event> = events_borrow
                        .get_events_by_name(&event_name)
                        .into_iter()
                        .cloned()
                        .collect();
                    
                    if let Some(data_filter) = filtered_ctx.attributeDataFilterExp() {
                        matching_events = self.visit_attribute_data_filter_exp(&data_filter, &matching_events, reference_event_for_timing)?;
                    }
                    Ok(matching_events)
                } else {
                    Err("Event name not found".to_string())
                }
            }
            crate::event_parser::timeexprgrammarparser::EventFilterExprContextAll::WildcardWithOptionalFilteredEventsContext(_wildcard_ctx) => {
                let events_borrow = self.context.events.borrow();
                let all_events = events_borrow.get_events();
                Ok(all_events.into_iter().cloned().collect())
            }
            crate::event_parser::timeexprgrammarparser::EventFilterExprContextAll::Error(_) => {
                Err("Invalid event filter expression".to_string())
            }
        }
    }

    fn visit_attribute_data_filter_exp(&self, ctx: &crate::event_parser::timeexprgrammarparser::AttributeDataFilterExpContextAll, list_of_events: &Vec<Event>, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<Vec<Event>, String> {
        match ctx {
            crate::event_parser::timeexprgrammarparser::AttributeDataFilterExpContextAll::FilterAttributeANDContext(and_ctx) => {
                let left_exprs = and_ctx.attributeDataFilterExp_all();
                if left_exprs.len() >= 2 {
                    let left_result = self.visit_attribute_data_filter_exp(&left_exprs[0], list_of_events, current_arrived_event_or_timer_event_for_reference)?;
                    let right_result = self.visit_attribute_data_filter_exp(&left_exprs[1], &left_result, current_arrived_event_or_timer_event_for_reference)?;
                    Ok(right_result)
                } else {
                    Err("AND expression requires two operands".to_string())
                }
            }
            crate::event_parser::timeexprgrammarparser::AttributeDataFilterExpContextAll::FilterAttributeORContext(or_ctx) => {
                let left_exprs = or_ctx.attributeDataFilterExp_all();
                if left_exprs.len() >= 2 {
                    let left_result = self.visit_attribute_data_filter_exp(&left_exprs[0], list_of_events, current_arrived_event_or_timer_event_for_reference)?;
                    let right_result = self.visit_attribute_data_filter_exp(&left_exprs[1], list_of_events, current_arrived_event_or_timer_event_for_reference)?;
                    // Combine results (union)
                    let mut combined = left_result;
                    for event in right_result {
                        if !combined.iter().any(|e| e.name == event.name && e.timestamp == event.timestamp) {
                            combined.push(event);
                        }
                    }
                    Ok(combined)
                } else {
                    Err("OR expression requires two operands".to_string())
                }
            }
            crate::event_parser::timeexprgrammarparser::AttributeDataFilterExpContextAll::ParentAttributeContext(paren_ctx) => {
                if let Some(expr) = paren_ctx.attributeDataFilterExp() {
                    self.visit_attribute_data_filter_exp(&expr, list_of_events, current_arrived_event_or_timer_event_for_reference)
                } else {
                    Err("Parent expression not found".to_string())
                }
            }
            crate::event_parser::timeexprgrammarparser::AttributeDataFilterExpContextAll::SimpleAttributeFilterExpressionContext(simple_ctx) => {
                if let Some(event_attrs_filter) = simple_ctx.eventAttributesDataFilter() {
                    self.visit_event_attributes_data_filter(&event_attrs_filter, list_of_events, current_arrived_event_or_timer_event_for_reference)
                } else {
                    Err("Simple attribute filter expression not found".to_string())
                }
            }
            crate::event_parser::timeexprgrammarparser::AttributeDataFilterExpContextAll::Error(_) => {
                Err("Invalid attribute data filter expression".to_string())
            }
        }
    }

    fn visit_event_attributes_data_filter(&self, ctx: &crate::event_parser::timeexprgrammarparser::EventAttributesDataFilterContextAll, list_of_events: &Vec<Event>, _current_arrived_event_or_timer_event_for_reference: &Event) -> Result<Vec<Event>, String> {
        match ctx {
            crate::event_parser::timeexprgrammarparser::EventAttributesDataFilterContextAll::AttributeExistsContext(exists_ctx) => {
                let attr_name = exists_ctx.attributeName().unwrap().ID().unwrap().get_text();
                let attr_name = attr_name.trim_matches('"');
                let filtered_events: Vec<Event> = list_of_events.iter()
                    .filter(|_event| {
                        self.context.get_attribute(attr_name).is_some()
                    })
                    .cloned()
                    .collect();
                Ok(filtered_events)
            }
            crate::event_parser::timeexprgrammarparser::EventAttributesDataFilterContextAll::AttributeConditionsContext(conditions_ctx) => {
                let attr_name = conditions_ctx.attributeName().unwrap().ID().unwrap().get_text();
                let attr_name = attr_name.trim_matches('"');
                
                let comparison_op = conditions_ctx.comparisonOp().unwrap();
                let op_text = comparison_op.get_text();
                
                let filter_value = conditions_ctx.value().unwrap();
                let value_text = if let Some(int_val) = filter_value.INT() {
                    EventValue::Integer(int_val.get_text().parse().unwrap_or(0))
                } else if let Some(bool_val) = filter_value.BOOL() {
                    let text = bool_val.get_text();
                    EventValue::Boolean(text == "true")
                } else if let Some(string_val) = filter_value.STRING() {
                    let text = string_val.get_text();
                    EventValue::String(text.trim_matches('"').to_string())
                } else {
                    return Err("Unsupported value type in filter".to_string());
                };
                
                let filtered_events: Vec<Event> = list_of_events.iter()
                    .filter(|event| {
                        if let Some(attr_value) = event.attributes.get(attr_name) {
                            if let Ok(result) = self.evaluate_comparison(attr_value, &op_text, &value_text) {
                                result
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    })
                    .cloned()
                    .collect();
                Ok(filtered_events)
            }
            crate::event_parser::timeexprgrammarparser::EventAttributesDataFilterContextAll::Error(_) => {
                Err("Invalid data filter".to_string())
            }
        }
    }

    fn visit_time_expr(&self, ctx: &crate::event_parser::timeexprgrammarparser::TimeExprContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<Option<u64>, String> {
        match ctx {
            crate::event_parser::timeexprgrammarparser::TimeExprContextAll::SinceExprContext(time_expr_ctx) => {
                if let Some(int_token) = time_expr_ctx.INT() {
                    let time_value: u64 = int_token.get_text().parse().unwrap_or(0);
                    if let Some(time_unit) = time_expr_ctx.TIMEUNIT() {
                        let unit_text = time_unit.get_text().to_lowercase();
                        let seconds = match unit_text.as_str() {
                            "second" | "seconds" => time_value,
                            "minute" | "minutes" => time_value * 60,
                            "hour" | "hours" => time_value * 3600,
                            _ => time_value,
                        };
                        
                        // Now handle the time reference part
                        if let Some(time_ref) = time_expr_ctx.timeRef() {
                            let reference_time = self.visit_time_ref(&time_ref, current_arrived_event_or_timer_event_for_reference)?;
                            if let Some(ref_time) = reference_time {
                                // Calculate the absolute time by adding seconds to reference time
                                Ok(Some(ref_time - seconds))
                            } else {
                                Ok(None)
                            }
                        } else {
                            Ok(Some(current_arrived_event_or_timer_event_for_reference.timestamp - seconds))
                        }
                    } else {
                        Ok(Some(current_arrived_event_or_timer_event_for_reference.timestamp - time_value))
                    }
                } else {
                    Err("No time value found in time expression".to_string())
                }
            }
            crate::event_parser::timeexprgrammarparser::TimeExprContextAll::Error(_) => {
                Err("Invalid time expression".to_string())
            }
        }
    }

    fn visit_time_ref(&self, ctx: &crate::event_parser::timeexprgrammarparser::TimeRefContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<Option<u64>, String> {
        match ctx {
            crate::event_parser::timeexprgrammarparser::TimeRefContextAll::NowTimeRefContext(_) => {
                // For now, return current timestamp or a fixed reference
                Ok(Some(current_arrived_event_or_timer_event_for_reference.timestamp)) // Placeholder - should be current time
            }
            crate::event_parser::timeexprgrammarparser::TimeRefContextAll::TimeReferenceWithListOfEventsContext(time_ref_ctx) => {
                if let Some(event_list_expr) = time_ref_ctx.eventListExpr() {
                    // Get the last event (by timestamp, the most recent one) from the specified event list
                    let events = self.visit_event_list_expr(&event_list_expr, current_arrived_event_or_timer_event_for_reference)?;
                    if let Some(first_event) = events.first_key_value() {
                        Ok(Some(first_event.0.timestamp))
                    } else {
                        Ok(None)
                    }
                } else {
                    Ok(None)
                }
            }
            crate::event_parser::timeexprgrammarparser::TimeRefContextAll::Error(_) => {
                Err("Invalid time reference".to_string())
            }
        }
    }

    fn visit_event_list_expr(&self, ctx: &crate::event_parser::timeexprgrammarparser::EventListExprContextAll, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<BTreeMap<EventKey, Event>, String> {
        match ctx {
            crate::event_parser::timeexprgrammarparser::EventListExprContextAll::EventListExpressionContext(list_ctx) => {
                let mut all_events = BTreeMap::new();
                
                // Get all event filter expressions in the list
                let event_filter_exprs = list_ctx.eventFilterExpr_all();
                for event_filter_expr in event_filter_exprs {
                    let events = self.visit_event_filter_expr(&event_filter_expr, current_arrived_event_or_timer_event_for_reference)?;
                    for event in events {
                        all_events.insert(EventKey {timestamp: event.timestamp, name: event.name.clone()}, event);
                    }
                }

                Ok(all_events)
            }
            crate::event_parser::timeexprgrammarparser::EventListExprContextAll::Error(_) => {
                Err("Invalid event list expression".to_string())
            }
        }
    }
}

#[derive(Clone)]
pub struct CompiledTimeExpression {
    pub expression: Rc<ExprContextAll<'static>>,
}


impl CompiledTimeExpression {
    fn new(expression: Rc<ExprContextAll<'static>>) -> Self {
        Self { expression }
    }

    pub fn evaluate(&self, events: &Rc<RefCell<Events>>, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<bool, String> {
        let filter_context = FilterContext::new(Rc::clone(events));        
        let evaluator = FilterEvaluator::new(filter_context);
        evaluator.evaluate(&self.expression, current_arrived_event_or_timer_event_for_reference) 
    }
}


pub struct TimeExpressionParser<'input> {
    parser: TimeExprGrammarParser<'input, CommonTokenStream<'input, TimeExprGrammarLexer<'input, InputStream<&'input str>>>, antlr_rust::error_strategy::DefaultErrorStrategy<'input, timeexprgrammarparser::TimeExprGrammarParserContextType>>,
    
    errors: Rc<RefCell<Vec<String>>>,
}

impl<'input> TimeExpressionParser<'input> {
    pub fn new(input: &'input str) -> Result<Self, ParseError> {
        let input_stream = InputStream::new(input);
        let mut lexer = TimeExprGrammarLexer::new(input_stream);
        let errors = Rc::new(RefCell::new(Vec::new()));
        let error_listener = CustomErrorListener::new(Rc::clone(&errors));
        lexer.remove_error_listeners();
        lexer.add_error_listener(Box::new(error_listener.clone()));
        let token_stream = CommonTokenStream::new(lexer);
        let mut parser = TimeExprGrammarParser::new(token_stream);
        parser.remove_error_listeners();
        parser.add_error_listener(Box::new(error_listener.clone()));
        
        Ok(TimeExpressionParser { 
            parser,
            
            errors,
        })
    }
    
    pub fn parse(&mut self) -> Result<Rc<ExprContextAll<'input>>, ParseError> {
        // Clear previous errors
        self.errors.borrow_mut().clear();
        
        let parse_result = self.parser.expr();

        // Debug print: show errors captured by the error listener
        log::debug!("DEBUG: error_listener.get_errors() = {:?}", self.errors.borrow().clone());

        // Check for parsing errors
        if self.errors.borrow().len() > 0 {
            let errors = self.errors.borrow().clone();
            let error_msg = errors.join("; ");
            return Err(ParseError::new(error_msg));
        }

        // Check that the entire input was consumed (current token is EOF)
        let current_token: &antlr_rust::token::CommonToken = self.parser.get_current_token();
        if current_token.get_token_type() != antlr_rust::token::TOKEN_EOF {
            return Err(ParseError::new("Input was not fully consumed (extra input after valid expression)".to_string()));
        }

        // Check for parse exceptions in a stable way
        if let Ok(parsed_expression) = &parse_result {
            if let Some(exception) = &parsed_expression.exception {
            return Err(ParseError::new(format!("Parsing failed: {:?}", exception)));
            }
        }


        // Only return Ok if there were no syntax errors and input was fully consumed
        match parse_result {
            Ok(parsed_expression) => Ok(parsed_expression),
            Err(e) => Err(ParseError::new(format!("Parsing failed: {:?}", e))),
        }
    }

    pub fn evaluate_filter(&self, expr: &ExprContextAll<'input>, context: FilterContext, current_arrived_event_or_timer_event_for_reference: &Event) -> Result<bool, String> {
        let evaluator = FilterEvaluator::new(context);
        evaluator.evaluate(expr, current_arrived_event_or_timer_event_for_reference)
    }
}

// Implement Clone for CustomErrorListener to satisfy the trait bounds
impl Clone for CustomErrorListener {
    fn clone(&self) -> Self {
        CustomErrorListener {
            errors: Rc::clone(&self.errors),
        }
    }
}

// Convenience function for parsing time expressions
pub fn parse_expression<'input>(input: &'input str) -> Result<Rc<ExprContextAll<'input>>, ParseError> {
    let mut parser = TimeExpressionParser::new(input)?;
    parser.parse()
}

// Convenience function for evaluating filter expressions
pub fn evaluate_filter_expression<'input>(
    input: &'input str, 
    context: FilterContext,
    current_arrived_event_or_timer_event_for_reference: &Event
) -> Result<bool, String> {
    let parsed = parse_expression(input)
        .map_err(|e| format!("Parse error: {}", e))?;
    
    let parser = TimeExpressionParser::new(input)
        .map_err(|e| format!("Parser creation error: {}", e))?;
    
    parser.evaluate_filter(&parsed, context, current_arrived_event_or_timer_event_for_reference)
}

// Convenience function for evaluating filter expressions and returning matching events
pub fn evaluate_filter_event_expr<'input>(
    input: &'input str, 
    context: FilterContext,
    current_arrived_event_or_timer_event_for_reference: &Event
) -> Result<Vec<Event>, String> {
    let parsed = parse_expression(input)
        .map_err(|e| format!("Parse error: {}", e))?;
    
    let evaluator = FilterEvaluator::new(context);
    evaluator.evaluate_events(&parsed, current_arrived_event_or_timer_event_for_reference)
}

/// Recursively collect all event names and wildcard usage from a parsed expression
pub fn collect_events_from_expr<'input>(expr: &ExprContextAll<'input>) -> (HashSet<String>, bool) {
    use crate::event_parser::timeexprgrammarparser::*;
    let mut events = HashSet::new();
    let mut has_wildcard = false;

    fn walk_boolean_expr<'input>(ctx: &BooleanExprContextAll<'input>, events: &mut HashSet<String>, has_wildcard: &mut bool) {
        match ctx {
            BooleanExprContextAll::AndExprContext(and_ctx) => {
                walk_boolean_expr(&and_ctx.booleanExpr(0).unwrap(), events, has_wildcard);
                walk_boolean_expr(&and_ctx.booleanExpr(1).unwrap(), events, has_wildcard);
            }
            BooleanExprContextAll::OrExprContext(or_ctx) => {
                walk_boolean_expr(&or_ctx.booleanExpr(0).unwrap(), events, has_wildcard);
                walk_boolean_expr(&or_ctx.booleanExpr(1).unwrap(), events, has_wildcard);
            }
            BooleanExprContextAll::NotExprContext(not_ctx) => {
                walk_boolean_expr(&not_ctx.booleanExpr().unwrap(), events, has_wildcard);
            }
            BooleanExprContextAll::ParenExprContext(paren_ctx) => {
                walk_boolean_expr(&paren_ctx.booleanExpr().unwrap(), events, has_wildcard);
            }
            BooleanExprContextAll::EventExprAsBoolContext(event_ctx) => {
                if let Some(any_event_expr) = event_ctx.anyEventExpression() {
                    walk_any_event_expr(&any_event_expr, events, has_wildcard);
                }
            }
            BooleanExprContextAll::Error(_) => {}
        }
    }

    fn walk_any_event_expr<'input>(ctx: &AnyEventExpressionContextAll<'input>, events: &mut HashSet<String>, has_wildcard: &mut bool) {
        match ctx {
            AnyEventExpressionContextAll::AnyExistingEventContext(any_event_ctx) => {
                if let Some(event_expr) = any_event_ctx.eventExpr() {
                    walk_event_expr(&event_expr, events, has_wildcard);
                }
            }
            AnyEventExpressionContextAll::Error(_) => {}
        }
    }

    fn walk_event_expr<'input>(ctx: &EventExprContextAll<'input>, events: &mut HashSet<String>, has_wildcard: &mut bool) {
        match ctx {
            EventExprContextAll::EventExprWithTimeResolutionContext(event_ctx) => {
                if let Some(event_filter_expr) = event_ctx.eventFilterExpr() {
                    walk_event_filter_expr(&event_filter_expr, events, has_wildcard);
                }
                // ignore timeExpr for event names
            }
            EventExprContextAll::Error(_) => {}
        }
    }

    fn walk_event_filter_expr<'input>(ctx: &EventFilterExprContextAll<'input>, events: &mut HashSet<String>, has_wildcard: &mut bool) {
        match ctx {
            EventFilterExprContextAll::FilteredEventWithDataAttributesContext(filtered_ctx) => {
                if let Some(event_name_ctx) = filtered_ctx.eventName() {
                    // eventName can be singleEventName or eventStateName
                    if let Some(single_event_name) = event_name_ctx.singleEventName() {
                        if let Some(id) = single_event_name.ID() {
                            events.insert(id.get_text().to_string());
                        }
                    } else if let Some(event_state_name) = event_name_ctx.eventStateName() {
                        if let Some(id) = event_state_name.ID() {
                            events.insert(id.get_text().to_string());
                        }
                    }
                }
            }
            EventFilterExprContextAll::WildcardWithOptionalFilteredEventsContext(_wildcard_ctx) => {
                *has_wildcard = true;
            }
            EventFilterExprContextAll::Error(_) => {}
        }
    }

    if let Some(boolean_expr) = expr.booleanExpr() {
        walk_boolean_expr(&boolean_expr, &mut events, &mut has_wildcard);
    }

    (events, has_wildcard)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_event_expression() {
        let result = parse_expression("event1");
        assert!(result.is_ok(), "Should parse simple event name");
    }

    #[test]
    fn test_event_with_filter() {
        let result = parse_expression("event1#[value > 10]");
        assert!(result.is_ok(), "Should parse event with data filter");
    }

    #[test]
    fn test_time_expression() {
        let result = parse_expression("event1 since 5 minutes from now");
        assert!(result.is_ok(), "Should parse time expression");
    }

    #[test]
    fn test_boolean_expression() {
        let result = parse_expression("event1 AND event2");
        assert!(result.is_ok(), "Should parse boolean expression");
    }

    #[test]
    fn test_complex_expression() {
        let result = parse_expression("(event1 OR event2) AND NOT event3");
        assert!(result.is_ok(), "Should parse complex boolean expression");
    }

    #[test]
    fn test_invalid_expression() {
        let result = parse_expression("since 10 minutes from now");
        assert!(result.is_err(), "Should fail to parse invalid expression");
        if let Err(e) = result {
            println!("Expected error: {}", e);
        }
    }

    #[test]
    fn test_invalid_syntax() {
        let result = parse_expression("last event of {event1, event2}");
        assert!(result.is_err(), "Should fail to parse invalid syntax");
        if let Err(e) = result {
            println!("Expected error: {}", e);
        }
    }

    #[test]
    fn test_filter_with_expressions() {
        let result = parse_expression("(ttt since 1 second from now) or tutu#[(titi==4) or (rr==3 and (vv==3))] or rr");
        assert!(result.is_ok(), "Should parse filter with expressions");
    }

    #[test]
    fn test_filter_evaluation() {
        let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).is_test(true).try_init();
        
        // Create test events
        // no attributes for event1
        let event1 = Event::new("event1".to_string(), false, HashMap::new(), 1000);
        // event2 has value 20
        let mut event2_attributes = HashMap::new();
        event2_attributes.insert("value".to_string(), EventValue::Integer(20));
        let event2 = Event::new("event2".to_string(), false, event2_attributes, 500);
        

        let mut event3_attributes = HashMap::new();
        event3_attributes.insert("value".to_string(), EventValue::Integer(30));
        let event3 = Event::new("event3".to_string(), false, event3_attributes, 100);

        let mut events = Events::new();
        events.push_event(event1.clone());
        events.push_event(event2.clone());
        events.push_event(event3.clone());
        let context = FilterContext::new(Rc::new(RefCell::new(events)));
        
        // Test simple comparison
        let result = evaluate_filter_expression("event1#[value > 10]", context.clone(), &event1);
        assert!(result.is_ok(), "Should evaluate filter successfully");
        assert!(!result.unwrap(), "Should evaluate filter successfully");
        
        // Test string comparison
        let result = evaluate_filter_expression("event1#[status == \"active\"]", context.clone(), &event1);
        assert!(result.is_ok(), "Should evaluate string filter successfully");
        assert!(!result.unwrap(), "Should evaluate string filter successfully, status is not defined");
        
        // Test boolean logic
        log::debug!("Testing boolean logic, there are events with value > 10, and event2 has value 20");
        let result = evaluate_filter_expression("event2#[value > 10] AND event2#[value == 20]", context.clone(), &event2);
        assert!(result.is_ok(), "Should evaluate boolean expression successfully");
        assert!(result.unwrap(), "Should evaluate boolean expression successfully");

        log::debug!("Testing boolean logic, there are events with value > 10, and event2 does not have value of 15");
        let result = evaluate_filter_expression("event2#[value > 10] AND event2#[value == 15]", context.clone(), &event2);
        assert!(result.is_ok(), "Should evaluate boolean expression successfully");
        assert!(!result.unwrap(), "Should evaluate boolean expression successfully");

        log::debug!("Testing boolean logic, there are no event2 with value < 10, or event2 does not have value 10");
        let result = evaluate_filter_expression("event2#[value < 10] OR event2#[value == 10]", context.clone(), &event2);
        assert!(result.is_ok(), "Should evaluate boolean expression successfully");
        assert!(!result.unwrap(), "Should evaluate boolean expression successfully");


        log::debug!("Testing boolean logic, there are no event2 with value < 10, or event2 does not have value 20, testing the or operator in data filter expressions");
        let result = evaluate_filter_expression("event2#[value < 10] OR event2#[value == 20]", context.clone(), &event2);
        assert!(result.is_ok(), "Should evaluate boolean expression successfully");
        assert!(result.unwrap(), "Should evaluate boolean expression successfully");


        let result = evaluate_filter_expression("event2#[value < 10 or value == 20]", context.clone(), &event2);
        assert!(result.is_ok(), "Should evaluate boolean expression successfully");
        assert!(result.unwrap(), "Should evaluate boolean expression successfully");


        let result = evaluate_filter_expression("event4", context.clone(), &event1);
        assert!(result.is_ok(), "Should evaluate filter successfully");
        // should be false because event3 does not exist
        assert!(!result.unwrap(), "Should evaluate filter successfully");


        let result = evaluate_filter_expression("event2 since 600 seconds from last event of [event1]", context.clone(), &event1);
        assert!(result.is_ok(), "Should evaluate filter successfully");
        // should be true because event1 is before 600 seconds from event2
        assert!(result.unwrap(), "Should evaluate filter successfully");

        let result = evaluate_filter_expression("event2 since 10 seconds from last event of [event1]", context.clone(), &event1);
        assert!(result.is_ok(), "Should evaluate filter successfully");
        // should be false because event1 is before 10 seconds from event2
        assert!(!result.unwrap(), "Should evaluate filter negatively, event 2 is not before 10 seconds from event1");



    }


    #[test]
    fn test_benchmark_filter_with_lots_of_events() {
        let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).is_test(true).try_init();

        let mut events = Events::new();
        let timestamp_start = 1000;
        let timestamp_end = 100000;
        
        // Measure setup time
        let setup_start = std::time::Instant::now();
        for i in timestamp_start..timestamp_end {
            let mut attributes = HashMap::new();
            attributes.insert("value".to_string(), EventValue::Integer(i as i64));
            let event = Event::new(format!("event{}", i), false, attributes, i as u64);
            events.push_event(event);
        }
        let setup_duration = setup_start.elapsed();
        println!("Setup time: {:?} for {} events", setup_duration, timestamp_end - timestamp_start);
        
        let context = FilterContext::new(Rc::new(RefCell::new(events)));
        let event1 = Event::new("event1".to_string(), false, HashMap::new(), 1000);
        
        // Measure filter parsing time
        let filter_start = std::time::Instant::now();
        let result = parse_expression("event2 since 10 seconds from last event of [event1]");
        let filter_duration = filter_start.elapsed();
        println!("Filter parsing time: {:?} for filter: event2 since 10 seconds from last event of [event1]", filter_duration);
        
        assert!(result.is_ok(), "Should parse filter successfully");
        
        // Benchmark multiple filter expressions
        let filter_expressions = vec![
            "event1#[value > 10]",
            "event2#[value == 20]",
            "event1 OR event2",
            "event1 AND event2",
            "event2 since 600 seconds from last event of [event100]",
            "* since 600 seconds from last event of [event2]",
            
        ];
        
        println!("\nBenchmarking different filter expressions:");
        for (i, expr) in filter_expressions.iter().enumerate() {
            let start = std::time::Instant::now();
            let result = evaluate_filter_expression(expr, context.clone(), &event1);
            let duration = start.elapsed();
            
            println!("  Filter {}: {:?} - Result: {:?} - {:?}", i + 1, filter_expressions[i], duration, result.is_ok());
        }
    }


    #[test]
    fn test_filter_evaluation_false() {
        let mut context = FilterContext::new(Rc::new(RefCell::new(Events::new())));
        context.add_attribute("value".to_string(), EventValue::Integer(5));
        
        let event1 = Event::new("event1".to_string(), false, HashMap::new(), 1000);
        let result = evaluate_filter_expression("event1#[value > 10]", context, &event1);
        assert!(result.is_ok(), "Should evaluate filter successfully");
        // Note: The result will be empty because the events list is empty in the context
    }

    #[test]
    fn test_evaluate_filter_event_expr() {
        let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).is_test(true).try_init();
        
        // Create test events
        let mut event1_attributes = HashMap::new();
        event1_attributes.insert("value".to_string(), EventValue::Integer(10));
        let event1 = Event::new("event1".to_string(), false, event1_attributes, 1000);
        
        let mut event2_attributes = HashMap::new();
        event2_attributes.insert("value".to_string(), EventValue::Integer(20));
        let event2 = Event::new("event2".to_string(), false, event2_attributes, 500);
        
        let mut event3_attributes = HashMap::new();
        event3_attributes.insert("value".to_string(), EventValue::Integer(30));
        let event3 = Event::new("event3".to_string(), false, event3_attributes, 100);
        
        let mut events = Events::new();
        events.push_event(event1.clone());
        events.push_event(event2.clone());
        events.push_event(event3.clone());
        let context = FilterContext::new(Rc::new(RefCell::new(events)));
        
        // Test getting events by name
        let result = evaluate_filter_event_expr("event1", context.clone(), &event1);
        assert!(result.is_ok(), "Should evaluate filter successfully");
        let events_result = result.unwrap();
        assert_eq!(events_result.len(), 1);
        assert_eq!(events_result[0].name, "event1");
        
        // Test getting events with data filter
        let result = evaluate_filter_event_expr("event2#[value == 20]", context.clone(), &event1);
        assert!(result.is_ok(), "Should evaluate filter successfully");
        let events_result = result.unwrap();
        assert_eq!(events_result.len(), 1);
        assert_eq!(events_result[0].name, "event2");
        assert_eq!(events_result[0].attributes.get("value").unwrap(), &EventValue::Integer(20));
        
        // Test getting events with OR expression
        let result = evaluate_filter_event_expr("event10000000 OR (event2 AND event3)", context.clone(), &event1);
        assert!(result.is_ok(), "Should evaluate filter successfully");
        let events_result = result.unwrap();
        assert!(events_result.len() > 0, "Should evaluate filter successfully");
        log::debug!("events_result from \"event10000000 OR (event2 AND event3)\": {:?}", events_result);
        
        // Test getting events with time expression
        let result = evaluate_filter_event_expr("event2 since 600 seconds from last event of [event1]", context.clone(), &event1);
        assert!(result.is_ok(), "Should evaluate filter successfully");
        let events_result = result.unwrap();
        log::debug!("events_result from \"event2 since 600 seconds from last event of [event1]\": {:?}", events_result);
        assert_eq!(events_result.len(), 1);
        assert_eq!(events_result[0].name, "event2");
        
        // Test getting events with NOT expression
        let result = evaluate_filter_event_expr("NOT event2", context.clone(), &event1);
        assert!(result.is_ok(), "Should evaluate filter successfully");
        let events_result = result.unwrap();
        log::debug!("events_result from \"NOT event2\": {:?}", events_result);
        assert_eq!(events_result.len(), 2);
        assert_eq!(events_result[0].name, "event1");
        assert_eq!(events_result[1].name, "event3");


        let result = evaluate_filter_event_expr("NOT (event2 OR event3)", context.clone(), &event1);
        assert!(result.is_ok(), "Should evaluate filter successfully");
        let events_result = result.unwrap();
        log::debug!("events_result from \"NOT (event2 OR event3)\": {:?}", events_result);
        assert_eq!(events_result.len(), 1);
        assert_eq!(events_result[0].name, "event1");

        println!("All evaluate_filter_event_expr tests passed!");
    }

    #[test]
    fn test_ddl_event_filter_expressions() {
        let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).is_test(true).try_init();
        
        // Create test events that match the DDL conditions
        let mut enrollment_attributes = HashMap::new();
        enrollment_attributes.insert("device_id".to_string(), EventValue::String("factory_display_001".to_string()));
        let enrollment_event = Event::new("enrollment".to_string(), false, enrollment_attributes, 9800);
        
        let mut connection_attributes = HashMap::new();
        connection_attributes.insert("device_id".to_string(), EventValue::String("factory_display_001".to_string()));
        connection_attributes.insert("status".to_string(), EventValue::String("connected".to_string()));
        let connection_event = Event::new("connection_to_cloud".to_string(), false, connection_attributes, 9500);
        
        let mut enrollment_id_attributes = HashMap::new();
        enrollment_id_attributes.insert("device_id".to_string(), EventValue::String("factory_display_001".to_string()));
        enrollment_id_attributes.insert("enrollment_id".to_string(), EventValue::String("enroll_123".to_string()));
        let enrollment_id_event = Event::new("enrollment_id_created".to_string(), false, enrollment_id_attributes, 9000);
        
        let mut connection_trace_attributes = HashMap::new();
        connection_trace_attributes.insert("wifi".to_string(), EventValue::String("factory_display_001".to_string()));
        connection_trace_attributes.insert("status".to_string(), EventValue::Boolean(true));
        let connection_trace_event = Event::new("ConnectionTraceFromDevice".to_string(), false, connection_trace_attributes, 9700);
        
        let mut data_collection_attributes = HashMap::new();
        data_collection_attributes.insert("wifi".to_string(), EventValue::String("factory_display_001".to_string()));
        data_collection_attributes.insert("status".to_string(), EventValue::Boolean(true));
        let data_collection_event = Event::new("DataCollectionSuccess".to_string(), false, data_collection_attributes, 9600);
        
        let mut schedule_missed_attributes = HashMap::new();
        schedule_missed_attributes.insert("wifi".to_string(), EventValue::String("factory_display_001".to_string()));
        schedule_missed_attributes.insert("status".to_string(), EventValue::Boolean(true));
        let schedule_missed_event = Event::new("ScheduleMissed".to_string(), false, schedule_missed_attributes, 9900);
        
        let mut events = Events::new();
        events.push_event(enrollment_event.clone());
        events.push_event(connection_event.clone());
        events.push_event(enrollment_id_event.clone());
        events.push_event(connection_trace_event.clone());
        events.push_event(data_collection_event.clone());
        events.push_event(schedule_missed_event.clone());
        
        let context = FilterContext::new(Rc::new(RefCell::new(events)));
        let reference_event = Event::new("reference".to_string(), false, HashMap::new(), 10000);
        
        // Test unconnected state: NOT (enrollment since 2 hours from now OR connection_to_cloud since 2 hours from now)
        let result = evaluate_filter_event_expr("NOT (enrollment since 2 hours from now OR connection_to_cloud since 2 hours from now)", context.clone(), &reference_event);
        assert!(result.is_ok(), "Should parse unconnected condition");
        let events_result = result.unwrap();
        log::debug!("unconnected condition result: {:?}", events_result);
        
        // Test un_enrolled state: NOT enrollment_id_created
        let result = evaluate_filter_event_expr("NOT enrollment_id_created", context.clone(), &reference_event);
        assert!(result.is_ok(), "Should parse un_enrolled condition");
        let events_result = result.unwrap();
        log::debug!("un_enrolled condition result: {:?}", events_result);
        
        // Test unconnected_enrolled state: enrollment_id_created AND NOT (connection_to_cloud since 1 hour from now)
        let result = evaluate_filter_event_expr("enrollment_id_created AND NOT (connection_to_cloud since 1 hour from now)", context.clone(), &reference_event);
        assert!(result.is_ok(), "Should parse unconnected_enrolled condition");
        let events_result = result.unwrap();
        log::debug!("unconnected_enrolled condition result: {:?}", events_result);
        
        // Test operating state: ConnectionTraceFromDevice#[wifi == "factory_display_001" AND status == true] AND DataCollectionSuccess#[wifi == "factory_display_001" AND status == true] since 2 hours from now
        let result = evaluate_filter_event_expr("ConnectionTraceFromDevice#[wifi == \"factory_display_001\" AND status == true] AND DataCollectionSuccess#[wifi == \"factory_display_001\" AND status == true] since 2 hours from now", context.clone(), &reference_event);
        assert!(result.is_ok(), "Should parse operating condition");
        let events_result = result.unwrap();
        log::debug!("operating condition result: {:?}", events_result);
        
        // Test missed_schedule state: ScheduleMissed#[wifi == "factory_display_001" AND status == true]
        let result = evaluate_filter_event_expr("ScheduleMissed#[wifi == \"factory_display_001\" AND status == true]", context.clone(), &reference_event);
        assert!(result.is_ok(), "Should parse missed_schedule condition");
        let events_result = result.unwrap();
        log::debug!("missed_schedule condition result: {:?}", events_result);
        
        println!("All DDL event filter expressions parsed successfully!");
    }

    #[test]
    fn test_boolean_literal_parsing() {
        let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).is_test(true).try_init();
        
        // Test simple boolean comparison
        let result = parse_expression("event1#[status == true]");
        assert!(result.is_ok(), "Should parse boolean literal 'true'");
        
        let result = parse_expression("event1#[status == false]");
        assert!(result.is_ok(), "Should parse boolean literal 'false'");
        
        // Test boolean in AND expression
        let result = parse_expression("event1#[status == true AND value > 10]");
        assert!(result.is_ok(), "Should parse boolean literal in AND expression");
        
        println!("Boolean literal parsing tests passed!");
    }
} 