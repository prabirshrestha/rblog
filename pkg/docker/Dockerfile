FROM alpine:3.23

ENV HOST 0.0.0.0
ENV PORT 8080
EXPOSE 8080

RUN addgroup -g 1001 rblog && \
    adduser -D -u 1001 -G rblog rblog

COPY ./rblog /usr/bin/rblog
ADD --chown=rblog:rblog ./data /data

USER rblog

CMD ["/usr/bin/rblog", "run", "-c", "/data/blog.yaml"]
