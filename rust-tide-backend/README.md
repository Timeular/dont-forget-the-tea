# rust-tide-backend

This is a backend implementation using [tide](https://github.com/http-rs/tide).

## TODOs

* Use stable rust as a Docker builder
* Add a data store (maybe just a global Concurrent Hash Map for now, initialized with lazy_static?)
* Implement CRUD for grocery lists
* Implement authentication
* Find a nice way to do input validation (missing fields / validation in general)
* Find a nicer way to do error handling (currently HandlerResponse wrapper), so ? can be used in handlers
* ..

## Setup and Run

You need the Rust stable 1.39+.

To start in development mode:

```bash
make dev run
```

To build:

```bash
make
```

To lint:

```bash
make lint
```

To style-check against rust-fmt:

```bash
make style-check
```

To run tests:

```bash
make test
```

### Docker

There is also a Dockfile, so you can build a container using:

```bash
docker build -t rust-tide-backend .
```

If you're wondering about the Dockerfile, in particular the `builder` - the point here is to create an empty cargo project, install all dependencies and THEN add our sources.

This helps with consecutive build times, as dependencies are only built once, not every time the source changes.
