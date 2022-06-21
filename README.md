# Commit Racer

## Development

### Setup
1. [Install Nix](https://nixos.org/download.html), if it's not already available.
1. From the project folder run `NIXPKGS_ALLOW_UNFREE=1 nix-shell` to start a Nix shell with the required dependencies.
1. `just initialize` will install required `cargo-*` dependencies, setup a new PostgreSQL database locally and run an initial build.
1. Add the "Client Secret" from the [Slack App](https://api.slack.com/apps/ARZ3JJPAQ) to your `.env.development.local` file.

### Auth against Localhost
1. Run `ngrok http 8000` to expose your local port `8000` via a custom URL.
2. Copy the custom URL to the the `redirect_uri` field in `ROCKET_OAUTH_SLACK_REDIRECT_URI` in your `.env.development` file.
3. Take the full `redirect_uri` and add it to the "Redirect URLs" section of the [Slack App](https://api.slack.com/apps/ARZ3JJPAQ/oauth). Don't forget to "Save URLs"!
3. In a new terminal window run `just run` to start the application locally.
4. Navigate to `<ngrok.io url>/auth/slack/login`, you should be redirected to Slack to authorize the app.

### Testing Slash commands locally
1. Run `ngrok http 8000` to expose your local port `8000` via a custom URL.
2. Copy the custom URL and update the domains in the "Slash Commands" section of the [Slack App](https://api.slack.com/apps/ARZ3JJPAQ/slash-commands).
3. In a new terminal window run `make run` to start the application locally.
4. Now try your command via Slack and you should see it hit your local application.

### Debugging
1. Ensure the scopes & redirect urls listed in the Slack app under "OAuth & Permissions" match those in `src/integrations/slack/auth.rs`.
2. Ensure you navigated to `<ngrok.io url>/login/slack` rather than starting the request from the `localhost:8000` url. Cookies are domain specific so the cookies set by `rocket_oauth2` won't be available to `ngrok.io` unless you start the request from there as well.

## Resources

### Dependencies

#### Rust
- [Rust Book (Introductory)](https://doc.rust-lang.org/book/)
- [Rocket Server Guide](https://rocket.rs/guide/)
- [SQLx Usage](https://github.com/launchbadge/sqlx#usage)
- [Rocket OAuth2](https://github.com/jebrosen/rocket_oauth2/tree/next)
- [Tera Template Language](https://tera.netlify.app)
- [Rust Package Search](https://lib.rs)

#### Database
- [Prisma Database Guide](https://www.prisma.io/dataguide/intro/what-are-databases)
- [Postgres Documentation](https://www.postgresql.org/docs/current/index.html)
- [Postgres Snippets](https://supabase-sql.vercel.app)
- [pgProve Test Harness](https://pgtap.org/pg_prove.html)

#### Nix
- [Nix Package Search](https://search.nixos.org/packages)

### Integrations
- [Slack API](https://api.slack.com)
- [GitHub API](https://docs.github.com/en/rest)

### Background Reading
- [Data models for Slack Apps](https://wilhelmklopp.com/posts/slack-database-modelling/)
