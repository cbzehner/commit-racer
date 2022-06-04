#!/usr/bin/env bash
set -euxo pipefail;

# [Fetch the executable](https://github.com/OpenAPITools/openapi-generator#13---download-jar)
wget https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/5.4.0/openapi-generator-cli-5.4.0.jar -O openapi-generator-cli.jar
# Generate the Rust code from the Slack OpenAPI v2 specification
java -jar openapi-generator-cli.jar generate --generator-name rust --input-spec https://api.slack.com/specs/openapi/v2/slack_web.json --additional-properties=packageName=slack,useSingleRequestParameter=true --output slack-types
# Build & open the documentation
cd slack-types && cargo doc --open