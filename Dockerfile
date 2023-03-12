FROM rust:1.67
WORKDIR /usr/src/afterburner
COPY . .
RUN cargo install --path .
CMD ["bash"]
