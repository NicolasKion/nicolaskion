<script lang="ts">
  // The <head> of the page, built by seo.ts.
  import { page } from '$app/state';
  import { pageHead } from '#lib/seo.ts';

  let { appName }: { appName: string } = $props();

  let head = $derived(pageHead({ appName, origin: page.url.origin }));
</script>

<svelte:head>
  <title>{head.title}</title>
  {#each head.meta as tag (tag.property ?? tag.name)}
    <meta {...tag} />
  {/each}
  {#each head.links as link (link.rel + link.href)}
    <link {...link} />
  {/each}
</svelte:head>
