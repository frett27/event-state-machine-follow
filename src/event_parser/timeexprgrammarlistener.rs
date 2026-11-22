#![allow(nonstandard_style)]
// Generated from TimeExprGrammar.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::timeexprgrammarparser::*;

pub trait TimeExprGrammarListener<'input> : ParseTreeListener<'input,TimeExprGrammarParserContextType>{
/**
 * Enter a parse tree produced by {@link TimeExprGrammarParser#expr}.
 * @param ctx the parse tree
 */
fn enter_expr(&mut self, _ctx: &ExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TimeExprGrammarParser#expr}.
 * @param ctx the parse tree
 */
fn exit_expr(&mut self, _ctx: &ExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code AndExpr}
 * labeled alternative in {@link TimeExprGrammarParser#booleanExpr}.
 * @param ctx the parse tree
 */
fn enter_AndExpr(&mut self, _ctx: &AndExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code AndExpr}
 * labeled alternative in {@link TimeExprGrammarParser#booleanExpr}.
 * @param ctx the parse tree
 */
fn exit_AndExpr(&mut self, _ctx: &AndExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code EventExprAsBool}
 * labeled alternative in {@link TimeExprGrammarParser#booleanExpr}.
 * @param ctx the parse tree
 */
fn enter_EventExprAsBool(&mut self, _ctx: &EventExprAsBoolContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code EventExprAsBool}
 * labeled alternative in {@link TimeExprGrammarParser#booleanExpr}.
 * @param ctx the parse tree
 */
fn exit_EventExprAsBool(&mut self, _ctx: &EventExprAsBoolContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code NotExpr}
 * labeled alternative in {@link TimeExprGrammarParser#booleanExpr}.
 * @param ctx the parse tree
 */
fn enter_NotExpr(&mut self, _ctx: &NotExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code NotExpr}
 * labeled alternative in {@link TimeExprGrammarParser#booleanExpr}.
 * @param ctx the parse tree
 */
fn exit_NotExpr(&mut self, _ctx: &NotExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ParenExpr}
 * labeled alternative in {@link TimeExprGrammarParser#booleanExpr}.
 * @param ctx the parse tree
 */
fn enter_ParenExpr(&mut self, _ctx: &ParenExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ParenExpr}
 * labeled alternative in {@link TimeExprGrammarParser#booleanExpr}.
 * @param ctx the parse tree
 */
fn exit_ParenExpr(&mut self, _ctx: &ParenExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code OrExpr}
 * labeled alternative in {@link TimeExprGrammarParser#booleanExpr}.
 * @param ctx the parse tree
 */
fn enter_OrExpr(&mut self, _ctx: &OrExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code OrExpr}
 * labeled alternative in {@link TimeExprGrammarParser#booleanExpr}.
 * @param ctx the parse tree
 */
fn exit_OrExpr(&mut self, _ctx: &OrExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code AnyExistingEvent}
 * labeled alternative in {@link TimeExprGrammarParser#anyEventExpression}.
 * @param ctx the parse tree
 */
fn enter_AnyExistingEvent(&mut self, _ctx: &AnyExistingEventContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code AnyExistingEvent}
 * labeled alternative in {@link TimeExprGrammarParser#anyEventExpression}.
 * @param ctx the parse tree
 */
fn exit_AnyExistingEvent(&mut self, _ctx: &AnyExistingEventContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code EventExprWithTimeResolution}
 * labeled alternative in {@link TimeExprGrammarParser#eventExpr}.
 * @param ctx the parse tree
 */
fn enter_EventExprWithTimeResolution(&mut self, _ctx: &EventExprWithTimeResolutionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code EventExprWithTimeResolution}
 * labeled alternative in {@link TimeExprGrammarParser#eventExpr}.
 * @param ctx the parse tree
 */
fn exit_EventExprWithTimeResolution(&mut self, _ctx: &EventExprWithTimeResolutionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code WildcardWithOptionalFilteredEvents}
 * labeled alternative in {@link TimeExprGrammarParser#eventFilterExpr}.
 * @param ctx the parse tree
 */
fn enter_WildcardWithOptionalFilteredEvents(&mut self, _ctx: &WildcardWithOptionalFilteredEventsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code WildcardWithOptionalFilteredEvents}
 * labeled alternative in {@link TimeExprGrammarParser#eventFilterExpr}.
 * @param ctx the parse tree
 */
fn exit_WildcardWithOptionalFilteredEvents(&mut self, _ctx: &WildcardWithOptionalFilteredEventsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code FilteredEventWithDataAttributes}
 * labeled alternative in {@link TimeExprGrammarParser#eventFilterExpr}.
 * @param ctx the parse tree
 */
fn enter_FilteredEventWithDataAttributes(&mut self, _ctx: &FilteredEventWithDataAttributesContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code FilteredEventWithDataAttributes}
 * labeled alternative in {@link TimeExprGrammarParser#eventFilterExpr}.
 * @param ctx the parse tree
 */
fn exit_FilteredEventWithDataAttributes(&mut self, _ctx: &FilteredEventWithDataAttributesContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code EventListExpression}
 * labeled alternative in {@link TimeExprGrammarParser#eventListExpr}.
 * @param ctx the parse tree
 */
fn enter_EventListExpression(&mut self, _ctx: &EventListExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code EventListExpression}
 * labeled alternative in {@link TimeExprGrammarParser#eventListExpr}.
 * @param ctx the parse tree
 */
fn exit_EventListExpression(&mut self, _ctx: &EventListExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code FilterAttributeAND}
 * labeled alternative in {@link TimeExprGrammarParser#attributeDataFilterExp}.
 * @param ctx the parse tree
 */
fn enter_FilterAttributeAND(&mut self, _ctx: &FilterAttributeANDContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code FilterAttributeAND}
 * labeled alternative in {@link TimeExprGrammarParser#attributeDataFilterExp}.
 * @param ctx the parse tree
 */
fn exit_FilterAttributeAND(&mut self, _ctx: &FilterAttributeANDContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code FilterAttributeOR}
 * labeled alternative in {@link TimeExprGrammarParser#attributeDataFilterExp}.
 * @param ctx the parse tree
 */
fn enter_FilterAttributeOR(&mut self, _ctx: &FilterAttributeORContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code FilterAttributeOR}
 * labeled alternative in {@link TimeExprGrammarParser#attributeDataFilterExp}.
 * @param ctx the parse tree
 */
fn exit_FilterAttributeOR(&mut self, _ctx: &FilterAttributeORContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code SimpleAttributeFilterExpression}
 * labeled alternative in {@link TimeExprGrammarParser#attributeDataFilterExp}.
 * @param ctx the parse tree
 */
fn enter_SimpleAttributeFilterExpression(&mut self, _ctx: &SimpleAttributeFilterExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code SimpleAttributeFilterExpression}
 * labeled alternative in {@link TimeExprGrammarParser#attributeDataFilterExp}.
 * @param ctx the parse tree
 */
fn exit_SimpleAttributeFilterExpression(&mut self, _ctx: &SimpleAttributeFilterExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ParentAttribute}
 * labeled alternative in {@link TimeExprGrammarParser#attributeDataFilterExp}.
 * @param ctx the parse tree
 */
fn enter_ParentAttribute(&mut self, _ctx: &ParentAttributeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ParentAttribute}
 * labeled alternative in {@link TimeExprGrammarParser#attributeDataFilterExp}.
 * @param ctx the parse tree
 */
fn exit_ParentAttribute(&mut self, _ctx: &ParentAttributeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code AttributeConditions}
 * labeled alternative in {@link TimeExprGrammarParser#eventAttributesDataFilter}.
 * @param ctx the parse tree
 */
fn enter_AttributeConditions(&mut self, _ctx: &AttributeConditionsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code AttributeConditions}
 * labeled alternative in {@link TimeExprGrammarParser#eventAttributesDataFilter}.
 * @param ctx the parse tree
 */
fn exit_AttributeConditions(&mut self, _ctx: &AttributeConditionsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code AttributeExists}
 * labeled alternative in {@link TimeExprGrammarParser#eventAttributesDataFilter}.
 * @param ctx the parse tree
 */
fn enter_AttributeExists(&mut self, _ctx: &AttributeExistsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code AttributeExists}
 * labeled alternative in {@link TimeExprGrammarParser#eventAttributesDataFilter}.
 * @param ctx the parse tree
 */
fn exit_AttributeExists(&mut self, _ctx: &AttributeExistsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TimeExprGrammarParser#attributeName}.
 * @param ctx the parse tree
 */
fn enter_attributeName(&mut self, _ctx: &AttributeNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TimeExprGrammarParser#attributeName}.
 * @param ctx the parse tree
 */
fn exit_attributeName(&mut self, _ctx: &AttributeNameContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code SinceExpr}
 * labeled alternative in {@link TimeExprGrammarParser#timeExpr}.
 * @param ctx the parse tree
 */
fn enter_SinceExpr(&mut self, _ctx: &SinceExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code SinceExpr}
 * labeled alternative in {@link TimeExprGrammarParser#timeExpr}.
 * @param ctx the parse tree
 */
fn exit_SinceExpr(&mut self, _ctx: &SinceExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TimeExprGrammarParser#value}.
 * @param ctx the parse tree
 */
fn enter_value(&mut self, _ctx: &ValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TimeExprGrammarParser#value}.
 * @param ctx the parse tree
 */
fn exit_value(&mut self, _ctx: &ValueContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code NowTimeRef}
 * labeled alternative in {@link TimeExprGrammarParser#timeRef}.
 * @param ctx the parse tree
 */
fn enter_NowTimeRef(&mut self, _ctx: &NowTimeRefContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code NowTimeRef}
 * labeled alternative in {@link TimeExprGrammarParser#timeRef}.
 * @param ctx the parse tree
 */
fn exit_NowTimeRef(&mut self, _ctx: &NowTimeRefContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code TimeReferenceWithListOfEvents}
 * labeled alternative in {@link TimeExprGrammarParser#timeRef}.
 * @param ctx the parse tree
 */
fn enter_TimeReferenceWithListOfEvents(&mut self, _ctx: &TimeReferenceWithListOfEventsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code TimeReferenceWithListOfEvents}
 * labeled alternative in {@link TimeExprGrammarParser#timeRef}.
 * @param ctx the parse tree
 */
fn exit_TimeReferenceWithListOfEvents(&mut self, _ctx: &TimeReferenceWithListOfEventsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TimeExprGrammarParser#comparisonOp}.
 * @param ctx the parse tree
 */
fn enter_comparisonOp(&mut self, _ctx: &ComparisonOpContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TimeExprGrammarParser#comparisonOp}.
 * @param ctx the parse tree
 */
fn exit_comparisonOp(&mut self, _ctx: &ComparisonOpContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TimeExprGrammarParser#logicalOp}.
 * @param ctx the parse tree
 */
fn enter_logicalOp(&mut self, _ctx: &LogicalOpContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TimeExprGrammarParser#logicalOp}.
 * @param ctx the parse tree
 */
fn exit_logicalOp(&mut self, _ctx: &LogicalOpContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TimeExprGrammarParser#eventName}.
 * @param ctx the parse tree
 */
fn enter_eventName(&mut self, _ctx: &EventNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TimeExprGrammarParser#eventName}.
 * @param ctx the parse tree
 */
fn exit_eventName(&mut self, _ctx: &EventNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TimeExprGrammarParser#singleEventName}.
 * @param ctx the parse tree
 */
fn enter_singleEventName(&mut self, _ctx: &SingleEventNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TimeExprGrammarParser#singleEventName}.
 * @param ctx the parse tree
 */
fn exit_singleEventName(&mut self, _ctx: &SingleEventNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TimeExprGrammarParser#eventStateName}.
 * @param ctx the parse tree
 */
fn enter_eventStateName(&mut self, _ctx: &EventStateNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TimeExprGrammarParser#eventStateName}.
 * @param ctx the parse tree
 */
fn exit_eventStateName(&mut self, _ctx: &EventStateNameContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : TimeExprGrammarListener<'input> }


