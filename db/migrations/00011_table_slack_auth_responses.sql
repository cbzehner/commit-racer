CREATE TABLE IF NOT EXISTS slack.auth_responses (
    access_token TEXT NOT NULL,
    authed_user_id TEXT NOT NULL,
    bot_user_id TEXT,
    enterprise_id TEXT,
    enterprise_name TEXT,
    scopes TEXT NOT NULL,
    workspace_id TEXT NOT NULL,
    workspace_name TEXT NOT NULL,
    json_response JSONB NOT NULL,
    created_at TEXT NOT NULL DEFAULT now()
);

CREATE OR REPLACE FUNCTION slack_insert_auth_response_rows() RETURNS TRIGGER AS $inserted_rows$
    BEGIN
        IF NEW.enterprise_id IS NOT NULL THEN
            INSERT INTO slack.enterprises (id, name)
            VALUES (NEW.enterprise_id, NEW.enterprise_name)
            ON CONFLICT (id) DO UPDATE SET name = NEW.enterprise_name;
        END IF;

        INSERT INTO slack.workspaces (id, name, enterprise_id)
        VALUES (NEW.workspace_id, NEW.workspace_name, NEW.enterprise_id)
        ON CONFLICT (id) DO UPDATE SET (name, enterprise_id) = (NEW.workspace_name, NEW.enterprise_id);

        INSERT INTO slack.workspace_users (id, workspace_id)
        VALUES (NEW.authed_user_id, NEW.workspace_id)
        ON CONFLICT DO NOTHING;

        INSERT INTO slack.bots (id, access_token, scopes, workspace_id)
        VALUES (NEW.bot_user_id, NEW.access_token, NEW.scopes, NEW.workspace_id)
        ON CONFLICT (id) DO UPDATE SET (access_token, scopes, workspace_id) = (NEW.access_token, NEW.scopes, NEW.workspace_id);

        RETURN NULL; -- result is ignored since this is an AFTER trigger
	END;
$inserted_rows$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER slack_insert_auth_responses AFTER INSERT ON slack.auth_responses
FOR EACH ROW EXECUTE FUNCTION slack_insert_auth_response_rows();
