#![allow(clippy::all, warnings)]
pub struct ProjectSecrets;
pub mod project_secrets {
    #![allow(dead_code)]
    use std::fmt;
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ProjectSecrets";
    pub const QUERY : & str = "query ProjectSecrets($id: uuid!) {\n  token_by_pk(id: $id) {\n    id\n    name\n\n    userSecret(limit: 1000) {\n      id\n      name\n      updatedAt\n      vendor\n    }\n  }\n}\n" ;
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
    impl fmt::Display for vendorEnum_enum {
        fn fmt(&self, value: &mut fmt::Formatter) -> fmt::Result {
            match self {
                vendorEnum_enum::agora => write!(value, "Agora"),
                vendorEnum_enum::aws => write!(value, "AWS"),
                vendorEnum_enum::azure => write!(value, "Azure"),
                vendorEnum_enum::braintree => write!(value, "Braintree"),
                vendorEnum_enum::digitalOcean => write!(value, "DigitalOcean"),
                vendorEnum_enum::googleCloud => write!(value, "GoogleCloud"),
                vendorEnum_enum::mailchimp => write!(value, "Mailchimp"),
                vendorEnum_enum::mixpanel => write!(value, "Mixpanel"),
                vendorEnum_enum::other => write!(value, "Other"),
                vendorEnum_enum::paypal => write!(value, "Paypal"),
                vendorEnum_enum::pulumi => write!(value, "Pulumi"),
                vendorEnum_enum::segment => write!(value, "Segment"),
                vendorEnum_enum::sendgrid => write!(value, "Sendgrid"),
                vendorEnum_enum::stripe => write!(value, "Stripe"),
                vendorEnum_enum::terraform => write!(value, "Terraform"),
                vendorEnum_enum::twilio => write!(value, "Twilio"),
                vendorEnum_enum::Other(other) => write!(value, "{}", other),
            }
        }
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
        pub token_by_pk: Option<ProjectSecretsTokenByPk>,
    }
    #[derive(Deserialize)]
    pub struct ProjectSecretsTokenByPk {
        pub id: uuid,
        pub name: String,
        #[serde(rename = "userSecret")]
        pub user_secret: Vec<ProjectSecretsTokenByPkUserSecret>,
    }
    #[derive(Deserialize)]
    pub struct ProjectSecretsTokenByPkUserSecret {
        pub id: uuid,
        pub name: String,
        #[serde(rename = "updatedAt")]
        pub updated_at: timestamptz,
        pub vendor: vendorEnum_enum,
    }
}
impl graphql_client::GraphQLQuery for ProjectSecrets {
    type Variables = project_secrets::Variables;
    type ResponseData = project_secrets::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: project_secrets::QUERY,
            operation_name: project_secrets::OPERATION_NAME,
        }
    }
}
