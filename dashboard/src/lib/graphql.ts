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
      messagesInbound
      messagesOutbound
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
      messagesInbound
      messagesOutbound
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
  messagesInbound?: number | null
  messagesOutbound?: number | null
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

