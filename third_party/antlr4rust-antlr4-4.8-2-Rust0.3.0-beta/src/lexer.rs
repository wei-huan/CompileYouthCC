//! Lexer implementation
use std::borrow::Cow::Borrowed;
use std::borrow::{Borrow, Cow};
use std::cell::{Cell, RefCell};

use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::char_stream::{CharStream, InputData};
use crate::error_listener::{ConsoleErrorListener, ErrorListener};
use crate::errors::ANTLRError;
use crate::int_stream::IntStream;
use crate::lexer_atn_simulator::{ILexerATNSimulator, LexerATNSimulator};
use crate::parser::ParserNodeType;

use crate::recognizer::{Actions, Recognizer};
use crate::rule_context::EmptyContextType;
use crate::token::TOKEN_INVALID_TYPE;
use crate::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use crate::token_source::TokenSource;
use std::ops::{Deref, DerefMut};

///  Lexer functionality required by `LexerATNSimulator` to work properly
pub trait Lexer<'input>:
    TokenSource<'input>
    + Recognizer<'input, Node = EmptyContextType<'input, <Self as TokenAware<'input>>::TF>>
{
    /// Concrete input stream used by this parser
    type Input: IntStream;
    /// Same as `TokenStream::get_input_stream` but returns concrete type instance
    /// important for proper inlining in hot code of `LexerATNSimulator`
    fn input(&mut self) -> &mut Self::Input;
    /// Sets channel where current token will be pushed
    ///
    /// By default two channels are available:
    ///  - `LEXER_DEFAULT_TOKEN_CHANNEL`
    ///  - `LEXER_HIDDEN`
    fn set_channel(&mut self, v: isize);

    /// Pushes current mode to internal mode stack and sets `m` as current lexer mode
    /// `pop_mode should be used to recover previous mode
    fn push_mode(&mut self, m: usize);

    /// Pops mode from internal mode stack
    fn pop_mode(&mut self) -> Option<usize>;

    /// Sets type of the current token
    /// Called from action to override token that will be emitted by lexer
    fn set_type(&mut self, t: isize);

    /// Sets lexer mode discarding current one
    fn set_mode(&mut self, m: usize);

    /// Used to informs lexer that it should consider next token as a continuation of the current one
    fn more(&mut self);

    /// Tells lexer to completely ignore and not emit current token.
    fn skip(&mut self);

    #[doc(hidden)]
    fn reset(&mut self);

    #[doc(hidden)]
    fn get_interpreter(&self) -> Option<&LexerATNSimulator>;
}

/// **! Usually generated by ANTLR !**
///
/// This trait combines everything that can be used to extend Lexer behavior
pub trait LexerRecog<'a, T: Recognizer<'a>>: Actions<'a, T> + Sized + 'static {
    /// Callback to extend emit behavior
    fn before_emit(_lexer: &mut T) {}
}

/// Default implementation of Lexer
///
/// Public fields in this struct are intended to be used by embedded actions
#[allow(missing_docs)]
pub struct BaseLexer<
    'input,
    T: LexerRecog<'input, Self> + 'static,
    Input: CharStream<TF::From>,
    TF: TokenFactory<'input> = CommonTokenFactory,
> {
    /// `LexerATNSimulator` instance of this lexer
    pub interpreter: Option<Box<LexerATNSimulator>>,
    /// `CharStream` used by this lexer
    pub input: Option<Input>,
    recog: T,

    factory: &'input TF,

    error_listeners: RefCell<Vec<Box<dyn ErrorListener<'input, Self>>>>,

    pub token_start_char_index: isize,
    pub token_start_line: isize,
    pub token_start_column: isize,
    current_pos: Rc<LexerPosition>,
    /// Overrides token type emitted by lexer for current token
    pub token_type: isize,
    /// Make it `Some` to override token that is currently being generated by lexer
    pub token: Option<TF::Tok>,
    hit_eof: bool,
    /// Channel lexer is currently assigning tokens to
    pub channel: isize,
    /// stack of modes, which is used for pushMode,popMode lexer actions
    pub mode_stack: Vec<usize>,
    /// Mode lexer is currently in
    pub mode: usize,
    /// Make it `Some` to override text for token that is currently being generated by lexer
    pub text: Option<<TF::Data as ToOwned>::Owned>,
}

