# Initial Setup

- Tried to use `cargo new --vcs git`, but it failed due to toolchain info.

- Ran `rustup default stable` and then re-ran the cargo new command. May also need `rustup update stable` according to the docs if your rust is out of date.

- Opened CLion, but since I store these programs on a separate HD there were permissions issues. Used `nix run nixpkgs#jetbrains.clion` instead and was able to install the Rust plugin with no issues.

# Getting Diesel Set Up
https://diesel.rs/guides/getting-started
- Ran `cargo install diesel_cli` to get the diesel tools
- Got an error that `cannot find -lmysqlclient`, beacuse I don't have mysql stuff installed. Instead, I just ran the cli installation process exclusind defaults but including pg stuff: `cargo install diesel_cli --no-default-features --features postgres`
- Create .env file with the one required variable: `echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env`
- Tried to run `diesel setup`, but it said diesel not found; it was because I skipped the step of adding diesel and dotenvy to the dependencies. Added:
```
[dependencies]
diesel = { version = "2.1.0", features = ["postgres"] }
dotenvy = "0.15"
```
to my Cargo.toml.
- re-run `diesel setup`, turns out it';s still not found.


# Looking at SurrealDB
Surreal seems interesting because it can run in different database modes depending on the commands you send it, which could open the door to more precise schema definition and relationships that don't have to fight as much with the structure of a relational database or the complexity of a document-based or non-relational database system

Probably not a production-level thing today, but interesting to experiment with and use to learn about different database types in a fun and interesting way

Install surreal using the installer script
`curl --proto '=https' --tlsv1.2 -sSf https://install.surrealdb.com | sh`

Moved the file into a folder that's on my path already
`sudo mv /home/wade_lofty/.surrealdb/surreal /usr/local/bin`

Restart terminal to reload the `$PATH` 
(you can add it to the path temporarily with the command they provided, but I like to refresh my terminal from time to time to make sure I don't have a bunch of layers of flakes left activated)

Verify that it worked
`surreal help`

Start the server
`surreal start`

# Jump-Start from a base repo
https://github.com/monroeclinton/diesel-learn/

- Looked at integrating it piece by piece, but instead decided to walk through the changes that I made to make it the app I need.

Opened up models.rs, schema.rs, and main.rs together as they form the backbone of the app
- Models is where we define the structure of the data we're handling
- Schema is where we define the actual structure of the data in the database
- Main handles running the app, creating a new record, and updating records (including marking them as done).

For Groceries, we're starting with changing the todo into a grocery. Let's start with the underlying model, `ToDo`, and turn it into a Grocery. Afterwards, we'll come back to `NewTodo` and turn it into the interface for our Grocery creation requests.

On the `ToDo` model, there is a String field used for holding text of a ToDo. For Groceries, we want to capture `name` and `amount` for each Grocery entry. I chose to use owned String values as the fields to represent these data points. We could make `amount` a numeric type, probably an unsigned int or float would work for most things. But I used a String so that the user can enter in any measurement notes or units along with their numbers, to make things clearer. i.e. (Bagels: "2-4") or (Flour: "6 cups").

To implement this change, We change the `Todo` struct to be named `Grocery` and we want to change a few fields. Instead of `text`, I want to have a name for the grocery and an amount. They're both freeform string fields, so we just use owned `String` fields here to get it up and running. We update the `NewTodo` to reflect the fields we need to take in from the user, `name` and `amount`. Finally, we look at the schema and need to use diesel to generate a new schema for us.

Having trouble, making sure that I'm actually creating the right table name in the migrations files.dock

Turns out, I was having trouble installing some of the diesel dependencies because of a missing C library. Funny enough, it's because i'm using cargo and rust tooling from a nix package, and they don't include extra stuff that most people might not need. But, of course, there's a diesel-cli package on nix as well, which is set up perfectly for running diesel. So we use `nix run nixpkgs#diesel-cli` for our diesel commands.

However, this project provides a helpful docker container for running diesel commands inside your environment. Just run `docker compose run diesel-lean` to spin that up to a shell with your .env variables and diesel set up.

`diesel setup` and then `diesel migration generate <migration_name>` gets you a pair of up and down migrations that you can edit to make sure your data is working.

If you run a migration, and then want to change it, you need to `diesel migration redo` to unapply and reapply the changes. The reason you can't just do `run` again is that the database looks at the names of the migrations you have and says "I already ran those, i'm not going to look at them". So if you make changes, the DB won't apply those changes unless you remove and re-apply the migration. 

Had to go into the backend and check on the database, turns out it's harder to get in using docker compose than just the dockerfile itself. UIse `docker exec -ti database-diesel /bin/bash`, 

Once into the pg container you can use `psql -U diesel` to log into psql as the diesel user that we defined in other files. Use `\d <table_name>` to view the info of your table. You should see the fields properly configured.

# Input and Output
Now let's figure out how to get some data into the database. I made some changes to the main.rs to use our new structs from the models file. New Grocery derives Insertable to handle being able to insert new records using this struct.
Schema.rs should be be updated at this point, so take a look there and make sure the fields are what you wanted.

Found some deprecation warnings and missing imports, so made the commits and moved on.

Got stuck messing around with diesel and container stuff, but overall if you're able to run the docker container and then get shells in both services, you should be able to use `cargo run` in the diesel-learn service to create new records, and then go to the database-diesel service and get into psql and run `SELECT * FROM grocery;` to see all the records you're creating.

Let's parse some args from the command line so we can have two separate functions from the CLI

# Parse Args
Added in some simple arg parsing; just needs to match a particular string to get the command started
`cargo run n`
`cargo run x`


In order to run these in the container, we can fire up the commands using docker compose: `docker compose run diesel-learn cargo run <args>`

I also added in the list function (if called with no args, you get the list of ids) in as the default handler, and have it print the list when you go to mark an item as updated.

Could probably add in a second argument slot for the specific ID to update, for the update command, that way you can skip prompting the user for info. Also could have a flag on the create function to directly pass in arguments.

From here, we have the ability to create new groceries, check them off, and list all the data in the database. Basic functionality implemented!

In the next branch, `conversion/add-rocket`, we will look at adding Rocket to get a basic web server that can handle get and post requests to our database. We will create some views and plop our handler functions into those views, and it should be able to handle backend and frontend in full stack rust!