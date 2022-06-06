CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    github_user_id TEXT REFERENCES github.users,
    slack_user_id TEXT REFERENCES slack.workspace_users,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_users_github_user_id ON users USING hash(github_user_id);
CREATE INDEX idx_users_slack_user_id ON users USING hash(slack_user_id);

-- Set the `updated_at` column to the current timestamp on each update
CREATE TRIGGER users_handle_updated_at BEFORE UPDATE ON users
FOR EACH ROW EXECUTE PROCEDURE extensions.moddatetime(updated_at);