impl<'input, T, Input> Debug for BaseLexer<'input, T, Input>
where
    T: LexerRecog<'input, BaseLexer<'input, T, Input>>,
    Input: CharStream<Cow<'input, str>>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BaseLexer")
            .field("token_start_char_index", &self.token_start_char_index)
            .field("token_start_line", &self.token_start_line)
            .field("token_start_column", &self.token_start_column)
            .field("token_type", &self.token_type)
            .field("token", &self.token)
            .field("hit_eof", &self.hit_eof)
            .field("channel", &self.channel)
            .field("mode_stack", &self.mode_stack)
            .field("mode", &self.mode)
            .field("text", &self.text)
            .finish()
    }
}

#[derive(Debug)]
pub(crate) struct LexerPosition {
    pub(crate) line: Cell<isize>,
    pub(crate) char_position_in_line: Cell<isize>,
}

impl<'input, T, Input, TF> Deref for BaseLexer<'input, T, Input, TF>
where
    T: LexerRecog<'input, Self> + 'static,
    Input: CharStream<TF::From>,
    TF: TokenFactory<'input>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.recog
    }
}

impl<'input, T, Input, TF> DerefMut for BaseLexer<'input, T, Input, TF>
where
    T: LexerRecog<'input, Self> + 'static,
    Input: CharStream<TF::From>,
    TF: TokenFactory<'input>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.recog
    }
}

impl<'input, T, Input, TF> Recognizer<'input> for BaseLexer<'input, T, Input, TF>
where
    T: LexerRecog<'input, Self> + 'static,
    Input: CharStream<TF::From>,
    TF: TokenFactory<'input>,
{
    type Node = EmptyContextType<'input, TF>;

    fn sempred(
        &mut self,
        _localctx: Option<&<Self::Node as ParserNodeType<'input>>::Type>,
        rule_index: isize,
        action_index: isize,
    ) -> bool {
        <T as Actions<'input, Self>>::sempred(_localctx, rule_index, action_index, self)
    }

    fn action(
        &mut self,
        _localctx: Option<&<Self::Node as ParserNodeType<'input>>::Type>,
        rule_index: isize,
        action_index: isize,
    ) {
        <T as Actions<'input, Self>>::action(_localctx, rule_index, action_index, self)
    }
}

/// Default lexer mode id
pub const LEXER_DEFAULT_MODE: usize = 0;
/// Special token type to indicate that lexer should continue current token on next iteration
/// see `Lexer::more()`
pub const LEXER_MORE: isize = -2;
/// Special token type to indicate that lexer should not return current token
/// usually used to skip whitespaces and comments
/// see `Lexer::skip()`
pub const LEXER_SKIP: isize = -3;

#[doc(inline)]
pub use super::token::TOKEN_DEFAULT_CHANNEL as LEXER_DEFAULT_TOKEN_CHANNEL;

#[doc(inline)]
pub use super::token::TOKEN_HIDDEN_CHANNEL as LEXER_HIDDEN;

pub(crate) const LEXER_MIN_CHAR_VALUE: isize = 0x0000;
pub(crate) const LEXER_MAX_CHAR_VALUE: isize = 0x10FFFF;

