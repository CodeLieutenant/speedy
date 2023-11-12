FROM rust:1.73.0-alpine as build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:3.18 as production

WORKDIR /app

COPY --from=build /app/target/release/speedy ./speedy
COPY ./config.example.timetable /etc/speedy/config.timetable

RUN apk update && apk add tini

ENTRYPOINT [ "tini" ]

CMD ["speedy", "serve", "--timetable", "/etc/speedy/config.timetable"]