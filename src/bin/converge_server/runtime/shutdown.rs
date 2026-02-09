pub(super) async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}
