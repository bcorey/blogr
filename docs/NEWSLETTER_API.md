# Newsletter REST API Documentation

The Blogr Newsletter system includes an optional REST API server that allows external tools and services to interact with the newsletter system programmatically. This API provides endpoints for subscriber management, newsletter operations, and system statistics.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Authentication](#authentication)
3. [API Endpoints](#api-endpoints)
4. [Data Models](#data-models)
5. [Error Handling](#error-handling)
6. [Rate Limiting](#rate-limiting)
7. [Examples](#examples)
8. [SDKs and Libraries](#sdks-and-libraries)

## Getting Started

### Starting the API Server

```bash
# Start API server on default port 3001
blogr newsletter api-server

# Start on custom port with authentication
blogr newsletter api-server --port 8080 --api-key your-secret-key

# Start with custom host and disable CORS
blogr newsletter api-server --host 0.0.0.0 --port 3001 --no-cors
```

### Base URL

When running locally, the API is available at:
```
http://127.0.0.1:3001
```

### Content Type

All API requests and responses use JSON:
```
Content-Type: application/json
```

## Authentication

API authentication is optional but recommended for production use.

### API Key Authentication

When an API key is configured, include it in the `Authorization` header:

```bash
curl -H "Authorization: Bearer your-secret-key" \
     http://127.0.0.1:3001/subscribers
```

### No Authentication

If no API key is configured, requests can be made without authentication:

```bash
curl http://127.0.0.1:3001/subscribers
```

## API Endpoints

### Health Check

#### GET /health

Check if the API server is running.

**Response:**
```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "service": "blogr-newsletter-api",
    "version": "0.2.0"
  },
  "error": null,
  "timestamp": "2024-09-25T10:30:00Z"
}
```

### Subscriber Management

#### GET /subscribers

List all subscribers with optional filtering and pagination.

**Query Parameters:**
- `status` (optional): Filter by status (`pending`, `approved`, `declined`)
- `limit` (optional): Maximum number of subscribers to return
- `offset` (optional): Number of subscribers to skip

**Example:**
```bash
curl "http://127.0.0.1:3001/subscribers?status=approved&limit=10&offset=0"
```

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "email": "user@example.com",
      "status": "approved",
      "subscribed_at": "2024-09-25T10:00:00Z",
      "approved_at": "2024-09-25T10:05:00Z",
      "source_email_id": "subscription-email-123",
      "notes": "Subscribed via website"
    }
  ],
  "error": null,
  "timestamp": "2024-09-25T10:30:00Z"
}
```

#### POST /subscribers

Create a new subscriber.

**Request Body:**
```json
{
  "email": "newuser@example.com",
  "status": "pending",
  "notes": "Added via API"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": 2,
    "email": "newuser@example.com",
    "status": "pending",
    "subscribed_at": "2024-09-25T10:30:00Z",
    "approved_at": null,
    "source_email_id": "api",
    "notes": "Added via API"
  },
  "error": null,
  "timestamp": "2024-09-25T10:30:00Z"
}
```

#### GET /subscribers/:email

Get a specific subscriber by email address.

**Example:**
```bash
curl "http://127.0.0.1:3001/subscribers/user@example.com"
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": 1,
    "email": "user@example.com",
    "status": "approved",
    "subscribed_at": "2024-09-25T10:00:00Z",
    "approved_at": "2024-09-25T10:05:00Z",
    "source_email_id": "subscription-email-123",
    "notes": "Subscribed via website"
  },
  "error": null,
  "timestamp": "2024-09-25T10:30:00Z"
}
```

#### PUT /subscribers/:email

Update a subscriber's information.

**Request Body:**
```json
{
  "status": "approved",
  "notes": "Updated via API"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": 1,
    "email": "user@example.com",
    "status": "approved",
    "subscribed_at": "2024-09-25T10:00:00Z",
    "approved_at": "2024-09-25T10:30:00Z",
    "source_email_id": "subscription-email-123",
    "notes": "Updated via API"
  },
  "error": null,
  "timestamp": "2024-09-25T10:30:00Z"
}
```

#### DELETE /subscribers/:email

Remove a subscriber.

**Example:**
```bash
curl -X DELETE "http://127.0.0.1:3001/subscribers/user@example.com"
```

**Response:**
```json
{
  "success": true,
  "data": null,
  "error": null,
  "timestamp": "2024-09-25T10:30:00Z"
}
```

### Statistics

#### GET /stats

Get newsletter statistics.

**Response:**
```json
{
  "success": true,
  "data": {
    "total_subscribers": 150,
    "approved_subscribers": 120,
    "pending_subscribers": 25,
    "declined_subscribers": 5
  },
  "error": null,
  "timestamp": "2024-09-25T10:30:00Z"
}
```

### Export

#### GET /export

Export subscribers (same as GET /subscribers but intended for bulk export).

**Query Parameters:**
- `status` (optional): Filter by status
- `limit` (optional): Maximum number to export
- `offset` (optional): Starting offset

**Response:** Same format as GET /subscribers

### Newsletter Operations

#### POST /newsletter/send

Send a custom newsletter (Not yet implemented).

**Request Body:**
```json
{
  "subject": "Weekly Newsletter",
  "content": "# This Week's Updates\n\nHere's what's new...",
  "test_mode": false,
  "test_email": null
}
```

#### POST /newsletter/send-latest

Send newsletter with the latest blog post (Not yet implemented).

#### POST /newsletter/preview

Preview a newsletter without sending (Not yet implemented).

### Import

#### POST /import

Import subscribers from external data (Not yet implemented).

**Request Body:**
```json
{
  "source": "csv",
  "data": "email,name,status\nuser1@example.com,User One,subscribed\nuser2@example.com,User Two,subscribed",
  "preview_only": false,
  "column_mappings": {
    "email": "email",
    "name": "name",
    "status": "status"
  }
}
```

## Data Models

### Subscriber

```json
{
  "id": 1,
  "email": "user@example.com",
  "status": "pending" | "approved" | "declined",
  "subscribed_at": "2024-09-25T10:00:00Z",
  "approved_at": "2024-09-25T10:05:00Z" | null,
  "source_email_id": "subscription-email-123" | null,
  "notes": "Optional notes" | null
}
```

### API Response

All API responses follow this structure:

```json
{
  "success": true | false,
  "data": {} | [] | null,
  "error": "Error message" | null,
  "timestamp": "2024-09-25T10:30:00Z"
}
```

### Statistics

```json
{
  "total_subscribers": 150,
  "approved_subscribers": 120,
  "pending_subscribers": 25,
  "declined_subscribers": 5
}
```

## Error Handling

### HTTP Status Codes

- `200 OK`: Successful request
- `201 Created`: Resource created successfully
- `400 Bad Request`: Invalid request data
- `401 Unauthorized`: Authentication required or invalid
- `404 Not Found`: Resource not found
- `409 Conflict`: Resource already exists (e.g., duplicate email)
- `500 Internal Server Error`: Server error

### Error Response

```json
{
  "success": false,
  "data": null,
  "error": "Detailed error message",
  "timestamp": "2024-09-25T10:30:00Z"
}
```

### Common Errors

#### 404 Not Found
```json
{
  "success": false,
  "data": null,
  "error": "Subscriber not found",
  "timestamp": "2024-09-25T10:30:00Z"
}
```

#### 409 Conflict
```json
{
  "success": false,
  "data": null,
  "error": "Subscriber with this email already exists",
  "timestamp": "2024-09-25T10:30:00Z"
}
```

#### 500 Internal Server Error
```json
{
  "success": false,
  "data": null,
  "error": "Database connection failed",
  "timestamp": "2024-09-25T10:30:00Z"
}
```

## Rate Limiting

The API includes built-in rate limiting to prevent abuse:

- **Default Limit**: 100 requests per minute per IP
- **Headers**: Rate limit information is included in response headers:
  - `X-RateLimit-Limit`: Maximum requests per window
  - `X-RateLimit-Remaining`: Remaining requests in current window
  - `X-RateLimit-Reset`: Time when the rate limit resets

When rate limit is exceeded:
```json
{
  "success": false,
  "data": null,
  "error": "Rate limit exceeded. Please try again later.",
  "timestamp": "2024-09-25T10:30:00Z"
}
```

## Examples

### JavaScript/Node.js

```javascript
const axios = require('axios');

