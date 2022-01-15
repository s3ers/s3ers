//! [GET /](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjects.html)

use s3ers_api::s3ers_api;

s3ers_api! {
    metadata: {
        description: "Return some or all of the objects in a bucket.",
        method: GET,
        name: "list_objects",
        path: "/",
        authentication: None,
        // scope: Bucket,
    }

    request: {
        /// A character used to group keys.
        #[s3ers_api(query)]
        pub delimiter: String,

        /// Where to start listing from.
        #[s3ers_api(query)]
        pub marker: String,

        /// Maximum number of keys returned in the response.
        #[s3ers_api(query)]
        pub max_keys: u64,

        /// Limits the response to keys that begin with the specified prefix.
        #[s3ers_api(query)]
        pub prefix: String,
    }

    response: {}
}
