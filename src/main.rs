use yew::prelude::*;
use indoc::indoc;
include!("components/news_feed.rs");

#[function_component(App)]
fn app() -> Html {
    let news_items: Vec<NewsItem> = vec![
        NewsItem {
            id: 1,
            title: "Bacon Ipsum".to_string(),
            author: "Big Bepis".to_string(),
            text: indoc! {"
            Bacon ipsum dolor amet ground round pork chop chuck chislic, venison pancetta pig capicola bresaola sausage drumstick frankfurter. Rump porchetta ball tip sausage cow picanha, kielbasa frankfurter tongue prosciutto leberkas pastrami t-bone jowl ham. Landjaeger frankfurter pork loin bacon shoulder. Sausage cow jerky chuck capicola tenderloin shankle short ribs filet mignon. Chislic salami filet mignon short loin cow ground round capicola venison picanha turkey pork loin biltong kevin pork belly.
            
            Landjaeger chicken jowl chuck pork pork loin shank kielbasa pork chop turducken ground round shankle drumstick fatback. Venison strip steak biltong swine short loin meatball fatback frankfurter jowl. Pancetta t-bone swine meatloaf brisket fatback. Drumstick tail chislic, sausage sirloin meatball corned beef pork chop kevin.
            
            T-bone beef venison beef ribs, leberkas jerky ball tip porchetta biltong tenderloin shank drumstick cupim kevin shankle. Pastrami buffalo frankfurter jerky cupim doner. Burgdoggen picanha pancetta cow salami jowl. Kielbasa hamburger spare ribs, capicola frankfurter pork belly pork chop shank brisket jerky t-bone fatback buffalo."}.to_string(),
        },
        NewsItem {
            id: 2,
            title: "Hipster Ipsum".to_string(),
            author: "Hip Bepis".to_string(),
            text: indoc! {"
            I'm baby beard master cleanse austin celiac ennui. Thundercats street art bespoke, shabby chic schlitz echo park distillery next level blue bottle synth. IPhone raw denim edison bulb, fanny pack 3 wolf moon kitsch helvetica franzen bitters ennui man bun bushwick. Intelligentsia hashtag flannel vice heirloom craft beer vaporware everyday carry post-ironic jianbing. Church-key viral vibecession chillwave williamsburg shabby chic bicycle rights mlkshk palo santo twee edison bulb. Man bun slow-carb craft beer, edison bulb fanny pack banh mi synth cronut ugh migas lumbersexual whatever listicle. Banh mi master cleanse Brooklyn literally yuccie narwhal jianbing biodiesel +1 iceland vexillologist truffaut.

            Hoodie everyday carry tumblr photo booth vinyl artisan godard tofu actually succulents listicle gentrify. Hammock ethical semiotics, direct trade literally bodega boys meh. Fingerstache ugh taiyaki tumeric. Lo-fi iceland pug pop-up forage keytar normcore mukbang big mood hashtag. Viral paleo locavore shoreditch.
            
            90's schlitz wolf mlkshk stumptown. Tbh pitchfork beard, roof party cold-pressed next level DSA. You probably haven't heard of them pabst occupy leggings. Gochujang same wayfarers, jean shorts thundercats af church-key taiyaki ascot. Swag tilde jean shorts lumbersexual williamsburg, affogato slow-carb glossier."}.to_string()
        },
        NewsItem {
            id: 3,
            title: "Space Ipsum".to_string(),
            author: "John F. Kennedy".to_string(),
            text: indoc! {"
            There is no strife, no prejudice, no national conflict in outer space as yet. Its hazards are hostile to us all. Its conquest deserves the best of all mankind, and its opportunity for peaceful cooperation many never come again. But why, some say, the moon? Why choose this as our goal? And they may well ask why climb the highest mountain? Why, 35 years ago, fly the Atlantic? Why does Rice play Texas?

            We choose to go to the moon. We choose to go to the moon in this decade and do the other things, not because they are easy, but because they are hard, because that goal will serve to organize and measure the best of our energies and skills, because that challenge is one that we are willing to accept, one we are unwilling to postpone, and one which we intend to win, and the others, too.

            It is for these reasons that I regard the decision last year to shift our efforts in space from low to high gear as among the most important decisions that will be made during my incumbency in the office of the Presidency.

            In the last 24 hours we have seen facilities now being created for the greatest and most complex exploration in man's history. We have felt the ground shake and the air shattered by the testing of a Saturn C-1 booster rocket, many times as powerful as the Atlas which launched John Glenn, generating power equivalent to 10,000 automobiles with their accelerators on the floor. We have seen the site where the F-1 rocket engines, each one as powerful as all eight engines of the Saturn combined, will be clustered together to make the advanced Saturn missile, assembled in a new building to be built at Cape Canaveral as tall as a 48 story structure, as wide as a city block, and as long as two lengths of this field.
            "}.to_string()
        },
    ];
    html! {
    <div>
        <h1>{"Why'd You Have to Go and Make Things So Complicated?"}</h1>
        
        <div class="explanation">
            <h3>{"Hey you! Yeah, you! On the other side of the screen!"}</h3>
            <p>{indoc! {"Have you ever just been sitting at home and thought to yourself: \"Gosh, I sure do love using React, but I wish that, instead of using something sensible like JavaScript or TypeScript, I could use everybody's favorite meme language Rust instead?\"\n
            Well look no further! Today I'd like to introduce to you: "}}<a href="https://yew.rs/" target="_blank">{"Yew.rs"}</a>{"!"}</p>
            <p>{"Have you ever heard a developer say something along the lines of \"Honestly, I'm not a super huge fan of React because JSX causes you to mix HTML with JS, and can really introduce some unneccesary readability problems\"? Well, show them Yew and watch them have an aneurysm! It'll be hilarious, I promise."}</p>
            <p>{"Okay, that's cool and all, but what are the advantages to using Rust + WASM instead of just using JavaScript like a normal person? Well that's the fun part: in terms of performance, there are literally "}<em>{"zero"}</em>{" advantages to using a Rust-based framework instead of a traditional JavaScript-based one. One of the primary reasons for this is that DOM manipulation cannot be done with pure WebAssembly and requires utilizing JavaScript. In fact, this actually results in Yew (along with other frameworks that use WASM) actually being "}<em>{"significantly slower"}</em>{" than basically every JavaScript framework, as seen below:"}</p>
            <img src="img/perftable.png" alt="Table of results for various JS frameworks and Yew.rs" />
            <p>{"So, of course, this begs the question: Why in the world would anyone even "}<em>{"want"}</em>{" to use Yew when it's one of the slowest web frameworks in existence, and only manages to barely edge out React due to React's abysmal performance with selecting and swapping rows?"}</p>
            <p>{"Because, quite frankly, I find it "}<em>{"incredibly"}</em>{" funny that, now, even front-end webdev isn't safe from the \""}<a href="https://transitiontech.ca/random/RIIR" target="_blank">{"rewrite it in Rust"}</a>{"\" meme."}</p>
            <p>{"To close, I decided to make a small component below that's just a feed of different \"news items\" that could be repurposed for something like blog posts or whatever. Something that actually, you know, plays into the whole purpose of using a framework like Yew or React... They're just various Lorem Ipsum variants, but here they are:"}</p>
        </div>

        <NewsItemsFeed news_items={news_items}/>
    </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