const API_BASE = 'http://127.0.0.1:3001';
const API_KEY = 'your-secret-key';

const headers = API_KEY ? {
  'Authorization': `Bearer ${API_KEY}`,
  'Content-Type': 'application/json'
} : {
  'Content-Type': 'application/json'
};

// List subscribers
async function getSubscribers() {
  try {
    const response = await axios.get(`${API_BASE}/subscribers`, { headers });
    return response.data.data;
  } catch (error) {
    console.error('Error fetching subscribers:', error.response.data);
  }
}

// Create subscriber
async function createSubscriber(email, status = 'pending') {
  try {
    const response = await axios.post(`${API_BASE}/subscribers`, {
      email,
      status,
      notes: 'Added via API'
    }, { headers });
    return response.data.data;
  } catch (error) {
    console.error('Error creating subscriber:', error.response.data);
  }
}

// Update subscriber
async function updateSubscriber(email, status, notes) {
  try {
    const response = await axios.put(`${API_BASE}/subscribers/${email}`, {
      status,
      notes
    }, { headers });
    return response.data.data;
  } catch (error) {
    console.error('Error updating subscriber:', error.response.data);
  }
}

// Get statistics
async function getStats() {
  try {
    const response = await axios.get(`${API_BASE}/stats`, { headers });
    return response.data.data;
  } catch (error) {
    console.error('Error fetching stats:', error.response.data);
  }
}

