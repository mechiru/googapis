pub mod clients {
    #[cfg(any(feature = "google-clients-auth"))]
    pub mod auth {
        use async_trait::async_trait;

        #[async_trait]
        pub trait TokenProvider {
            async fn token(&self) -> String;
        }
    }

    pub mod firestore {
        pub mod v1 {
            #[cfg(any(feature = "google-firestore-v1-client"))]
            pub mod client {
                //! This module provides a client implementation for connecting to Firestore via authentication.
                //!
                //! ## Example
                //! ```toml
                //! [dependencies]
                //! googapis = { version = "0.6", features = ["google-firestore-v1-client"] }
                //! ```
                //!
                //! ```ignore
                //! use googapis::clients::{
                //!     auth::TokenProvider,
                //!     firestore::v1::client
                //! };
                //! use std::time::Duration;
                //!  
                //! #[tokio::main]
                //! async fn main() -> Result<(), Box<dyn std::error::Error>> {
                //!     let keep_alive = Duration::from_sec(15);
                //!     let client = client::get_client(token, Some(keep_alive)).await?
                //! }
                //! ```
                use crate::clients::auth::TokenProvider;
                use crate::google::firestore::v1::firestore_client::FirestoreClient;
                use std::time::Duration;
                use tonic::{
                    codegen::{InterceptedService, StdError},
                    metadata::MetadataValue,
                    service::Interceptor,
                    transport::{Channel, ClientTlsConfig, Endpoint},
                    Request,
                };

                /// Get a new Firestore client.
                pub async fn get_client<T, E>(
                    project_id: &str,
                    token_provider: T,
                    keep_alive: Option<Duration>,
                ) -> Result<FirestoreClient<InterceptedService<Channel, impl Interceptor>>, E>
                where
                    T: TokenProvider,
                    E: Into<StdError>
                        + std::convert::From<tonic::transport::Error>
                        + std::convert::From<tonic::metadata::errors::InvalidMetadataValue>,
                {
                    let domain = "firestore.googleapis.com";
                    let url = format!("https://{}/v1", domain);

                    let endpoint: Endpoint = Channel::from_shared(url)
                        .unwrap()
                        .tcp_keepalive(keep_alive)
                        .tls_config(ClientTlsConfig::new().domain_name(domain))?;

                    let channel: Channel = endpoint.connect().await?;
                    let authorization =
                        MetadataValue::from_str(token_provider.token().await.as_ref())?;
                    let prefix = MetadataValue::from_str(
                        format!("projects/{}/databases/(default)", project_id).as_ref(),
                    )?;

                    Ok(FirestoreClient::with_interceptor(
                        channel,
                        move |mut req: Request<()>| {
                            req.metadata_mut()
                                .insert("authorization", authorization.clone());
                            req.metadata_mut()
                                .insert("google-cloud-resource-prefix", prefix.clone());
                            Ok(req)
                        },
                    ))
                }
            }
        }
    }
}
