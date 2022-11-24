use walle::{new_walle, walle_core::config::AppConfig, Matchers, MatchersConfig};
use walle_plugin_example::*;

#[tokio::main]
async fn main() {
    let matchers = Matchers::default()
    .add_matcher(prefix_matcher())
    .add_matcher(on_to_me())
    .add_matcher(prefix());
    let walle = new_walle(matchers);
    let joins = walle
        .start(AppConfig::default(), MatchersConfig::default(), true)
        .await
        .unwrap();
    for join in joins {
        join.await.ok();
    }
}
