FROM rust:1.67

WORKDIR /usr/src/app
COPY . .

# Add the required environment variables
ENV target_database_name=
ENV source_database_name=
ENV source_connection_string = "";
ENV target_connection_string = "";

RUN cargo install --path .

CMD ["cargo" , "run"]


