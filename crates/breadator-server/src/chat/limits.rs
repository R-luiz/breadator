//! What keeps the chat inside the owner's budget (USD 1 a month, see
//! `research/notes/chat-model/REPORT.md`): the dollar cut-offs, the per-account and per-network
//! allowances and the in-memory limiter of answers being written.
//!
//! Allowances are kept per [`Asker`]: a hash of the account's verified address (so deleting the
//! account and signing up again gives nothing back) and a hash of the client's network.

use std::collections::{HashMap, VecDeque};
use std::net::Ipv6Addr;
use std::time::{Duration, Instant};

use axum::http::StatusCode;
use tokio::sync::watch;

use crate::auth::{User, sha256_hex};

/// The chat stops for the rest of the UTC day once the day's spend reaches this (0.95 / 30).
pub const DAY_BUDGET_USD: f64 = 0.032;
/// The chat stops for the rest of the month once the month's spend reaches this. The OpenRouter
/// key's own USD 1.00 monthly limit is the backstop behind it.
pub const MONTH_BUDGET_USD: f64 = 0.95;
/// One account's share of the day's budget: a quarter, so that one account cannot spend the day.
pub const USER_DAY_BUDGET_USD: f64 = 0.008;
pub const SITE_ANSWERS_PER_MINUTE: usize = 6;
pub const SITE_IN_FLIGHT: usize = 2;
pub const USER_ANSWERS_PER_DAY: i64 = 10;
pub const USER_ANSWERS_PER_MONTH: i64 = 100;
/// Answers per day to one client network (an IPv4 address or an IPv6 /48), whatever the accounts.
pub const ADDRESS_ANSWERS_PER_DAY: i64 = 15;
pub const QUESTION_MAX_CHARS: usize = 1000;
/// Earlier question-and-answer pairs sent back to the model.
pub const HISTORY_TURNS: usize = 4;

/// Whose allowances a question counts against, as keys of `chat_usage`: hashes, so that the rows
/// can outlive the account without naming anyone.
#[derive(Debug, Clone)]
pub struct Asker {
    pub account: String,
    pub address: String,
}

impl Asker {
    /// `client` is the caller's rate-limit key ([`crate::auth::ClientIp`]).
    pub fn new(user: &User, client: &str) -> Self {
        Self { account: account_subject(user), address: address_subject(client) }
    }
}

/// `a:` and the SHA-256 of the lowercased verified address; `u:` and the SHA-256 of the user id
/// for an account without one (passkey only).
pub fn account_subject(user: &User) -> String {
    match &user.email {
        Some(email) => format!("a:{}", sha256_hex(&email.to_lowercase())),
        None => format!("u:{}", sha256_hex(&user.id)),
    }
}

/// `c:` and the SHA-256 of the client's IPv4 address or IPv6 /48 (one site's network: a /64 is
/// one of its 65,536 subnets).
pub fn address_subject(client: &str) -> String {
    let address = client.split('/').next().unwrap_or(client);
    let network = match address.parse::<Ipv6Addr>() {
        Ok(v6) => format!("{}/48", Ipv6Addr::from(u128::from(v6) >> 80 << 80)),
        Err(_) => address.to_owned(),
    };
    format!("c:{}", sha256_hex(&network))
}

/// Why a question is not answered. `code` is the JSON error code and the suffix of the
/// translation key (`chat-refusal-<code>`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// No `OPENROUTER_API_KEY`.
    Unavailable,
    Empty,
    TooLong,
    /// The conversation does not exist or belongs to someone else.
    NotFound,
    BudgetDay,
    BudgetMonth,
    UserDay,
    UserMonth,
    /// The account's share of the day's budget is spent.
    UserBudget,
    /// The client's network had its answers for the day.
    AddressDay,
    /// This account already has an answer being written.
    UserBusy,
    /// Too many answers in flight or in the last minute, site-wide.
    SiteBusy,
    /// The database failed (logged where it happened).
    Failed,
}

