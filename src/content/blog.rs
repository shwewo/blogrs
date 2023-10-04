use worker::{ *, kv::KvStore };
use chrono::{ NaiveDate, Datelike, DateTime, NaiveTime, NaiveDateTime, Utc };
use markdown;

#[derive(Debug)]
struct Post {
	title: String,
	date: DateTime<Utc>,
	formatted_date: String,
	short_desc: String,
	// content: String,
}

pub async fn get_post(
	name: &String,
	storage: &KvStore
) -> Option<(String, String, String, String)> {
	let content = storage.get(&name).text().await;

	match content {
		Ok(value) => {
			if let Some(post) = value {
				let (title, date, short_desc, content) = get_post_info(&post).await.unwrap();
				let marked = markdown::to_html(&content.as_str());
				Some((title, date, short_desc, marked))
			} else {
				None
			}
		}
		Err(_) => None,
	}
}

pub async fn get_post_info(markdown: &String) -> Option<(String, String, String, String)> {
	markdown.split_once('\n').map(|(short_title, long_description)| {
		let (a, bc) = short_title.split_once(' ').unwrap_or_else(|| panic!("invalid text format"));
		let (b, c) = bc.split_once(' ').unwrap_or_else(|| panic!("invalid text format"));
		(a.to_owned(), b.to_owned(), c.to_owned(), long_description.to_owned())
	})
}

#[allow(deprecated)]
pub async fn get_all_posts(context: &RouteContext<()>) -> String {
	let storage = context.kv("BLOG_POSTS").unwrap();
	let keys = storage.list().execute().await.unwrap().keys;
	let mut posts = String::new();
	let mut posts_data: Vec<Post> = Vec::new();

	for (_, value) in keys.iter().enumerate() {
		let (title, date, short_desc, _content) = get_post_info(
			&storage.get(&value.name).text().await.unwrap().unwrap()
		).await.unwrap();

		let date = NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d").unwrap();
		let time = NaiveTime::from_hms(0, 0, 0);
		let naive_datetime = NaiveDateTime::new(date, time);
		let datetime_utc = DateTime::<Utc>::from_utc(naive_datetime, Utc);
		let formatted_date = format!("{} {}", date.format("%B"), date.year());

		posts_data.push(Post {
			title: title,
			date: datetime_utc,
			formatted_date: formatted_date,
			short_desc: short_desc,
			// content: _content,
		});

		posts_data.sort_by(|a, b| b.date.cmp(&a.date));
	}

	for item in posts_data.iter() {
		posts.push_str(
			r#"
<div class="post">
  <a href="https://localhost:8080.localhost:8080/post/TITLE">
    <strong>
      SHORT_DESC
    </strong>
  </a>
  <p class="date">DATE</p>
</div>
    "#
				.replace("TITLE", item.title.as_str())
				.replace("SHORT_DESC", item.short_desc.as_str())
				.replace("DATE", item.formatted_date.as_str())
				.as_str()
		);
	}

	posts
}