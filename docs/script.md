# Rust Database Frameworks: SQLite and Postgres

## Diesel

Diesel is the framework I'm most familiar with. In Diesel, we define the structs for an insert and possibly an update, with attributes at the top. Those attributes define the objects we'll use in the database. This is object-relational mapping (ORM).

In our examples, we'll have two structs with attributes on them, and those attributes give us the object-relational mapping. We'll also have a separate migrations folder. Migrations are essentially raw SQL. I'm not particularly fond of that, because it means you need to understand SQL. But you should probably know what's going on underneath when you're working with a database anyway. At a minimum, you should have some fundamental knowledge of how to create and set up your tables, along with the indexes that go with them.

We'll set up create and drop statements for these tables, along with some indexes. Then we'll run the tests. They aren't really unit tests: they live in a separate crate called `bench-common`. These tests run several million inserts and tell us which frameworks are the most useful and the fastest, which ones have issues, and which ones aren't quite production-ready.

We'll start with a simple CRUD interface: we create the rows, update them, delete them, and retrieve them.

## SeaORM and SQLx

Next, we'll set up SeaORM, which is wrapped over the top of another framework I've got here called SQLx. SQLx's query macros check our queries as we write our SQL, making sure they'll actually work against a production database. The catch is that it also needs a live database to connect to in order to do that. That's pretty cool, but it also introduces the ability to break things in weird and wonderful ways. It also seems to have some performance problems, which we'll get to. I haven't used SQLx in earnest, and I haven't really used SeaORM in earnest either. I tend toward Diesel, or I go down to the `rusqlite` level if I need the performance.

Next, we'll talk about SeaORM and what its structures look like for simple CRUD operations. Then we'll look at SQLx, then `rusqlite`, and finally Turso, an interesting project that's a rewrite of SQLite in pure Rust.

Next, we'll talk about SeaORM and what its structures look like for simple CRUD operations. Then we'll look at SQLx, then `rusqlite`, and finally Turso. Turso is an interesting project: it's a rewrite of SQLite in pure Rust.

### Diesel structs

Looking at this slide, this is what a struct looks like in Diesel. The attributes here tell Diesel how to serialize and deserialize the struct to and from the database. We need to provide certain attributes on the structs, and we can annotate individual fields so their data goes into the database in different ways.

Here I have a few examples: how to handle UUIDs, how to handle date-times, and, to mix it up, some geospatial types. I'm interested in geospatial data and how we query it. If you're building something like a map with properties, or some kind of lookup of an area over time, geospatial types can be really useful. If we run a database that supports these types, such as PostGIS, we can wrap them in the ORM and have the same types in our Rust code. Everything works together nicely.

### Diesel selects and updates

Moving to the next slide, this shows what a select looks like. It also shows how we have to split some of these objects into separate structs in order to do updates, which is a bit nasty. You end up with some duplication of the objects, which isn't great.

### Wrapping it in a web framework

Now I'll quickly show how you might wrap this in a web framework. I like a framework called Dropshot, written by the folks at Oxide. They write only Rust across their organisation, which is awesome. Dropshot lets you define your endpoints, and it builds the server from those definitions easily. At a lower level, it's doing something similar to Utoipa, which generates an OpenAPI spec from your Rust code. Dropshot also has API traits: you define the API as a trait, generate the spec from it, and implement the trait later. I like the trait approach because I find it a more natural, Rust-like way of developing an API, but it does tie you to Dropshot. Here, I'll probably implement it with Axum.

This shows how easily you can put it together. You might have a `database.rs` with all the calls for your simple CRUD, or a separate file for each type with its CRUD operations. Then, in the Dropshot API, you wrap them in a RESTful endpoint for each object, since REST is a natural fit for data.

Integrating the ORM with a REST API is going to look very similar across all of these frameworks. You'll most likely have Dropshot, or whatever you're using, define a trait that handles create, retrieve, update and delete for each object type.

