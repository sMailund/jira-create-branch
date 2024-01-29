quick script that takes a link to a jira issue and creates a branch on the form `feature/<issue-key>-<issue-description>`

# install 
`cargo install --path .`

set the following envionment variables:
- `GIT_JIRA_TOKEN` api key for jira
- `GIT_JIRA_API_URL` url for the jira api, might vary depending if you're not running on jira cloud
- `GIT_JIRA_USERNAME` the username your api key is connected to

# usage

navigate to the root of your repository

`jira-create-branch <link to jira ticket>` 

# troubleshooting
The jira api responds with 404 if the credentials are wrong, not 401. This might cause some confusing error messages.
