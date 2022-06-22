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
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9);
