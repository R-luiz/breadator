//! Middleware around every request: resolves the session cookie (and slides its expiry), refuses
//! cross-origin state-changing requests, and tells the page layout who is signed in.
//!
//! **CSRF.** Besides `SameSite=Lax` on the cookie, every request that is not GET/HEAD/OPTIONS
//! must carry an `Origin` (or, failing that, a `Referer`) equal to `PUBLIC_URL`, else `403`.
//! One exemption: a JSON API call (`/api/…`) that carries no session cookie at all. It has no
//! ambient credential for a third-party page to abuse (the stateless engine endpoints are used
//! that way by scripts and by the CI smoke test); anything sent with a session cookie, and every
//! form, is checked.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::LazyLock;

use axum::extract::{ConnectInfo, Request, State};
use axum::http::{HeaderMap, HeaderValue, Method, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use super::session::{self, SLIDE_AFTER_SECS, TTL_SECS};
use super::{Badge, ClientIp, CurrentUser, MaybeUser, json_error, sha256_hex};
use crate::app::AppState;
use crate::db;

/// `scheme://host[:port]` of a URL, lowercased: the value browsers put in `Origin`.
pub fn origin_of(url: &str) -> String {
    let url = url.trim();
    let end = url.find("://").map_or(0, |i| i + 3);
    let rest = &url[end..];
    let authority_end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    url[..end + authority_end].to_ascii_lowercase()
}

/// Is this request from our own pages? `Origin` when present (browsers always send it on POST),
/// else `Referer`.
pub fn same_origin(headers: &HeaderMap, expected: &str) -> bool {
    if let Some(origin) = headers.get(header::ORIGIN) {
        return origin.to_str().is_ok_and(|o| o.eq_ignore_ascii_case(expected));
    }
    headers
        .get(header::REFERER)
        .and_then(|r| r.to_str().ok())
        .is_some_and(|r| origin_of(r) == expected)
}

pub fn read_cookie<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(';'))
        .find_map(|kv| kv.trim().strip_prefix(name)?.strip_prefix('='))
        .filter(|v| !v.is_empty())
}

/// `Set-Cookie` value: `HttpOnly; SameSite=Lax; Path=/`, `Secure` on HTTPS, no `Domain` (host-only,
/// as the `__Host-` prefix requires).
pub fn set_cookie(name: &str, value: &str, max_age_secs: i64, secure: bool) -> HeaderValue {
    let secure = if secure { "; Secure" } else { "" };
    let cookie =
        format!("{name}={value}; Max-Age={max_age_secs}; Path=/; HttpOnly; SameSite=Lax{secure}");
    // Tokens are base64url: always a valid header value.
    HeaderValue::from_str(&cookie).unwrap_or_else(|_| HeaderValue::from_static(""))
}

pub fn clear_cookie(name: &str, secure: bool) -> HeaderValue {
    set_cookie(name, "", 0, secure)
}

pub fn session_cookie(state: &AppState, token: &str) -> HeaderValue {
    set_cookie(state.auth.session_cookie(), token, TTL_SECS, state.auth.secure_cookies)
}

/// Pages and assets that never need the user: no session lookup.
fn wants_session(path: &str) -> bool {
    !(path.starts_with("/static/")
        || matches!(
            path,
            "/healthz" | "/sw.js" | "/manifest.webmanifest" | "/robots.txt" | "/sitemap.xml"
        ))
}

