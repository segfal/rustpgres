#image will be postgres:latest
FROM postgres:latest
ENV POSTGRES_PASSWORD=postgres
ENV POSTGRES_USER=postgres
ENV POSTGRES_DB=userdb
RUN apt update -y && apt upgrade -y
RUN apt install curl wget build-essential -y
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN apt install python3 python3-pip -y
RUN apt install libpq-dev -y
RUN pip3 install psycopg2-binary
COPY ./backend /backend


EXPOSE 5432