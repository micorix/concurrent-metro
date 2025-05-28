FROM rust:1.86-bookworm

ENV SOURCE=/opt/metro

ENV PNPM_VERSION=10.10.0
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

WORKDIR $SOURCE

# Install additional packages
# https://tauri.app/start/prerequisites/#linux
RUN apt-get update && apt-get install -y --no-install-recommends \
  trash-cli \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  at-spi2-core \
  libatspi2.0-0 \
  libcanberra-gtk-module \
  libcanberra-gtk3-module \
  libgtk-3-dev \
  x11-apps

RUN apt-get install -y --no-install-recommends xdg-utils

# Installation via corepack has its issues
RUN wget -qO- https://get.pnpm.io/install.sh | ENV="$HOME/.shrc" SHELL="$(which sh)" sh -

RUN pnpm env use --global lts

# Not needed after all, kept for future ref
#RUN  groupadd -g 109 render &&\
#   usermod -aG render,video developer &&\
#   mkdir -p /run/user/1000/dconf &&  chown -R 1000:1000 /run/user/1000

COPY package.json pnpm-lock.yaml ./

RUN pnpm install --frozen-lockfile

COPY src src
COPY src-tauri src-tauri
COPY public public

COPY index.html tsconfig.json tsconfig.node.json vite.config.ts ./

CMD ["pnpm", "run", "tauri", "dev"]