# To Do

## Database
- [x] Convert the insertion triggers to Postgres dialect
- [x] Replace the updated_at triggers with a function and calls to that function for each table
- [ ] Add [`sqlcheck`](https://github.com/jarulraj/sqlcheck) for additional guidance on SQL best practices
- [ ] Write tests for the insertion logic with [pgTap](https://github.com/NixOS/nixpkgs/blob/nixos-21.11/pkgs/servers/sql/postgresql/ext/pgtap.nix#L21)/pg_prove. Requires packaging for Nix
- [ ] Try setting up tests to use a per-test-run DB template rather than having a permanent test database
- [ ] Setup a SQL formatter (pgFormatter?, sqlfmt?)
- [ ] Read through [Awesome Postgres](https://dhamaniasad.github.io/awesome-postgres/)
- [ ] Investigate [Postgres Check-up](https://gitlab.com/postgres-ai/postgres-checkup)

## Source Code

## Meta
- [ ] Set up a Nix environment with dependencies (Rust, PGFormatter, JDK, Wget, etc)