# bookshop-rs

A simple book store API in need of input validation/sanitization.

This is a part of the University of Wyoming's Secure Software Design Course (Spring 2023). This is the base repository to be forked and updated for various assignments. Alternative language versions are available in:

- [Go](https://github.com/andey-robins/bookshop-go)
- [Javascript](https://github.com/andey-robins/bookshop-js)

## Versioning

`bookshop-rs` is built with:

- cargo 1.70.0-nightly (15d090969 2023-03-21)
- rust edition 2021

## Usage

Start the api using `cargo run`

I recommend using [`httpie`](https://httpie.io) for testing of HTTP endpoints on the terminal. Tutorials are available elsewhere online, and you're free to use whatever tools you deem appropriate for testing your code.

## Initial Assessment

Prior to any alterations to the code base, I will describe the areas that I believe can be improved.

### Logging

There is no logging functionality implemented. This is a relatively easy fix to make, and can improve development and production experiences; logs add auditability to the service and makes debugging significantly easier.

### Field Validation

When information is supplied to the API, there should be some checks to ensure that the input is 'valid'. For example, if I am looking for the price of a book, the response should be a *positive number*. So when a book is being added, a negative price should be rejected.

Thankfully this functionality can be fixed by adding in validation functions as needed, such as in handlers or database connection files.

### SQL Injection

Through my testing, I did not have a successful injection through database queries. This is due to the database files using the `execute()` function from the `Connection` module. This combines the 'prepare statement' functionality and the corresponding 'execute' from the `Statement` module of `rusqlite`. By using the `params[]` syntax of the execute function also helps prevent injections.

### Database

There are a couple problems found within the database logic. The most obvious being able to create duplicate book entries (i.e. I was able to add Dune by Frank Herbert into the database, even though it already existed there). There is also no check on the amount of spacing between words, so there should be a removal of extra spaces to minimize the chance of overflows.  

Entries were also case-sensitive, which can cause problems for GET requests; changing the case of one letter in a title can cause a 500 error. The strings should be standardized before the query is sent to the database.

### Error Handling

This is a common issue throughout the majority of the files. Error handling should be double-checked once initial prototyping is done. This can help prevent confusing error statements, but also can be a way to prevent the errors from propagating to the rest of your system. This should be paired with a logger, to ensure that any errors are noted; this allows developers to fix problems faster as they will know the context of an error.

## Alterations/Improvements

### Disclaimer

No additional functionality was added to the program. For example, while having the option to update the price of a book could be useful, this was not added.

### Assumptions

The following are a few assumptions that were made during the alteration process.

First, it is assumed that some other service will be validating/standardizing address entered into the database. This can be difficult to do without downloading several gigabytes of data for a geocoder to do its job. There are API options, such as one through Google, that can help assist this process.

Second, it is assumed that access to the database is strictly monitored. There are currently no protections in place to prevent alteration of the database while at rest. The ability to run the SQLite executable should be heavily restricted. A possible solution to this would be using the encryption extension available for SQLite, or transitioning over to other database options; alternatives like PostgreSQL should be seriously considered, since it handles security management and allows for significantly better performance when dealing with concurrent connections.

### Logging

I used [log4rs](https://docs.rs/log4rs/1.2.0/log4rs/) to provide the logging functionality. The logger is configured using the corresponding [yaml file](./log4rs.yaml), and stores its logs in the [logs folder](./log/). There are three levels of logs (info, warn, error) which are separated out into separate files. This allows the 'warn' and 'error' level logs to be separated from the 'info' logs, making it easier to see the bugs/issues.

### Validation

Each handler and database file has their own validators. These are customized to match the required type/format for each use case. 

The validation functions within the handlers do the following:

- standardized input
  - remove any excess spacing
  - check that only alphanumeric values are accepted
  - convert strings to 'title case', using the [titlecase crate](https://docs.rs/titlecase/2.2.1/titlecase/fn.titlecase.html)
- make sure that non-negative numbers are used for the prices
- return, and log, errors that arise

The validation functions for the database files do the following few things:

- check if the entered book exists
  - prevents duplicate books
  - prevents errors coming from nonexistent entries during GET requests
- confirms that cid/bid/poid are valid (actually found within database)

The database validators primarily use the `exists()` function provided by [rusqlite](https://docs.rs/rusqlite/0.29.0/rusqlite/). Since these validators check any input before trying to query the database directly, there is less of a need to do heavy validation in the handler files.

### Error Handling

The majority of functions were changed to return `Result` data types. This allows for more consistent behavior in a system like this one; with more error reporting present, the calling function can decide better what to do. This also prevents having to do error handling in every single function for every single error, therefore the current form has the calling function handle some errors encountered by the called function.

The use of `unwrap()` was avoided throughout the crate. This functionality is great when prototyping, but can lead to some very confusing bugs later. Most errors handling sections instead are written to the respective log file, while a few others were changed to use `expect()` instead.

### Final Touches

The code of the entire crate was formatting using [cargo fmt](https://github.com/rust-lang/rustfmt). [Clippy](https://github.com/rust-lang/rust-clippy) was used to catch minor mistakes and to make small fixes its linters were able to find/fix.
