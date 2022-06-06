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
  psql --command 'CREATE SCHEMA IF NOT EXISTS extensions AUTHORIZATION commit_racer; CREATE EXTENSION IF NOT EXISTS moddatetime SCHEMA extensions;'  --dbname commit_racer --username postgres --no-password

new MIGRATION_NAME:
	cargo sqlx migrate add --source ./migrations {{MIGRATION_NAME}} --database-url postgres://commit_racer:@localhost:5432/commit_racer

reset:
  psql --command 'DROP DATABASE commit_racer;'  --dbname postgres --username postgres --no-password
  psql --command 'DROP ROLE commit_racer;'  --dbname postgres --username postgres --no-password
  just initialize
  just run

run:
	cargo sqlx migrate run --source ./migrations --database-url postgres://commit_racer:@localhost:5432/commit_racer

test:
  psql --command "INSERT INTO slack.auth_responses (access_token, authed_user_id, bot_user_id, enterprise_id, enterprise_name, is_enterprise_install, scope, workspace_id, workspace_name, json_response) VALUES ('xoxb-1234-883122577908-nPLb86QGPGrncMVoNMERkBRh', 'UEP8P883F', 'URZ3LGZSQ', 'E1234', 'Evil Corp', 'false', 'commands,chat:write,users:read', 'TENV76K8T', 'International Technology Company', '{}');" --dbname commit_racer --username commit_racer --no-password
  @echo "Unimplemented!"