pub async fn guard(State(state): State<AppState>, mut req: Request, next: Next) -> Response {
    let path = req.uri().path().to_owned();
    let token = read_cookie(req.headers(), state.auth.session_cookie()).map(str::to_owned);

    if !matches!(*req.method(), Method::GET | Method::HEAD | Method::OPTIONS) {
        let exempt = path.starts_with("/api/") && token.is_none();
        if !exempt && !same_origin(req.headers(), &state.auth.origin) {
            return if path.starts_with("/api/") {
                json_error(StatusCode::FORBIDDEN, "cross_origin")
            } else {
                (StatusCode::FORBIDDEN, "forbidden: the request does not come from this site")
                    .into_response()
            };
        }
    }

    let peer = req.extensions().get::<ConnectInfo<SocketAddr>>().map(|c| c.0.ip());
    let ip = client_ip(req.headers(), peer);

    let secure = state.auth.secure_cookies;
    let mut cookies = Vec::new();
    let mut current = None;
    if let Some(token) = token.as_deref().filter(|_| wants_session(&path)) {
        let hash = sha256_hex(token);
        let now = db::now();
        match state.db.read(move |c| session::lookup(c, &hash, now)).await {
            Ok(Some(found)) => {
                if now - found.last_seen_at >= SLIDE_AFTER_SECS {
                    let id = found.session_id;
                    match state.db.write(move |c| session::slide(c, id, now)).await {
                        Ok(()) => cookies.push(session_cookie(&state, token)),
                        Err(e) => tracing::error!(error = %e, "cannot slide the session"),
                    }
                }
                current = Some(CurrentUser { user: found.user, session_id: found.session_id });
            }
            // Expired or revoked: drop the dead cookie.
            Ok(None) => cookies.push(clear_cookie(state.auth.session_cookie(), secure)),
            Err(e) => tracing::error!(error = %e, "cannot read the session"),
        }
    }

    let badge = current.as_ref().map(|c| Badge { name: badge_name(&c.user) });
    let signed_in = current.is_some();
    // Units shown: this device's choice, else the account's.
    let units = read_cookie(req.headers(), "units")
        .and_then(super::Units::parse)
        .or(current.as_ref().map(|c| c.user.units))
        .unwrap_or(super::Units::Metric);
    req.extensions_mut().insert(MaybeUser(current));
    req.extensions_mut().insert(ClientIp(ip));

    let mut response = super::BADGE.scope(badge, super::UNITS.scope(units, next.run(req))).await;
    for cookie in cookies {
        response.headers_mut().append(header::SET_COOKIE, cookie);
    }
    // Personal pages are never stored (the service worker honours this too); any other page
    // carrying the user's name in its header is not for shared caches.
    if !response.headers().contains_key(header::CACHE_CONTROL) {
        let policy = if is_personal(&path) {
            Some("no-store")
        } else if signed_in {
            Some("private, no-cache")
        } else {
            None
        };
        if let Some(policy) = policy {
            response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static(policy));
        }
    }
    response
}

/// Account pages, sign-in steps (their URL may carry a token) and OAuth callbacks.
fn is_personal(path: &str) -> bool {
    path.starts_with("/auth/") || path.split('/').nth(2) == Some("account")
}

fn badge_name(user: &super::User) -> String {
    let name = if user.display_name.is_empty() {
        user.email.as_deref().and_then(|e| e.split('@').next()).unwrap_or("")
    } else {
        &user.display_name
    };
    name.chars().take(24).collect()
}

/// Cloudflare's published ranges (https://www.cloudflare.com/ips/), refreshed when they change.
const CLOUDFLARE: &[&str] = &[
    "173.245.48.0/20",
    "103.21.244.0/22",
    "103.22.200.0/22",
    "103.31.4.0/22",
    "141.101.64.0/18",
    "108.162.192.0/18",
    "190.93.240.0/20",
    "188.114.96.0/20",
    "197.234.240.0/22",
    "198.41.128.0/17",
    "162.158.0.0/15",
    "104.16.0.0/13",
    "104.24.0.0/14",
    "172.64.0.0/13",
    "131.0.72.0/22",
    "2400:cb00::/32",
    "2606:4700::/32",
    "2803:f800::/32",
    "2405:b500::/32",
    "2405:8100::/32",
    "2a06:98c0::/29",
    "2c0f:f248::/32",
];

static CLOUDFLARE_NETS: LazyLock<Vec<(IpAddr, u8)>> = LazyLock::new(|| {
    CLOUDFLARE
        .iter()
        .filter_map(|cidr| {
            let (addr, bits) = cidr.split_once('/')?;
            Some((addr.parse().ok()?, bits.parse().ok()?))
        })
        .collect()
});

fn in_net(ip: IpAddr, net: IpAddr, bits: u8) -> bool {
    match (ip, net) {
        (IpAddr::V4(a), IpAddr::V4(n)) if bits <= 32 => {
            let mask = u32::MAX.checked_shl(32 - u32::from(bits)).unwrap_or(0);
            u32::from(a) & mask == u32::from(n) & mask
        }
        (IpAddr::V6(a), IpAddr::V6(n)) if bits <= 128 => {
            let mask = u128::MAX.checked_shl(128 - u32::from(bits)).unwrap_or(0);
            u128::from(a) & mask == u128::from(n) & mask
        }
        _ => false,
    }
}

