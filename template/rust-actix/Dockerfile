FROM openfaas/of-watchdog:0.7.2 as watchdog

FROM rust:1.59-alpine3.14 as builder

RUN USER=root

# Copy all the sources
COPY function /home/rust/function
COPY main /home/rust/main

WORKDIR /home/rust/main

RUN cargo build --release

FROM alpine:3.14 as runner

# Install packages and add non-root user
RUN apk --no-cache add curl ca-certificates \
    && addgroup -S app && adduser -S -g app app

ENV USER=app

# Copy of-watchdog binary
COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
RUN chmod +x /usr/bin/fwatchdog

COPY --from=builder /home/rust/main/target/release/main /usr/bin/main

# Set up watchdog for HTTP mode
ENV fprocess="main"
ENV mode="http"
ENV upstream_url="http://127.0.0.1:3000"

HEALTHCHECK --interval=3s CMD [ -e /tmp/.lock ] || exit 1

CMD ["fwatchdog"]

# TODO: CEHCK