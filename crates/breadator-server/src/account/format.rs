//! Dates and device names as the account page shows them.

use crate::i18n::Tr;

/// Calendar date of a day count since 1970-01-01 (proleptic Gregorian; H. Hinnant's algorithm).
fn civil(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    (year, month, day)
}

/// `2026-11-24` (UTC).
pub fn date(secs: i64) -> String {
    let (y, m, d) = civil(secs.div_euclid(86_400));
    format!("{y:04}-{m:02}-{d:02}")
}

/// `2026-11-24 14:05 UTC`.
pub fn datetime(secs: i64) -> String {
    let in_day = secs.rem_euclid(86_400);
    format!("{} {:02}:{:02} UTC", date(secs), in_day / 3600, in_day % 3600 / 60)
}

/// "Firefox · Windows" from a `User-Agent`; a translated placeholder when neither is known.
pub fn device_label(tr: &Tr, user_agent: &str) -> String {
    let has = |needle: &str| user_agent.contains(needle);
    // Order matters: Edge and Opera also say Chrome, Chrome also says Safari.
    let browser = if has("Edg/") || has("EdgA/") || has("EdgiOS/") {
        "Edge"
    } else if has("OPR/") || has("Opera") {
        "Opera"
    } else if has("Firefox/") || has("FxiOS/") {
        "Firefox"
    } else if has("Chrome/") || has("CriOS/") {
        "Chrome"
    } else if has("Safari/") {
        "Safari"
    } else {
        ""
    };
    // iPhones also say "like Mac OS X", Android also says "Linux".
    let system = if has("iPhone") || has("iPad") {
        "iOS"
    } else if has("Android") {
        "Android"
    } else if has("Windows") {
        "Windows"
    } else if has("Macintosh") {
        "macOS"
    } else if has("CrOS") {
        "ChromeOS"
    } else if has("Linux") {
        "Linux"
    } else {
        ""
    };
    match (browser, system) {
        ("", "") => tr.t("account-device-unknown"),
        (b, "") => b.to_owned(),
        ("", s) => s.to_owned(),
        (b, s) => format!("{b} · {s}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Lang;

    #[test]
    fn dates() {
        assert_eq!(date(0), "1970-01-01");
        assert_eq!(date(1_790_000_000), "2026-09-21");
        assert_eq!(datetime(951_782_400 + 3600 * 13 + 60 * 7), "2000-02-29 13:07 UTC");
        assert_eq!(date(-86_400), "1969-12-31");
    }

    #[test]
    fn devices() {
        let tr = Lang::En.tr();
        let chrome = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36";
        assert_eq!(device_label(&tr, chrome), "Chrome · Windows");
        let edge = format!("{chrome} Edg/130.0.0.0");
        assert_eq!(device_label(&tr, &edge), "Edge · Windows");
        let iphone = "Mozilla/5.0 (iPhone; CPU iPhone OS 18_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Mobile/15E148 Safari/604.1";
        assert_eq!(device_label(&tr, iphone), "Safari · iOS");
        assert_eq!(device_label(&tr, ""), tr.t("account-device-unknown"));
    }
}
