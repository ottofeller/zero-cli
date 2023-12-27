#![allow(clippy::all, warnings)]
pub struct CreateTeam;
pub mod create_team {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "CreateTeam";
    pub const QUERY : & str = "mutation CreateTeam($teamName: String!) {\n    createTeam(object: {teamName: $teamName}) {\n        teamId\n    }\n}\n" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "teamName")]
        pub team_name: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "createTeam")]
        pub create_team: CreateTeamCreateTeam,
    }
    #[derive(Deserialize)]
    pub struct CreateTeamCreateTeam {
        #[serde(rename = "teamId")]
        pub team_id: String,
    }
}
impl graphql_client::GraphQLQuery for CreateTeam {
    type Variables = create_team::Variables;
    type ResponseData = create_team::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: create_team::QUERY,
            operation_name: create_team::OPERATION_NAME,
        }
    }
}
