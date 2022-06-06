CREATE TABLE IF NOT EXISTS slack.workspaces (
    id TEXT PRIMARY KEY CONSTRAINT conforming_identifier CHECK (
        starts_with(upper(id), 'T')
    ),
    -- TODO: Test what happens when a workspace name is changed after initial configuration.
    name TEXT NOT NULL,
    has_app_installed BOOLEAN DEFAULT true,
    enterprise_id TEXT REFERENCES slack.enterprises,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_slack_workspaces_enterprise_id ON slack.workspaces USING hash(
    enterprise_id
);

-- Set the `updated_at` column to the current timestamp on each update
CREATE TRIGGER slack_workspaces_handle_updated_at BEFORE UPDATE ON slack.workspaces
FOR EACH ROW EXECUTE PROCEDURE extensions.moddatetime(updated_at);
