# Security notes (frontend)

The project-wide security policy — supported versions, the reporting channel and
the response process — lives in the root [SECURITY.md](../SECURITY.md). Report
vulnerabilities through GitHub private vulnerability reporting only:

* **Report:** https://github.com/cipansor/leptos-studio/security/advisories/new
* **Advisories:** https://github.com/cipansor/leptos-studio/security/advisories

Do not use public issues, pull requests, email or any other channel.

This file records the frontend-specific things worth knowing when assessing the
app.

## Security considerations

### Client-side storage

* Project data is stored in browser LocalStorage.
* That data is accessible to any script running on the same origin.
* Do not store sensitive information in projects.

### Custom components

* Custom components use HTML templates.
* Input is sanitised, but review custom HTML before adding it.
* Avoid custom components from untrusted sources.

### Export functionality

* Exported code is generated from user input and should be reviewed and
  validated before production use.

## Best practices for users

1. Keep your browser updated.
2. Only import project JSON from trusted sources.
3. Review custom components before adding them.
4. Serve the app over HTTPS in production.
5. Clear browser storage when working with sensitive data.
