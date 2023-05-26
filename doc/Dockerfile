FROM rust:1.64

ARG mdbook_dir=/home/mdbook
ARG mdbook_tgz=mdbook-v0.4.21-x86_64-unknown-linux-gnu.tar.gz
ARG mdbook_url=https://github.com/rust-lang/mdBook/releases/download/v0.4.21/${mdbook_tgz}

WORKDIR ${mdbook_dir}
RUN curl -L ${mdbook_url} -o ${mdbook_tgz} && \
    tar xf ${mdbook_tgz} && \
    chmod +x ./mdbook && \
    mv ./mdbook /usr/local/bin/mdbook && \
    mdbook init --theme && \
    rm ${mdbook_tgz}

RUN cargo install --git "https://github.com/lzanini/mdbook-katex"
RUN cargo install mdbook-linkcheck
RUN cargo install mdbook-toc

COPY book.toml .
COPY katex_macro.txt .
COPY src/ src/

EXPOSE 3000