impl<'input, T, Input, TF> BaseLexer<'input, T, Input, TF>
where
    T: LexerRecog<'input, Self> + 'static,
    Input: CharStream<TF::From>,
    TF: TokenFactory<'input>,
{
    fn emit_token(&mut self, token: TF::Tok) {
        self.token = Some(token);
    }

    fn emit(&mut self) {
        <T as LexerRecog<Self>>::before_emit(self);
        let stop = self.get_char_index() - 1;
        let token = self.factory.create(
            Some(self.input.as_mut().unwrap()),
            self.token_type,
            self.text.take(),
            self.channel,
            self.token_start_char_index,
            stop,
            self.token_start_line,
            self.token_start_column,
        );
        self.emit_token(token);
    }

    fn emit_eof(&mut self) {
        let token = self.factory.create(
            None::<&mut Input>,
            super::int_stream::EOF,
            None,
            LEXER_DEFAULT_TOKEN_CHANNEL,
            self.get_char_index(),
            self.get_char_index() - 1,
            self.get_line(),
            self.get_char_position_in_line(),
        );
        self.emit_token(token)
    }

    /// Current position in input stream
    pub fn get_char_index(&self) -> isize {
        self.input.as_ref().unwrap().index()
    }

    /// Current token text
    pub fn get_text<'a>(&'a self) -> Cow<'a, TF::Data>
    where
        'input: 'a,
    {
        self.text
            .as_ref()
            .map(|it| Borrowed(it.borrow()))
            // .unwrap_or("")
            .unwrap_or_else(|| {
                let text = self
                    .input
                    .as_ref()
                    .unwrap()
                    .get_text(self.token_start_char_index, self.get_char_index() - 1);
                TF::get_data(text)
            })
    }

    /// Used from lexer actions to override text of the token that will be emitted next
    pub fn set_text(&mut self, _text: <TF::Data as ToOwned>::Owned) {
        self.text = Some(_text);
    }

    // fn get_all_tokens(&mut self) -> Vec<TF::Tok> { unimplemented!() }

    // fn get_char_error_display(&self, _c: char) -> String { unimplemented!() }

    /// Add error listener
    pub fn add_error_listener(&mut self, listener: Box<dyn ErrorListener<'input, Self>>) {
        self.error_listeners.borrow_mut().push(listener);
    }

    /// Remove and drop all error listeners
    pub fn remove_error_listeners(&mut self) {
        self.error_listeners.borrow_mut().clear();
    }

    /// Creates new lexer instance
    pub fn new_base_lexer(
        input: Input,
        interpreter: LexerATNSimulator,
        recog: T,
        factory: &'input TF,
    ) -> Self {
        let mut lexer = Self {
            interpreter: Some(Box::new(interpreter)),
            input: Some(input),
            recog,
            factory,
            error_listeners: RefCell::new(vec![Box::new(ConsoleErrorListener {})]),
            token_start_char_index: 0,
            token_start_line: 0,
            token_start_column: 0,
            current_pos: Rc::new(LexerPosition {
                line: Cell::new(1),
                char_position_in_line: Cell::new(0),
            }),
            token_type: super::token::TOKEN_INVALID_TYPE,
            text: None,
            token: None,
            hit_eof: false,
            channel: super::token::TOKEN_DEFAULT_CHANNEL,
            //            token_factory_source_pair: None,
            mode_stack: Vec::new(),
            mode: self::LEXER_DEFAULT_MODE,
        };
        let pos = lexer.current_pos.clone();
        lexer.interpreter.as_mut().unwrap().current_pos = pos;
        lexer
    }
}

impl<'input, T, Input, TF> TokenAware<'input> for BaseLexer<'input, T, Input, TF>
where
    T: LexerRecog<'input, Self> + 'static,
    Input: CharStream<TF::From>,
    TF: TokenFactory<'input>,
{
    type TF = TF;
}

