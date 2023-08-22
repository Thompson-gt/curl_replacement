## **curl replacemant tool**

#### the goal of this program was to have a readable and simply way of querying websites/endpoints without the plain curl tool or without having to open up postman to have it displayed in a cleaner fashion

##### _was written in rust for the fun of it and was a good chance to learn the language_

## **How to Run:**

#### simple examples of how to use program

`cargo run -- --url "https://pokeapi.co/api/v2/pokemon/ditto"`

##### _**or**_

`cargo run -- --url "https://pokeapi.co/api/v2/pokemon/ditto" -t`

##### _**example on how to use a POST request**_

`cargo run -- --domain "localhost:5000/login -- body "{'username' : 'exampleUser', 'email' : 'exampleEmail@example.com'}" -r "post"`

#### _this to see all of the options this tool offers_

`cargo run -- -h`

##### _**or**_

`cargo run --` _both do the same thing_
