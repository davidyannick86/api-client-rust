use serde::Deserialize;

// * URL to fetch the data from
static URL: &'static str = "https://jsonplaceholder.typicode.com/todos";

// * Struct to represent the data fetched from the API
#[derive(Deserialize)]
struct Todo {
    #[serde(rename = "userId")] // ! Renaming the field to user_id
    user_id: i32,
    id: i32,
    title: String,
    completed: bool,
}

// * Implementing the Debug trait for the Todo struct
impl std::fmt::Debug for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "-> User ID: {}, id: {}, title: {}, completed: {} \n",
            self.user_id, self.id, self.title, self.completed
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // * Fetching the data from the API
    let request_url = URL.to_owned();

    // * Creating a new reqwest client
    let client = reqwest::Client::new();

    // * Sending a GET request to the API
    let response = client
        .get(&request_url)
        .header("USER_AGENT", "Rust Web API CLI")
        .send()
        .await?;

    // * Deserializing the response into a Vec<Todo>
    let todos: Vec<Todo> = response.json().await?;

    // * Printing the first 10 todos
    todos.iter().take(10).for_each(|todo| {
        println!("{:?}", todo);
    });

    Ok(())
}
