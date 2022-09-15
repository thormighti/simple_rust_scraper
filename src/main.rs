mod scapper;


fn main()  {

    scapper("https://twitter.com/home","div.DraftEditor-editorContainer>div");

  
}


fn scapper(url:&str, the_selector: &str){

    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);
    
    let selector = scraper::Selector::parse(the_selector).unwrap();

    let title = document.select(&selector).map(|x| x.inner_html());

    title.zip(1..100).for_each(|(item, number)| println!("{}, {} ", number, item));


}


