use std::env;

const LONG_ENV_FILTER_VAR_NAME: &str = "FORGE_TRACKER";

/// Version information
pub const VERSION: &str = match option_env!("APP_VERSION") {
    None => env!("CARGO_PKG_VERSION"),
    Some(v) => v,
};

/// Checks if tracking is enabled
pub fn can_track() -> bool {
    let is_dev = VERSION.contains("dev") | VERSION.contains("0.1.0");
    let usage_enabled = env::var(LONG_ENV_FILTER_VAR_NAME)
        .map(|v| !v.eq_ignore_ascii_case("false"))
        .ok();
    can_track_inner(!is_dev, usage_enabled)
}

fn can_track_inner(is_prod_build: bool, usage_enabled: Option<bool>) -> bool {
    if let Some(usage_enabled) = usage_enabled {
        usage_enabled
    } else {
        is_prod_build
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn usage_enabled_true() {
        assert!(can_track_inner(true, Some(true)));
        assert!(can_track_inner(false, Some(true)));
    }

    #[test]
    fn usage_enabled_false() {
        assert!(!can_track_inner(true, Some(false)));
        assert!(!can_track_inner(false, Some(false)));
    }

    #[test]
    fn usage_enabled_none_is_prod_true() {
        assert!(can_track_inner(true, None));
    }

    #[test]
    fn usage_enabled_none_is_prod_false() {
        assert!(!can_track_inner(false, None));
    }
}
