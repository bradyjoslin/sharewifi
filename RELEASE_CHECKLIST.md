# Release Checklist

Heavily references [How to Publish your Rust project on Homebrew](https://federicoterzi.com/blog/how-to-publish-your-rust-project-on-homebrew/) by Federico Terzi.

1. Increment the version number in `Cargo.toml`.

1. Create a release build:

   ```sh
   cargo build --release
   ```

1. Create a tar archive

   ```sh
   cd target/release
   tar -czf sharewifi-mac.tar.gz sharewifi
   ```

1. Generate SHA Hash

   ```sh
   shasum -a 256 sharewifi-mac.tar.gz
   ```

1. On GitHub go to Releases and create a new release. Create a tag for the next version number. Upload the tar file generated previously. Click Publish Release.

1. Get the download URL for the tar file under the assets section of the release page. i.e. https://github.com/bradyjoslin/sharewifi/releases/download/v0.1.5/sharewifi-mac.tar.gz

1. Go to https://github.com/bradyjoslin/homebrew-sharewifi/blob/master/Formula/sharewifi.rb and create a revision, updating the version number, download URL, and sha hash to reflect the latest.
