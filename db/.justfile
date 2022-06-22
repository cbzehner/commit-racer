default:
  @just --choose

help:
  @just --list

check:
  sqlfluff lint --dialect postgres ./migrations

connect:
  pgcli  --dbname commit_racer --user commit_racer --no-password

fix:
  sqlfluff fix --dialect postgres ./migrations

initialize:
  psql --command 'CREATE ROLE commit_racer WITH LOGIN;'  --dbname postgres --username postgres --no-password
  psql --command 'CREATE DATABASE commit_racer WITH OWNER commit_racer;'  --dbname postgres --username postgres --no-password
  psql --command 'CREATE SCHEMA IF NOT EXISTS extensions AUTHORIZATION commit_racer; CREATE EXTENSION IF NOT EXISTS moddatetime SCHEMA extensions;' --dbname commit_racer --username postgres --no-password
  psql --command 'CREATE EXTENSION IF NOT EXISTS pgtap;'  --dbname commit_racer --username postgres --no-password

new MIGRATION_NAME:
	cargo sqlx migrate add --source ./migrations {{MIGRATION_NAME}}

reset:
  psql --command 'DROP DATABASE commit_racer;'  --dbname postgres --username postgres --no-password
  psql --command 'DROP ROLE commit_racer;'  --dbname postgres --username postgres --no-password
  just initialize
  just run

run:
	cargo sqlx migrate run --source ./migrations

test:
  @pg_prove --dbname commit_racer --username commit_racer ./tests/**/** --verbose
