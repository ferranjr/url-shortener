# Url Shortener
This is a toy project to experiment with RUST, in this case working directly with
[hyper.rs](https://hyper.rs/) to build a small rest API to get short urls.

As storage I am using MongoDB

# Mongo DB
Provided in the docker compose, mongo DB init will create the user, collection and indexes required by the app.
You can run only mongodb from the docker compose and then run from IDE the rust app, or start everything if you only
want to play with the setup.

## Running only MongoDB
```shell
docker compose up -d mongodb
```

## Running all of it
```shell
docker compose up -d
```


TODO: 
1. Testing, been playing around many new things and follow many examples, so testing and refactoring is next goal
2. Metrics, adding metrics and bringing a grafana/prometheus set up for experimentation
3. Bring some load testing to verify metrics, goose set up from my rocket tests can be a starting point
