use wayland_submitter::*;

fn main() {
    pretty_env_logger::init();

    let content_connector = ContentConnector::default();
    let ui_connector = UIConnector::default();
    let mut submitter = Submitter::new(ui_connector, content_connector);
    submitter.submit_text("submission");
}
