FROM rust:1.70-alpine

ARG mdbook_dir=/home
ARG mdbook_url=https://github.com/rust-lang/mdBook/releases/download/v0.4.30/mdbook-v0.4.30-x86_64-unknown-linux-musl.tar.gz
ARG mdbook_toc_url=https://github.com/badboy/mdbook-toc/releases/download/0.12.0/mdbook-toc-0.12.0-x86_64-unknown-linux-musl.tar.gz
ARG mdbook_katex_url=https://github.com/lzanini/mdbook-katex/releases/download/v0.5.3/mdbook-katex-v0.5.3-x86_64-unknown-linux-musl.tar.gz

WORKDIR ${mdbook_dir}
RUN wget ${mdbook_url} -O mdbook.tar.gz && \
    tar xf mdbook.tar.gz && \
    chmod +x ./mdbook && \
    mv ./mdbook /usr/local/bin/mdbook && \
    rm mdbook.tar.gz
RUN wget ${mdbook_toc_url} -O mdbook-toc.tar.gz && \
    tar xf mdbook-toc.tar.gz && \
    chmod +x ./mdbook-toc && \
    mv ./mdbook-toc /usr/local/bin/mdbook-toc && \
    rm mdbook-toc.tar.gz
RUN wget ${mdbook_katex_url} -O mdbook-katex.tar.gz && \
    tar xf mdbook-katex.tar.gz && \
    chmod +x ./mdbook-katex && \
    mv ./mdbook-katex /usr/local/bin/mdbook-katex && \
    rm mdbook-katex.tar.gz

COPY book.toml .
COPY katex_macro.txt .
COPY src/ src/
COPY samples/ samples/
COPY theme/ theme/

EXPOSE 3000
