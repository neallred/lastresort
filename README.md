# lastresort

Your last resort for giving away free stuff before you throw it away.

## Why

I like Rust, I like building stuff, I like giving away stuff I build, and I like getting stuff for free. It's self-hosted because I think that's an underserved part of technology. How do we empower regular-to-somewhat-tech-savvy people to serve web content in a way that keeps them in control of the end-to-end experience?

## Developing

### DB

Run `./x dbup`. You need `docker`, `docker-compose`, and `sqlx-cli`.

### Back end

Database should be running, then run `cargo run`

### Front end

Change to `client` directory, then run `yarn start`
