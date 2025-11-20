import { GraphQLClient } from 'graphql-request'

// Determine the GraphQL endpoint based on environment
const getGraphQLEndpoint = () => {
  // In development, you might need to point to a different port
  // In production (when served by the Rust server), use the same origin
  if (import.meta.env.DEV) {
    // During development with Vite, assume the backend is on port 6789
    return import.meta.env.VITE_GRAPHQL_URL || 'http://localhost:6789/graphql'
  }
  // In production, use the same origin
  return `${window.location.origin}/graphql`
}

// Create a GraphQL client pointing to the local server
export const graphqlClient = new GraphQLClient(getGraphQLEndpoint(), {
  headers: {
    'Content-Type': 'application/json',
  },
})

// GraphQL Queries
export const PEER_CONNECTIONS_QUERY = `
  query GetPeerConnections {
    peerConnections {
      id
      connectionId
      clientAddr
      targetAddr
      connectedAt
      disconnectedAt
      bytesInbound
      bytesOutbound
    }
  }
`

export const ACTIVE_CONNECTIONS_QUERY = `
  query GetActiveConnections {
    activeConnections {
      id
      connectionId
      clientAddr
      targetAddr
      connectedAt
      disconnectedAt
      bytesInbound
      bytesOutbound
    }
  }
`

export const MESSAGES_BY_CONNECTION_QUERY = `
  query GetMessagesByConnection($connectionId: Int!) {
    messagesByConnection(connectionId: $connectionId) {
      id
      connectionId
      timestamp
      direction
      sourcePeer
      destinationPeer
      messageType
      payloadSize
      description
    }
  }
`

export const RECENT_MESSAGES_QUERY = `
  query GetRecentMessages($limit: Int = 100) {
    recentMessages(limit: $limit) {
      id
      connectionId
      timestamp
      direction
      sourcePeer
      destinationPeer
      messageType
      payloadSize
      description
    }
  }
`

export const MESSAGES_BY_PEER_QUERY = `
  query GetMessagesByPeer($peerAddr: String!) {
    messagesByPeer(peerAddr: $peerAddr) {
      id
      connectionId
      timestamp
      direction
      sourcePeer
      destinationPeer
      messageType
      payloadSize
      description
    }
  }
`

export const CONNECTION_STATS_QUERY = `
  query GetConnectionStats {
    connectionStats {
      totalConnections
      activeConnections
      totalBytesInbound
      totalBytesOutbound
      totalMessagesInbound
      totalMessagesOutbound
    }
  }
`

// TypeScript types
export interface PeerConnection {
  id: number
  connectionId: number
  clientAddr: string
  targetAddr: string
  connectedAt: string
  disconnectedAt?: string | null
  bytesInbound?: number | null
  bytesOutbound?: number | null
}

export interface Message {
  id: number
  connectionId: number
  timestamp: string
  direction: string
  sourcePeer: string
  destinationPeer: string
  messageType: string
  payloadSize: number
  description: string
}

export interface ConnectionStats {
  totalConnections: number
  activeConnections: number
  totalBytesInbound: number
  totalBytesOutbound: number
  totalMessagesInbound: number
  totalMessagesOutbound: number
}

export interface PeerConnectionsResponse {
  peerConnections: PeerConnection[]
}

export interface ActiveConnectionsResponse {
  activeConnections: PeerConnection[]
}

export interface ConnectionStatsResponse {
  connectionStats: ConnectionStats
}

export interface MessagesByConnectionResponse {
  messagesByConnection: Message[]
}

export interface RecentMessagesResponse {
  recentMessages: Message[]
}

export interface MessagesByPeerResponse {
  messagesByPeer: Message[]
}

