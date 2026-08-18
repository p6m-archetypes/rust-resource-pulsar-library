pub mod settings;

use anyhow::Result;
use pulsar::{
    producer, Authentication, DeserializeMessage, Producer, SerializeMessage,
    SubType, TokioExecutor,
};
use pulsar::Pulsar;
use settings::MessagingSettings;

#[derive(Clone)]
pub struct MessagingClient {
    pulsar: Pulsar<TokioExecutor>,
    settings: MessagingSettings,
}

impl MessagingClient {
    pub async fn connect(settings: &MessagingSettings) -> Result<Self> {
        let mut builder = Pulsar::builder(&settings.broker_url, TokioExecutor);

        if let Some(token) = &settings.jwt_token {
            builder = builder.with_auth(Authentication {
                name: "token".into(),
                data: token.as_bytes().to_vec(),
            });
        }

        let pulsar = builder.build().await?;
        tracing::info!(
            broker = %settings.broker_url,
            topic = %settings.topic,
            access = %settings.access,
            "Pulsar client connected"
        );

        Ok(Self { pulsar, settings: settings.clone() })
    }

    pub fn topic(&self) -> &str {
        &self.settings.topic
    }

    pub fn access(&self) -> &str {
        &self.settings.access
    }

    pub fn is_producer(&self) -> bool {
        self.settings.access == "produce"
    }

    pub fn is_consumer(&self) -> bool {
        self.settings.access == "consume"
    }

    pub async fn producer<T: SerializeMessage + Sized>(
        &self,
    ) -> Result<Producer<TokioExecutor>> {
        let producer = self
            .pulsar
            .producer()
            .with_topic(&self.settings.topic)
            .with_options(producer::ProducerOptions::default())
            .build()
            .await?;
        Ok(producer)
    }

    pub async fn consumer<T: DeserializeMessage>(
        &self,
    ) -> Result<pulsar::Consumer<T, TokioExecutor>> {
        let subscription = self
            .settings
            .subscription_name
            .clone()
            .unwrap_or_else(|| format!("{}-sub", self.settings.topic.replace('/', "-")));

        let consumer = self
            .pulsar
            .consumer()
            .with_topic(&self.settings.topic)
            .with_subscription(&subscription)
            .with_subscription_type(SubType::Shared)
            .build()
            .await?;
        Ok(consumer)
    }

    pub fn inner(&self) -> &Pulsar<TokioExecutor> {
        &self.pulsar
    }
}
