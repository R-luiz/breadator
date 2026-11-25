# Breadator — operations

State on 2026-11-25: **live at https://breadator.robinluiz.fr.** Public repository
`R-luiz/breadator`, Fly app `breadator` (one machine in `fra`, volume `breadator_data`), DNS and
certificate in place, every green push on `main` deploys.

Not configured yet, so switched off on the site:

| Missing | Effect on the site | Section |
|---|---|---|
| `OPENROUTER_API_KEY` | the chat page says the assistant is unavailable (left off on purpose) | "OpenRouter (chat)" |
| `RESEND_API_KEY`, Google and GitHub OAuth apps | the sign-in page says accounts are not open yet; everything else works without an account | "Resend", "Google sign-in", "GitHub sign-in" |
| R2 bucket and `LITESTREAM_*` | the database is not replicated. Acceptable while nobody can sign in (it holds no user data); **set it before opening accounts** | "Backups" |

## Configuration reference

| Variable | Where | Meaning |
|---|---|---|
| `PORT` | `fly.toml` | HTTP port (8080) |
| `PUBLIC_URL` | `fly.toml` | canonical origin, `https://breadator.robinluiz.fr`. Also: OAuth redirect URIs, WebAuthn relying party (the host is the passkey `rp_id`: changing the domain invalidates every passkey), the `Origin` that state-changing requests must carry, and cookie mode (`https` → `__Host-session` + `Secure`; `http://localhost…` → plain `session`, for development) |
| `DATABASE_PATH` | `fly.toml` | SQLite file, `/data/breadator.sqlite3` (default for local runs: `data/breadator.sqlite3`; the directory is created) |
| `LOG_FORMAT`, `LOG_LEVEL` | `fly.toml` | `json` / `info`. Request traces (`debug`) carry the method and the path, never the query string (it holds sign-in tokens and OAuth codes) |
| `RESEND_API_KEY` | secret | sends the sign-in emails. **Unset: nobody can sign in by email.** On a loopback `PUBLIC_URL` (local development) the link is logged at INFO instead; anywhere else only `mail is not configured` is logged, never the link, and the server warns at start-up |
| `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET` | secrets | Google sign-in; without both the button is hidden |
| `GITHUB_CLIENT_ID`, `GITHUB_CLIENT_SECRET` | secrets | GitHub sign-in; without both the button is hidden |
| `OPENROUTER_API_KEY` | secret | the chatbot's OpenRouter key (see "OpenRouter (chat)" below). **Unset: the chat page says the assistant is unavailable** and no question is accepted; the server warns at start-up on an `https` deployment |
| `LITESTREAM_BUCKET`, `LITESTREAM_ENDPOINT`, `LITESTREAM_ACCESS_KEY_ID`, `LITESTREAM_SECRET_ACCESS_KEY` | secrets | R2 replication; all four needed, otherwise the database is not replicated (the entrypoint says so at start-up). `LITESTREAM_PATH` (optional) is the folder in the bucket, default `breadator` |
| `RESEND_API_URL`, `GOOGLE_AUTH_URL`, `GOOGLE_TOKEN_URL`, `GOOGLE_ISSUER`, `GITHUB_AUTH_URL`, `GITHUB_TOKEN_URL`, `GITHUB_API_URL`, `OPENROUTER_URL` | — | endpoint overrides for tests; never set in production |

No `SESSION_SECRET` exists: sessions are random 256-bit tokens whose SHA-256 is stored, so there is
nothing to sign.

## First deployment (done on 2026-11-25)

What exists, and how to recreate it:

1. **Repository**: public `R-luiz/breadator`, branch `main`.
2. **Fly app and volume** (account already used by LoL Rank Bot). The volume must exist before the
   first deploy; it is mounted at `/data` and holds the SQLite database:
   ```sh
   fly apps create breadator
   fly volumes create breadator_data --region fra --size 1 -a breadator
   ```
   One machine only: a volume belongs to one machine and SQLite has one writer. `fly.toml` keeps
   it always on (`auto_stop_machines = "off"`, `min_machines_running = 1`); never scale to two.
