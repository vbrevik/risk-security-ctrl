# Section 01 Code Review Interview

## Auto-fixes Applied

### Issue 1: CORS allow_headers(Any) with allow_credentials(true)
**Action:** Auto-fix — use `AllowHeaders::mirror_request()`

### Issue 2: CORS allow_methods(Any) with allow_credentials(true)
**Action:** Auto-fix — use explicit method list

## User Decisions

### Issue 3: File permission check is advisory, not enforced
**Decision:** Keep as warning (user chose forgiving behavior for dev environments)
