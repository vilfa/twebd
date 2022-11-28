// TODO: Support implementing routes for handling different types of requests.
// TODO: Reuse connections / leave connections intact (maybe use `mio` for http also).
// TODO: Rework threading implementation with one master thread and multiple worker
// threads. This should be supported for both http and https.

fn main() {
    twebd::run();
}