3. **Secrets**: none set yet. Each can be added on its own, once the service below exists
   (`fly secrets set` restarts the machine):
   ```sh
   fly secrets set -a breadator RESEND_API_KEY=re_…
   fly secrets set -a breadator GOOGLE_CLIENT_ID=… GOOGLE_CLIENT_SECRET=…
   fly secrets set -a breadator GITHUB_CLIENT_ID=… GITHUB_CLIENT_SECRET=…
   fly secrets set -a breadator OPENROUTER_API_KEY=sk-or-v1-…
   fly secrets set -a breadator LITESTREAM_BUCKET=… LITESTREAM_ENDPOINT=https://<account id>.r2.cloudflarestorage.com \
       LITESTREAM_ACCESS_KEY_ID=… LITESTREAM_SECRET_ACCESS_KEY=…
   ```
4. **Deploy and certificate**:
   ```sh
   fly deploy --remote-only            # first build on Fly's builder
   fly certs add breadator.robinluiz.fr
   fly certs setup breadator.robinluiz.fr   # prints the DNS records below
   ```
5. **DNS** (Cloudflare zone `robinluiz.fr`, listed in `robinluiz-infra/README.md`):

   | Name | Type | Content | Proxy |
   |---|---|---|---|
   | `breadator` | CNAME | `0d6xmor.breadator.fly.dev` | proxied |
   | `_acme-challenge.breadator` | CNAME | `breadator.robinluiz.fr.0d6xmor.flydns.net` | DNS only |
   | `_fly-ownership.breadator` | TXT | `app-0d6xmor` | — |

   The challenge record lets Fly issue and renew the certificate behind the proxy; the
   ownership record is what Fly asks for when traffic comes through a CDN. Both must stay.
   Cloudflare must not cache HTML or the API (its default): pages carry the account link of the
   signed-in user (`/<lang>/account…` and `/auth/` answer `no-store`, other pages of a signed-in
   user `private`).
6. **GitHub**: environment `production`, secret `FLY_API_TOKEN`, a deploy token limited to this
   app. **It expires on 2027-11-25**; renew it with
   `fly tokens create deploy -a breadator -x 8760h | gh secret set FLY_API_TOKEN --env production -R R-luiz/breadator`.
   The Deploy workflow runs after each green CI run on `main` and then checks the public site.

Still to do:

- **Web Analytics**: Cloudflare already adds its script on the proxied hostname (the CSP allows
  it); check that the hostname shows up under Web Analytics in the dashboard.
- **Portfolio**: add a card on robinluiz.fr.

### OpenRouter (chat)

The chatbot spends the owner's OpenRouter credit: at most USD 1 a month, enforced twice.

1. OpenRouter → Keys → Create key: name `breadator`, **credit limit USD 1.00, reset monthly**.
   OpenRouter itself refuses calls once the month's dollar is spent, whatever the app does.
2. Settings → Privacy: **ZDR enforcement for "All other models": ZDR only** (Safety Retention
   excluded); leave "OpenRouter use of inputs/outputs" and input/output logging off. The app
   also sends `provider.zdr: true` with every request.
3. `fly secrets set -a breadator OPENROUTER_API_KEY=sk-or-v1-…` (the machine restarts with it).

The app's own cut-offs come first. Before each call it books the call's ceiling cost in
`chat_spend` and `chat_usage`, and replaces it with the call's `usage.cost` once the call
returns: a call cut short by a deploy or a crash stays booked at its ceiling. It stops
answering for the rest of the UTC day at USD 0.032 and for the rest of the month at USD 0.95;
each account gets 10 answers a day, 100 a month and USD 0.008 a day, and each client network
(IPv4 address or IPv6 /48) 15 answers a day. Every answer logs one line
`chat answer` with `model`, `provider`, `calls`, `tools`, `tokens_in`, `tokens_out` and
`cost_usd` (never the question). Spend so far: `select * from chat_spend` (month rows `YYYY-MM`,
today's row `YYYY-MM-DD`); compare with the key's usage on OpenRouter. To stop the chat at
once: `fly secrets unset -a breadator OPENROUTER_API_KEY`.

### Resend (sign-in emails)

The sender domain `robinluiz.fr` is already verified in Resend. Mails come from
`Breadator <no-reply@robinluiz.fr>`.

1. Resend dashboard → API Keys → Create API Key: permission **Sending access**, domain
   `robinluiz.fr`, name `breadator`.
2. `fly secrets set -a breadator RESEND_API_KEY=re_…`.
3. Check: request a link on `/en/account/sign-in` and read the mail; `fly logs` shows
   `cannot send the sign-in email` with Resend's answer if it fails (the page answers the same
   either way, by design).

