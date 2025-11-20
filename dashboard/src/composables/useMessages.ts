import { ref, onMounted, onUnmounted } from 'vue'
import {
  graphqlClient,
  RECENT_MESSAGES_QUERY,
  MESSAGES_BY_CONNECTION_QUERY,
  MESSAGES_BY_PEER_QUERY,
  type Message,
  type RecentMessagesResponse,
  type MessagesByConnectionResponse,
  type MessagesByPeerResponse,
} from '../lib/graphql'

export function useRecentMessages(limit: number = 100, autoRefresh = false, refreshInterval = 5000) {
  const messages = ref<Message[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  let intervalId: number | null = null

  const fetchMessages = async () => {
    loading.value = true
    error.value = null

    try {
      const data = await graphqlClient.request<RecentMessagesResponse>(RECENT_MESSAGES_QUERY, {
        limit,
      })
      messages.value = data.recentMessages
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to fetch messages'
      console.error('Error fetching messages:', err)
    } finally {
      loading.value = false
    }
  }

  onMounted(() => {
    fetchMessages()

    if (autoRefresh) {
      intervalId = window.setInterval(fetchMessages, refreshInterval)
    }
  })

  onUnmounted(() => {
    if (intervalId !== null) {
      clearInterval(intervalId)
    }
  })

  return {
    messages,
    loading,
    error,
    refetch: fetchMessages,
  }
}

export function useMessagesByConnection(connectionId: number) {
  const messages = ref<Message[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  const fetchMessages = async () => {
    loading.value = true
    error.value = null

    try {
      const data = await graphqlClient.request<MessagesByConnectionResponse>(
        MESSAGES_BY_CONNECTION_QUERY,
        { connectionId }
      )
      messages.value = data.messagesByConnection
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to fetch messages'
      console.error('Error fetching messages:', err)
    } finally {
      loading.value = false
    }
  }

  onMounted(() => {
    fetchMessages()
  })

  return {
    messages,
    loading,
    error,
    refetch: fetchMessages,
  }
}

export function useMessagesByPeer(peerAddr: string) {
  const messages = ref<Message[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  const fetchMessages = async () => {
    loading.value = true
    error.value = null

    try {
      const data = await graphqlClient.request<MessagesByPeerResponse>(MESSAGES_BY_PEER_QUERY, {
        peerAddr,
      })
      messages.value = data.messagesByPeer
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to fetch messages'
      console.error('Error fetching messages:', err)
    } finally {
      loading.value = false
    }
  }

  onMounted(() => {
    fetchMessages()
  })

  return {
    messages,
    loading,
    error,
    refetch: fetchMessages,
  }
}

