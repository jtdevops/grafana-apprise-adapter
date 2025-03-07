# grafana-apprise-adapter
Send [grafana](https://grafana.com/docs/grafana/latest/alerting/notifications/) alerts to [apprise](https://github.com/caronc/apprise) for notifications

![CI](https://github.com/RealOrangeOne/grafana-apprise-adapter/workflows/CI/badge.svg)

## Configuration

Environment variables:
- `$APPRISE_URL`: Base URL for [apprise API](https://github.com/caronc/apprise-api/). **required** (e.g http://apprise:8000)
- `$APPRISE_TAGS`: Comma-separated list of tags to filter notifications (e.g. "user1,user2"). Defaults to "all"
- `$PORT`: Port to listen on, defaults to `5000`
- `$WORKERS`: Worker processes to run. Defaults to 1. If you need more, you might be doing something wrong.

## Usage

### Endpoints

- `POST /notify/{key}`: Send notification to Apprise. The `key` parameter should match your Apprise configuration.
  - Query Parameters:
    - `tag`: Optional. Override the default tags set by `APPRISE_TAGS` environment variable.
  - Example: `POST /notify/your-key?tag=user1,user2`
  - Note: Communicates with Apprise API using JSON format (`Content-Type: application/json`)

- `GET /health`: Health check endpoint

### Notification Types

Grafana alert states are automatically mapped to Apprise notification types as follows:
- `Ok` → `success`
- `Paused` → `info`
- `Alerting` → `failure`
- `Pending` → `info`
- `NoData` → `warning`
