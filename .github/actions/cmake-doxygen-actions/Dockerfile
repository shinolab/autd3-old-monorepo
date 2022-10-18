FROM alpine:3.16.2
RUN apk add git cmake make gcc g++ libc-dev linux-headers doxygen graphviz ttf-freefont
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT [ "/entrypoint.sh" ]