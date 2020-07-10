use crate::rocket;
use rocket::local::Client;
use rocket::http::{Status, ContentType};

#[test]
fn get_post_get_put_get() {
    let client = Client::new(rocket()).unwrap();

    // Check that a message with slug 'ahayouga' doesn't exist.
    let res = client.get("/api/article/poop").header(ContentType::JSON).dispatch();
    assert_eq!(res.status(), Status::NotFound);

    // Add a new message with slug 'ahayouga'.
    let res = client.post("/api/article")
        .header(ContentType::JSON)
        .body(r#"{ "slug" : "ahayouga", "content" : "AHAYOUGA!YOUGA!YOUGA!" }"#)
        .dispatch();

    assert_eq!(res.status(), Status::Ok);

    // Check that the message exists with the correct contents.
    let mut res = client.get("/api/article/ahayouga").header(ContentType::JSON).dispatch();
    assert_eq!(res.status(), Status::Ok);
    let body = res.body().unwrap().into_string().unwrap();
    assert!(body.contains("AHAYOUGA!YOUGA!YOUGA!"));

    // Change the message contents.
    let res = client.put("/api/article")
        .header(ContentType::JSON)
        .body(r#"{ "slug": "ahayouga", "content": "OUGA! BOUGA! NO MORE YOUGA" }"#)
        .dispatch();

    assert_eq!(res.status(), Status::Ok);

    // Check that the message exists with the updated contents.
    let mut res = client.get("/api/article/ahayouga").header(ContentType::JSON).dispatch();
    assert_eq!(res.status(), Status::Ok);
    let body = res.body().unwrap().into_string().unwrap();
    assert!(!body.contains("AHAYOUGA"));
    assert!(body.contains("OUGA! BOUGA! NO MORE YOUGA"));
}