// Usage examples
(async () => {
  // Get all approved subscribers
  const approvedSubscribers = await axios.get(`${API_BASE}/subscribers?status=approved`, { headers });
  console.log('Approved subscribers:', approvedSubscribers.data.data.length);

  // Create a new subscriber
  const newSubscriber = await createSubscriber('newuser@example.com', 'pending');
  console.log('Created subscriber:', newSubscriber.email);

  // Get statistics
  const stats = await getStats();
  console.log('Newsletter stats:', stats);
})();
```

### Python

```python
import requests
import json

API_BASE = 'http://127.0.0.1:3001'
API_KEY = 'your-secret-key'  # or None if not using authentication

headers = {
    'Content-Type': 'application/json'
}

if API_KEY:
    headers['Authorization'] = f'Bearer {API_KEY}'

class NewsletterAPI:
    def __init__(self, base_url=API_BASE, api_key=API_KEY):
        self.base_url = base_url
        self.headers = {'Content-Type': 'application/json'}
        if api_key:
            self.headers['Authorization'] = f'Bearer {api_key}'

    def get_subscribers(self, status=None, limit=None, offset=None):
        """Get list of subscribers"""
        params = {}
        if status:
            params['status'] = status
        if limit:
            params['limit'] = limit
        if offset:
            params['offset'] = offset

        response = requests.get(f'{self.base_url}/subscribers', 
                              headers=self.headers, params=params)
        response.raise_for_status()
        return response.json()['data']

    def create_subscriber(self, email, status='pending', notes=None):
        """Create a new subscriber"""
        data = {
            'email': email,
            'status': status
        }
        if notes:
            data['notes'] = notes

        response = requests.post(f'{self.base_url}/subscribers',
                               headers=self.headers, json=data)
        response.raise_for_status()
        return response.json()['data']

    def get_subscriber(self, email):
        """Get specific subscriber by email"""
        response = requests.get(f'{self.base_url}/subscribers/{email}',
                              headers=self.headers)
        response.raise_for_status()
        return response.json()['data']

    def update_subscriber(self, email, status=None, notes=None):
        """Update subscriber information"""
        data = {}
        if status:
            data['status'] = status
        if notes:
            data['notes'] = notes

        response = requests.put(f'{self.base_url}/subscribers/{email}',
                              headers=self.headers, json=data)
        response.raise_for_status()
        return response.json()['data']

    def delete_subscriber(self, email):
        """Delete a subscriber"""
        response = requests.delete(f'{self.base_url}/subscribers/{email}',
                                 headers=self.headers)
        response.raise_for_status()
        return response.json()

    def get_stats(self):
        """Get newsletter statistics"""
        response = requests.get(f'{self.base_url}/stats', headers=self.headers)
        response.raise_for_status()
        return response.json()['data']

