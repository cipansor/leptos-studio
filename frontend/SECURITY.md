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

Saved projects live on the **backend**, not in the browser. Both the manual save
and the auto-save path call `ProjectManager::save_project`, which POSTs to
`/api/projects`; the backend writes the result to `projects.json` on the host
running it (`DATA_FILE` overrides the path). Treat that file as the authoritative
copy and protect it like any other data store.

**LocalStorage** holds a narrower and more miscellaneous set of things:

* settings (`leptos_studio_settings`) and sidebar widths;
* project snapshots recorded by the Git panel. When no backend project is
  selected the Git panel uses a LocalStorage backend, which stores full
  `Project` snapshots under `leptos_studio_git_repo`;
* the legacy canvas key (`leptos_studio_canvas`). It is only read, once, to
  migrate an old browser-only project to the backend, and is then deleted — it is
  no longer written.

Anything running on the same origin can read all of this, so do not put sensitive
information in a project.

Two things that are *not* persisted, despite storage keys existing for them:
custom library components are held in memory for the session only, and
`leptos_studio_project` is a dead constant with no writer. Do not assume either
survives a reload.

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
5. Deleting a project is a backend operation — clearing browser storage does not
   remove the copy in `projects.json`.
6. Custom library components are session-only; re-add them after a reload rather
   than relying on them being restored.
