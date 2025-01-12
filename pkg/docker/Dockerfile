FROM alpine:3.21

ENV HOST 0.0.0.0
ENV PORT 8080
EXPOSE 8080

COPY ./rblog /usr/bin/rblog
ADD ./data /data

CMD ["/usr/bin/rblog", "run", "-c", "/data/blog.yaml"]