## SeaORM

Moving on to SeaORM, we're doing a similar thing. This is what creating records looks like. This is how you select, or retrieve, them. This is how you update them. And this is how you'd write the code for deletion in SeaORM.

### SeaORM migrations

The next slide shows what migrations look like in SeaORM, and we'll talk about some of the nuances: the pros and cons of using SeaORM over Diesel. Both are very mature APIs, but I've predominantly used Diesel up to this point.

SeaORM migrations are written in Rust, so here's an example of what they look like. I didn't realise SeaORM worked this way, but I prefer this style.

## SQLx

Now let's say you actually hate ORMs, like one of my old work colleagues who used to come to this group. You think you shouldn't use them, and that you should learn SQL because SQL is its own thing. So we'll use SQLx directly for our development.

What does that look like? How do we validate it? What does it give us that we wouldn't otherwise get with an ORM? There should obviously be less boilerplate. I've got the create, retrieve, update and delete functions all on this slide, and they show how much simpler they are. But we lose out in other ways. How do we run the migrations? Do we have to learn the SQL? Are there issues with selection? And how do we do a cursor in SQLx?

Next, I'll show how to do cursors in SQLx. That leads into a broader look at cursors and how you do pagination in the other libraries. After that, we'll come back to the code examples for `rusqlite` and Turso.

### Keyset pagination

Let's talk about keyset pagination, which is a common way of doing pagination if you've never used it. Say you have 14 million rows and you need to filter them and return the results, but you don't know how many you'll end up with. You don't want the query to run for over 30 seconds and get killed by your load balancer or your website, so you may need to paginate.

With pagination, the front end needs a way to get just the records it's looking for within specific bounds, maybe geospatial bounds, and to step through them a page at a time, say 1,000 rows per page. You want each page to be efficient in the database. You don't want it to be slow, and you don't want to have to requery using an index that isn't built for it.

### Keyset pagination examples

The next slide shows examples of keyset pagination across these three frameworks, and I'll explain in a bit more detail why you'd use it.

There are multiple ways to do pagination. We might have a page number associated with each page, but that won't necessarily give us the exact query we asked for. When we paginate, we really want to say, "This is our exact query, and we want to step through it."

In the old days, databases did this with a cursor. You'd have a cursor for each client paging concurrently, and the cursor holds a position in the database that you come back to, similar to an iterator. That's how you'd paginate. More common these days is keyset pagination, which lets you keep that exact query and pass in the last key you saw as the starting point for the next page.

For keyset pagination to work correctly, we need a time-ordered key, like a UUIDv7, because an insert might happen after our query has started. Say you're on page 1, then you go to page 2, then page 3, and then an insert happens. There are five pages of data, so the new row should show up at the end, on page 5. Because a UUIDv7 is ordered by insertion time, the new row always sorts after everything that was already there, so it lands at the end instead of somewhere in the middle of pages we've already seen.

## Lower-level SQL libraries: rusqlite and Turso

Now we'll take a tour of the lower level. These aren't ORMs, they're SQL libraries. We'll look at what `rusqlite` and Turso look like, with examples of their CRUD interfaces. Then we'll look at how you'd handle geospatial types in these libraries, and how much more complex things get when you're dealing with that kind of data.

For geospatial support on SQLite, we'd use SpatiaLite, a geospatial extension that sits on top of SQLite. For Postgres, we'd use PostGIS, since that's what I've worked with before.

In our examples, assume we're running SQLite with SpatiaLite, and Postgres with PostGIS, and that the geospatial types are mapped for both in a `geospatial.rs` module.

## Benchmarks

Once we get to the end, we'll look at some benchmarks. Looking at these results, you can see that SQLx has some issues with batching. SQLx is an async library, but the SQLite C library blocks while it works. So SQLx runs each SQLite connection on its own background worker thread, and every execute has to be sent to that thread over a channel and its result sent back. That round-trip happens on every single statement, so across millions of small inserts it has a huge performance impact. `rusqlite`, which wraps the SQLite C interface directly, is very fast. So for any performance-critical database, that's probably what you'd want to run.

