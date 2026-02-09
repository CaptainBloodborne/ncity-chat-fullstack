<script setup>
import { useRouter } from 'vue-router'
import { ref, onMounted, computed } from 'vue'
import defaultAvatar from '../assets/avatar.png'
import { fetchApi } from '../utils/http'

const chats = ref([])
const isLoading = ref(true)
const error = ref(null)

const router = useRouter()

async function fetchChats() {
  try {
    isLoading.value = true
    error.value = null

    const response = await fetchApi('/api/chats')

    if (!response.ok) {

      throw new Error(`Server Error: ${response.status}`)

    }

    chats.value = await response.json()
  } catch (err) {
    error.value = err.message
  } finally {
    isLoading.value = false
  }
}

const searchQuery = ref('')
const sortKey = ref('name') // 'name' or 'users'
const sortOrder = ref('asc') // 'asc' or 'desc'


const filteredAndSortedChats = computed(() => {
  let result = chats.value.filter(chat => {
    return chat.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      chat.location.toLowerCase().includes(searchQuery.value.toLowerCase())
  })

  result.sort((a, b) => {

    let valA = a[sortKey.value]
    let valB = b[sortKey.value]

    if (typeof valA === 'string') {
      valA = valA.toLowerCase()
      valB = valB.toLowerCase()
    }

    if (valA < valB) return sortOrder.value === 'asc' ? -1 : 1
    if (valA > valB) return sortOrder.value === 'asc' ? 1 : -1
    return 0
  })

  return result
})

onMounted(() => {
  fetchChats()
})
</script>

<template>
  <div class="p-6 max-w-2xl auto">
    <h1 class="text-3xl font-bold text-gray-800">Сообщества и чаты Нижнего Новгорода</h1>
  </div>

  <div v-if="isLoading" class="text-gray-500 animate-pulse">
    Загрузка списка чатов...
  </div>

  <div v-else>

  <div class="flex gap-4">
    <input v-model="searchQuery" placeholder="Поиск чата..." class="border p-2 rounded w-full max-w-sm" />

    <select v-model="sortKey" class="border p-2 rounded">
      <option value="name">Сортировка по имени</option>
      <option value="users">Сортировка по популярности</option>
    </select>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-3 gap-6">

    <div v-for="chat in filteredAndSortedChats" :key="chat.id"
      class="bg-white p-6 rounded-lg shadow hover:shadow-lg transition border border-gray-100">
      <div class="flex justify-between items-start">
        <h3 class="font-bold text-lg">{{ chat.name }}</h3>
        <span class="bg-indigo-100 text-indigo-800 text-xs px-2 py-1 rounded-full">
          {{ chat.users_count }} users
        </span>
      </div>

      <p class="text-gray-500 mt-2 text-sm flex items-center gap-1">
        📍 {{ chat.location }}
      </p>

      <p class="text-black mt-2 text-sm flex items-center gap-1">
        {{ chat.description}}
      </p>

    </div>
  </div>
  </div>

  <div class="text-red-500 bg-red-50 p-4 rounded border border-red-200">
    <button @click="fetchChats" class="mt-2 text-sm underline">Обновить</button>
  </div>

</template>