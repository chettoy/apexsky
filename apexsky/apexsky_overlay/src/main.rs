mod navigator;
mod overlay;
mod pb;

fn main() {
    #[cfg(unix)]
    if uzers::get_current_uid() == 0 {
        println!("{}", obfstr::obfstr!("Do NOT run it with root privileges!"));
        return;
    }

    //#[cfg(feature = "native")]
    let _logger_guard = init_logger();

    println!("Hello, world!");
    overlay::main();
}

#[cfg(feature = "native")]
fn init_logger() -> tracing_appender::non_blocking::WorkerGuard {
    use obfstr::obfstr as s;
    use once_cell::sync::Lazy;
    use std::path::PathBuf;
    use tracing::Level;
    use tracing_subscriber::{fmt::writer::MakeWriterExt, layer::SubscriberExt, EnvFilter};

    static DATA_DIR: Lazy<PathBuf> = Lazy::new(|| {
        if cfg!(unix)
            && std::env::current_exe()
                .is_ok_and(|exe| exe.starts_with(s!("/usr/bin")) || exe.starts_with(s!("/bin")))
        {
            homedir::my_home()
                .unwrap()
                .unwrap()
                .join(".local")
                .join("share")
                .join(obfstr::obfstr!("apexsky"))
        } else {
            std::env::current_dir().unwrap()
        }
    });

    let print = true;
    let (non_blocking, guard) = tracing_appender::non_blocking(tracing_appender::rolling::daily(
        DATA_DIR.join(s!("log")),
        s!("overlay.log"),
    ));

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

    guard
}

#[cfg(feature = "web-wasm")]
fn init_logger() {
    use obfstr::obfstr as s;
    use tracing_subscriber::fmt::format::Pretty;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::EnvFilter;
    use tracing_web::{performance_layer, MakeWebConsoleWriter};

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(s!("apexsky_overlay=debug,apexsky=warn")))
        .unwrap();

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false) // Only partially supported across browsers
        .without_time() // std::time is not available in browsers, see note below
        .with_writer(MakeWebConsoleWriter::new()); // write events to the console
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(perf_layer)
        .init(); // Install these as subscribers to tracing events
}
