# FROM rust:latest

# ARG DEBIAN_FRONTEND=noninteractive

# ENV PATH="/root/.cargo/bin:${PATH}"
# RUN rustup default nightly-2024-03-09
# RUN rustup target add wasm32-unknown-unknown
# RUN cargo install --locked trunk
# RUN cargo install wasm-opt --locked

# WORKDIR /app

# # Copy the entire workspace
# COPY . .

# # Build for release
# RUN trunk build --release

# Final stage: use a lightweight image
FROM nginx:stable-alpine-perl
ARG DEBIAN_FRONTEND=noninteractive

# # Copy the build artifact from the build stage
# COPY --from=0 /app/dist /usr/share/nginx/html
RUN ls
COPY dist /usr/share/nginx/html

# # Copy the nginx configuration
COPY ./nginx.conf /etc/nginx/conf.d/default.conf

# # Expose the port
EXPOSE 8080