# Usage examples
if __name__ == '__main__':
    api = NewsletterAPI()
    
    # Get all subscribers
    subscribers = api.get_subscribers()
    print(f'Total subscribers: {len(subscribers)}')
    
    # Get only approved subscribers
    approved = api.get_subscribers(status='approved')
    print(f'Approved subscribers: {len(approved)}')
    
    # Create a new subscriber
    new_subscriber = api.create_subscriber(
        email='python-user@example.com',
        status='pending',
        notes='Added via Python API'
    )
    print(f'Created subscriber: {new_subscriber["email"]}')
    
    # Get statistics
    stats = api.get_stats()
    print('Newsletter Statistics:')
    print(f'  Total: {stats["total_subscribers"]}')
    print(f'  Approved: {stats["approved_subscribers"]}')
    print(f'  Pending: {stats["pending_subscribers"]}')
    print(f'  Declined: {stats["declined_subscribers"]}')
```

### cURL Examples

```bash
# Health check
curl http://127.0.0.1:3001/health

# List all subscribers
curl http://127.0.0.1:3001/subscribers

# List approved subscribers with pagination
curl "http://127.0.0.1:3001/subscribers?status=approved&limit=10&offset=0"

# Create a new subscriber
curl -X POST http://127.0.0.1:3001/subscribers \
  -H "Content-Type: application/json" \
  -d '{"email":"newuser@example.com","status":"pending","notes":"Added via cURL"}'

# Get specific subscriber
curl http://127.0.0.1:3001/subscribers/newuser@example.com

# Update subscriber status
curl -X PUT http://127.0.0.1:3001/subscribers/newuser@example.com \
  -H "Content-Type: application/json" \
  -d '{"status":"approved","notes":"Approved via API"}'

# Delete subscriber
curl -X DELETE http://127.0.0.1:3001/subscribers/newuser@example.com

# Get statistics
curl http://127.0.0.1:3001/stats

# With authentication
curl -H "Authorization: Bearer your-secret-key" \
     http://127.0.0.1:3001/subscribers
```

## SDKs and Libraries

Currently, no official SDKs are available, but the API is designed to be simple to integrate with any HTTP client library.

### Recommended Libraries

**JavaScript/Node.js:**
- axios
- fetch API
- node-fetch

**Python:**
- requests
- httpx
- aiohttp

**Go:**
- net/http
- resty

**Rust:**
- reqwest
- ureq

**PHP:**
- Guzzle
- cURL

## Future Enhancements

The following features are planned for future releases:

1. **Newsletter Operations**: Full implementation of send, preview, and template endpoints
2. **Import/Export**: Complete CSV and JSON import/export functionality
3. **Webhooks**: Configurable webhooks for subscriber events
4. **Authentication**: OAuth2 and JWT token support
5. **Bulk Operations**: Batch create, update, and delete operations
6. **Search**: Full-text search across subscribers
7. **Analytics**: Detailed analytics and reporting endpoints
8. **File Upload**: Direct file upload for imports
9. **WebSocket**: Real-time updates via WebSocket connections
10. **GraphQL**: Optional GraphQL endpoint for complex queries

## Support

For API support and questions:

1. Check the [Blogr repository](https://github.com/your-org/blogr) for issues and documentation
2. Join our community discussions
3. Submit bug reports and feature requests via GitHub Issues

## Changelog

### v0.2.0 (Current)
- Initial API implementation
- Basic subscriber CRUD operations
- Statistics endpoint
- Health check endpoint
- Optional API key authentication
- CORS support

### Future Versions
- Newsletter operations
- Import/export functionality
- Advanced authentication
- Webhook support
