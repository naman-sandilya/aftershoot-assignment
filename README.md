# Image Metadata Extractor

### Functionality
- Create a server using Rocket.
- Server takes in a image as a request.
- Response from server is metadata of the image.
- Response is also stored in sqlite database.

### How to run - 
- Clone the Repo
- In 'server' directory, build using `cargo build` and then `cargo run`
- In 'client' directory, build using `cargo buid` and then `cargo run <image_path>`

Currently, this supports metadata extractor for .tiff files.

### Current development status - 
- Multiple routes are setup, currently 'upload' route is managed at client.
- Error Handling at server end needs to be handled.
- SQLite DB queries are handled properly and data is contructed in a struct.



