use askama::Template;

#[derive(Template)] // this will generate the code...
#[template(path = "call.html")] // using the template in this path, relative
                                // to the templates dir in the crate root
struct CallTemplate {
    pub badge_title: String,
}

#[test]
fn test_call() {
    let call = CallTemplate {
        badge_title: "Active".to_owned(),
    }; // instantiate your struct

    assert_eq!("<span>Active</span>", call.render().unwrap().trim()); // then render it.
}
