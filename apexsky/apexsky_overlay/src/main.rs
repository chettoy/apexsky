use obfstr::obfstr as s;
use tracing_appender::non_blocking::NonBlocking;

mod overlay;

fn main() {
    let (non_blocking, _guard) = tracing_appender::non_blocking(tracing_appender::rolling::daily(
        s!("log"),
        s!("overlay.log"),
    ));
    init_logger(non_blocking, true);

    println!("Hello, world!");
    overlay::main();
}

fn init_logger(non_blocking: NonBlocking, print: bool) {
    use tracing::Level;
    use tracing_subscriber::{fmt::writer::MakeWriterExt, layer::SubscriberExt, EnvFilter};

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(s!("apexsky_overlay=debug,apexsky=warn")))
        .unwrap();

    let formatting_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr.with_max_level(Level::DEBUG))
        //.with_span_events(FmtSpan::ACTIVE)
        .pretty();

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking.with_max_level(Level::TRACE))
        .with_ansi(false)
        .pretty();

    // let provider = TracerProvider::builder()
    //     .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
    //     .build();
    // let tracer = provider.tracer(s!("apexsky_dma").to_string());

    // let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    //let console_layer = console_subscriber::spawn();

    let subscriber = tracing_subscriber::Registry::default()
        //.with(console_layer)
        .with(filter_layer)
        .with(file_layer);
    //.with(telemetry)

    if print {
        tracing::subscriber::set_global_default(subscriber.with(formatting_layer))
    } else {
        tracing::subscriber::set_global_default(subscriber)
    }
    .expect(s!("setting default subscriber failed"));
}
