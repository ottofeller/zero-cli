#![allow(clippy::all, warnings)]
pub struct RemoveTeam;
pub mod remove_team {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "RemoveTeam";
    pub const QUERY : & str = "mutation RemoveTeam($teamId: String!, $teamMemberIds: [String!]!, $teamName: String!) {\n  removeTeam(object: {teamId: $teamId, teamName: $teamName, teamMemberIds: $teamMemberIds}) {\n    success\n  }\n}\n" ;
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
        #[serde(rename = "teamId")]
        pub team_id: String,
        #[serde(rename = "teamMemberIds")]
        pub team_member_ids: Vec<String>,
        #[serde(rename = "teamName")]
        pub team_name: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "removeTeam")]
        pub remove_team: RemoveTeamRemoveTeam,
    }
    #[derive(Deserialize)]
    pub struct RemoveTeamRemoveTeam {
        pub success: Boolean,
    }
}
impl graphql_client::GraphQLQuery for RemoveTeam {
    type Variables = remove_team::Variables;
    type ResponseData = remove_team::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: remove_team::QUERY,
            operation_name: remove_team::OPERATION_NAME,
        }
    }
}
