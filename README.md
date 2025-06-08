# Back-end coding challenge

## Build tools & versions used
You will need `docker-compose` (version 2.36) and `cargo` (version 1.87) installed.

## Steps to run the app
First, to start the database, run:

```
docker compose up -d
```

Now, to run the application:

```
cargo run
```

You can access the endpoint at `localhost:3000/nodes`.

NOTE: the database server must be running even if you're just compiling the app,
because sqlx accesses it at compile-time.

## What was the reason for your focus? What problems were you trying to solve?

I focused mostly on the periodic database update from the external API. Because
the database could change significantly from one update to the next, depending
on the reponse of the API, I had to figure out a way to do the changes
atomically. Otherwise, if a request to `/nodes` was handled in the middle of an
update, the client could end up receiving a list of data mixed from two different
API calls.

## Did you make any trade-offs for this project? What would you have done differently with more time?

I had no time to add tests. If I had more time, I would have added tests
for the `get_response_from_node` and `convert_response_to_node` functions. If I
had a lot more time, I would also look into how to do mock tests with rust, so
I could test the database queries and polling of the API.

Secondly, the way the database is updated from the API is very simple, and might 
not scale very well. If the API response were bigger, I think doing everything 
in a single transaction would block requests from the clients for a long time. 
One way to fix it would be using a "shadow" table that is updated on the background 
and swapped with the main table when the update is done, but this would be more
complex.

Also, I commited the .env file. I know this is very bad in practice but I did
it because there's no secret information and this code is not going to be used
in real life anyway.

## What do you think is the weakest part of your project?

I think the weakest part is how the database is updated, as I explained before.
