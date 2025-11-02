FROM alpine:3.22

ENV HOST 0.0.0.0
ENV PORT 8080
EXPOSE 8080

# Create a non-root user
RUN addgroup -g 1001 rblog && \
    adduser -D -u 1001 -G rblog rblog

COPY ./rblog /usr/bin/rblog
ADD ./data /data

# Change ownership of data directory to non-root user
RUN chown -R rblog:rblog /data

# Switch to non-root user
USER rblog

CMD ["/usr/bin/rblog", "run", "-c", "/data/blog.yaml"]
