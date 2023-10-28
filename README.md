# aad64_Individual_Project_2
Rust CLI Binary with SQLite

[![Format](https://github.com/nogibjj/aad64_Individual_Project_2/actions/workflows/format.yml/badge.svg)](https://github.com/nogibjj/aad64_Individual_Project_2/actions/workflows/format.yml)
[![BuildBinary](https://github.com/nogibjj/aad64_Individual_Project_2/actions/workflows/build.yml/badge.svg)](https://github.com/nogibjj/aad64_Individual_Project_2/actions/workflows/build.yml)
[![Lint](https://github.com/nogibjj/aad64_Individual_Project_2/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/aad64_Individual_Project_2/actions/workflows/lint.yml)
[![Test](https://github.com/nogibjj/aad64_Individual_Project_2/actions/workflows/test.yml/badge.svg)](https://github.com/nogibjj/aad64_Individual_Project_2/actions/workflows/test.yml)

## Goals:
The goal of this assignment was to create a project that connects to a SQLite database using Rust source code. For the same, I chose to use the classic iris.csv dataset to load into a database and then perform CRUD operations on. This project carries out these SQL queries while maintaining functioning github workflows (namely, lint, format, test, and specifically for Rust- build). Furthermore, this project also used GitHub Co-Pilot through the process which will be further discussed below along with the other details of the project. 

## Contents of the Project:

### 1. GitHub Workflows:
All four of these workflows can be seen at a glance in the badges at the top of this README. For further details, the folliwing bullet list gives you some more information about each:
1. __[build.yml](https://github.com/nogibjj/aad64_Individual_Project_2/blob/main/.github/workflows/build.yml)__:
+ This workflow helps in creating a binary file for this Rust project. 
2. __[format.yml](https://github.com/nogibjj/aad64_Individual_Project_2/blob/main/.github/workflows/format.yml)__:
+ This worflow helps in formating the entire project using the command `cargo fmt --quiet`. 
3. __[lint.yml](https://github.com/nogibjj/aad64_Individual_Project_2/blob/main/.github/workflows/lint.yml)__:
+ This workflow lints the project to analyse the source code and flag any issues which may arise in it, such as, syntax errors, dead code, etc. 
4. __[test.yml](https://github.com/nogibjj/aad64_Individual_Project_2/blob/main/.github/workflows/test.yml)__:
+ This final workflow ensures to go through the unit tests that were written in my source code (specifically, in `lib.rs`). As seen in the following section, the project passed all tests successfully.

### 2. src Folder:
1. __[lib.rs](https://github.com/nogibjj/aad64_Individual_Project_2/blob/main/src/lib.rs)__: This file holds all the functions necesary to create a database connection, create a table in said database and then carry out the rest of the CRUD operations. For the same, 8 functions were written:
   + `create_iris_table` and `drop_iris_table` help in creating and deleting the Iris table in my database. 
   + `table_exists` was written as a complementary function for the above two, to check if they are working as required. This function was then called in main.rs along with the above functions to double-check their functionality. 
   + `insert_iris_data`: This inserts all the values from iris.csv to the Iris table in my databased called iris.db
   + `print_first_five_rows`: As the name suggests, this function helps in just seeing the first five rows of the database.
   + `insert_initial_data`: This was the first augementative query function written, which adds a new row to my database with new values. 
   + `update_iris_table`: Similar to the above function, this function added a new row to the database, and the carried out an `UPDATE` query on the newly made row. 
   + `order_iris_table`: This was an extra function written just to order the database as per one of the columns' values (namely, the sepal_length column).
  This file also has `unit tests` to test the functionality of all of the functions. As seen in the screenshot below, my project passes all the tests:
  <p align = 'center'><img width="1011" alt="image" src="https://github.com/nogibjj/aad64_cloud_hosted_nb/assets/143753050/9e2371ab-dc88-4a2e-bbe6-93880c8bfe80"></p>
  
2. __[main.rs](https://github.com/nogibjj/aad64_Individual_Project_2/blob/main/src/main.rs)__: This file calls the functions written in lib.rs. It then outputs the results with reference to a database in this project, namely, [iris.csv](https://github.com/nogibjj/aad64_Individual_Project_2/blob/main/iris.db). 

### 3. [Cargo.toml](https://github.com/nogibjj/aad64_Individual_Project_2/blob/main/Cargo.toml)
This file is the `Cargo.toml` manifest for a Rust project named `rust_sqlite` with dependencies on `csv`, `reqwest`, and `rusqlite` libraries. It specifies the project version, edition, and library configuration.

### 4. [Makefile](https://github.com/nogibjj/aad64_Individual_Project_2/blob/main/Makefile)
This is a `Makefile` used for automating various tasks in a Rust project:
- `format`: Invokes `cargo fmt` to automatically format the code according to Rust style guidelines.
- `lint`: Executes `cargo clippy` to perform linting and static analysis to catch potential issues or non-idiomatic code.
- `test`: Runs the project's test suite using `cargo test`.
- `release`: Creates a binary file by running `cargo build --release`. 
- `run`: Launches the project with `cargo run`.
- `all`: Combines the tasks `format`, `lint`, `test`, `release`, and `run` to perform all common project tasks in sequence.

## CRUD Operations:
This project had somewhat of user interaction, where the user coud specify what function they wanted to carry out by choosing one of the following commands:
+ `cargo run create`
+ `cargo run insert`
+ `cargo run print`
+ `cargo run initial`
+ `cargo run update`
+ `cargo run order`
+ `cargo run drop`

If one tries to run these commands in impossible situations, e.g., ordering a table after dropping it, an error would automatically pop up as this project has included error handling throughout all functions (below is an example of the same).
<p align = 'center'><img width="814" alt="image" src="https://github.com/nogibjj/aad64_cloud_hosted_nb/assets/143753050/b4c5a4e8-afa6-4054-93ea-aea103ff9261"></p>


### Create and Read:
As suggested earlier, the functions `create_iris_table` and `print_first_five_rows` help in creating the database and reading the first five rows. The functions are built in a way to show that this has been successfully created and then print the first five rows of the successfully created database. 

<img width="1267" alt="image" src="https://github.com/nogibjj/aad64_cloud_hosted_nb/assets/143753050/29d8ec18-3065-4392-8415-3dc023756f36">

The screenshot below shows the command `cargo run insert` successfully inserting and printing all the rows inserted into the Iris table in the database.

![image](https://github.com/nogibjj/aad64_cloud_hosted_nb/assets/143753050/bacd83d1-9f39-47fe-ad9a-0b1f77a93a4b)
<img width="872" alt="image" src="https://github.com/nogibjj/aad64_cloud_hosted_nb/assets/143753050/059e9da5-a46b-4936-87e0-e975e6c222de">

Then, using the command `cargo run print`, I could `READ` the table and print the first five entries to the command line successfully. 

<img width="609" alt="image" src="https://github.com/nogibjj/aad64_cloud_hosted_nb/assets/143753050/27b21fc8-39f4-40ce-ac73-f47e997d0970">


### Update:
There were two functions (`insert_initial_data` and `update_iris_table`) to allow the manipulation of the database as specified above. If the command `cargo run initial` was run, then a new row was added (highlighted with a box) to the database as seen below:

<img width="1238" alt="image" src="https://github.com/nogibjj/aad64_cloud_hosted_nb/assets/143753050/ee4bbd8f-43d4-47aa-b076-938872dbb2ed">

The other function created to update the database was run using `cargo run update`. The result of this function was updating the previouslt created row which can be seen in the following screenshot:

<img width="1256" alt="image" src="https://github.com/nogibjj/aad64_cloud_hosted_nb/assets/143753050/7163061f-03a5-43ad-80a4-0d34f3ad9c37">


### Drop:
The function `drop_iris_table` was created to drop the table is the database and was called by the command `cargo run drop`. The following screenshot shows the functionality of the same. 
<img width="1278" alt="image" src="https://github.com/nogibjj/aad64_cloud_hosted_nb/assets/143753050/b36137aa-8a4d-49bc-8c06-988c976cd658">

### Order:
An extra function was written to order the table as per the column sepal length (`order_iris_table`), which was called in the command line with the command `cargo run order`. This function also printed the first ten rows of the database to show that the function worked correctly, as seen in the screenshot below. 

<img width="632" alt="image" src="https://github.com/nogibjj/aad64_cloud_hosted_nb/assets/143753050/ca119bb9-10b9-4ea9-af50-c70fbad22d6d">



## Use of GitHub Co-Pilot 
This project definitely used the assistance of GitHub Copilot throughout the entire process. However, this wasn't as substantial as I thought it would be. Firstly, I believe that copilot is not as trained in Rust as Python, so some of the solutions it was giving me weren't as helpful. I had to resort to using ChatGPT and/or just browsing the web. Second, there were many times when I would go into a loop with copilot, where it would keep on suggesting the same solutions to certain errors with no modifications. That being said, I was able to try out three different structures to my project and finally ended up with this one, which was not as time consuming with the help of copilot. 
