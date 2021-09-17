FROM frolvlad/alpine-glibc:alpine-3.13_glibc-2.33
WORKDIR /app

COPY target/release/kpiexport /app
RUN mv /app/kpiexport /app/app
RUN chmod +x /app/app

ENTRYPOINT [ "/app/app" ]
