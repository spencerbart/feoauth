# Feo Auth

![feo auth logo](./miscellaneous/feoauth-logo.svg)

Feo Auth is an open-source Auth0 alternative written in Rust.

# Roadmap

**This project is not ready yet but we will update this readme when it is ready.**

## Our goal

Our goal is to create an open-source Auth0 alternative. This project was born out of a need for a fully featured authentication solution that was extensible and modular.

Our path to monetization is similar to Meilisearch, Supabase, and others. We plan on offering the product for free but offer a paid hosting solution for ease of use.

We believe in the power of open source. It allows its users to create the features that they need, with the ease of use of a proprietary solution but can be taken in-house once their project hits major scale.

# Setup

Three major things need to happen to setup Feo Auth for production use.

1. Setup the database
2. Setup the server
3. Setup the client

## Setup the database

Feo Auth uses PostgreSQL as its database. There are many options for a hosted Postgres instance such as:

1. [AWS RDS](https://aws.amazon.com/rds/) (enterprise-grade level)
2. [Neon](https://neon.tech/) (serverless Postgres)
3. [DigitalOcean](https://www.digitalocean.com/products/managed-databases-postgresql) (good in-between)
4. [Railway](https://docs.railway.app/databases/postgresql) (simple to setup)

There are many other alternatives, but in the end, you will need a Postgres URI in the format of `postgres://postgres:postgres@localhost:5432/postgres`. You will add that as an env variable to `DATABASE_URL`.

## Setup the server

All the server code is contained in the file `feoauth`. You can build from source by cloning this repo or you can download one of the release binaries.

There are many environment variables that you will need to add to properly configure your server. You can look at `.env.example` to view the environment variables that you will need.

You can then create an OCI-compliant image and deploy it to your platform of choice. There are many options to choose from such as:

1. [AWS ECS on Fargate](https://aws.amazon.com/ecs/) (enterprise-grade level)
2. [Railway](https://railway.app/) (simple to setup)
3. [DigitalOcean's App Platform](https://www.digitalocean.com/products/app-platform) (good in-between)

## Setup the client

Coming soon...