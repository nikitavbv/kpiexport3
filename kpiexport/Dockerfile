FROM frolvlad/alpine-glibc:alpine-3.13_glibc-2.33
WORKDIR /app

COPY target/release/kpiexport /app/app

ENTRYPOINT [ "/app/app" ]
