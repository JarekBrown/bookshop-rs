pub fn log_init() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or_else(|error| {
        println!(
            "ERROR: problem encountered when initializing logger -- {}",
            error
        )
    });
}
