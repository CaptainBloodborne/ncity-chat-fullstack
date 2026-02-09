<script setup>
import { ref, onMounted, computed } from 'vue'
import { fetchApi } from '../utils/http'

// 1. STATE FOR "CREATE CHAT" FORM
const newChat = ref({
    name: '', users_count: 0, location: '', description: ''
})
const isSubmitting = ref(false)

async function createChat() {
    isSubmitting.value = true
    try {
        // Fake API call
        await fetchApi('/api/admin/chat', { method: 'POST', body: JSON.stringify(newChat.value) })

        alert(`City Chat "${newChat.value.name}" created!`)
        newChat.value = {
            name: '', users_count: 0, location: '', description: ''
        }
    } catch (e) {
        alert("Error creating chat")
    } finally {
        isSubmitting.value = false
    }
}

// 2. STATE FOR "USER LIST"
const users = ref([])
const isLoading = ref(false)
const error = ref(null)

async function fetchUsers() {
    try {
        isLoading.value = true
        error.value = null

        const response = await fetchApi('/api/users')

        // Check if the backend actually said "OK" (Status 200-299)
        if (!response.ok) {
            throw new Error(`Server Error: ${response.status}`)
        }

        // Parse JSON
        users.value = await response.json()

    } catch (err) {
        // If network fails or server errors
        error.value = err.message
    } finally {
        // This runs whether we succeeded or failed
        isLoading.value = false
    }
}

async function handleDeleteUser(userId) {
    try {
        if (!confirm(`Are you sure you want to delete user #${userId}?`)) {
            return
        }
        isLoading.value = true
        error.value = null

        const response = await fetchApi('/api/admin/user', {
            method: 'DELETE',
            body: JSON.stringify({
                id: userId,
            })
        })

        // Check if the backend actually said "OK" (Status 200-299)
        if (!response.ok) {
            throw new Error(`Server Error: ${response.status}`)
        }

        console.log("Deleting ID:", userId, typeof userId)
        console.log("First User ID:", users.value[0]?.id, typeof users.value[0]?.id)

        users.value = users.value.filter(user => user.id != userId)

    } catch (err) {
        // If network fails or server errors
        error.value = err.message
    } finally {
        // This runs whether we succeeded or failed
        isLoading.value = false
    }
}

const sortKey = ref('name')
const sortOrder = ref('asc')

function sortBy(key) {
    if (sortKey.value === key) {
        sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc'
    } else {

        sortKey.value = key
        sortOrder.value = 'asc'
    }
}

const sortedUsers = computed(() => {

    return [...users.value].sort((a, b) => {

        let valA = a[sortKey.value]
        let valB = b[sortKey.value]

        if (typeof valA === 'string') valA = valA.toLowerCase()
        if (typeof valB === 'string') valB = valB.toLowerCase()

        let modifier = sortOrder.value === 'asc' ? 1 : -1

        if (valA < valB) return -1 * modifier
        if (valA > valB) return 1 * modifier
        return 0
    })
})

onMounted(() => {
    fetchUsers()
})
</script>

