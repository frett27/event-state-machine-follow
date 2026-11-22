#![allow(nonstandard_style)]
// Generated from TimeExprGrammar.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::timeexprgrammarparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link TimeExprGrammarParser}.
 */
pub trait TimeExprGrammarVisitor<'input>: ParseTreeVisitor<'input,TimeExprGrammarParserContextType>{
	/**
	 * Visit a parse tree produced by {@link TimeExprGrammarParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_expr(&mut self, ctx: &ExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code AndExpr}
	 * labeled alternative in {@link TimeExprGrammarParser#booleanExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_AndExpr(&mut self, ctx: &AndExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code EventExprAsBool}
	 * labeled alternative in {@link TimeExprGrammarParser#booleanExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_EventExprAsBool(&mut self, ctx: &EventExprAsBoolContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code NotExpr}
	 * labeled alternative in {@link TimeExprGrammarParser#booleanExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_NotExpr(&mut self, ctx: &NotExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ParenExpr}
	 * labeled alternative in {@link TimeExprGrammarParser#booleanExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_ParenExpr(&mut self, ctx: &ParenExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code OrExpr}
	 * labeled alternative in {@link TimeExprGrammarParser#booleanExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_OrExpr(&mut self, ctx: &OrExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code AnyExistingEvent}
	 * labeled alternative in {@link TimeExprGrammarParser#anyEventExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_AnyExistingEvent(&mut self, ctx: &AnyExistingEventContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code EventExprWithTimeResolution}
	 * labeled alternative in {@link TimeExprGrammarParser#eventExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_EventExprWithTimeResolution(&mut self, ctx: &EventExprWithTimeResolutionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code WildcardWithOptionalFilteredEvents}
	 * labeled alternative in {@link TimeExprGrammarParser#eventFilterExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_WildcardWithOptionalFilteredEvents(&mut self, ctx: &WildcardWithOptionalFilteredEventsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code FilteredEventWithDataAttributes}
	 * labeled alternative in {@link TimeExprGrammarParser#eventFilterExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_FilteredEventWithDataAttributes(&mut self, ctx: &FilteredEventWithDataAttributesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code EventListExpression}
	 * labeled alternative in {@link TimeExprGrammarParser#eventListExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_EventListExpression(&mut self, ctx: &EventListExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code FilterAttributeAND}
	 * labeled alternative in {@link TimeExprGrammarParser#attributeDataFilterExp}.
	 * @param ctx the parse tree
	 */
	fn visit_FilterAttributeAND(&mut self, ctx: &FilterAttributeANDContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code FilterAttributeOR}
	 * labeled alternative in {@link TimeExprGrammarParser#attributeDataFilterExp}.
	 * @param ctx the parse tree
	 */
	fn visit_FilterAttributeOR(&mut self, ctx: &FilterAttributeORContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code SimpleAttributeFilterExpression}
	 * labeled alternative in {@link TimeExprGrammarParser#attributeDataFilterExp}.
	 * @param ctx the parse tree
	 */
	fn visit_SimpleAttributeFilterExpression(&mut self, ctx: &SimpleAttributeFilterExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ParentAttribute}
	 * labeled alternative in {@link TimeExprGrammarParser#attributeDataFilterExp}.
	 * @param ctx the parse tree
	 */
	fn visit_ParentAttribute(&mut self, ctx: &ParentAttributeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code AttributeConditions}
	 * labeled alternative in {@link TimeExprGrammarParser#eventAttributesDataFilter}.
	 * @param ctx the parse tree
	 */
	fn visit_AttributeConditions(&mut self, ctx: &AttributeConditionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code AttributeExists}
	 * labeled alternative in {@link TimeExprGrammarParser#eventAttributesDataFilter}.
	 * @param ctx the parse tree
	 */
	fn visit_AttributeExists(&mut self, ctx: &AttributeExistsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TimeExprGrammarParser#attributeName}.
	 * @param ctx the parse tree
	 */
	fn visit_attributeName(&mut self, ctx: &AttributeNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code SinceExpr}
	 * labeled alternative in {@link TimeExprGrammarParser#timeExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_SinceExpr(&mut self, ctx: &SinceExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TimeExprGrammarParser#value}.
	 * @param ctx the parse tree
	 */
	fn visit_value(&mut self, ctx: &ValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code NowTimeRef}
	 * labeled alternative in {@link TimeExprGrammarParser#timeRef}.
	 * @param ctx the parse tree
	 */
	fn visit_NowTimeRef(&mut self, ctx: &NowTimeRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code TimeReferenceWithListOfEvents}
	 * labeled alternative in {@link TimeExprGrammarParser#timeRef}.
	 * @param ctx the parse tree
	 */
	fn visit_TimeReferenceWithListOfEvents(&mut self, ctx: &TimeReferenceWithListOfEventsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TimeExprGrammarParser#comparisonOp}.
	 * @param ctx the parse tree
	 */
	fn visit_comparisonOp(&mut self, ctx: &ComparisonOpContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TimeExprGrammarParser#logicalOp}.
	 * @param ctx the parse tree
	 */
	fn visit_logicalOp(&mut self, ctx: &LogicalOpContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TimeExprGrammarParser#eventName}.
	 * @param ctx the parse tree
	 */
	fn visit_eventName(&mut self, ctx: &EventNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TimeExprGrammarParser#singleEventName}.
	 * @param ctx the parse tree
	 */
	fn visit_singleEventName(&mut self, ctx: &SingleEventNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TimeExprGrammarParser#eventStateName}.
	 * @param ctx the parse tree
	 */
	fn visit_eventStateName(&mut self, ctx: &EventStateNameContext<'input>) { self.visit_children(ctx) }

}

pub trait TimeExprGrammarVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= TimeExprGrammarParserContextType>{
	/**
	 * Visit a parse tree produced by {@link TimeExprGrammarParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_expr(&mut self, ctx: &ExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code AndExpr}
	 * labeled alternative in {@link TimeExprGrammarParser#booleanExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_AndExpr(&mut self, ctx: &AndExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code EventExprAsBool}
	 * labeled alternative in {@link TimeExprGrammarParser#booleanExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_EventExprAsBool(&mut self, ctx: &EventExprAsBoolContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code NotExpr}
	 * labeled alternative in {@link TimeExprGrammarParser#booleanExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_NotExpr(&mut self, ctx: &NotExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ParenExpr}
	 * labeled alternative in {@link TimeExprGrammarParser#booleanExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_ParenExpr(&mut self, ctx: &ParenExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code OrExpr}
	 * labeled alternative in {@link TimeExprGrammarParser#booleanExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_OrExpr(&mut self, ctx: &OrExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code AnyExistingEvent}
	 * labeled alternative in {@link TimeExprGrammarParser#anyEventExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_AnyExistingEvent(&mut self, ctx: &AnyExistingEventContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code EventExprWithTimeResolution}
	 * labeled alternative in {@link TimeExprGrammarParser#eventExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_EventExprWithTimeResolution(&mut self, ctx: &EventExprWithTimeResolutionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code WildcardWithOptionalFilteredEvents}
	 * labeled alternative in {@link TimeExprGrammarParser#eventFilterExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_WildcardWithOptionalFilteredEvents(&mut self, ctx: &WildcardWithOptionalFilteredEventsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code FilteredEventWithDataAttributes}
	 * labeled alternative in {@link TimeExprGrammarParser#eventFilterExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_FilteredEventWithDataAttributes(&mut self, ctx: &FilteredEventWithDataAttributesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code EventListExpression}
	 * labeled alternative in {@link TimeExprGrammarParser#eventListExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_EventListExpression(&mut self, ctx: &EventListExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code FilterAttributeAND}
	 * labeled alternative in {@link TimeExprGrammarParser#attributeDataFilterExp}.
	 * @param ctx the parse tree
	 */
		fn visit_FilterAttributeAND(&mut self, ctx: &FilterAttributeANDContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code FilterAttributeOR}
	 * labeled alternative in {@link TimeExprGrammarParser#attributeDataFilterExp}.
	 * @param ctx the parse tree
	 */
		fn visit_FilterAttributeOR(&mut self, ctx: &FilterAttributeORContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code SimpleAttributeFilterExpression}
	 * labeled alternative in {@link TimeExprGrammarParser#attributeDataFilterExp}.
	 * @param ctx the parse tree
	 */
		fn visit_SimpleAttributeFilterExpression(&mut self, ctx: &SimpleAttributeFilterExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ParentAttribute}
	 * labeled alternative in {@link TimeExprGrammarParser#attributeDataFilterExp}.
	 * @param ctx the parse tree
	 */
		fn visit_ParentAttribute(&mut self, ctx: &ParentAttributeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code AttributeConditions}
	 * labeled alternative in {@link TimeExprGrammarParser#eventAttributesDataFilter}.
	 * @param ctx the parse tree
	 */
		fn visit_AttributeConditions(&mut self, ctx: &AttributeConditionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code AttributeExists}
	 * labeled alternative in {@link TimeExprGrammarParser#eventAttributesDataFilter}.
	 * @param ctx the parse tree
	 */
		fn visit_AttributeExists(&mut self, ctx: &AttributeExistsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TimeExprGrammarParser#attributeName}.
	 * @param ctx the parse tree
	 */
		fn visit_attributeName(&mut self, ctx: &AttributeNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code SinceExpr}
	 * labeled alternative in {@link TimeExprGrammarParser#timeExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_SinceExpr(&mut self, ctx: &SinceExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TimeExprGrammarParser#value}.
	 * @param ctx the parse tree
	 */
		fn visit_value(&mut self, ctx: &ValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code NowTimeRef}
	 * labeled alternative in {@link TimeExprGrammarParser#timeRef}.
	 * @param ctx the parse tree
	 */
		fn visit_NowTimeRef(&mut self, ctx: &NowTimeRefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code TimeReferenceWithListOfEvents}
	 * labeled alternative in {@link TimeExprGrammarParser#timeRef}.
	 * @param ctx the parse tree
	 */
		fn visit_TimeReferenceWithListOfEvents(&mut self, ctx: &TimeReferenceWithListOfEventsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TimeExprGrammarParser#comparisonOp}.
	 * @param ctx the parse tree
	 */
		fn visit_comparisonOp(&mut self, ctx: &ComparisonOpContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TimeExprGrammarParser#logicalOp}.
	 * @param ctx the parse tree
	 */
		fn visit_logicalOp(&mut self, ctx: &LogicalOpContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TimeExprGrammarParser#eventName}.
	 * @param ctx the parse tree
	 */
		fn visit_eventName(&mut self, ctx: &EventNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TimeExprGrammarParser#singleEventName}.
	 * @param ctx the parse tree
	 */
		fn visit_singleEventName(&mut self, ctx: &SingleEventNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TimeExprGrammarParser#eventStateName}.
	 * @param ctx the parse tree
	 */
		fn visit_eventStateName(&mut self, ctx: &EventStateNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> TimeExprGrammarVisitor<'input> for T
where
	T: TimeExprGrammarVisitorCompat<'input> + ParseTreeVisitor<'input, TimeExprGrammarParserContextType>
{
	fn visit_expr(&mut self, ctx: &ExprContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_expr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_AndExpr(&mut self, ctx: &AndExprContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_AndExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_EventExprAsBool(&mut self, ctx: &EventExprAsBoolContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_EventExprAsBool(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_NotExpr(&mut self, ctx: &NotExprContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_NotExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ParenExpr(&mut self, ctx: &ParenExprContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_ParenExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_OrExpr(&mut self, ctx: &OrExprContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_OrExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_AnyExistingEvent(&mut self, ctx: &AnyExistingEventContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_AnyExistingEvent(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_EventExprWithTimeResolution(&mut self, ctx: &EventExprWithTimeResolutionContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_EventExprWithTimeResolution(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_WildcardWithOptionalFilteredEvents(&mut self, ctx: &WildcardWithOptionalFilteredEventsContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_WildcardWithOptionalFilteredEvents(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_FilteredEventWithDataAttributes(&mut self, ctx: &FilteredEventWithDataAttributesContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_FilteredEventWithDataAttributes(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_EventListExpression(&mut self, ctx: &EventListExpressionContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_EventListExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_FilterAttributeAND(&mut self, ctx: &FilterAttributeANDContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_FilterAttributeAND(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_FilterAttributeOR(&mut self, ctx: &FilterAttributeORContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_FilterAttributeOR(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_SimpleAttributeFilterExpression(&mut self, ctx: &SimpleAttributeFilterExpressionContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_SimpleAttributeFilterExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ParentAttribute(&mut self, ctx: &ParentAttributeContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_ParentAttribute(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_AttributeConditions(&mut self, ctx: &AttributeConditionsContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_AttributeConditions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_AttributeExists(&mut self, ctx: &AttributeExistsContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_AttributeExists(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_attributeName(&mut self, ctx: &AttributeNameContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_attributeName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_SinceExpr(&mut self, ctx: &SinceExprContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_SinceExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_value(&mut self, ctx: &ValueContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_value(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_NowTimeRef(&mut self, ctx: &NowTimeRefContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_NowTimeRef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_TimeReferenceWithListOfEvents(&mut self, ctx: &TimeReferenceWithListOfEventsContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_TimeReferenceWithListOfEvents(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comparisonOp(&mut self, ctx: &ComparisonOpContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_comparisonOp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logicalOp(&mut self, ctx: &LogicalOpContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_logicalOp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_eventName(&mut self, ctx: &EventNameContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_eventName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_singleEventName(&mut self, ctx: &SingleEventNameContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_singleEventName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_eventStateName(&mut self, ctx: &EventStateNameContext<'input>){
		let result = <Self as TimeExprGrammarVisitorCompat>::visit_eventStateName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}