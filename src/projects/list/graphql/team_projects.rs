#![allow(clippy::all, warnings)]
pub struct TeamProjects;
pub mod team_projects {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "TeamProjects";
    pub const QUERY : & str = "query TeamProjects($id: uuid!, $userId: uuid!) {\n  project(where: {_and: [\n    {teams: {team: {id: {_eq: $id}}}},\n    {teams: {team: {members: {member: {id: {_eq: $userId}}}}}}\n  ]}, limit: 1000) {\n    id\n    name\n\n    owner {\n      name\n      email\n    }\n\n    usageHistories(order_by: {updatedAt: desc}, limit: 1) {\n      updatedAt\n    }\n  }\n}\n" ;
    use chrono::offset::Utc;
    use chrono::DateTime;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type timestamptz = DateTime<Utc>;
    type uuid = ::uuid::Uuid;
    #[derive(Serialize)]
    pub struct Variables {
        pub id: uuid,
        #[serde(rename = "userId")]
        pub user_id: uuid,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub project: Vec<TeamProjectsProject>,
    }
    #[derive(Deserialize)]
    pub struct TeamProjectsProject {
        pub id: uuid,
        pub name: String,
        pub owner: TeamProjectsProjectOwner,
        #[serde(rename = "usageHistories")]
        pub usage_histories: Vec<TeamProjectsProjectUsageHistories>,
    }
    #[derive(Deserialize)]
    pub struct TeamProjectsProjectOwner {
        pub name: String,
        pub email: String,
    }
    #[derive(Deserialize)]
    pub struct TeamProjectsProjectUsageHistories {
        #[serde(rename = "updatedAt")]
        pub updated_at: timestamptz,
    }
}
impl graphql_client::GraphQLQuery for TeamProjects {
    type Variables = team_projects::Variables;
    type ResponseData = team_projects::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: team_projects::QUERY,
            operation_name: team_projects::OPERATION_NAME,
        }
    }
}