<template>
    <div class="max-w-4xl mx-auto space-y-8">

        <h1 class="text-3xl font-bold text-gray-800">Панель управления</h1>

        <section class="bg-white p-6 rounded-lg shadow-md border border-gray-100">
            <h2 class="text-xl font-bold mb-4 flex items-center gap-2">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                    stroke="currentColor" class="w-6 h-6 text-indigo-600">
                    <path stroke-linecap="round" stroke-linejoin="round"
                        d="M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
                </svg>
                Добавить новый чат
            </h2>

            <form @submit.prevent="createChat" class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">Название чата</label>
                    <input v-model="newChat.name" type="text" placeholder="Мой чат" required
                        class="w-full p-2 border border-gray-300 rounded focus:ring-2 focus:ring-indigo-500 outline-none" />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">Количество пользователей</label>
                    <input v-model="newChat.users_count" type="number" placeholder="0" required
                        class="w-full p-2 border border-gray-300 rounded focus:ring-2 focus:ring-indigo-500 outline-none" />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">Местоположение</label>
                    <input v-model="newChat.location" type="text" placeholder="Городской район"
                        class="w-full p-2 border border-gray-300 rounded focus:ring-2 focus:ring-indigo-500 outline-none" />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">Описание</label>
                    <input v-model="newChat.description" type="text" placeholder="Краткое описание..."
                        class="w-full p-2 border border-gray-300 rounded focus:ring-2 focus:ring-indigo-500 outline-none" />
                </div>
                <div class="md:col-span-2">
                    <button type="submit" :disabled="isSubmitting"
                        class="bg-indigo-600 text-white px-4 py-2 rounded hover:bg-indigo-700 transition disabled:bg-gray-400">
                        {{ isSubmitting ? 'Creating...' : 'Create Chat' }}
                    </button>
                </div>
            </form>
        </section>

        <section class="bg-white p-6 rounded-lg shadow-md border border-gray-100">
            <h2 class="text-xl font-bold mb-4 flex items-center gap-2">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                    stroke="currentColor" class="w-6 h-6 text-indigo-600">
                    <path stroke-linecap="round" stroke-linejoin="round"
                        d="M15 19.128a9.38 9.38 0 0 0 2.625.372 9.337 9.337 0 0 0 4.121-.952 4.125 4.125 0 0 0-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 0 1 8.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0 1 11.964-3.07M12 6.375a3.375 3.375 0 1 1-6.75 0 3.375 3.375 0 0 1 6.75 0Zm8.25 2.25a2.625 2.625 0 1 1-5.25 0 2.625 2.625 0 0 1 5.25 0Z" />
                </svg>
                Пользователи
            </h2>

            <div class="overflow-x-auto">
                <table class="min-w-full text-left text-sm whitespace-nowrap">
                    <thead class="uppercase tracking-wider border-b-2 border-gray-200 bg-gray-50">
                        <tr>
                            <th @click="sortBy('id')" scope="col"
                                class="px-6 py-4 cursor-pointer hover:bg-gray-100 select-none transition-colors">
                                <div class="flex items-center gap-1">
                                    ID
                                    <span v-if="sortKey === 'id'" class="text-indigo-600">
                                        {{ sortOrder === 'asc' ? '↑' : '↓' }}
                                    </span>
                                </div>
                            </th>

                            <th @click="sortBy('display_name')" scope="col"
                                class="px-6 py-4 cursor-pointer hover:bg-gray-100 select-none transition-colors">
                                <div class="flex items-center gap-1">
                                    Имя
                                    <span v-if="sortKey === 'display_name'" class="text-indigo-600">
                                        {{ sortOrder === 'asc' ? '↑' : '↓' }}
                                    </span>
                                </div>
                            </th>

                            <th @click="sortBy('email')" scope="col"
                                class="px-6 py-4 cursor-pointer hover:bg-gray-100 select-none transition-colors">
                                <div class="flex items-center gap-1">
                                    Email
                                    <span v-if="sortKey === 'email'" class="text-indigo-600">
                                        {{ sortOrder === 'asc' ? '↑' : '↓' }}
                                    </span>
                                </div>
                            </th>

                            <th @click="sortBy('role')" scope="col"
                                class="px-6 py-4 cursor-pointer hover:bg-gray-100 select-none transition-colors">
                                <div class="flex items-center gap-1">
                                    Роль
                                    <span v-if="sortKey === 'role'" class="text-indigo-600">
                                        {{ sortOrder === 'asc' ? '↑' : '↓' }}
                                    </span>
                                </div>
                            </th>

                            <th scope="col" class="px-6 py-4">Действие</th>
                        </tr>
                    </thead>

                    <tbody>
                        <tr v-for="u in sortedUsers" :key="u.id" class="border-b hover:bg-gray-50 transition">
                            <td class="px-6 py-4 font-mono text-gray-500">#{{ u.id }}</td>
                            <td class="px-6 py-4 font-medium text-gray-900">{{ u.display_name }}</td>
                            <td class="px-6 py-4 text-gray-500">{{ u.email }}</td>
                            <td class="px-6 py-4">
                                <span class="px-2 py-1 rounded text-xs font-bold uppercase"
                                    :class="u.role === 'admin' ? 'bg-purple-100 text-purple-700' : 'bg-gray-100 text-gray-600'">
                                    {{ u.role }}
                                </span>
                            </td>
                            <td class="px-6 py-4">
                                <button @click="handleDeleteUser(u.id)"
                                    class="text-red-500 hover:text-red-700 font-semibold">Удалить</button>
                            </td>
                        </tr>
                    </tbody>
                </table>

                <div v-if="sortedUsers.length === 0" class="text-center py-4 text-gray-500">
                    Пользователи не найдены.
                </div>
            </div>
        </section>

    </div>
</template>