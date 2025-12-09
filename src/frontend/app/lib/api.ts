const apiBase = (import.meta.env.VITE_API_BASE_URL ?? "").replace(/\/$/, "")

const apiUrl = (path: string) => `${apiBase}${path}`

type ApiUser = {
  uuid: string
  username: string
  email: string | null
  first_name: string | null
  last_name: string | null
}

type ApiError = { message?: string }

async function parseJson<T>(response: Response) {
  const contentType = response.headers.get("content-type")
  if (contentType && contentType.includes("application/json")) {
    return (await response.json()) as T
  }

  return null
}

function buildAuthHeaders(request?: Request) {
  const cookie = request?.headers.get("cookie")
  if (!cookie) return undefined

  return { cookie }
}

export async function loginUser({
  username,
  password,
}: {
  username: string
  password: string
}) {
  const response = await fetch(apiUrl("/api/login"), {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    credentials: "include",
    body: JSON.stringify({ username, password }),
  })

  const payload = await parseJson<ApiUser | ApiError>(response)

  if (!response.ok) {
    const message = (payload as ApiError | null)?.message ?? "Unable to log in"
    throw new Error(message)
  }

  return payload as ApiUser
}

export async function fetchCurrentUser(request?: Request) {
  const response = await fetch(apiUrl("/api/me"), {
    method: "GET",
    headers: buildAuthHeaders(request),
    credentials: "include",
  })

  if (response.status === 401 || response.status === 403) {
    return null
  }

  const payload = await parseJson<ApiUser | ApiError>(response)

  if (!response.ok || !payload || !("username" in payload)) {
    const message = (payload as ApiError | null)?.message ?? "Unable to load session"
    throw new Error(message)
  }

  return payload as ApiUser
}