impl Refusal {
    pub fn code(self) -> &'static str {
        match self {
            Self::Unavailable => "unavailable",
            Self::Empty => "empty",
            Self::TooLong => "too-long",
            Self::NotFound => "not-found",
            Self::BudgetDay => "budget-day",
            Self::BudgetMonth => "budget-month",
            Self::UserDay => "user-day",
            Self::UserMonth => "user-month",
            Self::UserBudget => "user-budget",
            Self::AddressDay => "address-day",
            Self::UserBusy => "user-busy",
            Self::SiteBusy => "site-busy",
            Self::Failed => "failed",
        }
    }

    pub fn status(self) -> StatusCode {
        match self {
            Self::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
            Self::Empty | Self::TooLong => StatusCode::UNPROCESSABLE_ENTITY,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::BudgetDay
            | Self::BudgetMonth
            | Self::UserDay
            | Self::UserMonth
            | Self::UserBudget
            | Self::AddressDay
            | Self::UserBusy
            | Self::SiteBusy => StatusCode::TOO_MANY_REQUESTS,
            Self::Failed => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// Spend and answer counts read from the database before a question is accepted. Spends include
/// the reservations of calls not settled (in flight, or lost to a crash).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Standing {
    pub day_spend: f64,
    pub month_spend: f64,
    pub user_day: i64,
    pub user_month: i64,
    /// The account's spend today.
    pub user_spend: f64,
    /// Answers to the client's network today.
    pub address_day: i64,
}

impl Standing {
    /// The first limit already reached, the site's budget before the account's allowance.
    pub fn refusal(&self) -> Option<Refusal> {
        if self.month_spend >= MONTH_BUDGET_USD {
            Some(Refusal::BudgetMonth)
        } else if self.day_spend >= DAY_BUDGET_USD {
            Some(Refusal::BudgetDay)
        } else if self.user_month >= USER_ANSWERS_PER_MONTH {
            Some(Refusal::UserMonth)
        } else if self.user_day >= USER_ANSWERS_PER_DAY {
            Some(Refusal::UserDay)
        } else if self.user_spend >= USER_DAY_BUDGET_USD {
            Some(Refusal::UserBudget)
        } else if self.address_day >= ADDRESS_ANSWERS_PER_DAY {
            Some(Refusal::AddressDay)
        } else {
            None
        }
    }

    pub fn left_today(&self) -> i64 {
        (USER_ANSWERS_PER_DAY - self.user_day)
            .min(USER_ANSWERS_PER_MONTH - self.user_month)
            .min(ADDRESS_ANSWERS_PER_DAY - self.address_day)
            .max(0)
    }
}

struct InFlight {
    conversation: String,
    /// Closed when the answer is stored (or failed): wakes the page waiting for it.
    done: watch::Receiver<()>,
}

/// Answers being written: at most [`SITE_IN_FLIGHT`] at once and one per account, and at most
/// [`SITE_ANSWERS_PER_MINUTE`] started in any minute. In memory: a restart forgets it, and the
/// answers it was tracking died with the process.
#[derive(Default)]
pub struct Limiter {
    started: VecDeque<Instant>,
    in_flight: HashMap<String, InFlight>,
}

impl Limiter {
    /// Admits an answer for `user`; the returned sender is kept until the answer is finished.
    pub fn admit(
        &mut self,
        user: &str,
        conversation: &str,
        now: Instant,
    ) -> Result<watch::Sender<()>, Refusal> {
        if self.in_flight.contains_key(user) {
            return Err(Refusal::UserBusy);
        }
        if self.in_flight.len() >= SITE_IN_FLIGHT {
            return Err(Refusal::SiteBusy);
        }
        while self
            .started
            .front()
            .is_some_and(|t| now.duration_since(*t) >= Duration::from_secs(60))
        {
            self.started.pop_front();
        }
        if self.started.len() >= SITE_ANSWERS_PER_MINUTE {
            return Err(Refusal::SiteBusy);
        }
        self.started.push_back(now);
        let (finished, done) = watch::channel(());
        self.in_flight
            .insert(user.to_owned(), InFlight { conversation: conversation.to_owned(), done });
        Ok(finished)
    }

    pub fn release(&mut self, user: &str) {
        self.in_flight.remove(user);
    }

    /// A handle that closes when the answer `user` is waiting for in `conversation` is finished.
    pub fn waiting(&self, user: &str, conversation: &str) -> Option<watch::Receiver<()>> {
        self.in_flight.get(user).filter(|f| f.conversation == conversation).map(|f| f.done.clone())
    }
}

/// `2026-11-24` (UTC) of a Unix time; its first seven characters are the month.
pub fn utc_day(secs: i64) -> String {
    // Days since 1970-01-01 to a civil date (Howard Hinnant's `civil_from_days`).
    let z = secs.div_euclid(86_400) + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn days() {
        assert_eq!(utc_day(0), "1970-01-01");
        assert_eq!(utc_day(951_782_400), "2000-02-29");
        assert_eq!(utc_day(1_795_564_799), "2026-11-24");
        assert_eq!(utc_day(1_795_564_800), "2026-11-25");
        assert_eq!(utc_day(1_798_761_599), "2026-12-31");
    }

    #[test]
    fn one_answer_per_account_two_on_the_site_six_a_minute() {
        let mut limiter = Limiter::default();
        let t0 = Instant::now();
        let first = limiter.admit("a", "c1", t0).unwrap();
        assert_eq!(limiter.admit("a", "c2", t0).unwrap_err(), Refusal::UserBusy);
        let second = limiter.admit("b", "c3", t0).unwrap();
        assert_eq!(limiter.admit("c", "c4", t0).unwrap_err(), Refusal::SiteBusy);

        // The page of that conversation can wait for the answer; other pages do not.
        let waiting = limiter.waiting("a", "c1").unwrap();
        assert!(limiter.waiting("a", "other").is_none() && limiter.waiting("z", "c1").is_none());
        assert!(!waiting.has_changed().unwrap());
        limiter.release("a");
        drop(first);
        assert!(waiting.has_changed().is_err(), "closed once the answer is finished");
        assert!(limiter.waiting("a", "c1").is_none());
        limiter.release("b");
        drop(second);

        // Two were started at t0: four more fit in the same minute, the seventh waits.
        for i in 0..4 {
            let user = format!("u{i}");
            limiter.admit(&user, "c", t0 + Duration::from_secs(1)).unwrap();
            limiter.release(&user);
        }
        assert_eq!(
            limiter.admit("late", "c", t0 + Duration::from_secs(59)).unwrap_err(),
            Refusal::SiteBusy
        );
        limiter.admit("late", "c", t0 + Duration::from_secs(60)).unwrap();
    }

    #[test]
    fn budget_before_allowance() {
        let ok = Standing {
            day_spend: 0.0319,
            month_spend: 0.9499,
            user_day: 9,
            user_month: 99,
            user_spend: 0.0079,
            address_day: 14,
        };
        assert_eq!(ok.refusal(), None);
        assert_eq!(ok.left_today(), 1);
        let day = Standing { day_spend: DAY_BUDGET_USD, ..ok };
        assert_eq!(day.refusal(), Some(Refusal::BudgetDay));
        let month = Standing { month_spend: MONTH_BUDGET_USD, ..day };
        assert_eq!(month.refusal(), Some(Refusal::BudgetMonth));
        let user_day = Standing { user_day: USER_ANSWERS_PER_DAY, ..ok };
        assert_eq!(user_day.refusal(), Some(Refusal::UserDay));
        assert_eq!(user_day.left_today(), 0);
        let user_month = Standing { user_month: USER_ANSWERS_PER_MONTH, user_day: 0, ..ok };
        assert_eq!(user_month.refusal(), Some(Refusal::UserMonth));
        assert_eq!(user_month.left_today(), 0);
        let share = Standing { user_spend: USER_DAY_BUDGET_USD, ..ok };
        assert_eq!(share.refusal(), Some(Refusal::UserBudget));
        let network = Standing { address_day: ADDRESS_ANSWERS_PER_DAY, user_day: 0, ..ok };
        assert_eq!(network.refusal(), Some(Refusal::AddressDay));
        assert_eq!(network.left_today(), 0);
    }

    #[test]
    fn allowances_are_keyed_on_hashes_of_the_address_and_the_network() {
        let user = |email: Option<&str>| User {
            id: "u1".into(),
            email: email.map(str::to_owned),
            display_name: String::new(),
            locale: crate::i18n::Lang::En,
            units: crate::auth::Units::Metric,
            created_at: 0,
        };
        let mixed = account_subject(&user(Some("Me@Example.com")));
        assert_eq!(mixed, account_subject(&user(Some("me@example.com"))));
        assert_eq!(mixed, format!("a:{}", sha256_hex("me@example.com")));
        assert!(!mixed.contains("example"));
        assert_eq!(account_subject(&user(None)), format!("u:{}", sha256_hex("u1")));
        // Two /64s of one /48 are one network; another /48 is another; IPv4 is the address.
        assert_eq!(address_subject("2001:db8:1:2::/64"), address_subject("2001:db8:1:ffff::/64"));
        assert_ne!(address_subject("2001:db8:1:2::/64"), address_subject("2001:db8:2:2::/64"));
        assert_eq!(
            address_subject("2001:db8:1:2::/64"),
            format!("c:{}", sha256_hex("2001:db8:1::/48"))
        );
        assert_eq!(address_subject("203.0.113.7"), format!("c:{}", sha256_hex("203.0.113.7")));
        assert_ne!(address_subject("203.0.113.7"), address_subject("203.0.113.8"));
    }
}
