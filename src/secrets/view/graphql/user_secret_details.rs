#![allow(clippy::all, warnings)]
pub struct UserSecretDetails;
pub mod user_secret_details {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UserSecretDetails";
    pub const QUERY : & str = "query UserSecretDetails($id: uuid!) {\n   userSecret_by_pk(id: $id) {\n    id\n    name\n    vendor\n    updatedAt\n    tokenId\n\n    fields {\n      id\n      name\n    }\n  }\n}\n" ;
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
        #[serde(rename = "userSecret_by_pk")]
        pub user_secret_by_pk: Option<UserSecretDetailsUserSecretByPk>,
    }
    #[derive(Deserialize)]
    pub struct UserSecretDetailsUserSecretByPk {
        pub id: uuid,
        pub name: String,
        pub vendor: vendorEnum_enum,
        #[serde(rename = "updatedAt")]
        pub updated_at: timestamptz,
        #[serde(rename = "tokenId")]
        pub token_id: uuid,
        pub fields: Vec<UserSecretDetailsUserSecretByPkFields>,
    }
    #[derive(Deserialize)]
    pub struct UserSecretDetailsUserSecretByPkFields {
        pub id: uuid,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for UserSecretDetails {
    type Variables = user_secret_details::Variables;
    type ResponseData = user_secret_details::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: user_secret_details::QUERY,
            operation_name: user_secret_details::OPERATION_NAME,
        }
    }
}
