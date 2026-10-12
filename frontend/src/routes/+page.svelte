<script lang="ts">
  import PageHead from '#lib/components/page-head.svelte';
  import ProjectsPage from '#lib/components/projects-page.svelte';
  import type { PageProps } from './$types';

  let { data }: PageProps = $props();
</script>

<PageHead appName={data.name} />
<ProjectsPage {data} />
