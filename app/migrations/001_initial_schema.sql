-- Drop old tables if they exist (fresh start for schema refactor)
DROP TABLE IF EXISTS messages;
DROP TABLE IF EXISTS peer_connections;

-- Create peer_connections table
CREATE TABLE peer_connections (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    connection_id INTEGER NOT NULL UNIQUE,
    client_addr TEXT NOT NULL,
    target_addr TEXT NOT NULL,
    connected_at TEXT NOT NULL,
    disconnected_at TEXT,
    bytes_inbound INTEGER,
    bytes_outbound INTEGER
);

-- Create indexes for peer_connections
CREATE INDEX idx_connection_id ON peer_connections(connection_id);
CREATE INDEX idx_connected_at ON peer_connections(connected_at);

-- Create messages table
CREATE TABLE messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    connection_id INTEGER NOT NULL,
    timestamp TEXT NOT NULL,
    direction TEXT NOT NULL,
    source_peer TEXT NOT NULL,
    destination_peer TEXT NOT NULL,
    message_type TEXT NOT NULL,
    payload_size INTEGER NOT NULL,
    description TEXT NOT NULL,
    FOREIGN KEY (connection_id) REFERENCES peer_connections(connection_id) ON DELETE CASCADE
);

-- Create indexes for messages
CREATE INDEX idx_messages_connection_id ON messages(connection_id);
CREATE INDEX idx_messages_timestamp ON messages(timestamp);
CREATE INDEX idx_messages_type ON messages(message_type);

