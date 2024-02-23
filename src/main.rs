use reqwest;
// use serde::Deserialize;
// use reqwest::Error;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let email = "EMAIL-OR-USERNAME-HERE"; // replace with email or username given to you
    let password = "PASSWORD-HERE"; // replace with password given to you

    // Step 1: Login
    let login_data = json!({
        "email": email,
        "password": password
    });

    let login_response = reqwest::Client::new()
        .post("https://challenger.code100.dev/login")
        .json(&login_data)
        .send()
        .await?;

    if !login_response.status().is_success() {
        println!("Login failed");
        return Ok(());
    }

    let login_result = login_response.json::<serde_json::Value>().await?;
    let token = login_result["token"].as_str().unwrap();

    // Step 2: Call Authenticated Endpoint
    let auth_response = reqwest::Client::new()
        .get("https://challenger.code100.dev/testauthroute")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    let auth_result = auth_response.text().await?;
    println!("{}", auth_result);

    // Step 3: Get the puzzle
    let puzzle_response = reqwest::Client::new()
        .get("https://challenger.code100.dev/getpuzzle")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    let puzzle_result = puzzle_response.text().await?;
    println!("{}", puzzle_result);

    // Step 4: Solve the puzzle

    ////////////////////////////
    ////// YOUR CODE HERE //////
    ////////////////////////////

    let answer = json!({
        "answer": "some answer in required format"
    });

    // Step 5: Submit the solution
    let submit_response = reqwest::Client::new()
        .post("https://challenger.code100.dev/postanswer")
        .header("Authorization", format!("Bearer {}", token))
        .json(&answer)
        .send()
        .await?;
    
    let submit_result = submit_response.text().await?;
    println!("{}", submit_result);

    Ok(())
}