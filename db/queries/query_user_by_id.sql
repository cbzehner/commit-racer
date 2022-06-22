SELECT
    id,
    slack_user_id,
    github_user_id
FROM users
WHERE
    id = $1
