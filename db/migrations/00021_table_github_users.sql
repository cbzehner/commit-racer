CREATE TABLE IF NOT EXISTS github.users (
    id TEXT PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Set the `updated_at` column to the current timestamp on each update
CREATE TRIGGER github_users_handle_updated_at BEFORE UPDATE ON github.users
FOR EACH ROW EXECUTE PROCEDURE extensions.moddatetime(updated_at);
