CREATE TABLE IF NOT EXISTS slack.workspace_users (
    id TEXT PRIMARY KEY CONSTRAINT conforming_identifier CHECK (
        starts_with(upper(id), 'U')
    ),
    enterprise_user_id TEXT REFERENCES slack.enterprise_users,
    workspace_id TEXT NOT NULL REFERENCES slack.workspaces,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_slack_workspace_users_enterprise_user_id ON slack.workspace_users USING hash(
    enterprise_user_id
);
CREATE INDEX idx_slack_workspace_users_workspace_id ON slack.workspace_users USING hash(
    workspace_id
);

-- Set the `updated_at` column to the current timestamp on each update
CREATE TRIGGER slack_workspace_users_handle_updated_at BEFORE UPDATE ON slack.workspace_users
FOR EACH ROW EXECUTE PROCEDURE extensions.moddatetime(updated_at);
