#![allow(clippy::all, warnings)]
pub struct UsageDetails;
pub mod usage_details {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UsageDetails";
    pub const QUERY : & str = "query UsageDetails($id: uuid!) {\n  usageHistory_by_pk(id: $id) {\n    id\n    callerName\n    createdAt\n    remoteIp\n\n    token {\n      id\n      name\n    }\n\n    secrets {\n      id\n\n      userSecret {\n        id\n        vendor\n      }\n    }\n  }\n}\n" ;
    use ::uuid::Uuid;
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
    type inet = String;
    type timestamptz = DateTime<Utc>;
    type uuid = Uuid;
    #[derive()]
    pub enum vendorEnum_enum {
        agora,
        aws,
        azure,
        braintree,
        digitalOcean,
        googleCloud,
        mailchimp,
        mixpanel,
        other,
        paypal,
        pulumi,
        segment,
        sendgrid,
        stripe,
        terraform,
        twilio,
        Other(String),
    }
    impl ::serde::Serialize for vendorEnum_enum {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                vendorEnum_enum::agora => "agora",
                vendorEnum_enum::aws => "aws",
                vendorEnum_enum::azure => "azure",
                vendorEnum_enum::braintree => "braintree",
                vendorEnum_enum::digitalOcean => "digitalOcean",
                vendorEnum_enum::googleCloud => "googleCloud",
                vendorEnum_enum::mailchimp => "mailchimp",
                vendorEnum_enum::mixpanel => "mixpanel",
                vendorEnum_enum::other => "other",
                vendorEnum_enum::paypal => "paypal",
                vendorEnum_enum::pulumi => "pulumi",
                vendorEnum_enum::segment => "segment",
                vendorEnum_enum::sendgrid => "sendgrid",
                vendorEnum_enum::stripe => "stripe",
                vendorEnum_enum::terraform => "terraform",
                vendorEnum_enum::twilio => "twilio",
                vendorEnum_enum::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for vendorEnum_enum {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "agora" => Ok(vendorEnum_enum::agora),
                "aws" => Ok(vendorEnum_enum::aws),
                "azure" => Ok(vendorEnum_enum::azure),
                "braintree" => Ok(vendorEnum_enum::braintree),
                "digitalOcean" => Ok(vendorEnum_enum::digitalOcean),
                "googleCloud" => Ok(vendorEnum_enum::googleCloud),
                "mailchimp" => Ok(vendorEnum_enum::mailchimp),
                "mixpanel" => Ok(vendorEnum_enum::mixpanel),
                "other" => Ok(vendorEnum_enum::other),
                "paypal" => Ok(vendorEnum_enum::paypal),
                "pulumi" => Ok(vendorEnum_enum::pulumi),
                "segment" => Ok(vendorEnum_enum::segment),
                "sendgrid" => Ok(vendorEnum_enum::sendgrid),
                "stripe" => Ok(vendorEnum_enum::stripe),
                "terraform" => Ok(vendorEnum_enum::terraform),
                "twilio" => Ok(vendorEnum_enum::twilio),
                _ => Ok(vendorEnum_enum::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub id: uuid,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "usageHistory_by_pk")]
        pub usage_history_by_pk: Option<UsageDetailsUsageHistoryByPk>,
    }
    #[derive(Deserialize)]
    pub struct UsageDetailsUsageHistoryByPk {
        pub id: uuid,
        #[serde(rename = "callerName")]
        pub caller_name: Option<String>,
        #[serde(rename = "createdAt")]
        pub created_at: timestamptz,
        #[serde(rename = "remoteIp")]
        pub remote_ip: inet,
        pub token: UsageDetailsUsageHistoryByPkToken,
        pub secrets: Vec<UsageDetailsUsageHistoryByPkSecrets>,
    }
    #[derive(Deserialize)]
    pub struct UsageDetailsUsageHistoryByPkToken {
        pub id: uuid,
        pub name: String,
    }
    #[derive(Deserialize)]
    pub struct UsageDetailsUsageHistoryByPkSecrets {
        pub id: uuid,
        #[serde(rename = "userSecret")]
        pub user_secret: Option<UsageDetailsUsageHistoryByPkSecretsUserSecret>,
    }
    #[derive(Deserialize)]
    pub struct UsageDetailsUsageHistoryByPkSecretsUserSecret {
        pub id: uuid,
        pub vendor: vendorEnum_enum,
    }
}
impl graphql_client::GraphQLQuery for UsageDetails {
    type Variables = usage_details::Variables;
    type ResponseData = usage_details::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: usage_details::QUERY,
            operation_name: usage_details::OPERATION_NAME,
        }
    }
}
