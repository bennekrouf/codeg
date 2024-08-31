
use tracing::{Subscriber, Event};
use tracing_subscriber::{Layer, Registry};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::messenger_client::models::MessagingService;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

pub struct CustomLogLayer {
    messaging_service: Arc<MessagingService>,
}

impl<S> Layer<S> for CustomLogLayer
where
    S: Subscriber,
{
    fn on_event(&self, event: &Event, _ctx: Context<S>) {
        // Extract log metadata and format it
        let mut fields = String::new();
        let mut visitor = tracing_subscriber::fmt::format::JsonFields::new(&mut fields);
        event.record(&mut visitor);

        // Send the formatted log to the gRPC service
        let message = fields;
        let tags = Some(vec!["log".to_string()]); // Customize tags as needed

        tokio::spawn(async move {
            if let Err(e) = self.messaging_service.publish_message(message, tags).await {
                eprintln!("Failed to publish log message: {:?}", e);
            }
        });
    }
}

// pub async fn init_logging() {
//     // Initialize the gRPC client
//     let client = crate::messenger_client::connect_to_messenger_service().await;
//     let messaging_service = match client {
//         Some(client) => Arc::new(MessagingService::new(Arc::new(Mutex::new(client)), "log_tag".to_string())),
//         None => {
//             eprintln!("Failed to initialize MessagingService.");
//             return;
//         }
//     };
//
//     // Create and set up the custom logging layer
//     let custom_layer = CustomLogLayer {
//         messaging_service,
//     };
//
//     Registry::default()
//         .with(custom_layer)
//         .with(tracing_subscriber::fmt::layer())
//         .init();
//
//     // Optionally log an initialization message
//     tracing::info!("Custom logging initialized");
// }
