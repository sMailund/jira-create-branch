use config::{Config, Environment, File};
use reqwest::header;
use serde::{Deserialize, Serialize};
use std::env;
use std::process::exit;
use tokio;
use git2::{Repository, BranchType};

// Define a struct to represent the Jira issue fields you want to retrieve.
#[derive(Debug, Serialize, Deserialize)]
struct JiraIssue {
    key: String,
    fields: IssueFields,
}

#[derive(Debug, Serialize, Deserialize)]
struct IssueFields {
    summary: String,
    // Add more fields as needed.
}

#[tokio::main]
async fn main() {
    let jira_token = match env::var("GIT_JIRA_TOKEN") {
        Ok(value) => value,
        Err(_) => {
            eprintln!("missing environment variable GIT_JIRA_TOKEN");
            exit(-2)
        },
    };

    let jira_api_url = match env::var("GIT_JIRA_API_URL") {
        Ok(value) => value,
        Err(_) => {
            eprintln!("missing environment variable GIT_JIRA_API_URL");
            exit(-3)
        },
    };

    let jira_username = match env::var("GIT_JIRA_USERNAME") {
        Ok(value) => value,
        Err(_) => {
            eprintln!("missing environment variable GIT_JIRA_USERNAME");
            exit(-4)
        },
    };

    // Jira ticket key (e.g., "PROJECT-123").
    let ticket_key = "NTA-1966";

    // Jira API URL for the specific ticket.
    let jira_url = format!("{}/rest/api/latest/issue/{}", jira_api_url, ticket_key);
    println!("{}", jira_url);

    // Jira API credentials (username and token).
    let username = jira_username;
    let token = jira_token;

    // Create a basic authentication header.
    let auth_header = format!(
        "Basic {}",
        base64::encode(&format!("{}:{}", username, token))
    );

    // Create a Reqwest client with basic authentication.
    let client = reqwest::Client::new();

    // Make the HTTP request to the Jira API.
    let response = client
        .get(&jira_url)
        .header(header::AUTHORIZATION, auth_header)
        .send()
        .await; // Use the `await` keyword to await the completion of the future.

    // Check if the request was successful (HTTP status code 200).
    if let Ok(response) = response {
        if response.status().is_success() {
            // Parse the JSON response into a JiraIssue struct.
            let jira_issue: JiraIssue = response.json().await.expect("Failed to parse JSON");

            let key = jira_issue.key;
            let summary: String = jira_issue.fields.summary
                .replace(" ", "-")
                .chars()
                .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
                .collect();

            let repo = Repository::open(".").expect("Failed to open repository");

            let branch_name = format!("feature/{}-{}", key, summary);

            // Get the current commit
            let head = repo.head().expect("Failed to get HEAD reference");
            let commit = repo.find_commit(head.target().expect("Failed to get target commit"))
                .expect("Failed to find commit");

            // Create a new branch
            let branch = repo.branch(&branch_name, &commit, false)
                .expect("Failed to create branch");

            // Checkout the new branch
            repo.checkout_head(None)
                .expect("Failed to checkout branch");

            println!("Branch '{}' created and checked out successfully.", branch_name);

        } else {
            println!("Error: {}", response.status());
        }
    } else {
        println!("Failed to make the request");
    }
}
