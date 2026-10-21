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

### Where project data lives

Project data is not stored in one place, and the two places have different
exposure:

* **Browser LocalStorage** holds settings, sidebar widths, custom components and
  a copy of the project being edited. Anything running on the same origin can
  read it, so do not put sensitive information in a project.
* **The backend API** persists projects on the server: the frontend saves through
  `POST /api/projects`, and the backend writes them to `projects.json` on the
  host running it (`DATA_FILE` overrides the path). In the full stack a saved
  project therefore exists on the server disk as well as in the browser — treat
  the server file as the authoritative copy and protect it like any other data
  store. LocalStorage is a client cache, not the source of truth.

### Deploying the backend

The defaults target local development and should be changed before the backend is
reachable from an untrusted network:

* It binds `0.0.0.0:3000`, so it listens on every interface.
* CORS defaults to `Any` unless `CORS_ORIGIN` is set, which lets any origin call
  the API.
* The project endpoints carry no authentication of their own; put the server
  behind an authenticating reverse proxy, or keep it on a trusted network.

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
4. Serve the app over HTTPS in production, and do not expose the backend's
   default `0.0.0.0` / permissive-CORS configuration to an untrusted network.
5. Remember that clearing browser storage only clears the client copy; a project
   saved through the API also lives on the server, and deleting it there is a
   separate step.
6. Delete projects you no longer need from both places if they hold anything
   sensitive.
