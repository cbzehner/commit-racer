CREATE TABLE IF NOT EXISTS slack.enterprise_users (
    id TEXT PRIMARY KEY CONSTRAINT conforming_identifier CHECK (
        starts_with(upper(id), 'G')
    ),
    enterprise_id TEXT NOT NULL REFERENCES slack.enterprises,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_enterprise_users_enterprise_id ON slack.enterprise_users USING hash(
    enterprise_id
);

-- Set the `updated_at` column to the current timestamp on each update
CREATE TRIGGER slack_enterprise_users_handle_updated_at BEFORE UPDATE ON slack.enterprise_users
FOR EACH ROW EXECUTE PROCEDURE extensions.moddatetime(updated_at);
