FROM public.ecr.aws/docker/library/rust:1.77-bullseye AS builder
WORKDIR /usr/src/dimigomeal-back

COPY . .
RUN cargo build --release

FROM public.ecr.aws/debian/debian:bullseye
ENV TZ="Asia/Seoul"

COPY --from=builder /usr/src/dimigomeal-back/target/release/dimigomeal-back /usr/local/bin/dimigomeal-back

RUN apt-get update && apt-get install -y cron
COPY cron /etc/cron.d/cron
RUN crontab /etc/cron.d/cron

COPY run.sh /run.sh

CMD ["sh", "run.sh"]

LABEL org.opencontainers.image.source=https://github.com/dimigomeal/dimigomeal-back