Limits to know: Resend's free plan allows 100 mails a day. The app sends at most 80 sign-in mails
a day (UTC) in total, then sends none until midnight UTC: the page answers as usual and `fly logs`
shows the ERROR `daily cap of sign-in emails reached` once an hour. It also allows 5 links an hour
per address and 20 an hour per client address.

### Google sign-in (OpenID Connect)

1. Google Cloud console → create (or pick) a project → **APIs & Services → OAuth consent
   screen**: user type *External*, app name `Breadator`, support and developer email, authorized
   domain `robinluiz.fr`, scopes `openid`, `email`, `profile` (non-sensitive: no Google review),
   then **Publish app** (in *Testing* mode only listed test users can sign in).
2. **Credentials → Create credentials → OAuth client ID** → *Web application*, name `Breadator`:
   - Authorized redirect URI: `https://breadator.robinluiz.fr/auth/google/callback`
   - no JavaScript origin is needed (the flow runs on the server).
3. `fly secrets set -a breadator GOOGLE_CLIENT_ID=… GOOGLE_CLIENT_SECRET=…`.

For local development create a second client with the redirect URI
`http://localhost:8080/auth/google/callback` and run with `PUBLIC_URL=http://localhost:8080`.

### GitHub sign-in

1. GitHub → Settings → Developer settings → **OAuth Apps → New OAuth App**: name `Breadator`,
   homepage `https://breadator.robinluiz.fr`, **Authorization callback URL**
   `https://breadator.robinluiz.fr/auth/github/callback`. (A GitHub OAuth app takes one callback
   URL: use a second app for `http://localhost:8080/auth/github/callback`.)
2. Generate a client secret.
3. `fly secrets set -a breadator GITHUB_CLIENT_ID=… GITHUB_CLIENT_SECRET=…`.

The app asks for the `user:email` scope and signs in only with the primary **verified** address.

### Backups: Cloudflare R2 + Litestream

Litestream replicates the SQLite WAL to an R2 bucket continuously (about every second). With the
four `LITESTREAM_*` secrets set, the container's entrypoint (`docker-entrypoint.sh`) restores the
database from the bucket when `/data` has none (new machine, lost volume), then runs the server
under `litestream replicate`. Without them the server runs alone and says so in its log.

1. Cloudflare dashboard → R2 → **Create bucket** `breadator-backups` (location: European Union
   if you want EU storage; then the endpoint is `https://<account id>.eu.r2.cloudflarestorage.com`).
2. R2 → **Manage API tokens → Create API token**: permission *Object Read & Write*, scoped to that
   bucket only. Note the **Access Key ID**, the **Secret Access Key** and the account id (R2
   overview page).
3. `fly secrets set -a breadator LITESTREAM_BUCKET=breadator-backups LITESTREAM_ENDPOINT=https://<account id>.r2.cloudflarestorage.com LITESTREAM_ACCESS_KEY_ID=… LITESTREAM_SECRET_ACCESS_KEY=…`
4. Check `fly logs`: `litestream: replicating /data/breadator.sqlite3 to bucket …`, and the
   bucket fills with `breadator/` objects.

Restore drill (do it once, then after any change to the setup): from a machine with the
`litestream` binary and the same four variables,
`litestream restore -config litestream.yml -o /tmp/restored.sqlite3 /data/breadator.sqlite3`
(with `DATABASE_PATH=/data/breadator.sqlite3` exported), then
`sqlite3 /tmp/restored.sqlite3 'select count(*) from users'`. Disaster recovery needs no manual
step: destroy the volume, create a new one with the same name, redeploy — the machine boots, finds
no database, restores it from R2 and carries on.

