FROM rust:1.51 as builder
WORKDIR /usr/src/wondwise
COPY . .
RUN cargo build --release
RUN strip target/release/wondwise

FROM ubuntu:20.04
COPY --from=builder /usr/src/wondwise/target/release/wondwise /usr/bin
COPY --from=builder /usr/src/wondwise/LICENSE /usr/share/licenses/wondwise/LICENSE
RUN echo "/usr/bin/wondwise" >> /etc/shells
CMD [ "wondwise" ]
