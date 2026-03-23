use std::any::{type_name, Any, TypeId};
use std::collections::HashMap;
use std::marker::PhantomData;

use thiserror::Error;

/// Komenda udostępnia typ zwracanej odpowiedzi za pomocą powiązanego typu.
pub trait Command {
    type Output;
}

/// Handler przetwarza konkretny typ komendy i produkuje wynik.
pub trait CommandHandler<C: Command> {
    fn handle(&self, command: &C) -> C::Output;
}

impl<C, F> CommandHandler<C> for F
where
    C: Command,
    F: Fn(&C) -> C::Output,
{
    fn handle(&self, command: &C) -> C::Output {
        (self)(command)
    }
}

/// Błędy, które może zwrócić `CommandBus`.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DispatchError {
    /// Rejestracja handlera dla typu komendy, który już posiada handler.
    #[error("handler for `{command}` already registered")]
    HandlerAlreadyRegistered { command: &'static str },
    /// Próba wykonania komendy bez zarejestrowanego handlera.
    #[error("no handler registered for `{command}`")]
    HandlerNotFound { command: &'static str },
    /// Handler zwrócił wartość innego typu niż zadeklarowany w `Command::Output`.
    #[error("handler for `{command}` returned unexpected output type")]
    IncompatibleOutput { command: &'static str },
    /// Handler nie potrafił rzutować przekazanej komendy na oczekiwany typ.
    #[error("mismatched command type for handler of `{command}`")]
    WrongCommandType { command: &'static str },
}

/// Centralna szyna komend przechowująca zarejestrowane handlery.
pub struct CommandBus {
    handlers: HashMap<TypeId, HandlerEntry>,
}

struct HandlerEntry {
    command_name: &'static str,
    output_name: &'static str,
    handler: Box<dyn ErasedHandler>,
}

trait ErasedHandler: Send + Sync {
    fn handle(&self, command: &dyn Any) -> Result<Box<dyn Any>, DispatchError>;
}

struct TypedHandler<C, H> {
    handler: H,
    _marker: PhantomData<C>,
}

impl<C, H> TypedHandler<C, H> {
    fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<C, H> ErasedHandler for TypedHandler<C, H>
where
    C: Command + 'static + Send + Sync,
    C::Output: 'static,
    H: CommandHandler<C> + Send + Sync,
{
    fn handle(&self, command: &dyn Any) -> Result<Box<dyn Any>, DispatchError> {
        let cmd_opt = command.downcast_ref::<C>();

        if let Some(cmd) = cmd_opt {
            return Ok(Box::new(self.handler.handle(cmd)));
        } else {
            return Err(DispatchError::WrongCommandType {
                command: type_name::<C>(),
            });
        }
    }
}

impl CommandBus {
    /// Tworzy pustą szynę komend.
    pub fn new() -> Self {
        Self {
            handlers: HashMap::default(),
        }
    }

    /// Rejestruje handler dla komendy `C`. Zwraca błąd, gdy handler już istnieje.
    pub fn register<C, H>(&mut self, handler: H) -> Result<(), DispatchError>
    where
        C: Command + 'static + Send + Sync,
        C::Output: 'static,
        H: CommandHandler<C> + Send + Sync + 'static,
    {
        let new_handler = HandlerEntry {
            command_name: type_name::<C>(),
            output_name: type_name::<C::Output>(),
            handler: Box::new(TypedHandler::new(handler)),
        };

        let key = TypeId::of::<C>();

        let prev = self.handlers.insert(key, new_handler);
        if let Some(a) = prev {
            let err = DispatchError::HandlerAlreadyRegistered {
                command: a.command_name,
            };

            self.handlers.insert(key, a);

            Err(err)
        } else {
            Ok(())
        }
    }

    /// Sprawdza, czy handler dla komendy `C` został już zarejestrowany.
    pub fn contains<C>(&self) -> bool
    where
        C: Command + 'static,
    {
        self.handlers.contains_key(&TypeId::of::<C>())
    }

    /// Uruchamia handler dla komendy `C` i zwraca jego wynik.
    pub fn execute<C>(&self, command: &C) -> Result<C::Output, DispatchError>
    where
        C: Command + 'static,
        C::Output: 'static,
    {
        let handler_opt = self.handlers.get(&TypeId::of::<C>());

        if handler_opt.is_none() {
            return Err(DispatchError::HandlerNotFound {
                command: type_name::<C>(),
            });
        }

        let handler = handler_opt.unwrap();

        let retval = handler.handler.handle(command)?;
        if let Ok(ret) = retval.downcast::<C::Output>() {
            return Ok(*ret);
        } else {
            return Err(DispatchError::IncompatibleOutput {
                command: type_name::<C>(),
            });
        }
    }
}

impl Default for CommandBus {
    fn default() -> Self {
        Self::new()
    }
}
