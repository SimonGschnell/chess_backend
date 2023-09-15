# espace=`

#ARGS are variables that can be used in the FROM instruction
ARG VERSION=alpine3.18
#the line above is a parser directive, parser directives have to be at the top of the dockerfile
FROM rust:1.67 AS builder
#* FROM rust:$VERSION AS build_stage => equivalent with the usage of ARGS

RUN apt update && apt-get install sqlite3 -y

#set the work-directory inside the container other than /
WORKDIR /usr/src
#copy only files required to install dependencies (not available without hack in rust)
#*better layer caching of the dockerfile
COPY Cargo.toml Cargo.lock /usr/
COPY src /usr/src/
COPY db /usr/db/
COPY .env /usr


#build the binary & move to /bin
#*adding --mount=type=cache to cache dependencies
WORKDIR /usr
RUN --mount=type=cache,target=/home/rust/.cargo/git \
    --mount=type=cache,sharing=private,target=/home/rust/src/target \
    cargo build --release && \
    mv target/release/chess_backend /bin


#copy to executable build stage
FROM gcr.io/distroless/cc-debian11
#FALLBACK - FROM ubuntu

COPY --from=builder /bin/ /usr/local/bin/
COPY --from=builder /usr/db/ /usr/local/bin/db/

EXPOSE 8080
#run the application with non privilaged user
#* root user UID is reserved on 0
#* system users UID range from 1 - 999
#* normal users UID range from 1000 onwards
#! NOT ABLE TO RUN THE BINARY AS A NON PRIVILEGED USER
#USER 1000

WORKDIR /usr/local/bin
ENTRYPOINT ["chess_backend"]
