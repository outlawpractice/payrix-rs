//! Rate limiting for Payrix API requests.
//!
//! Implements a sliding window rate limiter to proactively avoid hitting
//! Payrix's rate limits.

use std::collections::VecDeque;
use std::time::Duration;
use tokio::time::Instant;

/// A simple sliding window rate limiter.
///
/// Tracks request timestamps and calculates wait times to stay within limits.
/// Uses `VecDeque` for O(1) front removal when expiring old requests.
#[derive(Debug)]
pub struct RateLimiter {
    /// Timestamps of recent requests
    requests: VecDeque<Instant>,
    /// Maximum requests allowed in the window
    max_requests: usize,
    /// Time window for rate limiting
    window: Duration,
}

impl RateLimiter {
    /// Create a new rate limiter.
    ///
    /// # Arguments
    ///
    /// * `max_requests` - Maximum number of requests allowed in the window
    /// * `window` - Duration of the sliding window
    pub fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            requests: VecDeque::with_capacity(max_requests),
            max_requests,
            window,
        }
    }

    /// Create a rate limiter with default settings for Payrix.
    ///
    /// Defaults to 100 requests per 60 seconds.
    pub fn default_payrix() -> Self {
        Self::new(100, Duration::from_secs(60))
    }

    /// Check if a request can be made, returning how long to wait if not.
    ///
    /// If the returned duration is zero, the request can proceed immediately.
    /// Otherwise, the caller should sleep for the returned duration before retrying.
    ///
    /// This method also records the request timestamp when it returns `Duration::ZERO`.
    pub fn check(&mut self) -> Duration {
        let now = Instant::now();

        // Remove expired timestamps from the front (O(1) with VecDeque)
        while let Some(&oldest) = self.requests.front() {
            if now.duration_since(oldest) >= self.window {
                self.requests.pop_front();
            } else {
                break;
            }
        }

        if self.requests.len() < self.max_requests {
            // We have capacity - record this request and proceed
            self.requests.push_back(now);
            Duration::ZERO
        } else {
            // At capacity - calculate wait time until oldest request expires
            let oldest = self.requests.front().expect("requests not empty at capacity");
            let elapsed = now.duration_since(*oldest);
            // Need to wait for oldest to expire
            self.window - elapsed
        }
    }

    /// Record a request without checking limits.
    ///
    /// Use this when you've already made a request and want to track it.
    #[allow(dead_code)]
    pub fn record(&mut self) {
        let now = Instant::now();
        // Remove expired timestamps from the front
        while let Some(&oldest) = self.requests.front() {
            if now.duration_since(oldest) >= self.window {
                self.requests.pop_front();
            } else {
                break;
            }
        }
        self.requests.push_back(now);
    }

    /// Get the current number of requests in the window.
    #[allow(dead_code)]
    pub fn current_count(&self) -> usize {
        self.requests.len()
    }

    /// Get the maximum requests allowed.
    #[allow(dead_code)]
    pub fn max_requests(&self) -> usize {
        self.max_requests
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allows_requests_under_limit() {
        let mut limiter = RateLimiter::new(3, Duration::from_secs(60));

        assert_eq!(limiter.check(), Duration::ZERO);
        assert_eq!(limiter.check(), Duration::ZERO);
        assert_eq!(limiter.check(), Duration::ZERO);
        assert_eq!(limiter.current_count(), 3);
    }

    #[test]
    fn test_blocks_requests_at_limit() {
        let mut limiter = RateLimiter::new(2, Duration::from_secs(60));

        assert_eq!(limiter.check(), Duration::ZERO);
        assert_eq!(limiter.check(), Duration::ZERO);

        // Third request should need to wait
        let wait = limiter.check();
        assert!(wait > Duration::ZERO);
        assert!(wait <= Duration::from_secs(60));
    }
}
