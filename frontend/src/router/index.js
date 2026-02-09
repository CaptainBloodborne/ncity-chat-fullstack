import { createRouter, createWebHistory } from "vue-router"
import HomeView from "../views/HomeView.vue"
import LoginView from "../views/LoginView.vue"
import { useAuth } from "../composables/useAuth"
import AdminView from "../views/AdminView.vue"
import RegisterView from "../views/RegisterView.vue"

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'home',
            component: HomeView
        },
        {
            path: '/login',
            name: 'login',
            component: LoginView
        },
        {
            path: '/admin',
            name: 'admin',
            component: AdminView,
            meta: { requiresAdmin: true }
        },
        {
            path: '/register',
            name: 'register',
            component: RegisterView
        },
    ]
})

router.beforeEach((to, from, next) => {
    const { isAuthenticated, isAdmin } = useAuth()

    const publicPages = ['login', 'register']
    const authRequired = !publicPages.includes(to.name)

    if (authRequired && !isAuthenticated.value) {
        next({ name: 'login' })

        return
    }

    if (to.meta.requiresAdmin) {
        if (!isAdmin.value) {
            next({ name: 'home' })

            return
        }
    }

    if (to.name === 'login' && isAuthenticated.value) {
        console.log(isAuthenticated.value)
        next({ name: 'home' })

        return
    }

    next()
})

export default router