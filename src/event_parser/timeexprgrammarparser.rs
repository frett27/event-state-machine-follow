// Generated from TimeExprGrammar.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::timeexprgrammarlistener::*;
use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const T__2:isize=3; 
		pub const T__3:isize=4; 
		pub const T__4:isize=5; 
		pub const T__5:isize=6; 
		pub const T__6:isize=7; 
		pub const T__7:isize=8; 
		pub const STRING:isize=9; 
		pub const LEFTSB:isize=10; 
		pub const RIGHTSB:isize=11; 
		pub const HASH:isize=12; 
		pub const IMPERATIVEMARK:isize=13; 
		pub const WILDCARD:isize=14; 
		pub const LPAR:isize=15; 
		pub const RPAR:isize=16; 
		pub const TIMEUNIT:isize=17; 
		pub const BUT:isize=18; 
		pub const NOT:isize=19; 
		pub const AND:isize=20; 
		pub const OR:isize=21; 
		pub const NOW:isize=22; 
		pub const SINCE:isize=23; 
		pub const FROM:isize=24; 
		pub const LAST:isize=25; 
		pub const EVENT:isize=26; 
		pub const OF:isize=27; 
		pub const WITHIN:isize=28; 
		pub const BOOL:isize=29; 
		pub const ID:isize=30; 
		pub const INT:isize=31; 
		pub const WS:isize=32;
	pub const RULE_expr:usize = 0; 
	pub const RULE_booleanExpr:usize = 1; 
	pub const RULE_anyEventExpression:usize = 2; 
	pub const RULE_eventExpr:usize = 3; 
	pub const RULE_eventFilterExpr:usize = 4; 
	pub const RULE_eventListExpr:usize = 5; 
	pub const RULE_attributeDataFilterExp:usize = 6; 
	pub const RULE_eventAttributesDataFilter:usize = 7; 
	pub const RULE_attributeName:usize = 8; 
	pub const RULE_timeExpr:usize = 9; 
	pub const RULE_value:usize = 10; 
	pub const RULE_timeRef:usize = 11; 
	pub const RULE_comparisonOp:usize = 12; 
	pub const RULE_logicalOp:usize = 13; 
	pub const RULE_eventName:usize = 14; 
	pub const RULE_singleEventName:usize = 15; 
	pub const RULE_eventStateName:usize = 16;
	pub const ruleNames: [&'static str; 17] =  [
		"expr", "booleanExpr", "anyEventExpression", "eventExpr", "eventFilterExpr", 
		"eventListExpr", "attributeDataFilterExp", "eventAttributesDataFilter", 
		"attributeName", "timeExpr", "value", "timeRef", "comparisonOp", "logicalOp", 
		"eventName", "singleEventName", "eventStateName"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;29] = [
		None, Some("','"), Some("'>'"), Some("'<'"), Some("'>='"), Some("'<='"), 
		Some("'=='"), Some("'!='"), Some("'@'"), None, Some("'['"), Some("']'"), 
		Some("'#'"), Some("'!'"), Some("'*'"), Some("'('"), Some("')'"), None, 
		None, None, None, None, Some("'now'"), Some("'since'"), Some("'from'"), 
		Some("'last'"), Some("'event'"), Some("'of'"), Some("'within'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;33]  = [
		None, None, None, None, None, None, None, None, None, Some("STRING"), 
		Some("LEFTSB"), Some("RIGHTSB"), Some("HASH"), Some("IMPERATIVEMARK"), 
		Some("WILDCARD"), Some("LPAR"), Some("RPAR"), Some("TIMEUNIT"), Some("BUT"), 
		Some("NOT"), Some("AND"), Some("OR"), Some("NOW"), Some("SINCE"), Some("FROM"), 
		Some("LAST"), Some("EVENT"), Some("OF"), Some("WITHIN"), Some("BOOL"), 
		Some("ID"), Some("INT"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,TimeExprGrammarParserExt<'input>, I, TimeExprGrammarParserContextType , dyn TimeExprGrammarListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type TimeExprGrammarTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, TimeExprGrammarParserContextType , dyn TimeExprGrammarListener<'input> + 'a>;

/// Parser for TimeExprGrammar grammar
pub struct TimeExprGrammarParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> TimeExprGrammarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				TimeExprGrammarParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> TimeExprGrammarParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> TimeExprGrammarParser<'input, I, DefaultErrorStrategy<'input,TimeExprGrammarParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for TimeExprGrammarParser
pub trait TimeExprGrammarParserContext<'input>:
	for<'x> Listenable<dyn TimeExprGrammarListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=TimeExprGrammarParserContextType>
{}

antlr_rust::coerce_from!{ 'input : TimeExprGrammarParserContext<'input> }

impl<'input> TimeExprGrammarParserContext<'input> for TerminalNode<'input,TimeExprGrammarParserContextType> {}
impl<'input> TimeExprGrammarParserContext<'input> for ErrorNode<'input,TimeExprGrammarParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn TimeExprGrammarParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn TimeExprGrammarListener<'input> + 'input }

pub struct TimeExprGrammarParserContextType;
antlr_rust::tid!{TimeExprGrammarParserContextType}

impl<'input> ParserNodeType<'input> for TimeExprGrammarParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn TimeExprGrammarParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for TimeExprGrammarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for TimeExprGrammarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct TimeExprGrammarParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> TimeExprGrammarParserExt<'input>{
}
antlr_rust::tid! { TimeExprGrammarParserExt<'a> }

impl<'input> TokenAware<'input> for TimeExprGrammarParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for TimeExprGrammarParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for TimeExprGrammarParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "TimeExprGrammar.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn TimeExprGrammarParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					1 => TimeExprGrammarParser::<'input,I,_>::booleanExpr_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					6 => TimeExprGrammarParser::<'input,I,_>::attributeDataFilterExp_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> TimeExprGrammarParser<'input, I, DefaultErrorStrategy<'input,TimeExprGrammarParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn booleanExpr_sempred(_localctx: Option<&BooleanExprContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 5)
				}
				1=>{
					recog.precpred(None, 4)
				}
			_ => true
		}
	}
	fn attributeDataFilterExp_sempred(_localctx: Option<&AttributeDataFilterExpContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				2=>{
					recog.precpred(None, 4)
				}
				3=>{
					recog.precpred(None, 3)
				}
			_ => true
		}
	}
}
//------------------- expr ----------------
pub type ExprContextAll<'input> = ExprContext<'input>;


pub type ExprContext<'input> = BaseParserRuleContext<'input,ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TimeExprGrammarParserContext<'input> for ExprContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for ExprContext<'input>{
		fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expr(self);
		}fn exit(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
			listener.exit_expr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}
antlr_rust::tid!{ExprContextExt<'a>}

impl<'input> ExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn TimeExprGrammarParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExprContextAttrs<'input>: TimeExprGrammarParserContext<'input> + BorrowMut<ExprContextExt<'input>>{

fn booleanExpr(&self) -> Option<Rc<BooleanExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}

}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input>{}

impl<'input, I, H> TimeExprGrammarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expr(&mut self,)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_expr);
        let mut _localctx: Rc<ExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule booleanExpr*/
			recog.base.set_state(34);
			recog.booleanExpr_rec(0)?;

			recog.base.set_state(35);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- booleanExpr ----------------
