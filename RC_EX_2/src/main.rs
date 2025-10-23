use pin_project_lite::pin_project;
use std::future::Future;
use std::num::NonZeroUsize;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::{Instant, Sleep};
use tower::{Layer, Service};

#[derive(Debug, Clone)]
pub struct TokenBucketConfig {
    pub capacity: NonZeroUsize,
    pub refill_amount: NonZeroUsize,
    pub refill_interval: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenBucketError {
    ZeroCapacity,
    ZeroRefillAmount,
    ZeroRefillInterval,
    RefillExceedsCapacity,
    RequestExceedsCapacity { requested: usize, capacity: usize },
}

#[derive(Debug)]
struct BucketState {
    tokens: f64,
    last_update: Instant,
}

#[derive(Debug)]
struct InnerBucket {
    config: TokenBucketConfig,
    state: Mutex<BucketState>,
}

#[derive(Debug, Clone)]
pub struct TokenBucket {
    inner: Arc<InnerBucket>,
}

#[derive(Debug)]
pub struct Permit {
    bucket: Arc<InnerBucket>,
    amount: usize,
    consumed: bool,
}

pin_project! {
    #[derive(Debug)]
    pub struct PermitFuture {
        bucket: Arc<InnerBucket>,
        amount: NonZeroUsize,
        #[pin]
        sleeper: Option<Sleep>,
    }
}

#[derive(Debug)]
pub enum RateLimitError<E> {
    RateLimit(TokenBucketError),
    Inner(E),
}

#[derive(Clone)]
pub struct RateLimitLayer {
    config: TokenBucketConfig,
}

#[derive(Clone)]
pub struct RateLimitService<S> {
    inner: S,
    bucket: TokenBucket,
}

impl TokenBucketConfig {
    pub fn new(
        capacity: NonZeroUsize,
        refill_amount: NonZeroUsize,
        refill_interval: Duration,
    ) -> Result<Self, TokenBucketError> {
        todo!("Zaimplementuj walidację konfiguracji token bucket");
    }
}

impl TokenBucket {
    pub fn new(config: TokenBucketConfig) -> Self {
        todo!("Zainicjalizuj kubełek pełną liczbą tokenów i aktualnym czasem");
    }

    pub async fn available(&self) -> usize {
        todo!("Odśwież stan tokenów i zwróć aktualną liczbę dostępnych jednostek");
    }

    pub async fn try_acquire(
        &self,
        amount: NonZeroUsize,
    ) -> Result<Permit, TokenBucketError> {
        todo!("Spróbuj atomowo zarezerwować tokeny bez oczekiwania");
    }

    pub fn acquire(&self, amount: NonZeroUsize) -> PermitFuture {
        todo!("Zwróć future oczekujące na pojawienie się tokenów");
    }
}

impl Permit {
    pub fn amount(&self) -> usize {
        self.amount
    }

    pub fn consume(mut self) {
        self.consumed = true;
    }

    pub fn release(mut self) {
        todo!("Oddaj tokeny przed zakończeniem życia strażnika");
    }
}

impl Drop for Permit {
    fn drop(&mut self) {
        if !self.consumed {
            todo!("Oddaj tokeny, jeśli strażnik nie został oznaczony jako zużyty");
        }
    }
}

impl Future for PermitFuture {
    type Output = Result<Permit, TokenBucketError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let _ = cx;
        let mut this = self.project();
        let _ = &mut this.bucket;
        let _ = &mut this.amount;
        let _ = &mut this.sleeper;
        todo!("Zaimplementuj maszynę stanów oczekującą na tokeny w kolejności FIFO");
    }
}

impl RateLimitLayer {
    pub fn new(config: TokenBucketConfig) -> Self {
        RateLimitLayer { config }
    }

    pub fn with_params(
        capacity: NonZeroUsize,
        refill_amount: NonZeroUsize,
        refill_interval: Duration,
    ) -> Result<Self, TokenBucketError> {
        Ok(Self::new(TokenBucketConfig::new(
            capacity,
            refill_amount,
            refill_interval,
        )?))
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitService {
            inner,
            bucket: TokenBucket::new(self.config.clone()),
        }
    }
}

impl<S, Request> Service<Request> for RateLimitService<S>
where
    S: Service<Request> + Clone + Send + 'static,
    S::Future: Send + 'static,
    Request: Send + 'static,
{
    type Response = S::Response;
    type Error = RateLimitError<S::Error>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!("Deleguj do wewnętrznego serwisu i mapuj błąd na RateLimitError::Inner");
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let _ = &request;
        todo!("Pobierz permit, wykonaj serwis i oznacz token jako zużyty po zakończeniu");
    }
}
