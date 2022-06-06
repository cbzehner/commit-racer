CREATE TABLE IF NOT EXISTS slack.enterprises (
    id TEXT PRIMARY KEY CONSTRAINT conforming_identifier CHECK (
        starts_with(upper(id), 'E')
    ),
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Set the `updated_at` column to the current timestamp on each update
CREATE TRIGGER slack_enterprises_handle_updated_at BEFORE UPDATE ON slack.enterprises
FOR EACH ROW EXECUTE PROCEDURE extensions.moddatetime(updated_at);
