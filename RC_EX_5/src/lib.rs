use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use thiserror::Error;

/// Wspólny alias na future zwracane przez poszczególne etapy handshake.
pub type HandshakeFuture<T> =
    Pin<Box<dyn Future<Output = Result<T, HandshakeError>> + Send + 'static>>;

/// Uchwyty pozwalające backendowi dowiedzieć się, że handshake został anulowany na danym etapie.
#[derive(Clone)]
pub struct CancellationHandle {
    inner: Arc<dyn Fn(HandshakePhase) + Send + Sync>,
}

impl CancellationHandle {
    /// Tworzy nowy uchwyt wywołujący podane zamknięcie przy anulowaniu.
    pub fn new<F>(notify: F) -> Self
    where
        F: Fn(HandshakePhase) + Send + Sync + 'static,
    {
        todo!()
    }

    /// Uchwyt, który ignoruje anulowania.
    pub fn noop() -> Self {
        todo!()
    }

    /// Wysyła informację o anulowaniu do backendu.
    pub fn notify(&self, phase: HandshakePhase) {
        todo!()
    }
}

impl Default for CancellationHandle {
    fn default() -> Self {
        Self::noop()
    }
}

/// Future pojedynczego kroku handshake wraz z powiązanym uchwytem anulowania.
pub struct StepFuture<T> {
    future: HandshakeFuture<T>,
    cancellation: CancellationHandle,
}

impl<T> StepFuture<T> {
    /// Pakuje dowolne future oraz uchwyt anulowania w strukturę kroku.
    pub fn new<F>(future: F, cancellation: CancellationHandle) -> Self
    where
        F: Future<Output = Result<T, HandshakeError>> + Send + 'static,
    {
        todo!()
    }

    /// Zwraca powiązane future oraz uchwyt anulowania.
    pub fn into_parts(self) -> (HandshakeFuture<T>, CancellationHandle) {
        todo!()
    }
}

impl<T> std::fmt::Debug for StepFuture<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StepFuture").finish_non_exhaustive()
    }
}

/// Publiczny stan maszyny handshake.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakePhase {
    Resolving,
    Connecting,
    Authenticating,
    Finished,
}

/// Wynik udanego handshake.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandshakeOutcome<R, C, A> {
    pub resolved: R,
    pub connection: C,
    pub session: A,
}

/// Błędy możliwe podczas handshake.
#[derive(Debug, Error)]
pub enum HandshakeError {
    #[error("resolve step failed: {0}")]
    Resolve(String),
    #[error("connect step failed: {0}")]
    Connect(String),
    #[error("authenticate step failed: {0}")]
    Authenticate(String),
    #[error("handshake cancelled in phase {phase:?}")]
    Cancelled { phase: HandshakePhase },
}

/// Backend dostarczający future kolejnych etapów handshake.
pub trait HandshakeBackend: Send {
    type ResolveOutput: Send + 'static;
    type ConnectOutput: Send + 'static;
    type AuthOutput: Send + 'static;

    fn resolve(&mut self) -> StepFuture<Self::ResolveOutput>;
    fn connect(&mut self, resolved: Self::ResolveOutput) -> StepFuture<Self::ConnectOutput>;
    fn authenticate(
        &mut self,
        connection: Self::ConnectOutput,
    ) -> StepFuture<Self::AuthOutput>;
}

/// Maszyna stanów realizująca asynchroniczny handshake.
pub struct HandshakeMachine<B: HandshakeBackend> {
    backend: B,
    // TODO: przechowuj tutaj enum ze stanem oraz danymi pośrednimi.
    _marker: std::marker::PhantomData<B>,
}

impl<B: HandshakeBackend> HandshakeMachine<B> {
    /// Tworzy maszynę i przygotowuje ją do natychmiastowego startu.
    pub fn new(backend: B) -> Self {
        todo!()
    }

    /// Zwraca aktualną fazę handshake.
    pub fn phase(&self) -> HandshakePhase {
        todo!()
    }
}

impl<B> Future for HandshakeMachine<B>
where
    B: HandshakeBackend,
{
    type Output = Result<
        HandshakeOutcome<B::ResolveOutput, B::ConnectOutput, B::AuthOutput>,
        HandshakeError,
    >;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

impl<B: HandshakeBackend> Drop for HandshakeMachine<B> {
    fn drop(&mut self) {
        // TODO: poinformuj backend o anulowaniu, jeśli handshake nie został ukończony.
    }
}
