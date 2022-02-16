FROM alpine
# create a group 'app'
RUN addgroup -S app
# create a user 'app' and assign it to the group 'add'
RUN adduser -G app -S app
# set ownership to 'app:app' to the workdir '/app'
RUN mkdir /app && chown -R app:app /app
USER app
WORKDIR /app
COPY target/x86_64-unknown-linux-musl/release/permutation_web_server .
CMD ["./permutation_web_server"]