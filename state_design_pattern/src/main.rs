use state_design_pattern::Post;



fn main (){

    let mut new_post = Post::new();

    new_post.add_text("screaminggg my unwanted opinion to the world.");
    assert_eq!(new_post.content(), "");


    new_post.request_review();
    assert_eq!(new_post.content(), "");

    new_post.approve();
    assert_eq!(new_post.content(), "screaminggg my unwanted opinion to the world.");

}