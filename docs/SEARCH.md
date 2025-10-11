# Search Feature

Blogr includes a powerful client-side full-text search feature powered by MiniSearch. Search is enabled by default and works entirely in the browser without requiring a server.

## Features

- **Full-text search** - Search across post titles, tags, and content
- **Real-time results** - Instant search results as you type
- **Keyboard navigation** - Use `/` to focus search, arrow keys to navigate, Enter to open
- **Smart excerpts** - Highlighted search terms in result snippets
- **Lazy loading** - Search index and library loaded only when needed (configurable)
- **Responsive design** - Works seamlessly on desktop and mobile

## How to Use

Once your site is built and deployed, readers can:

1. **Quick search** - Use the search bar in the site header
2. **Keyboard shortcut** - Press `/` to focus the search input
3. **Navigate results** - Use arrow keys to navigate, Enter to open a result
4. **Tag search** - Click on tags in search results to search by that tag
5. **Clear search** - Press Escape to clear and hide results

## Configuration Options

The search feature can be customized in your `blogr.toml`:

```toml
[search]
# Enable or disable search entirely
enabled = true

# Fields to include in search index
fields = ["title", "tags", "content"]

# Paths to exclude from indexing (relative to posts directory)
exclude = ["drafts/", "private/"]

# Maximum characters to include from post content
max_content_chars = 2000

# Number of words to include in search result excerpts
excerpt_words = 30

# Whether to minify the search index JSON (recommended for production)
minify = true

# Whether to lazy-load search assets (improves initial page load)
lazy_load = true

# Whether to remove common English stopwords (experimental)
remove_stopwords = false

# Custom field boost weights for search scoring
[search.field_boosts]
title = 5.0    # Matches in titles are weighted 5x
tags = 3.0     # Matches in tags are weighted 3x  
content = 1.0  # Matches in content have base weight
```

## Performance

- **Index size**: Typically 10-50KB for small to medium blogs
- **Load time**: < 100ms for initial search setup with lazy loading
- **Search speed**: < 10ms for typical queries on 100+ posts
- **Browser support**: Works in all modern browsers (IE11+)

## Customization

Search UI can be customized by modifying theme templates:
- Search input styling via CSS classes `.search-input`, `.search-results`
- Result item layout in theme templates
- Search behavior by modifying the generated `search.js`

## Troubleshooting

**Search not working?**
- Ensure `search.enabled = true` in your config
- Check that `search_index.json` exists in your `dist/` folder after build
- Verify JavaScript is enabled in the browser

**Large index files?**
- Reduce `max_content_chars` to limit content per post
- Enable `minify = true` to compress the JSON
- Consider adding more paths to `exclude` for draft content
