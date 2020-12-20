const NOT_FOUND_HTML_PAGE: &str = "<html><body>
  <h1>uh oh, we couldn't find that document</h1>
  <p>
    probably, this would be served from the file system or
    included with `include_bytes!`
  </p>
</body></html>";

const INTERNAL_SERVER_ERROR_HTML_PAGE: &str = "<html><body>
  <h1>whoops! it's not you, it's us</h1>
  <p>
    we're very sorry, but something seems to have gone wrong on our end
  </p>
</body></html>";
