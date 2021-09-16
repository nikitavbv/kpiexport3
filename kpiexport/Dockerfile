FROM frolvlad/alpine-glibc:glibc-2.33
WORKDIR /app

COPY target/release/kpiexport /app/app
RUN chmod +x /app/app

ENTRYPOINT [ "/app/app" ]
