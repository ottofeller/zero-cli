mod graphql;

use chrono::{Duration, Utc};
use crate::common::{
    authorization_headers::authorization_headers, colorful_theme::theme, config::Config,
    execute_graphql_request::execute_graphql_request,
    keyring::keyring, print_formatted_error::print_formatted_error,
};
use crate::projects::create::graphql::create_project::{create_project, CreateProject};
use clap::Args;
use dialoguer::{console::Style, theme::ColorfulTheme, Input, Sort, Confirm, Select};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::{
    crossterm::style::{style, Color, Stylize},
    minimad, MadSkin,
};

#[derive(Args, Debug)]
pub struct ProjectsCreateArgs {
    #[clap(
    short,
    long,
    help = "Project name, if not specified, the user will be prompted to enter the name"
    )]
    name: Option<String>,
    #[clap(
    short,
    long,
    help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn create(args: &ProjectsCreateArgs) {
    let items_per_page = Config::new().items_per_page;

    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let client = Client::new();
    let headers = authorization_headers(&access_token);

    let project_name = match &args.name {
        Some(name) => {
            if name.trim().chars().count() < 2 {
                print_formatted_error(
                    "Creation failed. The project name must be at least 2 characters long.",
                );

                std::process::exit(1);
            }

            name.clone()
        }

        None => {
            match Input::with_theme(&theme())
                .with_prompt("Type a project name:")
                .validate_with(|name: &String| -> Result<(), &str> {
                    if name.trim().chars().count() < 2 {
                        return Err("The project name must be at least 2 characters long.");
                    } else {
                        Ok(())
                    }
                })
                .interact()
            {
                Ok(new_name) => new_name,

                Err(_) => {
                    print_formatted_error("Creation failed. Failed to read the project name.");
                    std::process::exit(1);
                }
            }
        }
    };

    let is_token_needed = match Confirm::with_theme(&theme())
        .with_prompt("Do you want to generate a new token for this project?")
        .interact()
    {
        Ok(token_needed) => token_needed,

        Err(_) => {
            print_formatted_error("Creation failed. Failed to read the token generation option.");
            std::process::exit(1);
        }
    };

    let token;

    if is_token_needed {
        let token_name = match Input::with_theme(&theme())
            .with_prompt("Enter the token name:")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.trim().chars().count() < 2 {
                    return Err("The token name must be at least 2 characters long.");
                } else {
                    Ok(())
                }
            })
            .interact()
        {
            Ok(new_token_name) => new_token_name,

            Err(_) => {
                print_formatted_error("Creation failed. Failed to read the token name.");
                std::process::exit(1);
            }
        };

        let options = ["Endless", "7 days", "30 days", "60 days", "90 days"];

        let expires_at_selections = match Select::with_theme(&theme())
            .with_prompt(format!(
                "Expires in: {}",
                "Use <Up>/<Down> to navigate and <Enter>/<Space> to select".dark_grey()
            ))
            .default(0)
            .items(
                &options
                    .iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>(),
            )
            .max_length(items_per_page)
            .interact()
        {
            Ok(expires_at_selections) => expires_at_selections,

            Err(_) => {
                print_formatted_error("Sharing failed. Failed to read expiration time.");
                std::process::exit(1);
            }
        };

        let token_expires_in = match options[expires_at_selections] {
            "Endless" => None,
            "7 days" => Some((Utc::now() + Duration::days(7)).to_string()),
            "30 days" => Some((Utc::now() + Duration::days(30)).to_string()),
            "60 days" => Some((Utc::now() + Duration::days(60)).to_string()),
            "90 days" => Some((Utc::now() + Duration::days(90)).to_string()),

            _ => {
                print_formatted_error("Creation failed. Failed to read expiration time.");
                std::process::exit(1);
            }
        };

        token = Some(create_project::TokenObject {
            token_expires_in: token_expires_in,
            token_name: Some(token_name),
        });
    } else {
        token = None;
    }

    let create_project_error_message = format!(
        "Creation failed. Failed to create a project with the name '{}'.",
        project_name.clone()
    );

    let first_letters: String = project_name.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .take(2)
        .flat_map(|c| c.to_uppercase())
        .collect();

    // Create a project
    let project_response =
        execute_graphql_request::<create_project::Variables, create_project::ResponseData>(
            headers.clone(),
            CreateProject::build_query,
            &client,
            &create_project_error_message,
            create_project::Variables {
                object: create_project::CreateProjectInput {
                    project_icon: format!(">{}#{:02x}{:02x}{:02x}", first_letters, rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()),
                    project_name: project_name.clone(),
                    token,
                },
            },
        )
            .create_project;

    // MD template for the project creation message
    let text_template = if is_token_needed {
        minimad::TextTemplate::from(
            r#"
        ##### ✔ Project successfully created
        ${description}
        **Project link**: ${project-link}
        **Project ID for CLI**: ${short-project-id}

        > *Please make sure to store this token in a safe place.*
        > *If you ever lose or forget this token, you can regenerate it.*
        > *However, be aware that any scripts or applications using this token will need to be updated accordingly.*
        "#,
        )
    } else {
        minimad::TextTemplate::from(
            r#"
        ##### ✔ Project successfully created
        ${description}
        **Project link**: ${project-link}
        **Project ID for CLI**: ${short-project-id}
        "#,
        )
    };

    let mut expander = text_template.expander();

    let project_id = match project_response.project_id {
        Some(id) => id,

        None => {
            print_formatted_error(&create_project_error_message);
            std::process::exit(1);
        }
    };

    let description = format!("You created the project named '{}'.", &project_name);

    let project_link = style(format!(
        "{}/projects/{}",
        Config::new().webapp_url,
        &project_id.to_string().replace("-", "")
    ))
        .with(Color::Rgb {
            r: 0,
            g: 135,
            b: 255,
        })
        .to_string();

    let styled_short_id = format!("{}", &project_id[..4].with(Color::Green));
    let styled_project_id = format!("{}", &project_id.with(Color::Green));

    expander
        .set("description", &description)
        .set("project-link", &project_link)
        .set("project-id", &styled_project_id)
        .set("short-project-id", &styled_short_id);

    let mut skin = MadSkin::default();
    skin.headers[4].set_fg(Color::Green);
    skin.print_expander(expander);

    let sort_theme = ColorfulTheme {
        active_item_style: Style::new(),
        picked_item_prefix: dialoguer::console::style("".to_string()),
        unpicked_item_prefix: dialoguer::console::style("".to_string()),
        prompt_prefix: dialoguer::console::style("".to_string()),
        ..ColorfulTheme::default()
    };

    if let Some(token_value) = project_response.token_value {
        match Sort::with_theme(&sort_theme)
            .item(format!(
                "{} '{}'. {}",
                "Your token:",
                style(format!("{}", &token_value)).with(Color::Green),
                "Copy this token and press 'Enter' to remove it from the screen."
            ))
            .clear(true)
            .report(false)
            .interact()
        {
            Ok(confirmation) => confirmation,

            Err(_) => {
                print_formatted_error(
                    &format!("Creation failed. Failed to display information about the created project with the name '{}'.", &project_name),
                );

                std::process::exit(1);
            }
        };
    }
}
