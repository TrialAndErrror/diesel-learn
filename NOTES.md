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

To implement this change, We change the `Todo` struct to be named `Grocery` and we want to change a few fields. Instead of `text`, I want to have a name for the grocery and an amount. They're both freeform string fields, so we just use owned `String` fields here to get it up and running. We update the `NewTodo` to reflect the fields we need to take in from the user, `name` and `amount`. Finally, we look at the schema and need to update the 



