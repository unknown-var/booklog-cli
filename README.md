# booklog-cli
A minimalist command-line interface designed to track your reading history. Use it to log when you finished a book, assign a rating, and store short reviews for future reference.
## configuration
The application works out of the box using internal defaults. If you wish to customize the behavior, create a configuration file at: `~/.config/booklog-cli/config.toml`

### default settings
If no configuration file is found, the application defaults to the values shown below. You only need to create this file if you want to change these specific paths or your preferred text editor.
```toml
[general]
# The base directory for your data
data_path = "~"

# The folder name where logs will be stored
data_dir_name = ".booklog-cli-data"

# The filename for your book database
book_file = "books.csv"

# The editor used for writing book reviews
text_editor = "vim"
```

