#[cfg(test)]
mod tests {
    use reqwest::StatusCode;

    
    #[tokio::test]
    async fn get_personal_schedule() {
        // TODO: fix this test
        let res = reqwest::get("https://kpiexport.nikitavbv.com/schedule/%D0%86%D0%9F-82?lastName=Volobuev").await
            .unwrap();
        let response_status = &res.status();
        let response_text = res.text().await.unwrap();

        println!("result text is: {:?}", response_text);
        println!("result is {:?}", response_status);

        assert_eq!(response_status, &StatusCode::OK);
    }
}