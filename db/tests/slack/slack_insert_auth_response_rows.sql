-- Start transaction and plan the tests.
BEGIN;

-- Plan count should be the number of tests
SELECT plan(7);

-- #################################################
-- 1. Definition checks
-- #################################################
SELECT has_schema('slack');
SELECT has_function('slack_insert_auth_response_rows');

-- #################################################
-- 2. Setup data for tests
-- #################################################

INSERT INTO slack.auth_responses (
    access_token,
    authed_user_id,
    bot_user_id,
    enterprise_id,
    enterprise_name,
    scopes,
    workspace_id,
    workspace_name,
    json_response
) VALUES (
    'xoxb-1234-123456789012-aBCdEFGHIJklmNOpQRSTuVWx',
    'U1234ABCD',
    'U5678EFGH',
    NULL,
    NULL,
    'commands,chat:write,users:read',
    'T9876ZYXW',
    'Company Foo',
    '{}'
);

-- TODO: How to handle multiple rows? Should there be a primary key or uniqueness constraint on auth_responses?
-- INSERT INTO slack.auth_responses (
--     access_token,
--     authed_user_id,
--     bot_user_id,
--     enterprise_id,
--     enterprise_name,
--     scopes,
--     workspace_id,
--     workspace_name,
--     json_response
-- ) VALUES (
--     'xoxb-1234-123456789012-aBCdEFGHIJklmNOpQRSTuVWx',
--     'U1234ABCD',
--     'U5678EFGH',
--     'E1',
--     'Some',
--     'false',
--     'commands,chat:write,users:read',
--     'T9876ZYXW',
--     'Company Foo',
--     '{}'
-- );

-- #################################################
-- 3. Test success cases
-- #################################################
-- Checks the success cases and expected database state after execution
SELECT
    is(
        slack.auth_responses.*,
        ROW (
            'xoxb-1234-123456789012-aBCdEFGHIJklmNOpQRSTuVWx',
            'U1234ABCD', -- authed_user_id
            'U5678EFGH', -- bot_user_id
            NULL, -- enterprise_id
            NULL, -- enterprise_name
            'commands,chat:write,users:read', -- scopes
            'T9876ZYXW', -- workspace_id
            'Company Foo', -- workspace_name
            '{}', -- json_response
            (SELECT created_at FROM slack.auth_responses WHERE access_token = 'xoxb-1234-123456789012-aBCdEFGHIJklmNOpQRSTuVWx')::timestamptz
        )::slack.auth_responses,
        'Row should exist in slack.auth_responses'
    )
FROM slack.auth_responses
WHERE access_token = 'xoxb-1234-123456789012-aBCdEFGHIJklmNOpQRSTuVWx';

SELECT is_empty('SELECT * FROM slack.enterprises', 'No enterprises should be present in slack.enterprises');

SELECT
    is(
        slack.workspaces.*,
        ROW (
            'T9876ZYXW', -- id (primary key)
            'Company Foo', -- name
            'true', -- has_app_installed
            NULL, -- enterprise_id
            (SELECT created_at FROM slack.workspaces WHERE id = 'T9876ZYXW')::timestamptz,
            (SELECT updated_at FROM slack.workspaces WHERE id = 'T9876ZYXW')::timestamptz
        )::slack.workspaces,
        'Row should exist in slack.workspaces'
    )
FROM slack.workspaces
WHERE id = 'T9876ZYXW';

SELECT
    is(
        slack.workspace_users.*,
        ROW (
            'U1234ABCD', -- id (primary key)
            NULL, -- enterprise_user_id
            'T9876ZYXW', -- workspace_id
            (SELECT created_at FROM slack.workspace_users WHERE id = 'U1234ABCD')::timestamptz,
            (SELECT updated_at FROM slack.workspace_users WHERE id = 'U1234ABCD')::timestamptz
        )::slack.workspace_users,
        'Row should exist in slack.workspace_users'
    )
FROM slack.workspace_users
WHERE id = 'U1234ABCD';

SELECT
    is(
        slack.bots.*,
        ROW (
            'U5678EFGH', -- id (primary key)
            'xoxb-1234-123456789012-aBCdEFGHIJklmNOpQRSTuVWx', -- access_token
            'commands,chat:write,users:read', -- scopes
            'T9876ZYXW', -- workspace_id
            (SELECT created_at FROM slack.bots WHERE id = 'U5678EFGH')::timestamptz,
            (SELECT updated_at FROM slack.bots WHERE id = 'U5678EFGH')::timestamptz
        )::slack.bots,
        'Row should exist in slack.bots'
    )
FROM slack.bots
WHERE id = 'U5678EFGH';

-- #################################################
-- 4. Check error cases
-- #################################################


-- #################################################
-- 5. Finish tests and clean up
-- #################################################
SELECT * FROM finish();

ROLLBACK;
