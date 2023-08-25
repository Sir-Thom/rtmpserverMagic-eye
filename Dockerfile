# Use a base image with Rust and Cargo pre-installed
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy your Rust project files to the container
COPY . .

# Build your Rust application
RUN cargo build --release
#CMD [ "" ]
# Specify the command to run when the container starts
CMD ["target/release/rtmp_server_magic_eye"]