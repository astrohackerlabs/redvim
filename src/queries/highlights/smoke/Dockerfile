# syntax=docker/dockerfile:1.7
# café FROM is a comment
ARG BASE=alpine:3.21
FROM --platform=linux/arm64 ${BASE} AS builder
ARG MODE=release
ENV TITLE="hello ${MODE}" lower='value'
ENV AGAIN=$lower
LABEL org.example.name="café"
WORKDIR /workspace
USER nobody
RUN --mount=type=cache,target=/cache echo "$TITLE" \
  && echo continued
COPY --chown=1000:1000 . /workspace
ADD --checksum=sha256:abc https://example.com/file /file
EXPOSE 8080/tcp
SHELL ["python3", "-c"]
RUN print('host language only')
ONBUILD COPY . /child
HEALTHCHECK --interval=30s CMD true
VOLUME ["/data"]
STOPSIGNAL SIGTERM
RUN <<EOF
echo heredoc café
EOF
COPY <<'END' /message
literal heredoc
END
from alpine:3.21 as final
COPY --from=builder /workspace /app
ENV AFTER="after heredoc"
ENTRYPOINT ["/app/start"]
CMD ["--help", "escaped\nvalue"]