impl<'input, T, Input, TF> TokenSource<'input> for BaseLexer<'input, T, Input, TF>
where
    T: LexerRecog<'input, Self> + 'static,
    Input: CharStream<TF::From>,
    TF: TokenFactory<'input>,
{
    type TF = TF;
    #[inline]
    #[allow(unused_labels)]
    fn next_token(&mut self) -> <Self::TF as TokenFactory<'input>>::Tok {
        assert!(self.input.is_some());

        let _marker = self.input().mark();
        'outer: loop {
            if self.hit_eof {
                self.emit_eof();
                break;
            }
            self.token = None;
            self.channel = LEXER_DEFAULT_TOKEN_CHANNEL;
            self.token_start_column = self
                .interpreter
                .as_ref()
                .unwrap()
                .get_char_position_in_line();
            self.token_start_line = self.interpreter.as_ref().unwrap().get_line();
            self.text = None;
            let index = self.input().index();
            self.token_start_char_index = index;

            'inner: loop {
                self.token_type = TOKEN_INVALID_TYPE;
                // detach from self, to allow self to be passed deeper
                let mut interpreter = self.interpreter.take().unwrap();
                //                    let mut input = self.input.take().unwrap();
                let result = interpreter.match_token(self.mode, self);
                self.interpreter = Some(interpreter);

                let ttype = result.unwrap_or_else(|err| {
                    //                            println!("error, recovering");
                    notify_listeners(&mut self.error_listeners.borrow_mut(), &err, self);
                    self.interpreter
                        .as_mut()
                        .unwrap()
                        .recover(err, self.input.as_mut().unwrap());
                    LEXER_SKIP
                });
                //                    self.input = Some(input)

                if self.input().la(1) == super::int_stream::EOF {
                    self.hit_eof = true;
                }

                if self.token_type == TOKEN_INVALID_TYPE {
                    self.token_type = ttype;
                }

                if self.token_type == LEXER_SKIP {
                    continue 'outer;
                }

                if self.token_type != LEXER_MORE {
                    break;
                }
            }

            if self.token.is_none() {
                self.emit();
                break;
            }
        }
        self.input().release(_marker);
        self.token.take().unwrap()
    }

    fn get_line(&self) -> isize {
        self.current_pos.line.get()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.current_pos.char_position_in_line.get()
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn IntStream> {
        match &mut self.input {
            None => None,
            Some(x) => Some(x as _),
        }
    }

    fn get_source_name(&self) -> String {
        self.input
            .as_ref()
            .map(|it| it.get_source_name())
            .unwrap_or("<none>".to_string())
    }

    //    fn set_token_factory<'c: 'b>(&mut self, f: &'c TokenFactory) {
    //        self.factory = f;
    //    }

    fn get_token_factory(&self) -> &'input TF {
        self.factory
    }
}

#[cold]
#[inline(never)]
fn notify_listeners<'input, T, Input, TF>(
    liseners: &mut Vec<Box<dyn ErrorListener<'input, BaseLexer<'input, T, Input, TF>>>>,
    e: &ANTLRError,
    lexer: &BaseLexer<'input, T, Input, TF>,
) where
    T: LexerRecog<'input, BaseLexer<'input, T, Input, TF>> + 'static,
    Input: CharStream<TF::From>,
    TF: TokenFactory<'input>,
{
    let inner = lexer
        .input
        .as_ref()
        .unwrap()
        .get_text(lexer.token_start_char_index, lexer.get_char_index());
    let text = format!(
        "token recognition error at: '{}'",
        TF::get_data(inner).to_display()
    );
    for listener in liseners.iter_mut() {
        listener.syntax_error(
            lexer,
            None,
            lexer.token_start_line,
            lexer.token_start_column,
            &text,
            Some(e),
        )
    }
}

impl<'input, T, Input, TF> Lexer<'input> for BaseLexer<'input, T, Input, TF>
where
    T: LexerRecog<'input, Self> + 'static,
    Input: CharStream<TF::From>,
    TF: TokenFactory<'input>,
{
    type Input = Input;

    fn input(&mut self) -> &mut Self::Input {
        self.input.as_mut().unwrap()
    }

    fn set_channel(&mut self, v: isize) {
        self.channel = v;
    }

    fn push_mode(&mut self, m: usize) {
        self.mode_stack.push(self.mode);
        self.mode = m;
    }

    fn pop_mode(&mut self) -> Option<usize> {
        self.mode_stack.pop().map(|mode| {
            self.mode = mode;
            mode
        })
    }

    fn set_type(&mut self, t: isize) {
        self.token_type = t;
    }

    fn set_mode(&mut self, m: usize) {
        self.mode = m;
    }

    fn more(&mut self) {
        self.set_type(LEXER_MORE)
    }

    fn skip(&mut self) {
        self.set_type(LEXER_SKIP)
    }

    fn reset(&mut self) {
        unimplemented!()
    }

    fn get_interpreter(&self) -> Option<&LexerATNSimulator> {
        self.interpreter.as_deref()
    }
}
