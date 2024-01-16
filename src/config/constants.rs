use crate::config::{
    AddressFamilyConfig, AddressMode, AsMode, DnsResolveMethodConfig, GeoIpMode, IcmpExtensionMode,
    LogFormat, LogSpanEvents, Mode,
};
use std::time::Duration;

/// The maximum number of hops we allow.
///
/// The IP `ttl` is a u8 (0..255) but since a `ttl` of zero isn't useful we only allow 255 distinct
/// hops.
pub const MAX_HOPS: usize = u8::MAX as usize;

/// The default value for `mode`.
pub const DEFAULT_MODE: Mode = Mode::Tui;

/// The default value for `all_resolved_ips`.
pub const DEFAULT_DNS_RESOLVE_ALL: bool = false;

/// The default value for `log-format`.
pub const DEFAULT_LOG_FORMAT: LogFormat = LogFormat::Pretty;

/// The default value for `log-span-events`.
pub const DEFAULT_LOG_SPAN_EVENTS: LogSpanEvents = LogSpanEvents::Off;

/// The default value for `log-filter`.
pub const DEFAULT_LOG_FILTER: &str = "trippy=debug";

/// The default value for `tui-max-samples`.
pub const DEFAULT_TUI_MAX_SAMPLES: usize = 256;

/// The default value for `tui-max-flows`.
pub const DEFAULT_TUI_MAX_FLOWS: usize = 64;

/// The default value for `tui-preserve-screen`.
pub const DEFAULT_TUI_PRESERVE_SCREEN: bool = false;

/// The default value for `tui-as-mode`.
pub const DEFAULT_TUI_AS_MODE: AsMode = AsMode::Asn;

/// The default value for `tui-custom-columns`.
pub const DEFAULT_CUSTOM_COLUMNS: &str = "HOLSRAVBWDT";

/// The default value for `tui-icmp-extension-mode`.
pub const DEFAULT_TUI_ICMP_EXTENSION_MODE: IcmpExtensionMode = IcmpExtensionMode::Off;

/// The default value for `tui-geoip-mode`.
pub const DEFAULT_TUI_GEOIP_MODE: GeoIpMode = GeoIpMode::Off;

/// The default value for `tui-max-addrs`.
pub const DEFAULT_TUI_MAX_ADDRS: u8 = 0;

/// The default value for `tui-address-mode`.
pub const DEFAULT_TUI_ADDRESS_MODE: AddressMode = AddressMode::Host;

/// The default value for `tui-refresh-rate`.
pub const DEFAULT_TUI_REFRESH_RATE: &str = "100ms";

/// The default value for `tui_privacy_max_ttl`.
pub const DEFAULT_TUI_PRIVACY_MAX_TTL: u8 = 0;

/// The default value for `dns-resolve-method`.
pub const DEFAULT_DNS_RESOLVE_METHOD: DnsResolveMethodConfig = DnsResolveMethodConfig::System;

/// The default value for `addr-family`.
pub const DEFAULT_ADDR_FAMILY: AddressFamilyConfig = AddressFamilyConfig::Ipv4ThenIpv6;

/// The default value for `dns-lookup-as-info`.
pub const DEFAULT_DNS_LOOKUP_AS_INFO: bool = false;

/// The default value for `dns-timeout`.
pub const DEFAULT_DNS_TIMEOUT: &str = "5s";

/// The default value for `report-cycles`.
pub const DEFAULT_REPORT_CYCLES: usize = 10;

/// The minimum TUI refresh rate.
pub const TUI_MIN_REFRESH_RATE_MS: Duration = Duration::from_millis(50);

/// The maximum TUI refresh rate.
pub const TUI_MAX_REFRESH_RATE_MS: Duration = Duration::from_millis(1000);

/// The minimum socket read timeout.
pub const MIN_READ_TIMEOUT_MS: Duration = Duration::from_millis(10);

/// The maximum socket read timeout.
pub const MAX_READ_TIMEOUT_MS: Duration = Duration::from_millis(100);

/// The minimum grace duration.
pub const MIN_GRACE_DURATION_MS: Duration = Duration::from_millis(10);

/// The maximum grace duration.
pub const MAX_GRACE_DURATION_MS: Duration = Duration::from_millis(1000);

/// The minimum packet size we allow.
pub const MIN_PACKET_SIZE: u16 = 28;

/// The maximum packet size we allow.
pub const MAX_PACKET_SIZE: u16 = 1024;
