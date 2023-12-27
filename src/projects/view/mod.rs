use clap::Args;
mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    config::Config,
    execute_graphql_request::execute_graphql_request,
    format_relative_time::format_relative_time,
    keyring::keyring,
    print_formatted_error::print_formatted_error,
    query_full_id::{query_full_id, QueryType},
};
use crate::projects::view::graphql::project_details::{project_details, ProjectDetails};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::{
    crossterm::style::{style, Color, Stylize},
    MadSkin,
};

#[derive(Args, Debug)]
pub struct ProjectsViewArgs {
    #[clap(
        short,
        long,
        help = "Project ID (First 4 characters or more are allowed)"
    )]
    id: String,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn view(args: &ProjectsViewArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let project_id = query_full_id(QueryType::Project, args.id.clone(), &access_token);

    let project_details_error_message = format!(
        "Failed to retrieve detailed information for the project with ID '{}'.",
        &args.id
    );

    let project_details =
        execute_graphql_request::<project_details::Variables, project_details::ResponseData>(
            authorization_headers(&access_token),
            ProjectDetails::build_query,
            &Client::new(),
            &project_details_error_message,
            project_details::Variables { id: project_id },
        )
        .token;

    let current_project_info = match project_details.first() {
        Some(data) => data,

        None => {
            print_formatted_error(&project_details_error_message);
            std::process::exit(1);
        }
    };

    let last_usage = if let Some(usage_history) = &current_project_info.usage_history.first() {
        match format_relative_time(&usage_history.created_at.to_string()) {
            Ok(relative_time) => relative_time,

            Err(_) => {
                print_formatted_error("Failed to parse the last usage time. Please try again.");
                std::process::exit(1);
            }
        }
    } else {
        "never used".to_string().grey()
    };

    let skin = MadSkin {
        ..Default::default()
    };

    let markdown_text = format!(
        "**Name**: {} ({})\n**Owner**: {} ({})\n**Secrets**: {}\n**Integrations**: {}\n**Teams**: {}\n**URL**: {}\n**Last used**: {}",
        &current_project_info.name,
        &current_project_info.id,
        &current_project_info.owner.name,
        &current_project_info.owner.email,
        if let Some(secret_count) = &current_project_info.user_secret_aggregate.aggregate {
            secret_count.count
        } else {
            0
        },
        if let Some(integrations_count) = &current_project_info.integration_installations_aggregate.aggregate {
            integrations_count.count
        } else {
            0
        },
        if let Some(teams_count) = &current_project_info.teams_aggregate.aggregate {
            teams_count.count
        } else {
            0
        },
        style(format!("{}/projects/{}", Config::new().webapp_url, &current_project_info.id.to_string().replace("-", ""))).with(Color::Rgb {
            r: 0,
            g: 135,
            b: 255,
        }),
        last_usage,
    );

    skin.print_text(&markdown_text);
}
