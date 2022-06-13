CREATE TABLE IF NOT EXISTS slack.bots (
    id TEXT PRIMARY KEY,
    access_token TEXT NOT NULL,
    scopes TEXT NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES slack.workspaces,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_slack_bots_workspace_id ON slack.bots USING hash(workspace_id);

-- Set the `updated_at` column to the current timestamp on each update
CREATE TRIGGER slack_bots_handle_updated_at BEFORE UPDATE ON slack.bots
FOR EACH ROW EXECUTE PROCEDURE extensions.moddatetime(updated_at);
