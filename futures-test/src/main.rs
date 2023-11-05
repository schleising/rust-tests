use futures::future;
use reqwest::header::CONNECTION;
use std::sync::Arc;

async fn long_process(proc_number: u32, client: Arc<reqwest::Client>) -> Result<(), reqwest::Error> {
    // Print the number of the process that is starting
    println!("Process: {} Started", proc_number);

    // Get data from a url
    let url: &str = "https://google.com";

    // Make a request
    let response: Result<reqwest::Response, reqwest::Error> = client.get(url).header(CONNECTION, "keep-alive").send().await;

    // Print the response
    // println!("Response: {:?}", response);

    // Get the response body
    match response {
        Ok(response) => {
            // Print the request headers
            // println!("Headers: {:?}", response.headers());

            // Print the HTTP version
            // println!("HTTP Version: {:?}", response.version());

            // Get the response body
            let _body: String = response.text().await?;

            // Print the process number is successful
            println!("Proc {} - Successful", proc_number);

            // Print the response body
            // println!("Body: {}", body);
        },
        Err(e) => {
            // Print the error
            println!("Proc {} - Error: {}", proc_number, e);
        }
    }

    // Print the number of the process that is finished
    println!("Process: {} Finished", proc_number);

    // Return Ok
    Ok(())
}

async fn create_tasks() -> Result<(), reqwest::Error> {
    // Create a header map
    let mut headers = reqwest::header::HeaderMap::new();

    // Add a keep-alive header
    headers.insert(CONNECTION, reqwest::header::HeaderValue::from_static("keep-alive"));

    let duration = std::time::Duration::from_millis(3000);
    let duration_option = Some(duration);

    // Create a reqwest client
    let client: Arc<reqwest::Client> = Arc::new(
        reqwest::Client::builder()
            .tcp_keepalive(duration_option)
            .pool_max_idle_per_host(0)
            .default_headers(headers)
            .http2_prior_knowledge()
            .build()?
    );

    // Create 10 tasks
    let tasks = (0..1000).map(|i| long_process(i, client.clone()));

    // Wait for all the tasks to complete
    future::join_all(tasks).await;

    // Return Ok
    Ok(())
}

#[tokio::main]
async fn main() {
    // Start a timer
    let timer = std::time::Instant::now();

    // Call create_tasks
    let result: Result<(), reqwest::Error> = create_tasks().await;

    // Work out if there was an error
    match result {
        Ok(_) => {
            // Print the time taken
            println!("Time taken: {}s", timer.elapsed().as_secs());
        },
        Err(e) => {
            // Print the error
            println!("Error: {}", e);
        }
    }
}
