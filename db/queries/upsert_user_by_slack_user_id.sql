INSERT INTO users (slack_user_id)
    VALUES ($1)
ON CONFLICT (slack_user_id) DO UPDATE
    SET slack_user_id = EXCLUDED.slack_user_id
RETURNING id, github_user_id, slack_user_id;
