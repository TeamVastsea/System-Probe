use crate::monitor::refresh::refresh;
use crate::broadcast::fire::fire;

pub async fn query() {
    loop {
        let data = refresh();
        fire(data).await;

        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}