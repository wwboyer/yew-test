#[derive(Clone, PartialEq)]
struct NewsItem {
  id: u32,
  title: String,
  author: String,
  text: String,
}

#[derive(Properties, PartialEq)]
struct NewsItemsFeedProps {
  news_items: Vec<NewsItem>
}

#[function_component(NewsItemsFeed)]
fn news_items_feed(NewsItemsFeedProps {news_items}: &NewsItemsFeedProps) -> Html {
  news_items.iter().map(
    |item| html! {
      <div id={format!("news-item-{}", item.id)} class={"news-item"}>
        <h2>{&item.title}</h2>
        <h4>{format!("Written by: {}", &item.author)}</h4>
        <p>{&item.text}</p>
      </div>
    }
  ).collect()
}