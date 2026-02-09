const headers = {
    'Content-Type': 'application/json',
}

export async function fetchApi(url, options = {}) {
    const config = {
        ...options,
        headers: {
            ...headers,
            ...options.headers
        },
        credentials: 'include',
    }

    const response = await fetch(url, config)

    switch (response.status) {
        case 401:
        case 402:
        case 403:
            const isLoginPage = window.location.pathname === '/login'

            if (!isLoginPage) {
                window.location.href = '/login'
            }

            return response

    }

    return response

}