# NodeScope Changes Summary

## Overview
Refactored the system to track individual messages between peers with full peer information, instead of just aggregate message counts.

## Key Changes

### 1. Database Schema Migration
- **Location**: `app/migrations/001_initial_schema.sql`
- Created proper migrations directory structure
- **peer_connections table**: Removed redundant `messages_inbound` and `messages_outbound` columns (can be queried from messages table)
- **messages table**: New table with 1-to-many relationship to peer_connections
  - Tracks individual messages with source/destination peer information
  - Includes message type enum (version, verack, tx, block, etc.)
  - Records timestamp, direction, payload size, and description
  - Foreign key relationship with CASCADE delete

### 2. Backend Changes

#### Database Layer (`app/src/db.rs`)
- Added `MessageType` enum with 30+ Bitcoin P2P message types
- Added `Message` struct to represent individual messages
- New methods:
  - `record_message()`: Save individual messages with peer info
  - `get_messages_by_connection()`: Get all messages for a connection
  - `get_recent_messages()`: Get last N messages across all connections
  - `get_messages_by_peer()`: Get all messages involving a specific peer
- Updated `get_connection_stats()` to query message counts from messages table instead of denormalized fields
- Simplified `record_disconnection()` to only track bytes (not message counts)

#### Protocol Layer (`proxy/src/bitcoin_protocol.rs`)
- Added `message_type()` method to `BitcoinMessage` that extracts type directly from `NetworkMessage` payload
- Uses rust-bitcoin's NetworkMessage enum as source of truth

#### Connection Handler (`proxy/src/connection.rs`)
- Updated to record every individual message in the database with:
  - Connection ID
  - Direction (inbound/outbound)
  - Source peer address
  - Destination peer address
  - Message type from NetworkMessage
  - Payload size
  - Description
- Properly identifies source/destination based on direction:
  - **Inbound**: client → target (Bitcoin Core)
  - **Outbound**: target (Bitcoin Core) → client

### 3. GraphQL API Updates (`server/src/graphql/schema.rs`)
- Removed `messages_inbound` and `messages_outbound` from `PeerConnection` type
- Added new `Message` GraphQL type
- New queries:
  - `messagesByConnection(connectionId)`: Get all messages for a specific connection
  - `recentMessages(limit)`: Get recent messages (default 100)
  - `messagesByPeer(peerAddr)`: Get all messages involving a peer

### 4. Frontend Updates

#### GraphQL Client (`dashboard/src/lib/graphql.ts`)
- Removed message count fields from `PeerConnection` interface
- Added `Message` TypeScript interface
- New GraphQL queries:
  - `MESSAGES_BY_CONNECTION_QUERY`
  - `RECENT_MESSAGES_QUERY`
  - `MESSAGES_BY_PEER_QUERY`

#### Composables (`dashboard/src/composables/useMessages.ts`)
- New Vue composable for fetching messages
- Three hooks:
  - `useRecentMessages()`: Auto-refreshing recent messages
  - `useMessagesByConnection()`: Messages for specific connection
  - `useMessagesByPeer()`: Messages for specific peer

#### UI Components
- **MessagesView.vue**: New view showing live message stream
  - Real-time message table with auto-refresh (5 seconds)
  - Color-coded message types:
    - Yellow: Handshake (version, verack)
    - Red: Data (tx, block, inv, getdata)
    - Blue: Address (addr, addrv2, getaddr)
    - Gray: Sync (headers, getheaders, getblocks)
  - Direction badges (inbound/outbound)
  - Peer addresses with truncation for long addresses
  - Human-readable timestamps and byte sizes
- **Updated App.vue**: Added "Messages" link to navigation
- **Updated router**: Added `/messages` route

## Benefits

1. **Full Visibility**: Every message exchange is now recorded with complete peer information
2. **Direction Clarity**: Clear tracking of which peer sent what to whom
3. **Proper Data Modeling**: 1-to-many relationship instead of denormalized counts
4. **Queryable History**: Can analyze communication patterns by connection, peer, time, or message type
5. **Type Safety**: Uses rust-bitcoin's NetworkMessage as source of truth for message types

## Database Relationship

```
peer_connections (1) ─────< messages (many)
     │                          │
     ├─ connection_id           ├─ connection_id (FK)
     ├─ client_addr             ├─ source_peer
     ├─ target_addr             ├─ destination_peer
     ├─ connected_at            ├─ timestamp
     ├─ disconnected_at         ├─ direction
     ├─ bytes_inbound           ├─ message_type
     └─ bytes_outbound          ├─ payload_size
                                └─ description
```

## Direction Semantics

- **Inbound**: Traffic from client to Bitcoin Core (messages your node receives from peers)
- **Outbound**: Traffic from Bitcoin Core to client (messages your node sends to peers)

This ensures that on a brand new node, you'll see more inbound than outbound initially as peers send version/addr messages to help you sync.

