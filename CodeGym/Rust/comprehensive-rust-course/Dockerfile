FROM rust:1.67-bullseye

WORKDIR /usr/src/app

COPY comprehensive-rust-course/ .

RUN cargo install --path .

RUN echo "exercise" >> start-main.sh

CMD sh ./start-main.sh