Fly also snapshots volumes daily (5 days kept): a second, independent copy.

## Accounts and data

- Accounts are optional; everything works without one (data stays on the device). Signed-in
  users sync saved formulas, starters, bakes and calibrations (`/api/v1/sync`): 5 MB stored per
  account at most, tombstones included (`select id, sync_bytes from users order by sync_bytes
  desc limit 5` shows the largest), and 30 pushes carrying records per ten minutes.
- Sign-in: email link (Resend), Google, GitHub, passkeys. Addresses are verified (a link, or the
  provider's verified flag) and identical addresses are one account.
- Sessions: 60 days, sliding; cookie `__Host-session` (`HttpOnly; Secure; SameSite=Lax`), only the
  SHA-256 of the token is stored. Users can sign out of other devices from their account page.
- Every non-GET request must carry an `Origin` (or `Referer`) equal to `PUBLIC_URL`, else 403.
  Exception: a JSON `/api/…` call with no session cookie (the stateless engine endpoints), which
  has no credential to abuse. Calling the site through another hostname (for example
  `breadator.fly.dev`) therefore fails on forms: use the public URL.
- Rate limits (SQLite, survive restarts): 5 magic links an hour per address, 20 per client
  address, 80 sign-in mails a day for the whole site, 60 OAuth starts an hour, and 30 passkey
  sign-ins started and 30 finished per ten minutes per client address. The client address is
  `CF-Connecting-IP` when `Fly-Client-IP` is a Cloudflare address, else `Fly-Client-IP`; an IPv6
  client counts as its whole /48.
- A magic link signs in by itself only in the browser that asked for it (a 15-minute cookie,
  `__Host-signin`); opened anywhere else it shows the masked address and waits for a press. A
  Google or GitHub sign-in whose address already has an account mails such a link to the address
  and links the identity once it is used.
- Passkey ceremonies (a challenge, five minutes) live in memory, up to 1,000 sign-ins and 1,000
  registrations: a restart in the middle of one only makes that attempt fail.
- Housekeeping (expired sessions, links, OAuth requests, rate-limit windows, tombstones older
  than a year) runs at start-up and every six hours.
- Deleting a user (`delete from users where id = …`) cascades to every table; the account page
  does the same and also deletes the magic links of the address (`email_tokens`). It also offers
  a JSON export of everything stored, signed-in devices included.
- Chat (signed-in only): conversations are stored per account and deletable one by one; the
  export includes them with the daily usage (`chat.usage`), and deleting the account removes
  them. `chat_spend` holds only site-wide totals and survives deletions; so does `chat_usage`,
  keyed on SHA-256 hashes of the address and of the client network (no account id), so that a
  deleted account does not get its allowances back. Its rows of earlier months are deleted.

### Local development

```sh
PUBLIC_URL=http://localhost:8080 cargo run -p breadator-server     # data/breadator.sqlite3
```

With `PUBLIC_URL` on `localhost` (or `127.0.0.1`, `[::1]`) and no `RESEND_API_KEY`, the sign-in
link is logged at INFO (`RESEND_API_KEY is not set: sign-in link not emailed …
link=http://localhost:8080/…`). Open the site as `http://localhost:8080` (not `127.0.0.1`): the
origin check compares with `PUBLIC_URL`.
Passkeys work on `localhost`.

## Routine

- Deploy: push to `main`; `Deploy` runs after green CI and smoke-tests the public URL. Manual
  re-deploy: run the `Deploy` workflow on `main`.
- Logs: `fly logs -a breadator` (JSON lines).
- Health: `GET /healthz`; Fly checks it every 30 s.
- Schema changes: add `migrations/NNNN_name.sql` (next number, no gaps); it is applied at
  start-up inside a transaction, and the server refuses to start on a database newer than itself
  (a rollback to an older image after a migration needs the volume restored first).
- Look inside the database: `fly ssh sftp get /data/breadator.sqlite3` (take the `-wal` file too,
  or restore from R2) and open the copy locally. The server uses WAL: never copy the main file
  alone from a running machine and expect it to be current.