#[derive(Debug)]
pub enum BooleanExprContextAll<'input>{
	AndExprContext(AndExprContext<'input>),
	EventExprAsBoolContext(EventExprAsBoolContext<'input>),
	NotExprContext(NotExprContext<'input>),
	ParenExprContext(ParenExprContext<'input>),
	OrExprContext(OrExprContext<'input>),
Error(BooleanExprContext<'input>)
}
antlr_rust::tid!{BooleanExprContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for BooleanExprContextAll<'input>{}

impl<'input> TimeExprGrammarParserContext<'input> for BooleanExprContextAll<'input>{}

impl<'input> Deref for BooleanExprContextAll<'input>{
	type Target = dyn BooleanExprContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use BooleanExprContextAll::*;
		match self{
			AndExprContext(inner) => inner,
			EventExprAsBoolContext(inner) => inner,
			NotExprContext(inner) => inner,
			ParenExprContext(inner) => inner,
			OrExprContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for BooleanExprContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type BooleanExprContext<'input> = BaseParserRuleContext<'input,BooleanExprContextExt<'input>>;

#[derive(Clone)]
pub struct BooleanExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TimeExprGrammarParserContext<'input> for BooleanExprContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for BooleanExprContext<'input>{
}

impl<'input> CustomRuleContext<'input> for BooleanExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_booleanExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_booleanExpr }
}
antlr_rust::tid!{BooleanExprContextExt<'a>}

impl<'input> BooleanExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn TimeExprGrammarParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BooleanExprContextAll<'input>> {
		Rc::new(
		BooleanExprContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BooleanExprContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait BooleanExprContextAttrs<'input>: TimeExprGrammarParserContext<'input> + BorrowMut<BooleanExprContextExt<'input>>{


}

impl<'input> BooleanExprContextAttrs<'input> for BooleanExprContext<'input>{}

pub type AndExprContext<'input> = BaseParserRuleContext<'input,AndExprContextExt<'input>>;

pub trait AndExprContextAttrs<'input>: TimeExprGrammarParserContext<'input>{
	fn booleanExpr_all(&self) ->  Vec<Rc<BooleanExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn booleanExpr(&self, i: usize) -> Option<Rc<BooleanExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token AND
	/// Returns `None` if there is no child corresponding to token AND
	fn AND(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(AND, 0)
	}
}

impl<'input> AndExprContextAttrs<'input> for AndExprContext<'input>{}

pub struct AndExprContextExt<'input>{
	base:BooleanExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AndExprContextExt<'a>}

impl<'input> TimeExprGrammarParserContext<'input> for AndExprContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for AndExprContext<'input>{
	fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_AndExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for AndExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_booleanExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_booleanExpr }
}

impl<'input> Borrow<BooleanExprContextExt<'input>> for AndExprContext<'input>{
	fn borrow(&self) -> &BooleanExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<BooleanExprContextExt<'input>> for AndExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut BooleanExprContextExt<'input> { &mut self.base }
}

impl<'input> BooleanExprContextAttrs<'input> for AndExprContext<'input> {}

impl<'input> AndExprContextExt<'input>{
	fn new(ctx: &dyn BooleanExprContextAttrs<'input>) -> Rc<BooleanExprContextAll<'input>>  {
		Rc::new(
			BooleanExprContextAll::AndExprContext(
				BaseParserRuleContext::copy_from(ctx,AndExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type EventExprAsBoolContext<'input> = BaseParserRuleContext<'input,EventExprAsBoolContextExt<'input>>;

pub trait EventExprAsBoolContextAttrs<'input>: TimeExprGrammarParserContext<'input>{
	fn anyEventExpression(&self) -> Option<Rc<AnyEventExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> EventExprAsBoolContextAttrs<'input> for EventExprAsBoolContext<'input>{}

pub struct EventExprAsBoolContextExt<'input>{
	base:BooleanExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{EventExprAsBoolContextExt<'a>}

impl<'input> TimeExprGrammarParserContext<'input> for EventExprAsBoolContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for EventExprAsBoolContext<'input>{
	fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_EventExprAsBool(self);
	}
}

impl<'input> CustomRuleContext<'input> for EventExprAsBoolContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_booleanExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_booleanExpr }
}

impl<'input> Borrow<BooleanExprContextExt<'input>> for EventExprAsBoolContext<'input>{
	fn borrow(&self) -> &BooleanExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<BooleanExprContextExt<'input>> for EventExprAsBoolContext<'input>{
	fn borrow_mut(&mut self) -> &mut BooleanExprContextExt<'input> { &mut self.base }
}

impl<'input> BooleanExprContextAttrs<'input> for EventExprAsBoolContext<'input> {}

impl<'input> EventExprAsBoolContextExt<'input>{
	fn new(ctx: &dyn BooleanExprContextAttrs<'input>) -> Rc<BooleanExprContextAll<'input>>  {
		Rc::new(
			BooleanExprContextAll::EventExprAsBoolContext(
				BaseParserRuleContext::copy_from(ctx,EventExprAsBoolContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NotExprContext<'input> = BaseParserRuleContext<'input,NotExprContextExt<'input>>;

pub trait NotExprContextAttrs<'input>: TimeExprGrammarParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NOT
	/// Returns `None` if there is no child corresponding to token NOT
	fn NOT(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(NOT, 0)
	}
	fn booleanExpr(&self) -> Option<Rc<BooleanExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> NotExprContextAttrs<'input> for NotExprContext<'input>{}

pub struct NotExprContextExt<'input>{
	base:BooleanExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NotExprContextExt<'a>}

impl<'input> TimeExprGrammarParserContext<'input> for NotExprContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for NotExprContext<'input>{
	fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_NotExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for NotExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_booleanExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_booleanExpr }
}

impl<'input> Borrow<BooleanExprContextExt<'input>> for NotExprContext<'input>{
	fn borrow(&self) -> &BooleanExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<BooleanExprContextExt<'input>> for NotExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut BooleanExprContextExt<'input> { &mut self.base }
}

impl<'input> BooleanExprContextAttrs<'input> for NotExprContext<'input> {}

impl<'input> NotExprContextExt<'input>{
	fn new(ctx: &dyn BooleanExprContextAttrs<'input>) -> Rc<BooleanExprContextAll<'input>>  {
		Rc::new(
			BooleanExprContextAll::NotExprContext(
				BaseParserRuleContext::copy_from(ctx,NotExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ParenExprContext<'input> = BaseParserRuleContext<'input,ParenExprContextExt<'input>>;

pub trait ParenExprContextAttrs<'input>: TimeExprGrammarParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LPAR
	/// Returns `None` if there is no child corresponding to token LPAR
	fn LPAR(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(LPAR, 0)
	}
	fn booleanExpr(&self) -> Option<Rc<BooleanExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAR
	/// Returns `None` if there is no child corresponding to token RPAR
	fn RPAR(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(RPAR, 0)
	}
}

impl<'input> ParenExprContextAttrs<'input> for ParenExprContext<'input>{}

pub struct ParenExprContextExt<'input>{
	base:BooleanExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ParenExprContextExt<'a>}

impl<'input> TimeExprGrammarParserContext<'input> for ParenExprContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for ParenExprContext<'input>{
	fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ParenExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParenExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_booleanExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_booleanExpr }
}

impl<'input> Borrow<BooleanExprContextExt<'input>> for ParenExprContext<'input>{
	fn borrow(&self) -> &BooleanExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<BooleanExprContextExt<'input>> for ParenExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut BooleanExprContextExt<'input> { &mut self.base }
}

impl<'input> BooleanExprContextAttrs<'input> for ParenExprContext<'input> {}

impl<'input> ParenExprContextExt<'input>{
	fn new(ctx: &dyn BooleanExprContextAttrs<'input>) -> Rc<BooleanExprContextAll<'input>>  {
		Rc::new(
			BooleanExprContextAll::ParenExprContext(
				BaseParserRuleContext::copy_from(ctx,ParenExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type OrExprContext<'input> = BaseParserRuleContext<'input,OrExprContextExt<'input>>;

pub trait OrExprContextAttrs<'input>: TimeExprGrammarParserContext<'input>{
	fn booleanExpr_all(&self) ->  Vec<Rc<BooleanExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn booleanExpr(&self, i: usize) -> Option<Rc<BooleanExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token OR
	/// Returns `None` if there is no child corresponding to token OR
	fn OR(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(OR, 0)
	}
}

impl<'input> OrExprContextAttrs<'input> for OrExprContext<'input>{}

pub struct OrExprContextExt<'input>{
	base:BooleanExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{OrExprContextExt<'a>}

impl<'input> TimeExprGrammarParserContext<'input> for OrExprContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for OrExprContext<'input>{
	fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_OrExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for OrExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_booleanExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_booleanExpr }
}

impl<'input> Borrow<BooleanExprContextExt<'input>> for OrExprContext<'input>{
	fn borrow(&self) -> &BooleanExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<BooleanExprContextExt<'input>> for OrExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut BooleanExprContextExt<'input> { &mut self.base }
}

impl<'input> BooleanExprContextAttrs<'input> for OrExprContext<'input> {}

impl<'input> OrExprContextExt<'input>{
	fn new(ctx: &dyn BooleanExprContextAttrs<'input>) -> Rc<BooleanExprContextAll<'input>>  {
		Rc::new(
			BooleanExprContextAll::OrExprContext(
				BaseParserRuleContext::copy_from(ctx,OrExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TimeExprGrammarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  booleanExpr(&mut self,)
	-> Result<Rc<BooleanExprContextAll<'input>>,ANTLRError> {
		self.booleanExpr_rec(0)
	}

	fn booleanExpr_rec(&mut self, _p: isize)
	-> Result<Rc<BooleanExprContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = BooleanExprContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 2, RULE_booleanExpr, _p);
	    let mut _localctx: Rc<BooleanExprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 2;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(45);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 NOT 
				=> {
					{
					let mut tmp = NotExprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();


					recog.base.set_state(38);
					recog.base.match_token(NOT,&mut recog.err_handler)?;

					/*InvokeRule booleanExpr*/
					recog.base.set_state(39);
					recog.booleanExpr_rec(3)?;

					}
				}

			 WILDCARD | ID 
				=> {
					{
					let mut tmp = EventExprAsBoolContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule anyEventExpression*/
					recog.base.set_state(40);
					recog.anyEventExpression()?;

					}
				}

			 LPAR 
				=> {
					{
					let mut tmp = ParenExprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(41);
					recog.base.match_token(LPAR,&mut recog.err_handler)?;

					/*InvokeRule booleanExpr*/
					recog.base.set_state(42);
					recog.booleanExpr_rec(0)?;

					recog.base.set_state(43);
					recog.base.match_token(RPAR,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(55);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(53);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(1,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = AndExprContextExt::new(&**BooleanExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_booleanExpr);
							_localctx = tmp;
							recog.base.set_state(47);
							if !({recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(48);
							recog.base.match_token(AND,&mut recog.err_handler)?;

							/*InvokeRule booleanExpr*/
							recog.base.set_state(49);
							recog.booleanExpr_rec(6)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = OrExprContextExt::new(&**BooleanExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_booleanExpr);
							_localctx = tmp;
							recog.base.set_state(50);
							if !({recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(51);
							recog.base.match_token(OR,&mut recog.err_handler)?;

							/*InvokeRule booleanExpr*/
							recog.base.set_state(52);
							recog.booleanExpr_rec(5)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(57);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- anyEventExpression ----------------
#[derive(Debug)]
pub enum AnyEventExpressionContextAll<'input>{
	AnyExistingEventContext(AnyExistingEventContext<'input>),
Error(AnyEventExpressionContext<'input>)
}
antlr_rust::tid!{AnyEventExpressionContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for AnyEventExpressionContextAll<'input>{}

impl<'input> TimeExprGrammarParserContext<'input> for AnyEventExpressionContextAll<'input>{}

impl<'input> Deref for AnyEventExpressionContextAll<'input>{
	type Target = dyn AnyEventExpressionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use AnyEventExpressionContextAll::*;
		match self{
			AnyExistingEventContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for AnyEventExpressionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type AnyEventExpressionContext<'input> = BaseParserRuleContext<'input,AnyEventExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct AnyEventExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TimeExprGrammarParserContext<'input> for AnyEventExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for AnyEventExpressionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for AnyEventExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_anyEventExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_anyEventExpression }
}
antlr_rust::tid!{AnyEventExpressionContextExt<'a>}

impl<'input> AnyEventExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TimeExprGrammarParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AnyEventExpressionContextAll<'input>> {
		Rc::new(
		AnyEventExpressionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnyEventExpressionContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait AnyEventExpressionContextAttrs<'input>: TimeExprGrammarParserContext<'input> + BorrowMut<AnyEventExpressionContextExt<'input>>{


}

impl<'input> AnyEventExpressionContextAttrs<'input> for AnyEventExpressionContext<'input>{}

pub type AnyExistingEventContext<'input> = BaseParserRuleContext<'input,AnyExistingEventContextExt<'input>>;

pub trait AnyExistingEventContextAttrs<'input>: TimeExprGrammarParserContext<'input>{
	fn eventExpr(&self) -> Option<Rc<EventExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> AnyExistingEventContextAttrs<'input> for AnyExistingEventContext<'input>{}

pub struct AnyExistingEventContextExt<'input>{
	base:AnyEventExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AnyExistingEventContextExt<'a>}

impl<'input> TimeExprGrammarParserContext<'input> for AnyExistingEventContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for AnyExistingEventContext<'input>{
	fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_AnyExistingEvent(self);
	}
}

impl<'input> CustomRuleContext<'input> for AnyExistingEventContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_anyEventExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_anyEventExpression }
}

impl<'input> Borrow<AnyEventExpressionContextExt<'input>> for AnyExistingEventContext<'input>{
	fn borrow(&self) -> &AnyEventExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AnyEventExpressionContextExt<'input>> for AnyExistingEventContext<'input>{
	fn borrow_mut(&mut self) -> &mut AnyEventExpressionContextExt<'input> { &mut self.base }
}

impl<'input> AnyEventExpressionContextAttrs<'input> for AnyExistingEventContext<'input> {}

impl<'input> AnyExistingEventContextExt<'input>{
	fn new(ctx: &dyn AnyEventExpressionContextAttrs<'input>) -> Rc<AnyEventExpressionContextAll<'input>>  {
		Rc::new(
			AnyEventExpressionContextAll::AnyExistingEventContext(
				BaseParserRuleContext::copy_from(ctx,AnyExistingEventContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TimeExprGrammarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn anyEventExpression(&mut self,)
	-> Result<Rc<AnyEventExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnyEventExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_anyEventExpression);
        let mut _localctx: Rc<AnyEventExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let tmp = AnyExistingEventContextExt::new(&**_localctx);
			recog.base.enter_outer_alt(Some(tmp.clone()), 1);
			_localctx = tmp;
			{
			/*InvokeRule eventExpr*/
			recog.base.set_state(58);
			recog.eventExpr()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- eventExpr ----------------
#[derive(Debug)]
pub enum EventExprContextAll<'input>{
	EventExprWithTimeResolutionContext(EventExprWithTimeResolutionContext<'input>),
Error(EventExprContext<'input>)
}
antlr_rust::tid!{EventExprContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for EventExprContextAll<'input>{}

impl<'input> TimeExprGrammarParserContext<'input> for EventExprContextAll<'input>{}

impl<'input> Deref for EventExprContextAll<'input>{
	type Target = dyn EventExprContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use EventExprContextAll::*;
		match self{
			EventExprWithTimeResolutionContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for EventExprContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type EventExprContext<'input> = BaseParserRuleContext<'input,EventExprContextExt<'input>>;

#[derive(Clone)]
pub struct EventExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TimeExprGrammarParserContext<'input> for EventExprContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for EventExprContext<'input>{
}

impl<'input> CustomRuleContext<'input> for EventExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eventExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eventExpr }
}
antlr_rust::tid!{EventExprContextExt<'a>}

impl<'input> EventExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn TimeExprGrammarParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EventExprContextAll<'input>> {
		Rc::new(
		EventExprContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EventExprContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait EventExprContextAttrs<'input>: TimeExprGrammarParserContext<'input> + BorrowMut<EventExprContextExt<'input>>{


}

impl<'input> EventExprContextAttrs<'input> for EventExprContext<'input>{}

pub type EventExprWithTimeResolutionContext<'input> = BaseParserRuleContext<'input,EventExprWithTimeResolutionContextExt<'input>>;

pub trait EventExprWithTimeResolutionContextAttrs<'input>: TimeExprGrammarParserContext<'input>{
	fn eventFilterExpr(&self) -> Option<Rc<EventFilterExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn timeExpr(&self) -> Option<Rc<TimeExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> EventExprWithTimeResolutionContextAttrs<'input> for EventExprWithTimeResolutionContext<'input>{}

pub struct EventExprWithTimeResolutionContextExt<'input>{
	base:EventExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{EventExprWithTimeResolutionContextExt<'a>}

impl<'input> TimeExprGrammarParserContext<'input> for EventExprWithTimeResolutionContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for EventExprWithTimeResolutionContext<'input>{
	fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_EventExprWithTimeResolution(self);
	}
}

impl<'input> CustomRuleContext<'input> for EventExprWithTimeResolutionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eventExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eventExpr }
}

impl<'input> Borrow<EventExprContextExt<'input>> for EventExprWithTimeResolutionContext<'input>{
	fn borrow(&self) -> &EventExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<EventExprContextExt<'input>> for EventExprWithTimeResolutionContext<'input>{
	fn borrow_mut(&mut self) -> &mut EventExprContextExt<'input> { &mut self.base }
}

impl<'input> EventExprContextAttrs<'input> for EventExprWithTimeResolutionContext<'input> {}

impl<'input> EventExprWithTimeResolutionContextExt<'input>{
	fn new(ctx: &dyn EventExprContextAttrs<'input>) -> Rc<EventExprContextAll<'input>>  {
		Rc::new(
			EventExprContextAll::EventExprWithTimeResolutionContext(
				BaseParserRuleContext::copy_from(ctx,EventExprWithTimeResolutionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TimeExprGrammarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eventExpr(&mut self,)
	-> Result<Rc<EventExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EventExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_eventExpr);
        let mut _localctx: Rc<EventExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let tmp = EventExprWithTimeResolutionContextExt::new(&**_localctx);
			recog.base.enter_outer_alt(Some(tmp.clone()), 1);
			_localctx = tmp;
			{
			/*InvokeRule eventFilterExpr*/
			recog.base.set_state(60);
			recog.eventFilterExpr()?;

			recog.base.set_state(62);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(3,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule timeExpr*/
					recog.base.set_state(61);
					recog.timeExpr()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- eventFilterExpr ----------------
#[derive(Debug)]
pub enum EventFilterExprContextAll<'input>{
	FilteredEventWithDataAttributesContext(FilteredEventWithDataAttributesContext<'input>),
	WildcardWithOptionalFilteredEventsContext(WildcardWithOptionalFilteredEventsContext<'input>),
Error(EventFilterExprContext<'input>)
}
antlr_rust::tid!{EventFilterExprContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for EventFilterExprContextAll<'input>{}

impl<'input> TimeExprGrammarParserContext<'input> for EventFilterExprContextAll<'input>{}

impl<'input> Deref for EventFilterExprContextAll<'input>{
	type Target = dyn EventFilterExprContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use EventFilterExprContextAll::*;
		match self{
			FilteredEventWithDataAttributesContext(inner) => inner,
			WildcardWithOptionalFilteredEventsContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for EventFilterExprContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type EventFilterExprContext<'input> = BaseParserRuleContext<'input,EventFilterExprContextExt<'input>>;

#[derive(Clone)]
pub struct EventFilterExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TimeExprGrammarParserContext<'input> for EventFilterExprContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for EventFilterExprContext<'input>{
}

impl<'input> CustomRuleContext<'input> for EventFilterExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eventFilterExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eventFilterExpr }
}
antlr_rust::tid!{EventFilterExprContextExt<'a>}

impl<'input> EventFilterExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn TimeExprGrammarParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EventFilterExprContextAll<'input>> {
		Rc::new(
		EventFilterExprContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EventFilterExprContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait EventFilterExprContextAttrs<'input>: TimeExprGrammarParserContext<'input> + BorrowMut<EventFilterExprContextExt<'input>>{


}

impl<'input> EventFilterExprContextAttrs<'input> for EventFilterExprContext<'input>{}

pub type FilteredEventWithDataAttributesContext<'input> = BaseParserRuleContext<'input,FilteredEventWithDataAttributesContextExt<'input>>;

pub trait FilteredEventWithDataAttributesContextAttrs<'input>: TimeExprGrammarParserContext<'input>{
	fn eventName(&self) -> Option<Rc<EventNameContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token HASH
	/// Returns `None` if there is no child corresponding to token HASH
	fn HASH(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(HASH, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LEFTSB
	/// Returns `None` if there is no child corresponding to token LEFTSB
	fn LEFTSB(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(LEFTSB, 0)
	}
	fn attributeDataFilterExp(&self) -> Option<Rc<AttributeDataFilterExpContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RIGHTSB
	/// Returns `None` if there is no child corresponding to token RIGHTSB
	fn RIGHTSB(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(RIGHTSB, 0)
	}
}

impl<'input> FilteredEventWithDataAttributesContextAttrs<'input> for FilteredEventWithDataAttributesContext<'input>{}

pub struct FilteredEventWithDataAttributesContextExt<'input>{
	base:EventFilterExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{FilteredEventWithDataAttributesContextExt<'a>}

impl<'input> TimeExprGrammarParserContext<'input> for FilteredEventWithDataAttributesContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for FilteredEventWithDataAttributesContext<'input>{
	fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_FilteredEventWithDataAttributes(self);
	}
}

impl<'input> CustomRuleContext<'input> for FilteredEventWithDataAttributesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eventFilterExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eventFilterExpr }
}

impl<'input> Borrow<EventFilterExprContextExt<'input>> for FilteredEventWithDataAttributesContext<'input>{
	fn borrow(&self) -> &EventFilterExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<EventFilterExprContextExt<'input>> for FilteredEventWithDataAttributesContext<'input>{
	fn borrow_mut(&mut self) -> &mut EventFilterExprContextExt<'input> { &mut self.base }
}

impl<'input> EventFilterExprContextAttrs<'input> for FilteredEventWithDataAttributesContext<'input> {}

impl<'input> FilteredEventWithDataAttributesContextExt<'input>{
	fn new(ctx: &dyn EventFilterExprContextAttrs<'input>) -> Rc<EventFilterExprContextAll<'input>>  {
		Rc::new(
			EventFilterExprContextAll::FilteredEventWithDataAttributesContext(
				BaseParserRuleContext::copy_from(ctx,FilteredEventWithDataAttributesContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type WildcardWithOptionalFilteredEventsContext<'input> = BaseParserRuleContext<'input,WildcardWithOptionalFilteredEventsContextExt<'input>>;

pub trait WildcardWithOptionalFilteredEventsContextAttrs<'input>: TimeExprGrammarParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token WILDCARD
	/// Returns `None` if there is no child corresponding to token WILDCARD
	fn WILDCARD(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(WILDCARD, 0)
	}
	/// Retrieves first TerminalNode corresponding to token BUT
	/// Returns `None` if there is no child corresponding to token BUT
	fn BUT(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(BUT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token NOT
	/// Returns `None` if there is no child corresponding to token NOT
	fn NOT(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(NOT, 0)
	}
	fn eventExpr(&self) -> Option<Rc<EventExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> WildcardWithOptionalFilteredEventsContextAttrs<'input> for WildcardWithOptionalFilteredEventsContext<'input>{}

pub struct WildcardWithOptionalFilteredEventsContextExt<'input>{
	base:EventFilterExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{WildcardWithOptionalFilteredEventsContextExt<'a>}

impl<'input> TimeExprGrammarParserContext<'input> for WildcardWithOptionalFilteredEventsContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for WildcardWithOptionalFilteredEventsContext<'input>{
	fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_WildcardWithOptionalFilteredEvents(self);
	}
}

impl<'input> CustomRuleContext<'input> for WildcardWithOptionalFilteredEventsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eventFilterExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eventFilterExpr }
}

impl<'input> Borrow<EventFilterExprContextExt<'input>> for WildcardWithOptionalFilteredEventsContext<'input>{
	fn borrow(&self) -> &EventFilterExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<EventFilterExprContextExt<'input>> for WildcardWithOptionalFilteredEventsContext<'input>{
	fn borrow_mut(&mut self) -> &mut EventFilterExprContextExt<'input> { &mut self.base }
}

impl<'input> EventFilterExprContextAttrs<'input> for WildcardWithOptionalFilteredEventsContext<'input> {}

impl<'input> WildcardWithOptionalFilteredEventsContextExt<'input>{
	fn new(ctx: &dyn EventFilterExprContextAttrs<'input>) -> Rc<EventFilterExprContextAll<'input>>  {
		Rc::new(
			EventFilterExprContextAll::WildcardWithOptionalFilteredEventsContext(
				BaseParserRuleContext::copy_from(ctx,WildcardWithOptionalFilteredEventsContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TimeExprGrammarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eventFilterExpr(&mut self,)
	-> Result<Rc<EventFilterExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EventFilterExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_eventFilterExpr);
        let mut _localctx: Rc<EventFilterExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(78);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 WILDCARD 
				=> {
					let tmp = WildcardWithOptionalFilteredEventsContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(64);
					recog.base.match_token(WILDCARD,&mut recog.err_handler)?;

					recog.base.set_state(68);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(4,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(65);
							recog.base.match_token(BUT,&mut recog.err_handler)?;

							recog.base.set_state(66);
							recog.base.match_token(NOT,&mut recog.err_handler)?;

							/*InvokeRule eventExpr*/
							recog.base.set_state(67);
							recog.eventExpr()?;

							}
						}

						_ => {}
					}
					}
				}

			 ID 
				=> {
					let tmp = FilteredEventWithDataAttributesContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule eventName*/
					recog.base.set_state(70);
					recog.eventName()?;

					recog.base.set_state(76);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(5,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(71);
							recog.base.match_token(HASH,&mut recog.err_handler)?;

							recog.base.set_state(72);
							recog.base.match_token(LEFTSB,&mut recog.err_handler)?;

							/*InvokeRule attributeDataFilterExp*/
							recog.base.set_state(73);
							recog.attributeDataFilterExp_rec(0)?;

							recog.base.set_state(74);
							recog.base.match_token(RIGHTSB,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- eventListExpr ----------------
#[derive(Debug)]
pub enum EventListExprContextAll<'input>{
	EventListExpressionContext(EventListExpressionContext<'input>),
Error(EventListExprContext<'input>)
}
antlr_rust::tid!{EventListExprContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for EventListExprContextAll<'input>{}

impl<'input> TimeExprGrammarParserContext<'input> for EventListExprContextAll<'input>{}

impl<'input> Deref for EventListExprContextAll<'input>{
	type Target = dyn EventListExprContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use EventListExprContextAll::*;
		match self{
			EventListExpressionContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for EventListExprContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type EventListExprContext<'input> = BaseParserRuleContext<'input,EventListExprContextExt<'input>>;

#[derive(Clone)]
pub struct EventListExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TimeExprGrammarParserContext<'input> for EventListExprContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for EventListExprContext<'input>{
}

impl<'input> CustomRuleContext<'input> for EventListExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eventListExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eventListExpr }
}
antlr_rust::tid!{EventListExprContextExt<'a>}

impl<'input> EventListExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn TimeExprGrammarParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EventListExprContextAll<'input>> {
		Rc::new(
		EventListExprContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EventListExprContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait EventListExprContextAttrs<'input>: TimeExprGrammarParserContext<'input> + BorrowMut<EventListExprContextExt<'input>>{


}

impl<'input> EventListExprContextAttrs<'input> for EventListExprContext<'input>{}

pub type EventListExpressionContext<'input> = BaseParserRuleContext<'input,EventListExpressionContextExt<'input>>;

pub trait EventListExpressionContextAttrs<'input>: TimeExprGrammarParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LEFTSB
	/// Returns `None` if there is no child corresponding to token LEFTSB
	fn LEFTSB(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(LEFTSB, 0)
	}
	fn eventFilterExpr_all(&self) ->  Vec<Rc<EventFilterExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn eventFilterExpr(&self, i: usize) -> Option<Rc<EventFilterExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token RIGHTSB
	/// Returns `None` if there is no child corresponding to token RIGHTSB
	fn RIGHTSB(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(RIGHTSB, 0)
	}
}

impl<'input> EventListExpressionContextAttrs<'input> for EventListExpressionContext<'input>{}

pub struct EventListExpressionContextExt<'input>{
	base:EventListExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{EventListExpressionContextExt<'a>}

impl<'input> TimeExprGrammarParserContext<'input> for EventListExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for EventListExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_EventListExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for EventListExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eventListExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eventListExpr }
}

impl<'input> Borrow<EventListExprContextExt<'input>> for EventListExpressionContext<'input>{
	fn borrow(&self) -> &EventListExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<EventListExprContextExt<'input>> for EventListExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut EventListExprContextExt<'input> { &mut self.base }
}

impl<'input> EventListExprContextAttrs<'input> for EventListExpressionContext<'input> {}

impl<'input> EventListExpressionContextExt<'input>{
	fn new(ctx: &dyn EventListExprContextAttrs<'input>) -> Rc<EventListExprContextAll<'input>>  {
		Rc::new(
			EventListExprContextAll::EventListExpressionContext(
				BaseParserRuleContext::copy_from(ctx,EventListExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TimeExprGrammarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eventListExpr(&mut self,)
	-> Result<Rc<EventListExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EventListExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_eventListExpr);
        let mut _localctx: Rc<EventListExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let tmp = EventListExpressionContextExt::new(&**_localctx);
			recog.base.enter_outer_alt(Some(tmp.clone()), 1);
			_localctx = tmp;
			{
			recog.base.set_state(80);
			recog.base.match_token(LEFTSB,&mut recog.err_handler)?;

			/*InvokeRule eventFilterExpr*/
			recog.base.set_state(81);
			recog.eventFilterExpr()?;

			recog.base.set_state(86);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__0 {
				{
				{
				recog.base.set_state(82);
				recog.base.match_token(T__0,&mut recog.err_handler)?;

				/*InvokeRule eventFilterExpr*/
				recog.base.set_state(83);
				recog.eventFilterExpr()?;

				}
				}
				recog.base.set_state(88);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(89);
			recog.base.match_token(RIGHTSB,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- attributeDataFilterExp ----------------
#[derive(Debug)]
pub enum AttributeDataFilterExpContextAll<'input>{
	FilterAttributeANDContext(FilterAttributeANDContext<'input>),
	FilterAttributeORContext(FilterAttributeORContext<'input>),
	SimpleAttributeFilterExpressionContext(SimpleAttributeFilterExpressionContext<'input>),
	ParentAttributeContext(ParentAttributeContext<'input>),
Error(AttributeDataFilterExpContext<'input>)
}
antlr_rust::tid!{AttributeDataFilterExpContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for AttributeDataFilterExpContextAll<'input>{}

impl<'input> TimeExprGrammarParserContext<'input> for AttributeDataFilterExpContextAll<'input>{}

impl<'input> Deref for AttributeDataFilterExpContextAll<'input>{
	type Target = dyn AttributeDataFilterExpContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use AttributeDataFilterExpContextAll::*;
		match self{
			FilterAttributeANDContext(inner) => inner,
			FilterAttributeORContext(inner) => inner,
			SimpleAttributeFilterExpressionContext(inner) => inner,
			ParentAttributeContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for AttributeDataFilterExpContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type AttributeDataFilterExpContext<'input> = BaseParserRuleContext<'input,AttributeDataFilterExpContextExt<'input>>;

#[derive(Clone)]
pub struct AttributeDataFilterExpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TimeExprGrammarParserContext<'input> for AttributeDataFilterExpContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for AttributeDataFilterExpContext<'input>{
}

impl<'input> CustomRuleContext<'input> for AttributeDataFilterExpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_attributeDataFilterExp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_attributeDataFilterExp }
}
antlr_rust::tid!{AttributeDataFilterExpContextExt<'a>}

impl<'input> AttributeDataFilterExpContextExt<'input>{
	fn new(parent: Option<Rc<dyn TimeExprGrammarParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AttributeDataFilterExpContextAll<'input>> {
		Rc::new(
		AttributeDataFilterExpContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AttributeDataFilterExpContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait AttributeDataFilterExpContextAttrs<'input>: TimeExprGrammarParserContext<'input> + BorrowMut<AttributeDataFilterExpContextExt<'input>>{


}

impl<'input> AttributeDataFilterExpContextAttrs<'input> for AttributeDataFilterExpContext<'input>{}

pub type FilterAttributeANDContext<'input> = BaseParserRuleContext<'input,FilterAttributeANDContextExt<'input>>;

pub trait FilterAttributeANDContextAttrs<'input>: TimeExprGrammarParserContext<'input>{
	fn attributeDataFilterExp_all(&self) ->  Vec<Rc<AttributeDataFilterExpContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn attributeDataFilterExp(&self, i: usize) -> Option<Rc<AttributeDataFilterExpContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token AND
	/// Returns `None` if there is no child corresponding to token AND
	fn AND(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(AND, 0)
	}
}

impl<'input> FilterAttributeANDContextAttrs<'input> for FilterAttributeANDContext<'input>{}

pub struct FilterAttributeANDContextExt<'input>{
	base:AttributeDataFilterExpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{FilterAttributeANDContextExt<'a>}

impl<'input> TimeExprGrammarParserContext<'input> for FilterAttributeANDContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for FilterAttributeANDContext<'input>{
	fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_FilterAttributeAND(self);
	}
}

impl<'input> CustomRuleContext<'input> for FilterAttributeANDContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_attributeDataFilterExp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_attributeDataFilterExp }
}

impl<'input> Borrow<AttributeDataFilterExpContextExt<'input>> for FilterAttributeANDContext<'input>{
	fn borrow(&self) -> &AttributeDataFilterExpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AttributeDataFilterExpContextExt<'input>> for FilterAttributeANDContext<'input>{
	fn borrow_mut(&mut self) -> &mut AttributeDataFilterExpContextExt<'input> { &mut self.base }
}

impl<'input> AttributeDataFilterExpContextAttrs<'input> for FilterAttributeANDContext<'input> {}

impl<'input> FilterAttributeANDContextExt<'input>{
	fn new(ctx: &dyn AttributeDataFilterExpContextAttrs<'input>) -> Rc<AttributeDataFilterExpContextAll<'input>>  {
		Rc::new(
			AttributeDataFilterExpContextAll::FilterAttributeANDContext(
				BaseParserRuleContext::copy_from(ctx,FilterAttributeANDContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type FilterAttributeORContext<'input> = BaseParserRuleContext<'input,FilterAttributeORContextExt<'input>>;

pub trait FilterAttributeORContextAttrs<'input>: TimeExprGrammarParserContext<'input>{
	fn attributeDataFilterExp_all(&self) ->  Vec<Rc<AttributeDataFilterExpContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn attributeDataFilterExp(&self, i: usize) -> Option<Rc<AttributeDataFilterExpContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token OR
	/// Returns `None` if there is no child corresponding to token OR
	fn OR(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(OR, 0)
	}
}

impl<'input> FilterAttributeORContextAttrs<'input> for FilterAttributeORContext<'input>{}

pub struct FilterAttributeORContextExt<'input>{
	base:AttributeDataFilterExpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{FilterAttributeORContextExt<'a>}

impl<'input> TimeExprGrammarParserContext<'input> for FilterAttributeORContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for FilterAttributeORContext<'input>{
	fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_FilterAttributeOR(self);
	}
}

impl<'input> CustomRuleContext<'input> for FilterAttributeORContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_attributeDataFilterExp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_attributeDataFilterExp }
}

impl<'input> Borrow<AttributeDataFilterExpContextExt<'input>> for FilterAttributeORContext<'input>{
	fn borrow(&self) -> &AttributeDataFilterExpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AttributeDataFilterExpContextExt<'input>> for FilterAttributeORContext<'input>{
	fn borrow_mut(&mut self) -> &mut AttributeDataFilterExpContextExt<'input> { &mut self.base }
}

impl<'input> AttributeDataFilterExpContextAttrs<'input> for FilterAttributeORContext<'input> {}

impl<'input> FilterAttributeORContextExt<'input>{
	fn new(ctx: &dyn AttributeDataFilterExpContextAttrs<'input>) -> Rc<AttributeDataFilterExpContextAll<'input>>  {
		Rc::new(
			AttributeDataFilterExpContextAll::FilterAttributeORContext(
				BaseParserRuleContext::copy_from(ctx,FilterAttributeORContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SimpleAttributeFilterExpressionContext<'input> = BaseParserRuleContext<'input,SimpleAttributeFilterExpressionContextExt<'input>>;

pub trait SimpleAttributeFilterExpressionContextAttrs<'input>: TimeExprGrammarParserContext<'input>{
	fn eventAttributesDataFilter(&self) -> Option<Rc<EventAttributesDataFilterContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> SimpleAttributeFilterExpressionContextAttrs<'input> for SimpleAttributeFilterExpressionContext<'input>{}

pub struct SimpleAttributeFilterExpressionContextExt<'input>{
	base:AttributeDataFilterExpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SimpleAttributeFilterExpressionContextExt<'a>}

impl<'input> TimeExprGrammarParserContext<'input> for SimpleAttributeFilterExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for SimpleAttributeFilterExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_SimpleAttributeFilterExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for SimpleAttributeFilterExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_attributeDataFilterExp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_attributeDataFilterExp }
}

impl<'input> Borrow<AttributeDataFilterExpContextExt<'input>> for SimpleAttributeFilterExpressionContext<'input>{
	fn borrow(&self) -> &AttributeDataFilterExpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AttributeDataFilterExpContextExt<'input>> for SimpleAttributeFilterExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut AttributeDataFilterExpContextExt<'input> { &mut self.base }
}

impl<'input> AttributeDataFilterExpContextAttrs<'input> for SimpleAttributeFilterExpressionContext<'input> {}

impl<'input> SimpleAttributeFilterExpressionContextExt<'input>{
	fn new(ctx: &dyn AttributeDataFilterExpContextAttrs<'input>) -> Rc<AttributeDataFilterExpContextAll<'input>>  {
		Rc::new(
			AttributeDataFilterExpContextAll::SimpleAttributeFilterExpressionContext(
				BaseParserRuleContext::copy_from(ctx,SimpleAttributeFilterExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ParentAttributeContext<'input> = BaseParserRuleContext<'input,ParentAttributeContextExt<'input>>;

pub trait ParentAttributeContextAttrs<'input>: TimeExprGrammarParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LPAR
	/// Returns `None` if there is no child corresponding to token LPAR
	fn LPAR(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(LPAR, 0)
	}
	fn attributeDataFilterExp(&self) -> Option<Rc<AttributeDataFilterExpContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAR
	/// Returns `None` if there is no child corresponding to token RPAR
	fn RPAR(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(RPAR, 0)
	}
}

impl<'input> ParentAttributeContextAttrs<'input> for ParentAttributeContext<'input>{}

pub struct ParentAttributeContextExt<'input>{
	base:AttributeDataFilterExpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ParentAttributeContextExt<'a>}

impl<'input> TimeExprGrammarParserContext<'input> for ParentAttributeContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for ParentAttributeContext<'input>{
	fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ParentAttribute(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParentAttributeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_attributeDataFilterExp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_attributeDataFilterExp }
}

impl<'input> Borrow<AttributeDataFilterExpContextExt<'input>> for ParentAttributeContext<'input>{
	fn borrow(&self) -> &AttributeDataFilterExpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AttributeDataFilterExpContextExt<'input>> for ParentAttributeContext<'input>{
	fn borrow_mut(&mut self) -> &mut AttributeDataFilterExpContextExt<'input> { &mut self.base }
}

impl<'input> AttributeDataFilterExpContextAttrs<'input> for ParentAttributeContext<'input> {}

impl<'input> ParentAttributeContextExt<'input>{
	fn new(ctx: &dyn AttributeDataFilterExpContextAttrs<'input>) -> Rc<AttributeDataFilterExpContextAll<'input>>  {
		Rc::new(
			AttributeDataFilterExpContextAll::ParentAttributeContext(
				BaseParserRuleContext::copy_from(ctx,ParentAttributeContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TimeExprGrammarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  attributeDataFilterExp(&mut self,)
	-> Result<Rc<AttributeDataFilterExpContextAll<'input>>,ANTLRError> {
		self.attributeDataFilterExp_rec(0)
	}

	fn attributeDataFilterExp_rec(&mut self, _p: isize)
	-> Result<Rc<AttributeDataFilterExpContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = AttributeDataFilterExpContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 12, RULE_attributeDataFilterExp, _p);
	    let mut _localctx: Rc<AttributeDataFilterExpContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 12;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(97);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LPAR 
				=> {
					{
					let mut tmp = ParentAttributeContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();


					recog.base.set_state(92);
					recog.base.match_token(LPAR,&mut recog.err_handler)?;

					/*InvokeRule attributeDataFilterExp*/
					recog.base.set_state(93);
					recog.attributeDataFilterExp_rec(0)?;

					recog.base.set_state(94);
					recog.base.match_token(RPAR,&mut recog.err_handler)?;

					}
				}

			 ID 
				=> {
					{
					let mut tmp = SimpleAttributeFilterExpressionContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule eventAttributesDataFilter*/
					recog.base.set_state(96);
					recog.eventAttributesDataFilter()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(107);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(10,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(105);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(9,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = FilterAttributeANDContextExt::new(&**AttributeDataFilterExpContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_attributeDataFilterExp);
							_localctx = tmp;
							recog.base.set_state(99);
							if !({recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(100);
							recog.base.match_token(AND,&mut recog.err_handler)?;

							/*InvokeRule attributeDataFilterExp*/
							recog.base.set_state(101);
							recog.attributeDataFilterExp_rec(5)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = FilterAttributeORContextExt::new(&**AttributeDataFilterExpContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_attributeDataFilterExp);
							_localctx = tmp;
							recog.base.set_state(102);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(103);
							recog.base.match_token(OR,&mut recog.err_handler)?;

							/*InvokeRule attributeDataFilterExp*/
							recog.base.set_state(104);
							recog.attributeDataFilterExp_rec(4)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(109);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(10,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- eventAttributesDataFilter ----------------
#[derive(Debug)]
pub enum EventAttributesDataFilterContextAll<'input>{
	AttributeExistsContext(AttributeExistsContext<'input>),
	AttributeConditionsContext(AttributeConditionsContext<'input>),
Error(EventAttributesDataFilterContext<'input>)
}
antlr_rust::tid!{EventAttributesDataFilterContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for EventAttributesDataFilterContextAll<'input>{}

impl<'input> TimeExprGrammarParserContext<'input> for EventAttributesDataFilterContextAll<'input>{}

impl<'input> Deref for EventAttributesDataFilterContextAll<'input>{
	type Target = dyn EventAttributesDataFilterContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use EventAttributesDataFilterContextAll::*;
		match self{
			AttributeExistsContext(inner) => inner,
			AttributeConditionsContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for EventAttributesDataFilterContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type EventAttributesDataFilterContext<'input> = BaseParserRuleContext<'input,EventAttributesDataFilterContextExt<'input>>;

#[derive(Clone)]
pub struct EventAttributesDataFilterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TimeExprGrammarParserContext<'input> for EventAttributesDataFilterContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for EventAttributesDataFilterContext<'input>{
}

impl<'input> CustomRuleContext<'input> for EventAttributesDataFilterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eventAttributesDataFilter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eventAttributesDataFilter }
}
antlr_rust::tid!{EventAttributesDataFilterContextExt<'a>}

impl<'input> EventAttributesDataFilterContextExt<'input>{
	fn new(parent: Option<Rc<dyn TimeExprGrammarParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EventAttributesDataFilterContextAll<'input>> {
		Rc::new(
		EventAttributesDataFilterContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EventAttributesDataFilterContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait EventAttributesDataFilterContextAttrs<'input>: TimeExprGrammarParserContext<'input> + BorrowMut<EventAttributesDataFilterContextExt<'input>>{


}

impl<'input> EventAttributesDataFilterContextAttrs<'input> for EventAttributesDataFilterContext<'input>{}

pub type AttributeExistsContext<'input> = BaseParserRuleContext<'input,AttributeExistsContextExt<'input>>;

pub trait AttributeExistsContextAttrs<'input>: TimeExprGrammarParserContext<'input>{
	fn attributeName(&self) -> Option<Rc<AttributeNameContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token IMPERATIVEMARK
	/// Returns `None` if there is no child corresponding to token IMPERATIVEMARK
	fn IMPERATIVEMARK(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(IMPERATIVEMARK, 0)
	}
}

impl<'input> AttributeExistsContextAttrs<'input> for AttributeExistsContext<'input>{}

pub struct AttributeExistsContextExt<'input>{
	base:EventAttributesDataFilterContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AttributeExistsContextExt<'a>}

impl<'input> TimeExprGrammarParserContext<'input> for AttributeExistsContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for AttributeExistsContext<'input>{
	fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_AttributeExists(self);
	}
}

impl<'input> CustomRuleContext<'input> for AttributeExistsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eventAttributesDataFilter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eventAttributesDataFilter }
}

impl<'input> Borrow<EventAttributesDataFilterContextExt<'input>> for AttributeExistsContext<'input>{
	fn borrow(&self) -> &EventAttributesDataFilterContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<EventAttributesDataFilterContextExt<'input>> for AttributeExistsContext<'input>{
	fn borrow_mut(&mut self) -> &mut EventAttributesDataFilterContextExt<'input> { &mut self.base }
}

impl<'input> EventAttributesDataFilterContextAttrs<'input> for AttributeExistsContext<'input> {}

impl<'input> AttributeExistsContextExt<'input>{
	fn new(ctx: &dyn EventAttributesDataFilterContextAttrs<'input>) -> Rc<EventAttributesDataFilterContextAll<'input>>  {
		Rc::new(
			EventAttributesDataFilterContextAll::AttributeExistsContext(
				BaseParserRuleContext::copy_from(ctx,AttributeExistsContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AttributeConditionsContext<'input> = BaseParserRuleContext<'input,AttributeConditionsContextExt<'input>>;

pub trait AttributeConditionsContextAttrs<'input>: TimeExprGrammarParserContext<'input>{
	fn attributeName(&self) -> Option<Rc<AttributeNameContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn comparisonOp(&self) -> Option<Rc<ComparisonOpContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn value(&self) -> Option<Rc<ValueContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> AttributeConditionsContextAttrs<'input> for AttributeConditionsContext<'input>{}

pub struct AttributeConditionsContextExt<'input>{
	base:EventAttributesDataFilterContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AttributeConditionsContextExt<'a>}

impl<'input> TimeExprGrammarParserContext<'input> for AttributeConditionsContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for AttributeConditionsContext<'input>{
	fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_AttributeConditions(self);
	}
}

impl<'input> CustomRuleContext<'input> for AttributeConditionsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eventAttributesDataFilter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eventAttributesDataFilter }
}

impl<'input> Borrow<EventAttributesDataFilterContextExt<'input>> for AttributeConditionsContext<'input>{
	fn borrow(&self) -> &EventAttributesDataFilterContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<EventAttributesDataFilterContextExt<'input>> for AttributeConditionsContext<'input>{
	fn borrow_mut(&mut self) -> &mut EventAttributesDataFilterContextExt<'input> { &mut self.base }
}

impl<'input> EventAttributesDataFilterContextAttrs<'input> for AttributeConditionsContext<'input> {}

impl<'input> AttributeConditionsContextExt<'input>{
	fn new(ctx: &dyn EventAttributesDataFilterContextAttrs<'input>) -> Rc<EventAttributesDataFilterContextAll<'input>>  {
		Rc::new(
			EventAttributesDataFilterContextAll::AttributeConditionsContext(
				BaseParserRuleContext::copy_from(ctx,AttributeConditionsContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TimeExprGrammarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eventAttributesDataFilter(&mut self,)
	-> Result<Rc<EventAttributesDataFilterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EventAttributesDataFilterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_eventAttributesDataFilter);
        let mut _localctx: Rc<EventAttributesDataFilterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(117);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(11,&mut recog.base)? {
				1 =>{
					let tmp = AttributeConditionsContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule attributeName*/
					recog.base.set_state(110);
					recog.attributeName()?;

					/*InvokeRule comparisonOp*/
					recog.base.set_state(111);
					recog.comparisonOp()?;

					/*InvokeRule value*/
					recog.base.set_state(112);
					recog.value()?;

					}
				}
			,
				2 =>{
					let tmp = AttributeExistsContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule attributeName*/
					recog.base.set_state(114);
					recog.attributeName()?;

					recog.base.set_state(115);
					recog.base.match_token(IMPERATIVEMARK,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- attributeName ----------------
pub type AttributeNameContextAll<'input> = AttributeNameContext<'input>;


pub type AttributeNameContext<'input> = BaseParserRuleContext<'input,AttributeNameContextExt<'input>>;

#[derive(Clone)]
pub struct AttributeNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TimeExprGrammarParserContext<'input> for AttributeNameContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for AttributeNameContext<'input>{
		fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_attributeName(self);
		}fn exit(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
			listener.exit_attributeName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for AttributeNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_attributeName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_attributeName }
}
antlr_rust::tid!{AttributeNameContextExt<'a>}

impl<'input> AttributeNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn TimeExprGrammarParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AttributeNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AttributeNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AttributeNameContextAttrs<'input>: TimeExprGrammarParserContext<'input> + BorrowMut<AttributeNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
	self.get_token(ID, 0)
}

}

impl<'input> AttributeNameContextAttrs<'input> for AttributeNameContext<'input>{}

impl<'input, I, H> TimeExprGrammarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn attributeName(&mut self,)
	-> Result<Rc<AttributeNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AttributeNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_attributeName);
        let mut _localctx: Rc<AttributeNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(119);
			recog.base.match_token(ID,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- timeExpr ----------------
#[derive(Debug)]
pub enum TimeExprContextAll<'input>{
	SinceExprContext(SinceExprContext<'input>),
Error(TimeExprContext<'input>)
}
antlr_rust::tid!{TimeExprContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for TimeExprContextAll<'input>{}

impl<'input> TimeExprGrammarParserContext<'input> for TimeExprContextAll<'input>{}

impl<'input> Deref for TimeExprContextAll<'input>{
	type Target = dyn TimeExprContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use TimeExprContextAll::*;
		match self{
			SinceExprContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for TimeExprContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type TimeExprContext<'input> = BaseParserRuleContext<'input,TimeExprContextExt<'input>>;

#[derive(Clone)]
pub struct TimeExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TimeExprGrammarParserContext<'input> for TimeExprContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for TimeExprContext<'input>{
}

impl<'input> CustomRuleContext<'input> for TimeExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_timeExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_timeExpr }
}
antlr_rust::tid!{TimeExprContextExt<'a>}

impl<'input> TimeExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn TimeExprGrammarParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TimeExprContextAll<'input>> {
		Rc::new(
		TimeExprContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TimeExprContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait TimeExprContextAttrs<'input>: TimeExprGrammarParserContext<'input> + BorrowMut<TimeExprContextExt<'input>>{


}

impl<'input> TimeExprContextAttrs<'input> for TimeExprContext<'input>{}

pub type SinceExprContext<'input> = BaseParserRuleContext<'input,SinceExprContextExt<'input>>;

pub trait SinceExprContextAttrs<'input>: TimeExprGrammarParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token SINCE
	/// Returns `None` if there is no child corresponding to token SINCE
	fn SINCE(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(SINCE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token INT
	/// Returns `None` if there is no child corresponding to token INT
	fn INT(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(INT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token TIMEUNIT
	/// Returns `None` if there is no child corresponding to token TIMEUNIT
	fn TIMEUNIT(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(TIMEUNIT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token FROM
	/// Returns `None` if there is no child corresponding to token FROM
	fn FROM(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(FROM, 0)
	}
	fn timeRef(&self) -> Option<Rc<TimeRefContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> SinceExprContextAttrs<'input> for SinceExprContext<'input>{}

pub struct SinceExprContextExt<'input>{
	base:TimeExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SinceExprContextExt<'a>}

impl<'input> TimeExprGrammarParserContext<'input> for SinceExprContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for SinceExprContext<'input>{
	fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_SinceExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for SinceExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_timeExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_timeExpr }
}

impl<'input> Borrow<TimeExprContextExt<'input>> for SinceExprContext<'input>{
	fn borrow(&self) -> &TimeExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TimeExprContextExt<'input>> for SinceExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut TimeExprContextExt<'input> { &mut self.base }
}

impl<'input> TimeExprContextAttrs<'input> for SinceExprContext<'input> {}

impl<'input> SinceExprContextExt<'input>{
	fn new(ctx: &dyn TimeExprContextAttrs<'input>) -> Rc<TimeExprContextAll<'input>>  {
		Rc::new(
			TimeExprContextAll::SinceExprContext(
				BaseParserRuleContext::copy_from(ctx,SinceExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TimeExprGrammarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn timeExpr(&mut self,)
	-> Result<Rc<TimeExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TimeExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_timeExpr);
        let mut _localctx: Rc<TimeExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let tmp = SinceExprContextExt::new(&**_localctx);
			recog.base.enter_outer_alt(Some(tmp.clone()), 1);
			_localctx = tmp;
			{
			recog.base.set_state(121);
			recog.base.match_token(SINCE,&mut recog.err_handler)?;

			recog.base.set_state(122);
			recog.base.match_token(INT,&mut recog.err_handler)?;

			recog.base.set_state(123);
			recog.base.match_token(TIMEUNIT,&mut recog.err_handler)?;

			recog.base.set_state(124);
			recog.base.match_token(FROM,&mut recog.err_handler)?;

			/*InvokeRule timeRef*/
			recog.base.set_state(125);
			recog.timeRef()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- value ----------------
pub type ValueContextAll<'input> = ValueContext<'input>;


pub type ValueContext<'input> = BaseParserRuleContext<'input,ValueContextExt<'input>>;

#[derive(Clone)]
pub struct ValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TimeExprGrammarParserContext<'input> for ValueContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for ValueContext<'input>{
		fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_value(self);
		}fn exit(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
			listener.exit_value(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}
antlr_rust::tid!{ValueContextExt<'a>}

impl<'input> ValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn TimeExprGrammarParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValueContextAttrs<'input>: TimeExprGrammarParserContext<'input> + BorrowMut<ValueContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token INT
/// Returns `None` if there is no child corresponding to token INT
fn INT(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
	self.get_token(INT, 0)
}
/// Retrieves first TerminalNode corresponding to token BOOL
/// Returns `None` if there is no child corresponding to token BOOL
fn BOOL(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
	self.get_token(BOOL, 0)
}

}

impl<'input> ValueContextAttrs<'input> for ValueContext<'input>{}

impl<'input, I, H> TimeExprGrammarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn value(&mut self,)
	-> Result<Rc<ValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_value);
        let mut _localctx: Rc<ValueContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(127);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << STRING) | (1usize << BOOL) | (1usize << INT))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- timeRef ----------------
#[derive(Debug)]
pub enum TimeRefContextAll<'input>{
	TimeReferenceWithListOfEventsContext(TimeReferenceWithListOfEventsContext<'input>),
	NowTimeRefContext(NowTimeRefContext<'input>),
Error(TimeRefContext<'input>)
}
antlr_rust::tid!{TimeRefContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for TimeRefContextAll<'input>{}

impl<'input> TimeExprGrammarParserContext<'input> for TimeRefContextAll<'input>{}

impl<'input> Deref for TimeRefContextAll<'input>{
	type Target = dyn TimeRefContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use TimeRefContextAll::*;
		match self{
			TimeReferenceWithListOfEventsContext(inner) => inner,
			NowTimeRefContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for TimeRefContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type TimeRefContext<'input> = BaseParserRuleContext<'input,TimeRefContextExt<'input>>;

#[derive(Clone)]
pub struct TimeRefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TimeExprGrammarParserContext<'input> for TimeRefContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for TimeRefContext<'input>{
}

impl<'input> CustomRuleContext<'input> for TimeRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_timeRef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_timeRef }
}
antlr_rust::tid!{TimeRefContextExt<'a>}

impl<'input> TimeRefContextExt<'input>{
	fn new(parent: Option<Rc<dyn TimeExprGrammarParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TimeRefContextAll<'input>> {
		Rc::new(
		TimeRefContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TimeRefContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait TimeRefContextAttrs<'input>: TimeExprGrammarParserContext<'input> + BorrowMut<TimeRefContextExt<'input>>{


}

impl<'input> TimeRefContextAttrs<'input> for TimeRefContext<'input>{}

pub type TimeReferenceWithListOfEventsContext<'input> = BaseParserRuleContext<'input,TimeReferenceWithListOfEventsContextExt<'input>>;

pub trait TimeReferenceWithListOfEventsContextAttrs<'input>: TimeExprGrammarParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LAST
	/// Returns `None` if there is no child corresponding to token LAST
	fn LAST(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(LAST, 0)
	}
	/// Retrieves first TerminalNode corresponding to token EVENT
	/// Returns `None` if there is no child corresponding to token EVENT
	fn EVENT(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(EVENT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token OF
	/// Returns `None` if there is no child corresponding to token OF
	fn OF(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(OF, 0)
	}
	fn eventListExpr(&self) -> Option<Rc<EventListExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TimeReferenceWithListOfEventsContextAttrs<'input> for TimeReferenceWithListOfEventsContext<'input>{}

pub struct TimeReferenceWithListOfEventsContextExt<'input>{
	base:TimeRefContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{TimeReferenceWithListOfEventsContextExt<'a>}

impl<'input> TimeExprGrammarParserContext<'input> for TimeReferenceWithListOfEventsContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for TimeReferenceWithListOfEventsContext<'input>{
	fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_TimeReferenceWithListOfEvents(self);
	}
}

impl<'input> CustomRuleContext<'input> for TimeReferenceWithListOfEventsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_timeRef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_timeRef }
}

impl<'input> Borrow<TimeRefContextExt<'input>> for TimeReferenceWithListOfEventsContext<'input>{
	fn borrow(&self) -> &TimeRefContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TimeRefContextExt<'input>> for TimeReferenceWithListOfEventsContext<'input>{
	fn borrow_mut(&mut self) -> &mut TimeRefContextExt<'input> { &mut self.base }
}

impl<'input> TimeRefContextAttrs<'input> for TimeReferenceWithListOfEventsContext<'input> {}

impl<'input> TimeReferenceWithListOfEventsContextExt<'input>{
	fn new(ctx: &dyn TimeRefContextAttrs<'input>) -> Rc<TimeRefContextAll<'input>>  {
		Rc::new(
			TimeRefContextAll::TimeReferenceWithListOfEventsContext(
				BaseParserRuleContext::copy_from(ctx,TimeReferenceWithListOfEventsContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NowTimeRefContext<'input> = BaseParserRuleContext<'input,NowTimeRefContextExt<'input>>;

pub trait NowTimeRefContextAttrs<'input>: TimeExprGrammarParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NOW
	/// Returns `None` if there is no child corresponding to token NOW
	fn NOW(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
		self.get_token(NOW, 0)
	}
}

impl<'input> NowTimeRefContextAttrs<'input> for NowTimeRefContext<'input>{}

pub struct NowTimeRefContextExt<'input>{
	base:TimeRefContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NowTimeRefContextExt<'a>}

impl<'input> TimeExprGrammarParserContext<'input> for NowTimeRefContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for NowTimeRefContext<'input>{
	fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_NowTimeRef(self);
	}
}

impl<'input> CustomRuleContext<'input> for NowTimeRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_timeRef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_timeRef }
}

impl<'input> Borrow<TimeRefContextExt<'input>> for NowTimeRefContext<'input>{
	fn borrow(&self) -> &TimeRefContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TimeRefContextExt<'input>> for NowTimeRefContext<'input>{
	fn borrow_mut(&mut self) -> &mut TimeRefContextExt<'input> { &mut self.base }
}

impl<'input> TimeRefContextAttrs<'input> for NowTimeRefContext<'input> {}

impl<'input> NowTimeRefContextExt<'input>{
	fn new(ctx: &dyn TimeRefContextAttrs<'input>) -> Rc<TimeRefContextAll<'input>>  {
		Rc::new(
			TimeRefContextAll::NowTimeRefContext(
				BaseParserRuleContext::copy_from(ctx,NowTimeRefContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TimeExprGrammarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn timeRef(&mut self,)
	-> Result<Rc<TimeRefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TimeRefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_timeRef);
        let mut _localctx: Rc<TimeRefContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(134);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 NOW 
				=> {
					let tmp = NowTimeRefContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(129);
					recog.base.match_token(NOW,&mut recog.err_handler)?;

					}
				}

			 LAST 
				=> {
					let tmp = TimeReferenceWithListOfEventsContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(130);
					recog.base.match_token(LAST,&mut recog.err_handler)?;

					recog.base.set_state(131);
					recog.base.match_token(EVENT,&mut recog.err_handler)?;

					recog.base.set_state(132);
					recog.base.match_token(OF,&mut recog.err_handler)?;

					/*InvokeRule eventListExpr*/
					recog.base.set_state(133);
					recog.eventListExpr()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- comparisonOp ----------------
pub type ComparisonOpContextAll<'input> = ComparisonOpContext<'input>;


pub type ComparisonOpContext<'input> = BaseParserRuleContext<'input,ComparisonOpContextExt<'input>>;

#[derive(Clone)]
pub struct ComparisonOpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TimeExprGrammarParserContext<'input> for ComparisonOpContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for ComparisonOpContext<'input>{
		fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_comparisonOp(self);
		}fn exit(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
			listener.exit_comparisonOp(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ComparisonOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comparisonOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comparisonOp }
}
antlr_rust::tid!{ComparisonOpContextExt<'a>}

impl<'input> ComparisonOpContextExt<'input>{
	fn new(parent: Option<Rc<dyn TimeExprGrammarParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComparisonOpContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComparisonOpContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComparisonOpContextAttrs<'input>: TimeExprGrammarParserContext<'input> + BorrowMut<ComparisonOpContextExt<'input>>{


}

impl<'input> ComparisonOpContextAttrs<'input> for ComparisonOpContext<'input>{}

impl<'input, I, H> TimeExprGrammarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn comparisonOp(&mut self,)
	-> Result<Rc<ComparisonOpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComparisonOpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_comparisonOp);
        let mut _localctx: Rc<ComparisonOpContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(136);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__1) | (1usize << T__2) | (1usize << T__3) | (1usize << T__4) | (1usize << T__5) | (1usize << T__6))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- logicalOp ----------------
pub type LogicalOpContextAll<'input> = LogicalOpContext<'input>;


pub type LogicalOpContext<'input> = BaseParserRuleContext<'input,LogicalOpContextExt<'input>>;

#[derive(Clone)]
pub struct LogicalOpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TimeExprGrammarParserContext<'input> for LogicalOpContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for LogicalOpContext<'input>{
		fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_logicalOp(self);
		}fn exit(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
			listener.exit_logicalOp(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for LogicalOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicalOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicalOp }
}
antlr_rust::tid!{LogicalOpContextExt<'a>}

impl<'input> LogicalOpContextExt<'input>{
	fn new(parent: Option<Rc<dyn TimeExprGrammarParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LogicalOpContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LogicalOpContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LogicalOpContextAttrs<'input>: TimeExprGrammarParserContext<'input> + BorrowMut<LogicalOpContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token AND
/// Returns `None` if there is no child corresponding to token AND
fn AND(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
	self.get_token(AND, 0)
}
/// Retrieves first TerminalNode corresponding to token OR
/// Returns `None` if there is no child corresponding to token OR
fn OR(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
	self.get_token(OR, 0)
}

}

impl<'input> LogicalOpContextAttrs<'input> for LogicalOpContext<'input>{}

impl<'input, I, H> TimeExprGrammarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn logicalOp(&mut self,)
	-> Result<Rc<LogicalOpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LogicalOpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_logicalOp);
        let mut _localctx: Rc<LogicalOpContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(138);
			_la = recog.base.input.la(1);
			if { !(_la==AND || _la==OR) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- eventName ----------------
pub type EventNameContextAll<'input> = EventNameContext<'input>;


pub type EventNameContext<'input> = BaseParserRuleContext<'input,EventNameContextExt<'input>>;

#[derive(Clone)]
pub struct EventNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TimeExprGrammarParserContext<'input> for EventNameContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for EventNameContext<'input>{
		fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_eventName(self);
		}fn exit(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
			listener.exit_eventName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for EventNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eventName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eventName }
}
antlr_rust::tid!{EventNameContextExt<'a>}

impl<'input> EventNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn TimeExprGrammarParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EventNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EventNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EventNameContextAttrs<'input>: TimeExprGrammarParserContext<'input> + BorrowMut<EventNameContextExt<'input>>{

fn singleEventName(&self) -> Option<Rc<SingleEventNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn eventStateName(&self) -> Option<Rc<EventStateNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EventNameContextAttrs<'input> for EventNameContext<'input>{}

impl<'input, I, H> TimeExprGrammarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eventName(&mut self,)
	-> Result<Rc<EventNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EventNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_eventName);
        let mut _localctx: Rc<EventNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(142);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(13,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule singleEventName*/
					recog.base.set_state(140);
					recog.singleEventName()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule eventStateName*/
					recog.base.set_state(141);
					recog.eventStateName()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- singleEventName ----------------
pub type SingleEventNameContextAll<'input> = SingleEventNameContext<'input>;


pub type SingleEventNameContext<'input> = BaseParserRuleContext<'input,SingleEventNameContextExt<'input>>;

#[derive(Clone)]
pub struct SingleEventNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TimeExprGrammarParserContext<'input> for SingleEventNameContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for SingleEventNameContext<'input>{
		fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_singleEventName(self);
		}fn exit(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
			listener.exit_singleEventName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for SingleEventNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_singleEventName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_singleEventName }
}
antlr_rust::tid!{SingleEventNameContextExt<'a>}

impl<'input> SingleEventNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn TimeExprGrammarParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SingleEventNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SingleEventNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SingleEventNameContextAttrs<'input>: TimeExprGrammarParserContext<'input> + BorrowMut<SingleEventNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
	self.get_token(ID, 0)
}

}

impl<'input> SingleEventNameContextAttrs<'input> for SingleEventNameContext<'input>{}

impl<'input, I, H> TimeExprGrammarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn singleEventName(&mut self,)
	-> Result<Rc<SingleEventNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SingleEventNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_singleEventName);
        let mut _localctx: Rc<SingleEventNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(144);
			recog.base.match_token(ID,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- eventStateName ----------------
pub type EventStateNameContextAll<'input> = EventStateNameContext<'input>;


pub type EventStateNameContext<'input> = BaseParserRuleContext<'input,EventStateNameContextExt<'input>>;

#[derive(Clone)]
pub struct EventStateNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TimeExprGrammarParserContext<'input> for EventStateNameContext<'input>{}

impl<'input,'a> Listenable<dyn TimeExprGrammarListener<'input> + 'a> for EventStateNameContext<'input>{
		fn enter(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_eventStateName(self);
		}fn exit(&self,listener: &mut (dyn TimeExprGrammarListener<'input> + 'a)) {
			listener.exit_eventStateName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for EventStateNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TimeExprGrammarParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eventStateName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eventStateName }
}
antlr_rust::tid!{EventStateNameContextExt<'a>}

impl<'input> EventStateNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn TimeExprGrammarParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EventStateNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EventStateNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EventStateNameContextAttrs<'input>: TimeExprGrammarParserContext<'input> + BorrowMut<EventStateNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,TimeExprGrammarParserContextType>>> where Self:Sized{
	self.get_token(ID, 0)
}

}

impl<'input> EventStateNameContextAttrs<'input> for EventStateNameContext<'input>{}

impl<'input, I, H> TimeExprGrammarParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eventStateName(&mut self,)
	-> Result<Rc<EventStateNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EventStateNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_eventStateName);
        let mut _localctx: Rc<EventStateNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(146);
			recog.base.match_token(ID,&mut recog.err_handler)?;

			recog.base.set_state(147);
			recog.base.match_token(T__7,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x22\u{98}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\x0e\
	\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x03\x02\
	\x03\x02\x03\x02\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\
	\x03\x03\x05\x03\x30\x0a\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\
	\x03\x07\x03\x38\x0a\x03\x0c\x03\x0e\x03\x3b\x0b\x03\x03\x04\x03\x04\x03\
	\x05\x03\x05\x05\x05\x41\x0a\x05\x03\x06\x03\x06\x03\x06\x03\x06\x05\x06\
	\x47\x0a\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x05\x06\x4f\
	\x0a\x06\x05\x06\x51\x0a\x06\x03\x07\x03\x07\x03\x07\x03\x07\x07\x07\x57\
	\x0a\x07\x0c\x07\x0e\x07\x5a\x0b\x07\x03\x07\x03\x07\x03\x08\x03\x08\x03\
	\x08\x03\x08\x03\x08\x03\x08\x05\x08\x64\x0a\x08\x03\x08\x03\x08\x03\x08\
	\x03\x08\x03\x08\x03\x08\x07\x08\x6c\x0a\x08\x0c\x08\x0e\x08\x6f\x0b\x08\
	\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x05\x09\x78\x0a\
	\x09\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\
	\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{89}\x0a\x0d\
	\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x03\x10\x03\x10\x05\x10\u{91}\x0a\x10\x03\
	\x11\x03\x11\x03\x12\x03\x12\x03\x12\x03\x12\x02\x04\x04\x0e\x13\x02\x04\
	\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x02\x05\x05\
	\x02\x0b\x0b\x1f\x1f\x21\x21\x03\x02\x04\x09\x03\x02\x16\x17\x02\u{95}\x02\
	\x24\x03\x02\x02\x02\x04\x2f\x03\x02\x02\x02\x06\x3c\x03\x02\x02\x02\x08\
	\x3e\x03\x02\x02\x02\x0a\x50\x03\x02\x02\x02\x0c\x52\x03\x02\x02\x02\x0e\
	\x63\x03\x02\x02\x02\x10\x77\x03\x02\x02\x02\x12\x79\x03\x02\x02\x02\x14\
	\x7b\x03\x02\x02\x02\x16\u{81}\x03\x02\x02\x02\x18\u{88}\x03\x02\x02\x02\
	\x1a\u{8a}\x03\x02\x02\x02\x1c\u{8c}\x03\x02\x02\x02\x1e\u{90}\x03\x02\x02\
	\x02\x20\u{92}\x03\x02\x02\x02\x22\u{94}\x03\x02\x02\x02\x24\x25\x05\x04\
	\x03\x02\x25\x26\x07\x02\x02\x03\x26\x03\x03\x02\x02\x02\x27\x28\x08\x03\
	\x01\x02\x28\x29\x07\x15\x02\x02\x29\x30\x05\x04\x03\x05\x2a\x30\x05\x06\
	\x04\x02\x2b\x2c\x07\x11\x02\x02\x2c\x2d\x05\x04\x03\x02\x2d\x2e\x07\x12\
	\x02\x02\x2e\x30\x03\x02\x02\x02\x2f\x27\x03\x02\x02\x02\x2f\x2a\x03\x02\
	\x02\x02\x2f\x2b\x03\x02\x02\x02\x30\x39\x03\x02\x02\x02\x31\x32\x0c\x07\
	\x02\x02\x32\x33\x07\x16\x02\x02\x33\x38\x05\x04\x03\x08\x34\x35\x0c\x06\
	\x02\x02\x35\x36\x07\x17\x02\x02\x36\x38\x05\x04\x03\x07\x37\x31\x03\x02\
	\x02\x02\x37\x34\x03\x02\x02\x02\x38\x3b\x03\x02\x02\x02\x39\x37\x03\x02\
	\x02\x02\x39\x3a\x03\x02\x02\x02\x3a\x05\x03\x02\x02\x02\x3b\x39\x03\x02\
	\x02\x02\x3c\x3d\x05\x08\x05\x02\x3d\x07\x03\x02\x02\x02\x3e\x40\x05\x0a\
	\x06\x02\x3f\x41\x05\x14\x0b\x02\x40\x3f\x03\x02\x02\x02\x40\x41\x03\x02\
	\x02\x02\x41\x09\x03\x02\x02\x02\x42\x46\x07\x10\x02\x02\x43\x44\x07\x14\
	\x02\x02\x44\x45\x07\x15\x02\x02\x45\x47\x05\x08\x05\x02\x46\x43\x03\x02\
	\x02\x02\x46\x47\x03\x02\x02\x02\x47\x51\x03\x02\x02\x02\x48\x4e\x05\x1e\
	\x10\x02\x49\x4a\x07\x0e\x02\x02\x4a\x4b\x07\x0c\x02\x02\x4b\x4c\x05\x0e\
	\x08\x02\x4c\x4d\x07\x0d\x02\x02\x4d\x4f\x03\x02\x02\x02\x4e\x49\x03\x02\
	\x02\x02\x4e\x4f\x03\x02\x02\x02\x4f\x51\x03\x02\x02\x02\x50\x42\x03\x02\
	\x02\x02\x50\x48\x03\x02\x02\x02\x51\x0b\x03\x02\x02\x02\x52\x53\x07\x0c\
	\x02\x02\x53\x58\x05\x0a\x06\x02\x54\x55\x07\x03\x02\x02\x55\x57\x05\x0a\
	\x06\x02\x56\x54\x03\x02\x02\x02\x57\x5a\x03\x02\x02\x02\x58\x56\x03\x02\
	\x02\x02\x58\x59\x03\x02\x02\x02\x59\x5b\x03\x02\x02\x02\x5a\x58\x03\x02\
	\x02\x02\x5b\x5c\x07\x0d\x02\x02\x5c\x0d\x03\x02\x02\x02\x5d\x5e\x08\x08\
	\x01\x02\x5e\x5f\x07\x11\x02\x02\x5f\x60\x05\x0e\x08\x02\x60\x61\x07\x12\
	\x02\x02\x61\x64\x03\x02\x02\x02\x62\x64\x05\x10\x09\x02\x63\x5d\x03\x02\
	\x02\x02\x63\x62\x03\x02\x02\x02\x64\x6d\x03\x02\x02\x02\x65\x66\x0c\x06\
	\x02\x02\x66\x67\x07\x16\x02\x02\x67\x6c\x05\x0e\x08\x07\x68\x69\x0c\x05\
	\x02\x02\x69\x6a\x07\x17\x02\x02\x6a\x6c\x05\x0e\x08\x06\x6b\x65\x03\x02\
	\x02\x02\x6b\x68\x03\x02\x02\x02\x6c\x6f\x03\x02\x02\x02\x6d\x6b\x03\x02\
	\x02\x02\x6d\x6e\x03\x02\x02\x02\x6e\x0f\x03\x02\x02\x02\x6f\x6d\x03\x02\
	\x02\x02\x70\x71\x05\x12\x0a\x02\x71\x72\x05\x1a\x0e\x02\x72\x73\x05\x16\
	\x0c\x02\x73\x78\x03\x02\x02\x02\x74\x75\x05\x12\x0a\x02\x75\x76\x07\x0f\
	\x02\x02\x76\x78\x03\x02\x02\x02\x77\x70\x03\x02\x02\x02\x77\x74\x03\x02\
	\x02\x02\x78\x11\x03\x02\x02\x02\x79\x7a\x07\x20\x02\x02\x7a\x13\x03\x02\
	\x02\x02\x7b\x7c\x07\x19\x02\x02\x7c\x7d\x07\x21\x02\x02\x7d\x7e\x07\x13\
	\x02\x02\x7e\x7f\x07\x1a\x02\x02\x7f\u{80}\x05\x18\x0d\x02\u{80}\x15\x03\
	\x02\x02\x02\u{81}\u{82}\x09\x02\x02\x02\u{82}\x17\x03\x02\x02\x02\u{83}\
	\u{89}\x07\x18\x02\x02\u{84}\u{85}\x07\x1b\x02\x02\u{85}\u{86}\x07\x1c\x02\
	\x02\u{86}\u{87}\x07\x1d\x02\x02\u{87}\u{89}\x05\x0c\x07\x02\u{88}\u{83}\
	\x03\x02\x02\x02\u{88}\u{84}\x03\x02\x02\x02\u{89}\x19\x03\x02\x02\x02\u{8a}\
	\u{8b}\x09\x03\x02\x02\u{8b}\x1b\x03\x02\x02\x02\u{8c}\u{8d}\x09\x04\x02\
	\x02\u{8d}\x1d\x03\x02\x02\x02\u{8e}\u{91}\x05\x20\x11\x02\u{8f}\u{91}\x05\
	\x22\x12\x02\u{90}\u{8e}\x03\x02\x02\x02\u{90}\u{8f}\x03\x02\x02\x02\u{91}\
	\x1f\x03\x02\x02\x02\u{92}\u{93}\x07\x20\x02\x02\u{93}\x21\x03\x02\x02\x02\
	\u{94}\u{95}\x07\x20\x02\x02\u{95}\u{96}\x07\x0a\x02\x02\u{96}\x23\x03\x02\
	\x02\x02\x10\x2f\x37\x39\x40\x46\x4e\x50\x58\x63\x6b\x6d\x77\u{88}\u{90}";

