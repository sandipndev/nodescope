import { ref, onMounted, onUnmounted } from 'vue'
import { graphqlClient, ACTIVE_CONNECTIONS_QUERY, CONNECTION_STATS_QUERY, type PeerConnection, type ConnectionStats, type ActiveConnectionsResponse, type ConnectionStatsResponse } from '../lib/graphql'

export function usePeerConnections(autoRefresh = true, refreshInterval = 5000) {
  const connections = ref<PeerConnection[]>([])
  const stats = ref<ConnectionStats | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)
  let intervalId: number | null = null

  const fetchConnections = async () => {
    loading.value = true
    error.value = null
    
    try {
      const [connectionsData, statsData] = await Promise.all([
        graphqlClient.request<ActiveConnectionsResponse>(ACTIVE_CONNECTIONS_QUERY),
        graphqlClient.request<ConnectionStatsResponse>(CONNECTION_STATS_QUERY)
      ])
      
      connections.value = connectionsData.activeConnections
      stats.value = statsData.connectionStats
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to fetch peer connections'
      console.error('Error fetching peer connections:', err)
    } finally {
      loading.value = false
    }
  }

  const startAutoRefresh = () => {
    if (autoRefresh && !intervalId) {
      intervalId = window.setInterval(fetchConnections, refreshInterval)
    }
  }

  const stopAutoRefresh = () => {
    if (intervalId) {
      clearInterval(intervalId)
      intervalId = null
    }
  }

  onMounted(() => {
    fetchConnections()
    startAutoRefresh()
  })

  onUnmounted(() => {
    stopAutoRefresh()
  })

  return {
    connections,
    stats,
    loading,
    error,
    refetch: fetchConnections,
    stopAutoRefresh,
    startAutoRefresh
  }
}

