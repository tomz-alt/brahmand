#!/usr/bin/env python3
"""
chdb HTTP Server for Brahmand
Provides a ClickHouse-compatible HTTP interface using chdb
"""

import json
import sys
from http.server import BaseHTTPRequestHandler, HTTPServer
from urllib.parse import parse_qs, urlparse
import chdb

class ChdbHTTPHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        """Handle GET requests (query parameter)"""
        try:
            parsed = urlparse(self.path)
            params = parse_qs(parsed.query)

            if 'query' in params:
                query = params['query'][0]
                self.execute_query(query)
            else:
                self.send_response(400)
                self.end_headers()
                self.wfile.write(b"Missing query parameter")
        except Exception as e:
            self.send_error_response(str(e))

    def do_POST(self):
        """Handle POST requests (query in body)"""
        try:
            content_length = int(self.headers.get('Content-Length', 0))
            query = self.rfile.read(content_length).decode('utf-8')

            if not query:
                self.send_response(400)
                self.end_headers()
                self.wfile.write(b"Empty query")
                return

            self.execute_query(query)
        except Exception as e:
            self.send_error_response(str(e))

    def execute_query(self, query):
        """Execute query using chdb and return results"""
        try:
            print(f"\n[chdb] Executing query: {query[:100]}...")

            # Execute query with chdb
            result = chdb.query(query, "JSONEachRow")

            # Get the result as bytes
            result_data = result.data()

            # Send response
            self.send_response(200)
            self.send_header('Content-Type', 'text/plain; charset=utf-8')
            self.end_headers()

            if isinstance(result_data, bytes):
                self.wfile.write(result_data)
            else:
                self.wfile.write(str(result_data).encode('utf-8'))

            print(f"[chdb] Query executed successfully, returned {len(result_data)} bytes")

        except Exception as e:
            print(f"[chdb] Error executing query: {e}")
            self.send_error_response(str(e))

    def send_error_response(self, error_msg):
        """Send error response"""
        self.send_response(500)
        self.send_header('Content-Type', 'text/plain')
        self.end_headers()
        error_json = json.dumps({"error": error_msg})
        self.wfile.write(error_json.encode('utf-8'))

    def log_message(self, format, *args):
        """Custom log format"""
        sys.stdout.write(f"[chdb-server] {format % args}\n")

def run_server(port=8123):
    """Start the chdb HTTP server"""
    server_address = ('', port)
    httpd = HTTPServer(server_address, ChdbHTTPHandler)

    print(f"""
╔══════════════════════════════════════════╗
║   chdb HTTP Server for Brahmand         ║
╚══════════════════════════════════════════╝

🚀 Server running on: http://0.0.0.0:{port}
📊 Database: chdb (embedded ClickHouse)
🔗 Compatible with: ClickHouse HTTP protocol

Ready to accept queries from Brahmand!
Press Ctrl+C to stop.
""")

    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("\n\n[chdb-server] Shutting down...")
        httpd.server_close()

if __name__ == '__main__':
    port = 8123
    if len(sys.argv) > 1:
        port = int(sys.argv[1])

    run_server(port)
