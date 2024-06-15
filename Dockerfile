FROM rust:1.77-bullseye AS builder
WORKDIR /usr/src/dimigomeal-back

COPY . .
RUN cargo build --release

FROM debian:bullseye

COPY --from=builder /usr/src/dimigomeal-back/target/release/dimigomeal-back /usr/local/bin/dimigomeal-back

RUN apt-get update && apt-get install -y cron
COPY cron /etc/cron.d/cron
RUN crontab /etc/cron.d/cron

COPY run.sh /run.sh

CMD ["sh", "run.sh"]