/// The caller's address for rate limits, as a key: IPv4 as is, IPv6 reduced to its /48 (the
/// usual size of one site's allocation: a single customer often holds a /56 or a whole /48).
///
/// On Fly, `Fly-Client-IP` is set by Fly's proxy and cannot be forged. When the request came
/// through Cloudflare (that header holds a Cloudflare address) the visitor is in
/// `CF-Connecting-IP`; that header is believed only then, since anyone can send it to the
/// origin directly. Without Fly (local development) the socket peer is used.
pub fn client_ip(headers: &HeaderMap, peer: Option<IpAddr>) -> String {
    let parse = |name: &str| -> Option<IpAddr> {
        headers.get(name)?.to_str().ok()?.trim().parse::<IpAddr>().ok().map(|ip| ip.to_canonical())
    };
    let ip = match parse("fly-client-ip") {
        Some(fly) if CLOUDFLARE_NETS.iter().any(|(net, bits)| in_net(fly, *net, *bits)) => {
            parse("cf-connecting-ip").unwrap_or(fly)
        }
        Some(fly) => fly,
        None => peer.map_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED), |p| p.to_canonical()),
    };
    match ip {
        IpAddr::V4(v4) => v4.to_string(),
        IpAddr::V6(v6) => {
            let prefix = u128::from(v6) >> 80 << 80;
            format!("{}/48", Ipv6Addr::from(prefix))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn headers(pairs: &[(&'static str, &str)]) -> HeaderMap {
        let mut h = HeaderMap::new();
        for (k, v) in pairs {
            h.insert(*k, v.parse().unwrap());
        }
        h
    }

    #[test]
    fn origins() {
        assert_eq!(
            origin_of("https://Breadator.RobinLuiz.fr/fr/x?y=1"),
            "https://breadator.robinluiz.fr"
        );
        assert_eq!(origin_of("http://localhost:8080"), "http://localhost:8080");
        let expected = "https://breadator.robinluiz.fr";
        assert!(same_origin(&headers(&[("origin", expected)]), expected));
        assert!(!same_origin(&headers(&[("origin", "https://evil.example")]), expected));
        assert!(!same_origin(&headers(&[("origin", "null")]), expected));
        assert!(!same_origin(&HeaderMap::new(), expected));
        // Referer is the fallback, never a prefix trick.
        assert!(same_origin(
            &headers(&[("referer", "https://breadator.robinluiz.fr/fr/account")]),
            expected
        ));
        assert!(!same_origin(
            &headers(&[("referer", "https://breadator.robinluiz.fr.evil.example/")]),
            expected
        ));
        // A wrong Origin is not rescued by a good Referer.
        assert!(!same_origin(
            &headers(&[
                ("origin", "https://evil.example"),
                ("referer", "https://breadator.robinluiz.fr/")
            ]),
            expected
        ));
    }

    #[test]
    fn cookies() {
        let h = headers(&[("cookie", "a=1; __Host-session=tok; b=2")]);
        assert_eq!(read_cookie(&h, "__Host-session"), Some("tok"));
        assert_eq!(read_cookie(&h, "session"), None);
        let secure = set_cookie("__Host-session", "tok", 5, true);
        assert_eq!(
            secure.to_str().unwrap(),
            "__Host-session=tok; Max-Age=5; Path=/; HttpOnly; SameSite=Lax; Secure"
        );
        assert!(!set_cookie("session", "tok", 5, false).to_str().unwrap().contains("Secure"));
    }

    #[test]
    fn client_addresses() {
        let peer = Some("10.1.2.3".parse().unwrap());
        assert_eq!(client_ip(&HeaderMap::new(), peer), "10.1.2.3");
        // A spoofed CF header without Fly's header is ignored.
        assert_eq!(client_ip(&headers(&[("cf-connecting-ip", "6.6.6.6")]), peer), "10.1.2.3");
        // Fly's header from a non-Cloudflare address: the visitor.
        assert_eq!(
            client_ip(
                &headers(&[("fly-client-ip", "203.0.113.7"), ("cf-connecting-ip", "6.6.6.6")]),
                peer
            ),
            "203.0.113.7"
        );
        // Through Cloudflare: the visitor named by Cloudflare.
        assert_eq!(
            client_ip(
                &headers(&[("fly-client-ip", "172.70.1.1"), ("cf-connecting-ip", "198.51.100.9")]),
                peer
            ),
            "198.51.100.9"
        );
        assert_eq!(
            client_ip(
                &headers(&[
                    ("fly-client-ip", "2606:4700::1"),
                    ("cf-connecting-ip", "2001:db8:1:2:3:4:5:6")
                ]),
                peer
            ),
            "2001:db8:1::/48"
        );
        // Every /64 of one /48 is the same client.
        assert_eq!(
            client_ip(&headers(&[("fly-client-ip", "2001:db8:1:ff00::9")]), peer),
            "2001:db8:1::/48"
        );
        assert_eq!(
            client_ip(&headers(&[("fly-client-ip", "2001:db8:2::1")]), peer),
            "2001:db8:2::/48"
        );
    }
}