Diesel has some niceties, and I've used it in production systems, or what I'd consider production systems. I'll try SeaORM next time because I like the look of it, but since it's built on SQLx, I'm a bit concerned about the performance hit it might take.

### [Slide: How SQLx talks to SQLite vs rusqlite]

*Image description:* two side-by-side flows.

- **Left — SQLx (async):** an async task on a Tokio worker thread sends a statement message down a channel to a dedicated SQLite worker thread. The task sleeps (`.await`). The worker thread makes the blocking SQLite C call, then sends the result back over a channel, and Tokio wakes the task. Label the channel hops in each direction as the per-statement overhead.
- **Right — rusqlite (sync):** the calling thread calls the SQLite C function directly and gets the result back on the same thread. No channel, no thread switch.

So why doesn't SQLx just call the SQLite C library directly? It could, but that call would block a Tokio worker thread. A Tokio runtime has only a handful of worker threads, often one per core, running thousands of tasks. While one of them is stuck inside SQLite, every task on it stalls: HTTP requests, timers, other queries. A quick insert is fine, but SQLite calls can block for a long time: waiting on another writer's lock, an `fsync` on commit, or a big scan or sort without an index. SQLx can't know ahead of time which calls will be fast, so it pays the channel cost on every call to guarantee it never stalls the runtime.

There's also API consistency. SQLx gives Postgres, MySQL and SQLite the same async interface, the same pools, and the same `query!` macros. Postgres is naturally async because it's a network protocol, so for SQLite to fit that same interface it has to look async too, and the worker thread is how SQLx does that.

So that's the trade-off. SQLx buys a runtime that never stalls and one API across all your databases, and it pays for that with overhead on every call. With `rusqlite`, you take on the job of not blocking the runtime yourself, usually by running database work in `spawn_blocking` or on a dedicated thread. But you can do that in batches, one hop for a thousand inserts instead of one hop per statement, and that's likely where most of the gap in our benchmark comes from.

## Connection pooling

Since we're talking about a CRUD app, the benchmarks need a realistic implementation for each framework. That means shared connections across threads and tasks, which means connection pooling. Here's how each one handles it, if it does at all.

- **Diesel:** plain Diesel is synchronous and comes with optional `r2d2` pool support. Inside an async web server, we'd use `diesel-async` instead, which works with async pools such as `deadpool` and `bb8`. The catch is that `diesel-async` only talks natively to Postgres and MySQL. For SQLite, it wraps the synchronous connection and runs it on a blocking thread, so it pays the same kind of thread hop as SQLx.
- **SeaORM:** pooling comes for free. A SeaORM database connection is a SQLx pool underneath, and you configure it with connection options such as the maximum number of connections.
- **SQLx:** has its own built-in async pool, such as `PgPool` and `SqlitePool`. For SQLite, every pooled connection still has its own worker thread, as we saw earlier.
- **rusqlite:** has no pool of its own. We'd add one: `r2d2_sqlite` for synchronous code, or `deadpool-sqlite` for async code, which runs our closures on a blocking thread. The difference is that we choose how much work goes into each hop.
- **Turso:** is async-native, so there's no worker thread to hide blocking calls. You open a database and call `connect()` for each connection. By default only one connection can write at a time, but Turso has an opt-in MVCC mode for concurrent writes.

Whatever pool we use, SQLite still allows only one writer at a time, even in WAL mode. So for SQLite, a big write pool mostly means connections queuing on the same lock. Reads are where the pool helps.

That's what makes Turso interesting: unlike SQLite, it can do concurrent writes. It's one flag away rather than fully out of the box, since you have to switch on MVCC mode, but SQLite can't do it at all.
