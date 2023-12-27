mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    colorful_theme::theme,
    execute_graphql_request::execute_graphql_request,
    keyring::keyring,
    print_formatted_error::print_formatted_error,
    query_full_id::{query_full_id, QueryType},
};
use crate::teams::common::team_info::team_info;
use clap::Args;
use dialoguer::Input;
use graphql::remove_team::{remove_team, RemoveTeam};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::Stylize;

#[derive(Args, Debug)]
pub struct TeamsDeleteArgs {
    #[clap(short, long, help = "Team ID (First 4 characters or more are allowed)")]
    id: String,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn delete(args: &TeamsDeleteArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let team_id = query_full_id(QueryType::Teams, args.id.clone(), &access_token);
    let authorization_headers = authorization_headers(&access_token);
    let client = Client::new();
    let team_details = team_info(&access_token, team_id);

    let input: String = Input::with_theme(&theme())
        .with_prompt(format!(
            "Type {} to confirm deletion:",
            &team_id.to_string()[..4]
        ))
        .interact_text()
        .expect("Deletion failed. Failed to read the user's reply.");

    if input != team_id.to_string()[..4] {
        println!(
            "{}",
            format!("X Sorry your reply was invalid: You entered {}.", input).red()
        );

        std::process::exit(0);
    }

    let remove_team_error_message = format!(
        "Deletion failed. Failed to delete the team with ID '{}'.",
        args.id.clone()
    );

    let remove_team_response =
        execute_graphql_request::<remove_team::Variables, remove_team::ResponseData>(
            authorization_headers.clone(),
            RemoveTeam::build_query,
            &client,
            &remove_team_error_message,
            remove_team::Variables {
                team_id: team_id.to_string(),
                team_member_ids: team_details
                    .members
                    .iter()
                    .map(|member| member.member.id.to_string())
                    .collect::<Vec<String>>(),
                team_name: team_details.name,
            },
        )
        .remove_team
        .success;

    if !remove_team_response {
        print_formatted_error(&remove_team_error_message);
        std::process::exit(1);
    }

    println!("{} {}", "✔".green(), "Team successfully deleted");
}
