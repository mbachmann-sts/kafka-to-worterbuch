use rdkafka::{
    consumer::{ConsumerContext, Rebalance, StreamConsumer},
    error::KafkaResult,
    ClientContext, TopicPartitionList,
};
use tokio_graceful_shutdown::SubsystemHandle;

pub struct K2WbContext {
    subsys: SubsystemHandle,
}

impl ClientContext for K2WbContext {}

impl K2WbContext {
    pub fn new(subsys: SubsystemHandle) -> Self {
        K2WbContext { subsys }
    }
}

impl ConsumerContext for K2WbContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        match rebalance {
            Rebalance::Assign(ass) => {
                log::info!(
                    "Starting rebalance; assigned: {:?}",
                    ass.to_topic_map()
                        .keys()
                        .map(|(topic, part)| format!("{topic}-{part}"))
                        .collect::<Vec<String>>()
                )
            }
            Rebalance::Revoke(rev) => {
                log::info!(
                    "Starting rebalance; assignment revoked: {:?}",
                    rev.to_topic_map()
                        .keys()
                        .map(|(topic, part)| format!("{topic}-{part}"))
                        .collect::<Vec<String>>()
                )
            }
            Rebalance::Error(err) => {
                log::error!("Rebalance error: {err}");
                self.subsys.request_global_shutdown();
            }
        }
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        match rebalance {
            Rebalance::Assign(ass) => {
                log::info!(
                    "Rebalance complete; assigned: {:?}",
                    ass.to_topic_map()
                        .keys()
                        .map(|(topic, part)| format!("{topic}-{part}"))
                        .collect::<Vec<String>>()
                );
            }
            Rebalance::Revoke(rev) => {
                log::info!(
                    "Rebalance complete; assignment revoked: {:?}",
                    rev.to_topic_map()
                        .keys()
                        .map(|(topic, part)| format!("{topic}-{part}"))
                        .collect::<Vec<String>>()
                )
            }
            Rebalance::Error(err) => {
                log::error!("Rebalance error: {err}");
                self.subsys.request_global_shutdown();
            }
        }
    }

    fn commit_callback(&self, result: KafkaResult<()>, offsets: &TopicPartitionList) {
        match result {
            Ok(_) => log::debug!(
                "Offsets committed: {:?}",
                offsets
                    .to_topic_map()
                    .keys()
                    .map(|(topic, part)| format!("{topic}-{part}"))
                    .collect::<Vec<String>>()
            ),
            Err(e) => {
                log::error!("Error committing offsets: {e}");
                self.subsys.request_global_shutdown();
            }
        }
    }
}

pub type K2WbConsumer = StreamConsumer<K2WbContext>;
