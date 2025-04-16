# Development research for Copywriter

## Synopsis

**Copywriter** is a static website generator geared towards collaborative use by teams of copywriters.

## Problem that this solves

This project aims to be the statically generated solution to the same sort of experience that software like Wordpress and Wikimedia provide for small and medium sized self-hosted websites.

A common approach to developing small and medium websites is to customize Wordpress or Wikimedia for a client. The web-developer "skins" the HTML (structure) and CSS (theme) to match the client's branding and content requirements. Most importantly, the client then has access to a very simple interface for editing content live.

The problem with this approach is that it requires a technology stack to always be available online just to *view* the website. The client inherits this maintenance burden.

With dynamic systems, a webpage is never *just an HTML file*, even though it probably should be:
- A database server needs to be running. 
- A cache server needs to be running.
- Each webpage request from a browser invokes either a database query or a cache server query. 
- A server-side scripting language and environment must be available.
- A server-side scripting language package manager must be available.
- All server-side scripting language package dependencies must be available.
- Database server dies: Content is lost.
- Cache server dies: The website is slow.
- Server updates the scripting language improperly: Website goes offline.
- Restoring content from backup: Web developer required. 
- Transferring to a different hosting service:
  - The host must be running the *same* technology stack.
  - Web developer required. 
  
On the other hand, a static website *is* just a bunch of files and directories that can be zipped up, transferred, and unzipped onto just about any webserver without any other requirements. Copy and paste.

Static websites, however, are not easily edited, even just for content.

## Technologies
- Git
- TOML
- Handlebars
- Markdown
