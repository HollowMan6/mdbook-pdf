FROM rust AS builder
COPY . .
RUN apt update && apt install -y build-essential
# Install mdbook
RUN cargo install --git https://github.com/rust-lang/mdBook mdbook
RUN cargo install --path .

FROM python AS base
RUN apt-get update \
    && apt-get install -y \
        chromium fonts-wqy-microhei fonts-wqy-zenhei \
    && apt-get autoremove -y \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/* \
    && rm -rf /root/.cache \
    && mkdir /mdbook-pdf
COPY . /mdbook-pdf
RUN pip3 install --no-cache-dir /mdbook-pdf \
    && rm -rf /mdbook-pdf
COPY --from=builder /usr/local/cargo/bin/mdbook-pdf /usr/local/bin/mdbook-pdf
COPY --from=builder /usr/local/cargo/bin/mdbook /usr/local/bin/mdbook
WORKDIR /book

# Add other mdbook backend at /mdbook dir
ENV PATH="$PATH:/mdbook"

ENTRYPOINT [ "mdbook", "build" ]
