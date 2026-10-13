<script lang="ts">
  // The minimal error page: the status and its message, nothing else. The
  // layout renders error pages bare.
  import { page } from '$app/state';

  /** The message of each error status. */
  const MESSAGES: Record<number, string> = {
    401: 'Unauthorized',
    402: 'Payment Required',
    403: 'Forbidden',
    404: 'Not Found',
    405: 'Method Not Allowed',
    419: 'Page Expired',
    429: 'Too Many Requests',
    500: 'Server Error',
    503: 'Service Unavailable',
  };

  let message = $derived(MESSAGES[page.status] ?? 'Server Error');
</script>

<svelte:head>
  <title>{message}</title>
</svelte:head>

<div
  class="relative flex min-h-screen justify-center bg-white text-black antialiased sm:items-center sm:pt-0"
  role="main"
>
  <div class="mx-auto max-w-xl sm:px-6 lg:px-8">
    <div class="flex items-center pt-8 sm:justify-start sm:pt-0">
      <h1 class="border-r border-gray-400 px-4 text-lg text-black">
        {page.status}
      </h1>

      <div class="ml-4 text-lg text-black">
        {message}
      </div>
    </div>
  </div>
</div>
