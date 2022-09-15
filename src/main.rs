// use scraper::Html;

mod scapper;

fn main() {
    scapper("https://fedoramagazine.org/", "div.magazine-about>p>a");
}

fn scapper(url: &str, the_selector: &str) {
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);
    // let fragment = Html::parse_fragment(the_selector); tried selecting attribute

    let selector = scraper::Selector::parse(the_selector).unwrap();

    // let input = fragment.select(&selector).next().unwrap(); same here

    let result = document.select(&selector).map(|x| x.inner_html()); // concern with the inner html or html depending

    result.for_each(|items| println!("{} ", items));
}
