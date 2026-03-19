use tower_governor::governor::GovernorConfigBuilder;

/// Create a GovernorConfigBuilder for auth endpoints (login/register).
/// Strict: burst_size=5, replenish 1 per 4 seconds.
pub fn auth_governor_builder() -> GovernorConfigBuilder<
    tower_governor::key_extractor::PeerIpKeyExtractor,
    governor::middleware::StateInformationMiddleware,
> {
    GovernorConfigBuilder::default()
        .per_second(4)
        .burst_size(5)
        .use_headers()
}

/// Create a GovernorConfigBuilder for general API endpoints.
/// Moderate: burst_size=30, replenish 1 per second.
pub fn api_governor_builder() -> GovernorConfigBuilder<
    tower_governor::key_extractor::PeerIpKeyExtractor,
    governor::middleware::StateInformationMiddleware,
> {
    GovernorConfigBuilder::default()
        .per_second(1)
        .burst_size(30)
        .use_headers()
}
