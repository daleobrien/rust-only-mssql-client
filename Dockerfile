# Linux + Wine Dockerfile — runs the pre-compiled Windows .exe via Wine 9.
#
# BUILD & RUN:
#   docker compose up --build --abort-on-container-exit

FROM ubuntu:24.04

# Prevent interactive prompts during package install
ENV DEBIAN_FRONTEND=noninteractive

# Install Wine 9 from Ubuntu repos and ca-certificates (for TLS)
RUN dpkg --add-architecture i386 && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
        wine wine32 wine64 \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set Wine to run without a display
ENV WINEDEBUG=-all
ENV DISPLAY=:0
ENV WINEPREFIX=/root/.wine

# Initialize Wine prefix (silently)
RUN wineboot --init 2>/dev/null || true

WORKDIR /app
COPY target.tmp/x86_64-pc-windows-gnu/release/ms-sql-query.exe .

ENTRYPOINT ["wine", "ms-sql-query.exe"]
