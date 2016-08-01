#![allow(missing_docs)]
use serde_json::Value;
use responses::BasicThing;
use responses::comment::CommentListing;

/// The 'listing' format returned by the Reddit API for post lists.
pub type Listing = BasicThing<ListingData<Submission>>;

/// The structure returned by the Reddit API for comments, consisting of the original post
/// and a listing of the comments.
pub type CommentResponse = (Listing, CommentListing);

/// API response from /r/subreddit/about
pub type SubredditAbout = BasicThing<SubredditAboutData>;

#[derive(Deserialize, Debug)]
pub struct SubredditAboutData {
    pub subscribers: u64,
    pub accounts_active: u64,
    pub subreddit_type: String,
    pub title: String,
    pub url: String,
    pub wiki_enabled: bool,
    pub over18: bool,
    pub public_description: String,
    pub public_description_html: String,
    pub public_traffic: bool,
    pub name: String,
    pub id: String,
    pub display_name: String,
    pub description: String,
    pub description_html: String,
    pub created: i64,
    pub created_utc: i64,
    pub quarantine: bool,
    pub submission_type: String,
    pub lang: String,
    pub submit_text: String,
    pub submit_text_html: String,
    pub submit_text_label: Option<String>,
    pub submit_link_label: Option<String>,
    pub comment_score_hide_mins: u64
    // CSS fields omitted
}

/// The contents of a call to a 'listing' endpoint.
#[derive(Deserialize, Debug)]
pub struct ListingData<T> {
    /// A modhash (essentially a CSRF token) generated for this request. This is generally
    /// not required for any use-case, but is provided nevertheless.
    pub modhash: Option<String>,
    pub before: Option<String>,
    pub after: Option<String>,
    pub children: Vec<BasicThing<T>>,
}

/// Represents all types of link posts and self posts on Reddit.
#[derive(Deserialize, Debug)]
pub struct Submission {
    /// The domain of the link (if link post) or self.subreddit (if self post).
    /// Domains do not include a protocol, e.g. `i.redd.it` or `self.learnprogramming`
    pub domain: String,
    /// Contains the name of the moderator who banned this, if the logged-in user is a moderator
    /// of this subreddit and this is banned.
    pub banned_by: Option<String>,
    // pub media_embed: MediaEmbed,
    /// The subreddit that this submission was posted in (not including `/r/`)
    pub subreddit: String,
    /// If this is a self post, it contains the HTML of the post body. Otherwise, it is `None`.
    pub selftext_html: Option<String>,
    /// The self text in **Markdown** format, if this is a self post. Unlike `selftext_html`, this
    /// is an **empty string** if this is a link post.
    pub selftext: String,
    /// This is `Some(true)` if the logged-in user has upvoted this submission, `Some(false)` if
    /// the user has downvoted this submission or `None` if the user has not voted.
    pub likes: Option<bool>,
    /// If a specifc sort method is suggested, this is set to the string name of it, otherwise
    /// it is `None`.
    /// # Possible values
    /// - top
    /// - new
    /// - controversial
    /// - old
    /// - qa
    /// - confidence
    pub suggested_sort: Option<String>,
    // skipped user_reports and secure_media
    /// If this post is flaired, this set to `Some(FLAIR TEXT)`. Otherwise, it is `None`.
    /// Link flairs **can** be empty strings.
    pub link_flair_text: Option<String>,
    /// The ID of the post in base-36 form, as used in Reddit's links.
    pub id: String,
    // skipped from_kind
    /// The amount of times that a user has been gilded (gifted Reddit Gold).
    pub gilded: u64,
    /// This is `true` if Reddit has archived the submission (usually done after 6 months).
    /// Archived submissions cannot be voted or commented upon.
    pub archived: bool,
    /// This is `true` if the logged-in user has already followed this link, otherwise `false`.
    pub clicked: bool,
    // skipped report_reasons
    /// The name of the author of the submission (not including the leading `/u/`)
    pub author: String,
    // skipped media
    /// The overall points score of this post, as shown on the upvote counter. This is the
    /// same as upvotes - downvotes (however, this figure may be fuzzed by Reddit, and may not
    /// be exact)
    pub score: i64,
    /// This contains the name of the user who approved this submission. This is `None` unless
    /// you are a mod of the subreddit **and** a user has approved this post.
    pub approved_by: Option<String>,
    /// This is `true` if the 'nsfw' option has been selected for this submission.
    pub over_18: bool,
    /// This is `true` if the logged-in user has clicked 'hide' on this post.
    pub hidden: bool,
    // TODO: skipped preview
    /// The number of comment replies to this submission.
    pub num_comments: u64,
    /// The URL to the link thumbnail. This is "self" if this is a self post, or "default" if
    /// a thumbnail is not available.
    pub thumbnail: String,
    /// The Reddit ID for the subreddit where this was posted, **including the leading `t5_`**.
    pub subreddit_id: String,
    /// This is `true` if the score is being hidden.
    pub hide_score: bool,
    /// This is `false` if the submission is not edited and is the edit timestamp if it is edited.
    /// Access through the functions of `Submission` instead.
    pub edited: Value,
    /// The CSS class set for the link's flair (if available), otherwise `None`.
    pub link_flair_css_class: Option<String>,
    /// The CSS class set for the author's flair (if available). If there is no flair, this is
    /// `None`.
    pub author_flair_css_class: Option<String>,
    /// The number of downvotes (fuzzed; see `score` for further explanation)
    pub downs: i64,
    /// The number of upvotes (fuzzed; see `score` for further explanation)
    pub ups: i64,
    // TODO: skipped secure_media_embed
    /// True if the logged-in user has saved this submission.
    pub saved: bool,
    /// The reason for the post removal, if you are a moderator **and** this post has been
    /// removed.
    pub removal_reason: Option<String>,
    // TODO: skipped post_hint
    /// This is `true` if this submission is stickied (an 'annoucement' thread)
    pub stickied: bool,
    // TODO: skipped from
    /// This is `true` if this is a self post.
    pub is_self: bool,
    // TODO: skipped from_id
    /// The permanent, long link for this submission.
    pub permalink: String,
    /// This is `true` if the submission has been locked by a moderator, and no replies can be
    /// made.
    pub locked: bool,
    /// The full 'Thing ID', consisting of a 'kind' and a base-36 identifier. The valid kinds are:
    /// - t1_ - Comment
    /// - t2_ - Account
    /// - t3_ - Link
    /// - t4_ - Message
    /// - t5_ - Subreddit
    /// - t6_ - Award
    /// - t8_ - PromoCampaign
    pub name: String,
    /// A timestamp of the time when the post was created, in the logged-in user's **local**
    /// time.
    pub created: i64,
    /// The linked URL, if this is a link post.
    pub url: Option<String>,
    /// The text of the author's flair, if present. Can be an empty string if the flair is present
    /// but contains no text.
    pub author_flair_text: Option<String>,
    /// This is `true` if the post is from a quarantined subreddit.
    pub quarantine: bool,
    /// The title of the post.
    pub title: String,
    /// A timestamp of the time when the post was created, in **UTC**.
    pub created_utc: i64,
    /// Indicates whether the user has used a special flag for themselves, e.g. [M] or [A].
    /// Possible values:
    /// - None - Normal user
    /// - Some("moderator") - [M]
    /// - Some("admin") - [A]
    /// - Some("special") - other special 'distinguishes' e.g. [Δ]
    pub distinguished: Option<String>,
    // TODO: skipped mod_reports
    /// This is `true` if the user has visited this link.
    pub visited: bool,
    /// The number of reports, if the user is a moderator of this subreddit.
    pub num_reports: Option<u64>
}