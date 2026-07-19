use rand::seq::SliceRandom;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum GeoLocation {
    Disabled,
    US,
    UK,
    Japan,
    Australia,
    Switzerland,
    Iceland,
    Netherlands,
    Singapore,
}

impl GeoLocation {
    pub fn from_config_value(value: &str) -> Self {
        match value.trim().to_lowercase().as_str() {
            "us" => GeoLocation::US,
            "uk" => GeoLocation::UK,
            "japan" | "jp" => GeoLocation::Japan,
            "australia" | "au" => GeoLocation::Australia,
            "switzerland" | "ch" => GeoLocation::Switzerland,
            "iceland" | "is" => GeoLocation::Iceland,
            "netherlands" | "nl" => GeoLocation::Netherlands,
            "singapore" | "sg" => GeoLocation::Singapore,
            _ => GeoLocation::Disabled,
        }
    }

    pub fn as_config_value(&self) -> &'static str {
        match self {
            GeoLocation::Disabled => "disabled",
            GeoLocation::US => "us",
            GeoLocation::UK => "uk",
            GeoLocation::Japan => "japan",
            GeoLocation::Australia => "australia",
            GeoLocation::Switzerland => "switzerland",
            GeoLocation::Iceland => "iceland",
            GeoLocation::Netherlands => "netherlands",
            GeoLocation::Singapore => "singapore",
        }
    }

    pub fn x_forwarded_for_value(&self) -> Option<&'static str> {
        match self {
            GeoLocation::US => Some("104.28.1.1"),
            GeoLocation::UK => Some("5.148.169.10"),
            GeoLocation::Japan => Some("157.14.218.10"),
            GeoLocation::Australia => Some("1.1.1.1"),
            GeoLocation::Switzerland => Some("194.230.158.42"),
            GeoLocation::Iceland => Some("185.166.108.99"),
            GeoLocation::Netherlands => Some("37.139.11.196"),
            GeoLocation::Singapore => Some("139.59.229.41"),
            GeoLocation::Disabled => None,
        }
    }

    pub fn to_js_coords(&self) -> (&'static str, &'static str) {
        match self {
            GeoLocation::US => ("37.7749", "-122.4194"),
            GeoLocation::UK => ("51.5074", "-0.1278"),
            GeoLocation::Japan => ("35.6762", "139.6503"),
            GeoLocation::Australia => ("-33.8688", "151.2093"),
            GeoLocation::Switzerland => ("47.3769", "8.5417"),
            GeoLocation::Iceland => ("64.1466", "-21.9426"),
            GeoLocation::Netherlands => ("52.3676", "4.9041"),
            GeoLocation::Singapore => ("1.3521", "103.8198"),
            GeoLocation::Disabled => ("0.0", "0.0"),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum BrowserProfile {
    AutoDesktop,
    Windows,
    MacOS,
    Linux,
    Android,
}

impl BrowserProfile {
    pub fn from_config_value(value: &str) -> Self {
        match value.trim().to_lowercase().as_str() {
            "windows" | "win" => BrowserProfile::Windows,
            "macos" | "mac" => BrowserProfile::MacOS,
            "linux" => BrowserProfile::Linux,
            "android" => BrowserProfile::Android,
            _ => BrowserProfile::AutoDesktop,
        }
    }

    pub fn as_config_value(&self) -> &'static str {
        match self {
            BrowserProfile::AutoDesktop => "auto_desktop",
            BrowserProfile::Windows => "windows",
            BrowserProfile::MacOS => "macos",
            BrowserProfile::Linux => "linux",
            BrowserProfile::Android => "android",
        }
    }

    fn user_agents(&self) -> &'static [&'static str] {
        match self {
            BrowserProfile::AutoDesktop => &[
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
                "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 13_6_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
            ],
            BrowserProfile::Windows => &[
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:124.0) Gecko/20100101 Firefox/124.0",
            ],
            BrowserProfile::MacOS => &[
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 13_6_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 13.6; rv:124.0) Gecko/20100101 Firefox/124.0",
            ],
            BrowserProfile::Linux => &[
                "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
                "Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0",
            ],
            BrowserProfile::Android => &[
                "Mozilla/5.0 (Linux; Android 14; Pixel 7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.6261.105 Mobile Safari/537.36",
                "Mozilla/5.0 (Linux; Android 13; SM-S918B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.6167.144 Mobile Safari/537.36",
            ],
        }
    }

    fn device_hints(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            BrowserProfile::AutoDesktop | BrowserProfile::Windows => ("Win32", "en-US", "America/New_York"),
            BrowserProfile::MacOS => ("MacIntel", "en-US", "Europe/Zurich"),
            BrowserProfile::Linux => ("Linux x86_64", "en-GB", "Europe/Amsterdam"),
            BrowserProfile::Android => ("Linux armv8l", "en-SG", "Asia/Singapore"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FingerprintOptions {
    pub hardening_enabled: bool,
    pub spoof_canvas_webgl: bool,
    pub spoof_webdriver: bool,
    pub browser_profile: BrowserProfile,
}

pub fn pick_user_agent(profile: BrowserProfile) -> &'static str {
    let mut rng = rand::thread_rng();
    profile
        .user_agents()
        .choose(&mut rng)
        .copied()
        .unwrap_or("Mozilla/5.0")
}

pub fn build_stealth_script(geo_location: GeoLocation, fp: FingerprintOptions) -> Option<String> {
    if !fp.hardening_enabled && geo_location == GeoLocation::Disabled {
        return None;
    }

    let coords = geo_location.to_js_coords();
    let (platform, language, timezone) = fp.browser_profile.device_hints();
    let webdriver_block = if fp.spoof_webdriver {
        "Object.defineProperty(navigator, 'webdriver', { get: () => undefined });"
    } else {
        ""
    };

    let canvas_webgl_block = if fp.spoof_canvas_webgl {
        r#"
            const origToDataURL = HTMLCanvasElement.prototype.toDataURL;
            HTMLCanvasElement.prototype.toDataURL = function(...args) {
              const ctx = this.getContext('2d');
              if (ctx) {
                ctx.save();
                ctx.globalAlpha = 0.01;
                ctx.fillStyle = '#010101';
                ctx.fillRect(0, 0, 1, 1);
                ctx.restore();
              }
              return origToDataURL.apply(this, args);
            };

            const origGetParameter = WebGLRenderingContext.prototype.getParameter;
            WebGLRenderingContext.prototype.getParameter = function(param) {
              if (param === 37445) return 'Intel Inc.';
              if (param === 37446) return 'Intel Iris OpenGL Engine';
              return origGetParameter.call(this, param);
            };
        "#
    } else {
        ""
    };

    Some(format!(
        "<script>\n(() => {{\n  try {{\n    {}\n    Object.defineProperty(navigator, 'platform', {{ get: () => '{}' }});\n    Object.defineProperty(navigator, 'language', {{ get: () => '{}' }});\n    Object.defineProperty(navigator, 'languages', {{ get: () => ['{}', 'en'] }});\n    Object.defineProperty(navigator, 'hardwareConcurrency', {{ get: () => 8 }});\n    Object.defineProperty(navigator, 'deviceMemory', {{ get: () => 8 }});\n    Object.defineProperty(navigator, 'plugins', {{ get: () => [{{name:'Chrome PDF Plugin'}}, {{name:'Chrome PDF Viewer'}}] }});\n    Object.defineProperty(navigator, 'mimeTypes', {{ get: () => [{{type:'application/pdf'}}] }});\n\n    Object.defineProperty(navigator, 'webdriver', {{ get: () => false }});\n    Object.defineProperty(navigator, 'pdfViewerEnabled', {{ get: () => true }});\n    window.chrome = {{ runtime: {{}} }};\n\n    const originalPermissionsQuery = navigator.permissions && navigator.permissions.query;\n    if (originalPermissionsQuery) {{\n      navigator.permissions.query = (parameters) => (\n        parameters && parameters.name === 'notifications'\n          ? Promise.resolve({{ state: Notification.permission }})\n          : originalPermissionsQuery(parameters)\n      );\n    }}\n\n    const originalResolvedOptions = Intl.DateTimeFormat.prototype.resolvedOptions;\n    Intl.DateTimeFormat.prototype.resolvedOptions = function() {{\n      const options = originalResolvedOptions.call(this);\n      options.timeZone = '{}';\n      return options;\n    }};\n\n    navigator.geolocation.getCurrentPosition = function(success) {{\n      success({{ coords: {{ latitude: {}, longitude: {} }} }});\n    }};\n\n    {}\n  }} catch (_) {{}}\n}})();\n</script>",
        webdriver_block,
        platform,
        language,
        language,
        timezone,
        coords.0,
        coords.1,
        canvas_webgl_block
    ))
}

fn parse_bool_env(var_name: &str, default: bool) -> bool {
    match std::env::var(var_name) {
        Ok(v) => match v.trim().to_lowercase().as_str() {
            "1" | "true" | "on" | "yes" => true,
            "0" | "false" | "off" | "no" => false,
            _ => default,
        },
        Err(_) => default,
    }
}

pub fn parse_geo_env() -> GeoLocation {
    match std::env::var("CATISEN_GEO_LOCATION") {
        Ok(v) => GeoLocation::from_config_value(&v),
        Err(_) => GeoLocation::Disabled,
    }
}

pub fn parse_profile_env() -> BrowserProfile {
    match std::env::var("CATISEN_BROWSER_PROFILE") {
        Ok(v) => BrowserProfile::from_config_value(&v),
        Err(_) => BrowserProfile::AutoDesktop,
    }
}

pub fn parse_fp_hardening_env(default: bool) -> bool {
    parse_bool_env("CATISEN_FP_HARDENING", default)
}

pub fn parse_spoof_canvas_env(default: bool) -> bool {
    parse_bool_env("CATISEN_SPOOF_CANVAS_WEBGL", default)
}

pub fn parse_spoof_webdriver_env(default: bool) -> bool {
    parse_bool_env("CATISEN_SPOOF_WEBDRIVER", default)
}
